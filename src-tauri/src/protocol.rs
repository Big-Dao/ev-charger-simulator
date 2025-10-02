/// 协议抽象层
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::ocpp_client::OcppClient;
use crate::ocpp_messages::{
    ChargePointErrorCode, ChargePointStatus, Measurand, Reason, SampledValue, UnitOfMeasure,
};

/// 协议消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMessage {
    pub message_type: String,
    pub payload: serde_json::Value,
}

/// 协议接口 Trait
#[async_trait]
pub trait Protocol: Send + Sync {
    /// 连接到服务器
    async fn connect(&mut self, url: &str) -> Result<(), String>;

    /// 断开连接
    async fn disconnect(&mut self) -> Result<(), String>;

    /// 发送消息
    async fn send_message(&mut self, message: ProtocolMessage) -> Result<(), String>;

    /// 接收消息
    async fn receive_message(&mut self) -> Result<Option<ProtocolMessage>, String>;

    /// 是否已连接
    async fn is_connected(&self) -> bool;

    /// 发送状态通知
    async fn send_status(&mut self, status: &str) -> Result<(), String>;

    /// 开始充电
    async fn start_charging(&mut self, id_tag: String, meter_start: i32) -> Result<(), String>;

    /// 停止充电
    async fn stop_charging(
        &mut self,
        meter_stop: i32,
        reason: Option<String>,
    ) -> Result<(), String>;

    /// 发送电表数据
    async fn send_meter_values(&mut self, power: f64, energy: f64) -> Result<(), String>;
}

/// OCPP 协议实现
pub struct OcppProtocol {
    connected: bool,
    charger_id: String,
    client: Arc<RwLock<Option<OcppClient>>>,
}

impl OcppProtocol {
    pub fn new(charger_id: String) -> Self {
        Self {
            connected: false,
            charger_id,
            client: Arc::new(RwLock::new(None)),
        }
    }
}

#[async_trait]
impl Protocol for OcppProtocol {
    async fn connect(&mut self, url: &str) -> Result<(), String> {
        tracing::info!("Connecting to OCPP server: {}", url);
        let client = OcppClient::new(self.charger_id.clone(), url.to_string());
        client.start().await.map_err(|e| e.to_string())?;
        *self.client.write().await = Some(client);
        self.connected = true;
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), String> {
        tracing::info!("Disconnecting from OCPP server");
        if let Some(client) = self.client.write().await.take() {
            client.stop().await;
        }
        self.connected = false;
        Ok(())
    }

    async fn send_message(&mut self, message: ProtocolMessage) -> Result<(), String> {
        tracing::debug!("Sending message: {:?}", message);
        // 通用消息发送（如果需要）
        Ok(())
    }

    async fn receive_message(&mut self) -> Result<Option<ProtocolMessage>, String> {
        // OCPP 客户端自动处理消息接收
        Ok(None)
    }

    async fn is_connected(&self) -> bool {
        if let Some(client) = self.client.read().await.as_ref() {
            client.is_connected().await
        } else {
            false
        }
    }

