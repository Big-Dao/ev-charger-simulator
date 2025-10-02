/// Tauri Commands - 前后端通信接口
use crate::charger::ChargerConfig;
use crate::manager::ChargerManager;
use crate::script_engine::{ScriptEngine, ScriptResult};
use crate::state::{ChargerState, ChargerStatus};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

/// 充电桩命令
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChargerCommand {
    StartCharging,
    StopCharging,
    SetPower { power: f64 },
    Reset,
}

/// 获取充电桩列表
#[tauri::command]
pub async fn get_charger_list(
    manager: State<'_, Arc<ChargerManager>>,
) -> Result<Vec<ChargerState>, String> {
    Ok(manager.get_charger_list().await)
}

/// 启动单个充电桩
#[tauri::command]
pub async fn start_charger(
    charger_id: String,
    manager: State<'_, Arc<ChargerManager>>,
    engine: State<'_, ScriptEngine>,
) -> Result<(), String> {
    manager.start_charger(&charger_id).await?;
    let manager_arc = manager.inner().clone();
    let engine_clone = engine.inner().clone();
    auto_start_script_if_needed(&charger_id, manager_arc, engine_clone).await
}

/// 停止单个充电桩
#[tauri::command]
pub async fn stop_charger(
    charger_id: String,
    manager: State<'_, Arc<ChargerManager>>,
    engine: State<'_, ScriptEngine>,
) -> Result<(), String> {
    let manager_arc = manager.inner().clone();
    let engine_clone = engine.inner().clone();
    if let Some(entry) = manager_arc.get_charger_script(&charger_id).await {
        if entry.running {
            stop_charger_script_internal(&charger_id, manager_arc.clone(), engine_clone.clone())
                .await?;
        }
    }
    manager.stop_charger(&charger_id).await
}

/// 启动所有充电桩
#[tauri::command]
pub async fn start_all_chargers(
    manager: State<'_, Arc<ChargerManager>>,
    engine: State<'_, ScriptEngine>,
) -> Result<usize, String> {
    let count = manager.start_all().await?;
    let manager_arc = manager.inner().clone();
    let engine_clone = engine.inner().clone();

    let scripts = manager_arc.list_scripts().await;
    for (charger_id, entry) in scripts {
        if entry.auto_start && !entry.running {
            start_charger_script_internal(&charger_id, manager_arc.clone(), engine_clone.clone())
                .await?;
        }
    }

    Ok(count)
}

/// 停止所有充电桩
#[tauri::command]
pub async fn stop_all_chargers(
    manager: State<'_, Arc<ChargerManager>>,
    engine: State<'_, ScriptEngine>,
) -> Result<usize, String> {
    let manager_arc = manager.inner().clone();
    let engine_clone = engine.inner().clone();

    let scripts = manager_arc.list_scripts().await;
    for (charger_id, entry) in scripts {
        if entry.running {
            stop_charger_script_internal(&charger_id, manager_arc.clone(), engine_clone.clone())
                .await?;
        }
    }

    manager.stop_all().await
}

/// 获取充电桩状态
#[tauri::command]
pub async fn get_charger_status(
    charger_id: String,
    manager: State<'_, Arc<ChargerManager>>,
) -> Result<ChargerState, String> {
    manager.get_charger_state(&charger_id).await
}

/// 获取充电桩配置
#[tauri::command]
pub async fn get_charger_config(
    charger_id: String,
    manager: State<'_, Arc<ChargerManager>>,
) -> Result<ChargerConfig, String> {
    manager.get_charger_config(&charger_id).await
}

/// 更新充电桩配置
#[tauri::command]
pub async fn update_charger_config(
    charger_id: String,
    config: ChargerConfig,
    manager: State<'_, Arc<ChargerManager>>,
) -> Result<(), String> {
    manager.update_charger_config(&charger_id, config).await?;
    
    // 自动保存配置
    let manager_arc = manager.inner().clone();
    if let Err(e) = crate::config_loader::save_to_file(manager_arc).await {
        tracing::warn!("Failed to save config after updating charger: {}", e);
    }
    
    Ok(())
}

/// 发送充电桩命令
#[tauri::command]
pub async fn send_charger_command(
    charger_id: String,
    command: ChargerCommand,
    manager: State<'_, Arc<ChargerManager>>,
    engine: State<'_, ScriptEngine>,
) -> Result<(), String> {
    match command {
        ChargerCommand::StartCharging => manager.start_charging(&charger_id).await,
        ChargerCommand::StopCharging => manager.stop_charging(&charger_id).await,
        ChargerCommand::SetPower { power } => manager.set_power(&charger_id, power).await,
        ChargerCommand::Reset => {
            let manager_arc = manager.inner().clone();
            let engine_clone = engine.inner().clone();

            manager.stop_charging(&charger_id).await?;
            if let Some(entry) = manager_arc.get_charger_script(&charger_id).await {
                if entry.running {
                    stop_charger_script_internal(
                        &charger_id,
                        manager_arc.clone(),
                        engine_clone.clone(),
                    )
                    .await?;
                }
            }
            manager.stop_charger(&charger_id).await?;
            manager.start_charger(&charger_id).await?;
            auto_start_script_if_needed(&charger_id, manager_arc, engine_clone).await
        }
    }
}

