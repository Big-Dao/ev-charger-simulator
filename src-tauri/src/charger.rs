/// 充电桩模拟器核心
use crate::protocol::{create_protocol, Protocol};
use crate::state::{ChargerState, ChargerStatus};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 充电桩配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargerConfig {
    /// 充电桩唯一标识
    pub id: String,
    /// 充电桩名称
    pub name: String,
    /// 协议类型 (OCPP / YunKuaiChong)
    pub protocol_type: String,
    /// 服务器地址
    pub server_url: String,
    /// 最大功率 (kW)
    pub max_power: f64,
    /// 脚本路径
    pub script_path: Option<String>,
    /// 是否启用
    pub enabled: bool,
}

impl Default for ChargerConfig {
    fn default() -> Self {
        Self {
            id: format!("CP{:06}", rand::random::<u32>() % 1000000),
            name: "充电桩".to_string(),
            protocol_type: "OCPP".to_string(),
            server_url: "ws://localhost:8080".to_string(),
            max_power: 60.0,
            script_path: None,
            enabled: true,
        }
    }
}

/// 充电桩实例
pub struct Charger {
    /// 配置
    pub config: ChargerConfig,
    /// 状态
    pub state: Arc<RwLock<ChargerState>>,
    /// 协议实现
    protocol: Option<Box<dyn Protocol>>,
    /// 运行标志
    running: Arc<RwLock<bool>>,
}

impl Charger {
    /// 创建新充电桩
    pub fn new(config: ChargerConfig) -> Self {
        let state = ChargerState::new(config.id.clone());

        Self {
            config,
            state: Arc::new(RwLock::new(state)),
            protocol: None,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// 启动充电桩
    pub async fn start(&mut self) -> Result<(), String> {
        let mut running = self.running.write().await;
        if *running {
            return Err("Charger is already running".to_string());
        }

        tracing::info!("Starting charger: {}", self.config.id);

        // 创建协议实例
        let mut protocol = create_protocol(&self.config.protocol_type, self.config.id.clone());

        // 连接到服务器
        match protocol.connect(&self.config.server_url).await {
            Ok(_) => {
                tracing::info!("Protocol connected for charger: {}", self.config.id);
                self.protocol = Some(protocol);
            }
            Err(e) => {
                tracing::error!("Failed to connect protocol: {}", e);
                return Err(format!("Failed to connect: {}", e));
            }
        }

        // 更新状态为连接
        {
            let mut state = self.state.write().await;
            state.connected = true;
        }

        *running = true;

        // TODO: 启动脚本引擎

        Ok(())
    }

    /// 停止充电桩
    pub async fn stop(&mut self) -> Result<(), String> {
        let mut running = self.running.write().await;
        if !*running {
            return Err("Charger is not running".to_string());
        }

        tracing::info!("Stopping charger: {}", self.config.id);

        // 断开协议连接
        if let Some(protocol) = &mut self.protocol {
            let _ = protocol.disconnect().await;
        }
        self.protocol = None;

        // 更新状态
        {
            let mut state = self.state.write().await;
            state.connected = false;
            let _ = state.update_status(ChargerStatus::Available);
        }

        *running = false;

        // TODO: 停止脚本引擎

        Ok(())
    }

    /// 获取状态
    pub async fn get_state(&self) -> ChargerState {
        self.state.read().await.clone()
    }

    /// 是否在运行
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// 开始充电
    pub async fn start_charging(&mut self) -> Result<(), String> {
        let mut state = self.state.write().await;

        state.update_status(ChargerStatus::Preparing)?;

        // 发送状态通知
        if let Some(protocol) = &mut self.protocol {
            protocol.send_status("Preparing").await?;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // 开始交易
        if let Some(protocol) = &mut self.protocol {
            let id_tag = format!("USER_{}", rand::random::<u32>() % 10000);
            protocol
                .start_charging(id_tag, state.total_energy as i32)
                .await?;
        }

        // 生成交易 ID
        state.transaction_id = Some(rand::random());
        state.update_status(ChargerStatus::Charging)?;

        // 发送状态通知
        if let Some(protocol) = &mut self.protocol {
            protocol.send_status("Charging").await?;
        }

        Ok(())
    }

    /// 停止充电
    pub async fn stop_charging(&mut self) -> Result<(), String> {
        let mut state = self.state.write().await;

        state.update_status(ChargerStatus::Finishing)?;

        // 发送状态通知
        if let Some(protocol) = &mut self.protocol {
            protocol.send_status("Finishing").await?;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // 停止交易
        if let Some(protocol) = &mut self.protocol {
            protocol
                .stop_charging(state.total_energy as i32, Some("Local".to_string()))
                .await?;
        }

        state.update_status(ChargerStatus::Available)?;
        state.transaction_id = None;
        state.current_power = 0.0;

        // 发送状态通知
        if let Some(protocol) = &mut self.protocol {
            protocol.send_status("Available").await?;
        }

        Ok(())
    }

    /// 设置功率
    pub async fn set_power(&self, power: f64) -> Result<(), String> {
        let mut state = self.state.write().await;

        if power > self.config.max_power {
            return Err(format!(
                "Power {} kW exceeds max power {} kW",
                power, self.config.max_power
            ));
        }

        state.current_power = power;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_charger_lifecycle() {
        let config = ChargerConfig::default();
        let mut charger = Charger::new(config);

        // 启动
        assert!(charger.start().await.is_ok());
        assert!(charger.is_running().await);

        // 开始充电
        assert!(charger.start_charging().await.is_ok());

        let state = charger.get_state().await;
        assert_eq!(state.status, ChargerStatus::Charging);

        // 停止充电
        assert!(charger.stop_charging().await.is_ok());

        let state = charger.get_state().await;
        assert_eq!(state.status, ChargerStatus::Available);

        // 停止
        assert!(charger.stop().await.is_ok());
        assert!(!charger.is_running().await);
    }
}
