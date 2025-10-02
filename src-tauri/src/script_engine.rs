use crate::manager::ChargerManager;
use deno_core::{extension, op2, JsRuntime, OpState, PollEventLoopOptions, RuntimeOptions};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

// ============================================================================
// Ops Definition - 使用 #[op2] 宏
// ============================================================================

/// Console log
#[op2]
#[string]
fn op_console_log(#[string] msg: String) -> Result<String, deno_core::error::AnyError> {
    println!("[Script] {}", msg);
    Ok("".to_string())
}

/// Sleep for specified milliseconds
#[op2(async)]
async fn op_sleep(#[smi] ms: u32) -> Result<(), deno_core::error::AnyError> {
    tokio::time::sleep(tokio::time::Duration::from_millis(ms as u64)).await;
    Ok(())
}

/// Check if script should stop
#[op2(async)]
async fn op_should_stop(state: Rc<RefCell<OpState>>) -> Result<bool, deno_core::error::AnyError> {
    let script_id = {
        let state = state.borrow();
        state.borrow::<String>().clone()
    };

    let running_scripts = {
        let state = state.borrow();
        state.borrow::<Arc<RwLock<HashMap<String, bool>>>>().clone()
    };

    let scripts = running_scripts.read().await;
    Ok(!scripts.get(&script_id).copied().unwrap_or(false))
}

/// Start a charger
#[op2(async)]
async fn op_start_charger(
    state: Rc<RefCell<OpState>>,
    #[string] charger_id: String,
) -> Result<(), deno_core::error::AnyError> {
    let manager = {
        let state = state.borrow();
        state.borrow::<Arc<ChargerManager>>().clone()
    };

    manager
        .start_charger(&charger_id)
        .await
        .map_err(|e| deno_core::error::generic_error(e))
}

/// Stop a charger
#[op2(async)]
async fn op_stop_charger(
    state: Rc<RefCell<OpState>>,
    #[string] charger_id: String,
) -> Result<(), deno_core::error::AnyError> {
    let manager = {
        let state = state.borrow();
        state.borrow::<Arc<ChargerManager>>().clone()
    };

    manager
        .stop_charger(&charger_id)
        .await
        .map_err(|e| deno_core::error::generic_error(e))
}

/// Start charging session
#[op2(async)]
async fn op_start_charging(
    state: Rc<RefCell<OpState>>,
    #[string] charger_id: String,
) -> Result<(), deno_core::error::AnyError> {
    let manager = {
        let state = state.borrow();
        state.borrow::<Arc<ChargerManager>>().clone()
    };

    manager
        .start_charging(&charger_id)
        .await
        .map_err(|e| deno_core::error::generic_error(e))
}

/// Stop charging session
#[op2(async)]
async fn op_stop_charging(
    state: Rc<RefCell<OpState>>,
    #[string] charger_id: String,
) -> Result<(), deno_core::error::AnyError> {
    let manager = {
        let state = state.borrow();
        state.borrow::<Arc<ChargerManager>>().clone()
    };

    manager
        .stop_charging(&charger_id)
        .await
        .map_err(|e| deno_core::error::generic_error(e))
}

/// Set charging power
#[op2(async)]
async fn op_set_power(
    state: Rc<RefCell<OpState>>,
    #[string] charger_id: String,
    power: f64,
) -> Result<(), deno_core::error::AnyError> {
    let manager = {
        let state = state.borrow();
        state.borrow::<Arc<ChargerManager>>().clone()
    };

    manager
        .set_power(&charger_id, power)
        .await
        .map_err(|e| deno_core::error::generic_error(e))
}

/// Get charger state
#[op2(async)]
#[serde]
async fn op_get_charger_state(
    state: Rc<RefCell<OpState>>,
    #[string] charger_id: String,
) -> Result<serde_json::Value, deno_core::error::AnyError> {
    let manager = {
        let state = state.borrow();
        state.borrow::<Arc<ChargerManager>>().clone()
    };

    let charger_state = manager
        .get_charger_state(&charger_id)
        .await
        .map_err(|e| deno_core::error::generic_error(e))?;

    serde_json::to_value(&charger_state).map_err(|e| deno_core::error::generic_error(e.to_string()))
}