/// 添加充电桩
#[tauri::command]
pub async fn add_charger(
    config: ChargerConfig,
    manager: State<'_, Arc<ChargerManager>>,
) -> Result<String, String> {
    let result = manager.add_charger(config).await?;
    
    // 自动保存配置
    let manager_arc = manager.inner().clone();
    if let Err(e) = crate::config_loader::save_to_file(manager_arc).await {
        tracing::warn!("Failed to save config after adding charger: {}", e);
    }
    
    Ok(result)
}

/// 移除充电桩
#[tauri::command]
pub async fn remove_charger(
    charger_id: String,
    manager: State<'_, Arc<ChargerManager>>,
    engine: State<'_, ScriptEngine>,
) -> Result<(), String> {
    let manager_arc = manager.inner().clone();
    let engine_clone = engine.inner().clone();

    if let Some(entry) = manager_arc.get_charger_script(&charger_id).await {
        if entry.running {
            stop_charger_script_internal(&charger_id, manager_arc.clone(), engine_clone.clone())
                .await?;
        }
        manager_arc.clear_charger_script(&charger_id).await?;
    }

    manager.remove_charger(&charger_id).await?;
    
    // 自动保存配置
    if let Err(e) = crate::config_loader::save_to_file(manager_arc).await {
        tracing::warn!("Failed to save config after removing charger: {}", e);
    }
    
    Ok(())
}

/// 批量创建充电桩
#[tauri::command]
pub async fn create_batch_chargers(
    count: usize,
    base_config: ChargerConfig,
    manager: State<'_, Arc<ChargerManager>>,
) -> Result<Vec<String>, String> {
    manager.create_batch(count, base_config).await
}

/// 获取统计信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    pub total_chargers: usize,
    pub online_count: usize,
    pub charging_count: usize,
    pub total_power: f64,
}

#[tauri::command]
pub async fn get_statistics(manager: State<'_, Arc<ChargerManager>>) -> Result<Statistics, String> {
    let states = manager.get_charger_list().await;

    let total_chargers = states.len();
    let online_count = states.iter().filter(|s| s.connected).count();
    let charging_count = states
        .iter()
        .filter(|s| s.status == ChargerStatus::Charging)
        .count();
    let total_power: f64 = states.iter().map(|s| s.current_power).sum();

    Ok(Statistics {
        total_chargers,
        online_count,
        charging_count,
        total_power,
    })
}

// ==================== 脚本管理 Commands ====================

/// 执行脚本
#[tauri::command]
pub async fn execute_script(
    script_id: String,
    script_code: String,
    engine: State<'_, ScriptEngine>,
) -> Result<ScriptResult, String> {
    engine.execute_script(script_id, script_code).await
}

/// 停止脚本
#[tauri::command]
pub async fn stop_script(script_id: String, engine: State<'_, ScriptEngine>) -> Result<(), String> {
    engine.stop_script(script_id).await
}