    async fn send_status(&mut self, status: &str) -> Result<(), String> {
        if let Some(client) = self.client.read().await.as_ref() {
            let ocpp_status = match status {
                "Available" => ChargePointStatus::Available,
                "Preparing" => ChargePointStatus::Preparing,
                "Charging" => ChargePointStatus::Charging,
                "SuspendedEV" => ChargePointStatus::SuspendedEV,
                "SuspendedEVSE" => ChargePointStatus::SuspendedEVSE,
                "Finishing" => ChargePointStatus::Finishing,
                "Reserved" => ChargePointStatus::Reserved,
                "Unavailable" => ChargePointStatus::Unavailable,
                "Faulted" => ChargePointStatus::Faulted,
                _ => ChargePointStatus::Available,
            };

            client
                .send_status_notification(1, ocpp_status, ChargePointErrorCode::NoError)
                .await
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    async fn start_charging(&mut self, id_tag: String, meter_start: i32) -> Result<(), String> {
        if let Some(client) = self.client.read().await.as_ref() {
            client
                .start_transaction(1, id_tag, meter_start)
                .await
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    async fn stop_charging(
        &mut self,
        meter_stop: i32,
        reason: Option<String>,
    ) -> Result<(), String> {
        if let Some(client) = self.client.read().await.as_ref() {
            let ocpp_reason = reason.and_then(|r| match r.as_str() {
                "EmergencyStop" => Some(Reason::EmergencyStop),
                "EVDisconnected" => Some(Reason::EVDisconnected),
                "PowerLoss" => Some(Reason::PowerLoss),
                "Remote" => Some(Reason::Remote),
                "Local" => Some(Reason::Local),
                _ => None,
            });

            client
                .stop_transaction(meter_stop, ocpp_reason)
                .await
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    async fn send_meter_values(&mut self, power: f64, energy: f64) -> Result<(), String> {
        if let Some(client) = self.client.read().await.as_ref() {
            let sampled_values = vec![
                SampledValue {
                    value: power.to_string(),
                    context: None,
                    format: None,
                    measurand: Some(Measurand::PowerActiveImport),
                    phase: None,
                    location: None,
                    unit: Some(UnitOfMeasure::KW),
                },
                SampledValue {
                    value: energy.to_string(),
                    context: None,
                    format: None,
                    measurand: Some(Measurand::EnergyActiveImportRegister),
                    phase: None,
                    location: None,
                    unit: Some(UnitOfMeasure::KWh),
                },
            ];

            client
                .send_meter_values(1, sampled_values)
                .await
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

/// 云快充协议实现
pub struct YunKuaiChongProtocol {
    connected: bool,
    charger_id: String,
    // TODO: HTTP 客户端
}

impl YunKuaiChongProtocol {
    pub fn new(charger_id: String) -> Self {
        Self {
            connected: false,
            charger_id,
        }
    }
}

#[async_trait]
impl Protocol for YunKuaiChongProtocol {
    async fn connect(&mut self, url: &str) -> Result<(), String> {
        tracing::info!("Connecting to YunKuaiChong server: {}", url);
        // TODO: 实现 HTTP 连接
        self.connected = true;
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), String> {
        tracing::info!("Disconnecting from YunKuaiChong server");
        self.connected = false;
        Ok(())
    }

    async fn send_message(&mut self, message: ProtocolMessage) -> Result<(), String> {
        if !self.connected {
            return Err("Not connected".to_string());
        }

        tracing::debug!("Sending YunKuaiChong message: {:?}", message.message_type);
        // TODO: 实现消息发送
        Ok(())
    }

    async fn receive_message(&mut self) -> Result<Option<ProtocolMessage>, String> {
        if !self.connected {
            return Err("Not connected".to_string());
        }

        // TODO: 实现消息接收
        Ok(None)
    }

    async fn is_connected(&self) -> bool {
        self.connected
    }

    async fn send_status(&mut self, _status: &str) -> Result<(), String> {
        tracing::debug!("YunKuaiChong: send_status not yet implemented");
        // TODO: 实现云快充状态上报
        Ok(())
    }

    async fn start_charging(&mut self, _id_tag: String, _meter_start: i32) -> Result<(), String> {
        tracing::debug!("YunKuaiChong: start_charging not yet implemented");
        // TODO: 实现云快充开始充电
        Ok(())
    }

    async fn stop_charging(
        &mut self,
        _meter_stop: i32,
        _reason: Option<String>,
    ) -> Result<(), String> {
        tracing::debug!("YunKuaiChong: stop_charging not yet implemented");
        // TODO: 实现云快充停止充电
        Ok(())
    }

    async fn send_meter_values(&mut self, _power: f64, _energy: f64) -> Result<(), String> {
        tracing::debug!("YunKuaiChong: send_meter_values not yet implemented");
        // TODO: 实现云快充电表数据上报
        Ok(())
    }
}

/// 协议工厂
pub fn create_protocol(protocol_type: &str, charger_id: String) -> Box<dyn Protocol> {
    match protocol_type.to_uppercase().as_str() {
        "OCPP" => Box::new(OcppProtocol::new(charger_id)),
        "YUNKUAICHONG" | "YKC" => Box::new(YunKuaiChongProtocol::new(charger_id)),
        _ => {
            tracing::warn!(
                "Unknown protocol type: {}, using OCPP as default",
                protocol_type
            );
            Box::new(OcppProtocol::new(charger_id))
        }
    }
}