// ============================================================================
// Extension Definition - 使用 extension! 宏
// ============================================================================

extension!(
    charger_extension,
    ops = [
        op_console_log,
        op_sleep,
        op_should_stop,
        op_start_charger,
        op_stop_charger,
        op_start_charging,
        op_stop_charging,
        op_set_power,
        op_get_charger_state,
    ],
    esm_entry_point = "ext:charger_extension/charger.js",
    esm = ["charger.js" = {
        source = r#"
// Console API
globalThis.console = {
    log: (msg) => Deno.core.ops.op_console_log(String(msg))
};

// Sleep function
globalThis.sleep = (ms) => Deno.core.ops.op_sleep(ms);

// Charger API
globalThis.charger = {
    shouldStop: () => Deno.core.ops.op_should_stop(),
    startCharger: (id) => Deno.core.ops.op_start_charger(id),
    stopCharger: (id) => Deno.core.ops.op_stop_charger(id),
    startCharging: (id) => Deno.core.ops.op_start_charging(id),
    stopCharging: (id) => Deno.core.ops.op_stop_charging(id),
    setPower: (id, power) => Deno.core.ops.op_set_power(id, power),
    getChargerState: async (id) => {
        const json = await Deno.core.ops.op_get_charger_state(id);
        return JSON.parse(json);
    }
};
            "#
    }]
);

// ============================================================================
// Script Engine
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptResult {
    pub success: bool,
    pub message: String,
}

enum ScriptEngineCommand {
    Execute {
        script_id: String,
        script_code: String,
        response: tokio::sync::oneshot::Sender<Result<ScriptResult, String>>,
    },
    Stop {
        script_id: String,
    },
    IsRunning {
        script_id: String,
        response: tokio::sync::oneshot::Sender<bool>,
    },
}

#[derive(Clone)]
pub struct ScriptEngine {
    tx: mpsc::UnboundedSender<ScriptEngineCommand>,
}