/// 检查脚本是否正在运行
#[tauri::command]
pub async fn is_script_running(
    script_id: String,
    engine: State<'_, ScriptEngine>,
) -> Result<bool, String> {
    engine.is_script_running(script_id).await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChargerScriptPayload {
    pub name: String,
    pub code: String,
    #[serde(default)]
    pub auto_start: bool,
}

/// 为充电桩配置脚本
#[tauri::command]
pub async fn set_charger_script(
    charger_id: String,
    payload: ChargerScriptPayload,
    manager: State<'_, Arc<ChargerManager>>,
    engine: State<'_, ScriptEngine>,
) -> Result<(), String> {
    manager
        .set_charger_script(&charger_id, payload.name, payload.code, payload.auto_start)
        .await?;

    if payload.auto_start {
        if let Ok(state) = manager.get_charger_state(&charger_id).await {
            if state.connected {
                let manager_arc = manager.inner().clone();
                let engine_clone = engine.inner().clone();
                auto_start_script_if_needed(&charger_id, manager_arc, engine_clone).await?;
            }
        }
    }

    Ok(())
}

/// 清除充电桩脚本
#[tauri::command]
pub async fn clear_charger_script(
    charger_id: String,
    manager: State<'_, Arc<ChargerManager>>,
    engine: State<'_, ScriptEngine>,
) -> Result<(), String> {
    let manager_arc = manager.inner().clone();
    let engine_clone = engine.inner().clone();

    if let Some(entry) = manager_arc.get_charger_script(&charger_id).await {
        if entry.running {
            stop_charger_script_internal(&charger_id, manager_arc.clone(), engine_clone.clone())
                .await?;
        }
        manager_arc.clear_charger_script(&charger_id).await
    } else {
        Err(format!("Script for charger {} not found", charger_id))
    }
}

/// 启动充电桩脚本
#[tauri::command]
pub async fn start_charger_script(
    charger_id: String,
    manager: State<'_, Arc<ChargerManager>>,
    engine: State<'_, ScriptEngine>,
) -> Result<(), String> {
    start_charger_script_internal(&charger_id, manager.inner().clone(), engine.inner().clone())
        .await
}

/// 停止充电桩脚本
#[tauri::command]
pub async fn stop_charger_script(
    charger_id: String,
    manager: State<'_, Arc<ChargerManager>>,
    engine: State<'_, ScriptEngine>,
) -> Result<(), String> {
    stop_charger_script_internal(&charger_id, manager.inner().clone(), engine.inner().clone()).await
}

fn script_storage_key(charger_id: &str) -> String {
    format!("charger:{}", charger_id)
}

async fn auto_start_script_if_needed(
    charger_id: &str,
    manager: Arc<ChargerManager>,
    engine: ScriptEngine,
) -> Result<(), String> {
    if let Some(entry) = manager.get_charger_script(charger_id).await {
        if entry.auto_start && !entry.running {
            start_charger_script_internal(charger_id, manager, engine).await?;
        }
    }
    Ok(())
}

async fn start_charger_script_internal(
    charger_id: &str,
    manager: Arc<ChargerManager>,
    engine: ScriptEngine,
) -> Result<(), String> {
    if let Some(entry) = manager.get_charger_script(charger_id).await {
        if entry.running {
            return Err(format!("Charger {} script is already running", charger_id));
        }

        let script_id = script_storage_key(charger_id);
        let script_code = entry.code.clone();

        manager
            .update_script_run_state(charger_id, true, None, None)
            .await;

        let manager_clone = manager.clone();
        let engine_clone = engine.clone();
        let charger_id_owned = charger_id.to_string();

        tauri::async_runtime::spawn(async move {
            let result = engine_clone.execute_script(script_id, script_code).await;

            let (success, message) = match result {
                Ok(script_result) => (Some(script_result.success), Some(script_result.message)),
                Err(err) => (Some(false), Some(err)),
            };

            manager_clone
                .update_script_run_state(&charger_id_owned, false, success, message)
                .await;
        });

        Ok(())
    } else {
        Err(format!("Charger {} has no script configured", charger_id))
    }
}

async fn stop_charger_script_internal(
    charger_id: &str,
    manager: Arc<ChargerManager>,
    engine: ScriptEngine,
) -> Result<(), String> {
    let script_id = script_storage_key(charger_id);
    let result = engine.stop_script(script_id).await;

    manager
        .update_script_run_state(charger_id, false, None, Some("Script stopped".to_string()))
        .await;

    result
}

// ==================== 预设脚本 Commands ====================

/// 预设脚本信息
#[derive(Debug, Serialize, Deserialize)]
pub struct PresetScript {
    pub key: String,
    pub name: String,
    pub description: String,
}

/// 获取预设脚本列表
#[tauri::command]
pub fn get_preset_scripts() -> Result<Vec<PresetScript>, String> {
    Ok(vec![
        PresetScript {
            key: "basic_test".to_string(),
            name: "basic_test.js".to_string(),
            description: "基础测试脚本 - 验证脚本引擎的所有功能".to_string(),
        },
        PresetScript {
            key: "normal_charging".to_string(),
            name: "normal_charging.js".to_string(),
            description: "正常充电流程 - 模拟完整的充电流程".to_string(),
        },
        PresetScript {
            key: "fast_charging".to_string(),
            name: "fast_charging.js".to_string(),
            description: "快速充电流程 - 模拟快充场景".to_string(),
        },
        PresetScript {
            key: "fault_test".to_string(),
            name: "fault_test.js".to_string(),
            description: "故障测试脚本 - 模拟充电桩故障情况".to_string(),
        },
    ])
}

/// 读取预设脚本内容
#[tauri::command]
pub fn read_preset_script(script_key: String) -> Result<String, String> {
    let script_content = match script_key.as_str() {
        "basic_test" => include_str!("../../scripts/basic_test.js"),
        "normal_charging" => include_str!("../../scripts/normal_charging.js"),
        "fast_charging" => include_str!("../../scripts/fast_charging.js"),
        "fault_test" => include_str!("../../scripts/fault_test.js"),
        _ => return Err(format!("Unknown preset script: {}", script_key)),
    };
    
    Ok(script_content.to_string())
}