impl ScriptEngine {
    pub fn new(manager: Arc<ChargerManager>) -> Result<Self, String> {
        let (tx, mut rx) = mpsc::unbounded_channel::<ScriptEngineCommand>();

        // 在单独的线程中运行 tokio runtime 和脚本引擎
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build script engine runtime");
            rt.block_on(async move {
                let running_scripts = Arc::new(RwLock::new(HashMap::<String, bool>::new()));

                while let Some(cmd) = rx.recv().await {
                    match cmd {
                        ScriptEngineCommand::Execute {
                            script_id,
                            script_code,
                            response,
                        } => {
                            let manager = manager.clone();
                            let running_scripts = running_scripts.clone();
                            let script_id_clone = script_id.clone();

                            // 标记脚本为运行中
                            {
                                let mut scripts = running_scripts.write().await;
                                scripts.insert(script_id.clone(), true);
                            }

                            // 执行脚本
                            let result = Self::execute_script_internal(
                                script_id_clone.clone(),
                                script_code,
                                manager,
                                running_scripts.clone(),
                            )
                            .await;

                            // 清理运行状态
                            {
                                let mut scripts = running_scripts.write().await;
                                scripts.remove(&script_id_clone);
                            }

                            let _ = response.send(result);
                        }
                        ScriptEngineCommand::Stop { script_id } => {
                            let mut scripts = running_scripts.write().await;
                            scripts.insert(script_id, false);
                        }
                        ScriptEngineCommand::IsRunning {
                            script_id,
                            response,
                        } => {
                            let scripts = running_scripts.read().await;
                            let is_running = scripts.get(&script_id).copied().unwrap_or(false);
                            let _ = response.send(is_running);
                        }
                    }
                }
            });
        });

        Ok(ScriptEngine { tx })
    }

    async fn execute_script_internal(
        script_id: String,
        script_code: String,
        manager: Arc<ChargerManager>,
        running_scripts: Arc<RwLock<HashMap<String, bool>>>,
    ) -> Result<ScriptResult, String> {
        // 创建一个 LocalSet 来运行 !Send 的 JsRuntime
        let local = tokio::task::LocalSet::new();

        local
            .run_until(async move {
                // 创建 JsRuntime
                let mut runtime = JsRuntime::new(RuntimeOptions {
                    extensions: vec![charger_extension::init_ops()],
                    ..Default::default()
                });

                // 初始化 OpState
                {
                    let op_state = runtime.op_state();
                    let mut state = op_state.borrow_mut();
                    state.put(manager);
                    state.put(running_scripts);
                    state.put(script_id.clone());
                }

                // 注册 JavaScript 全局工具函数
                runtime
                    .execute_script(
                        "charger:bootstrap",
                        r#"
// Console API
globalThis.console = {
    log: (msg) => Deno.core.ops.op_console_log(String(msg ?? '')),
};

// Sleep helper（返回 Promise）
globalThis.sleep = (ms) => Deno.core.ops.op_sleep(Number(ms ?? 0));

// Charger control helpers
globalThis.charger = {
    shouldStop: () => Deno.core.ops.op_should_stop().then(Boolean),
    startCharger: (id) => Deno.core.ops.op_start_charger(String(id ?? '')),
    stopCharger: (id) => Deno.core.ops.op_stop_charger(String(id ?? '')),
    startCharging: (id) => Deno.core.ops.op_start_charging(String(id ?? '')),
    stopCharging: (id) => Deno.core.ops.op_stop_charging(String(id ?? '')),
    setPower: (id, power) => Deno.core.ops.op_set_power(String(id ?? ''), Number(power ?? 0)),
    getChargerState: (id) => Deno.core.ops.op_get_charger_state(String(id ?? '')).then((json) =>
        typeof json === 'string' ? JSON.parse(json) : json
    ),
};
                    "#,
                    )
                    .map_err(|e| format!("Failed to bootstrap runtime: {e}"))?;

                // 包装脚本代码为异步函数
                let wrapped_code = format!(
                    r#"
                (async () => {{
                    {}
                }})()
                "#,
                    script_code
                );

                // 执行脚本
                let script_id_static: &'static str = Box::leak(script_id.clone().into_boxed_str());
                let result = match runtime.execute_script(script_id_static, wrapped_code) {
                    Ok(_) => {
                        // 运行事件循环
                        match runtime
                            .run_event_loop(PollEventLoopOptions::default())
                            .await
                        {
                            Ok(_) => Ok(ScriptResult {
                                success: true,
                                message: "Script executed successfully".to_string(),
                            }),
                            Err(e) => Err(format!("Event loop error: {}", e)),
                        }
                    }
                    Err(e) => Err(format!("Execution error: {}", e)),
                };

                result
            })
            .await
    }

    pub async fn execute_script(
        &self,
        script_id: String,
        script_code: String,
    ) -> Result<ScriptResult, String> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.tx
            .send(ScriptEngineCommand::Execute {
                script_id,
                script_code,
                response: tx,
            })
            .map_err(|e| format!("Failed to send command: {}", e))?;

        rx.await
            .map_err(|e| format!("Failed to receive response: {}", e))?
    }

    pub async fn stop_script(&self, script_id: String) -> Result<(), String> {
        self.tx
            .send(ScriptEngineCommand::Stop { script_id })
            .map_err(|e| format!("Failed to send stop command: {}", e))
    }

    pub async fn is_script_running(&self, script_id: String) -> Result<bool, String> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.tx
            .send(ScriptEngineCommand::IsRunning {
                script_id,
                response: tx,
            })
            .map_err(|e| format!("Failed to send command: {}", e))?;

        rx.await
            .map_err(|e| format!("Failed to receive response: {}", e))
    }
}
