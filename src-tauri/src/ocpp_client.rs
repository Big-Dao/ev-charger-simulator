// OCPP WebSocket 客户端
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::time::{interval, sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info, warn};

use crate::ocpp_messages::*;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// OCPP 客户端状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OcppClientState {
    Disconnected,
    Connecting,
    Connected,
    Registered,
    Error,
}

/// 待响应的请求
struct PendingRequest {
    action: String,
    sent_at: std::time::Instant,
}

/// OCPP WebSocket 客户端
pub struct OcppClient {
    /// 充电桩ID
    pub charger_id: String,
    /// WebSocket URL
    pub url: String,
    /// 客户端状态
    state: Arc<RwLock<OcppClientState>>,
    /// WebSocket 连接
    ws_stream: Arc<RwLock<Option<WsStream>>>,
    /// 待响应的请求
    pending_requests: Arc<RwLock<HashMap<String, PendingRequest>>>,
    /// 心跳间隔（秒）
    heartbeat_interval: Arc<RwLock<u64>>,
    /// 是否应该运行
    should_run: Arc<RwLock<bool>>,
    /// 事务ID
    transaction_id: Arc<RwLock<Option<i32>>>,
}

impl OcppClient {
    /// 创建新的 OCPP 客户端
    pub fn new(charger_id: String, url: String) -> Self {
        Self {
            charger_id,
            url,
            state: Arc::new(RwLock::new(OcppClientState::Disconnected)),
            ws_stream: Arc::new(RwLock::new(None)),
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
            heartbeat_interval: Arc::new(RwLock::new(60)), // 默认60秒
            should_run: Arc::new(RwLock::new(false)),
            transaction_id: Arc::new(RwLock::new(None)),
        }
    }

    /// 启动客户端
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        *self.should_run.write().await = true;

        // 启动连接循环
        let client = self.clone();
        tokio::spawn(async move {
            client.connection_loop().await;
        });

        Ok(())
    }

    /// 停止客户端
    pub async fn stop(&self) {
        info!("Stopping OCPP client for charger: {}", self.charger_id);
        *self.should_run.write().await = false;

        // 关闭连接
        if let Some(mut ws) = self.ws_stream.write().await.take() {
            let _ = ws.close(None).await;
        }

        *self.state.write().await = OcppClientState::Disconnected;
    }

    /// 连接循环（自动重连）
    async fn connection_loop(&self) {
        let mut retry_count = 0;
        let max_retry_delay = 300; // 最大5分钟

        while *self.should_run.read().await {
            match self.connect().await {
                Ok(_) => {
                    retry_count = 0;
                    info!("OCPP client connected: {}", self.charger_id);

                    // 发送 BootNotification
                    if let Err(e) = self.send_boot_notification().await {
                        error!("Failed to send BootNotification: {}", e);
                        continue;
                    }

                    // 启动心跳
                    self.start_heartbeat();

                    // 接收消息
                    self.receive_loop().await;
                }
                Err(e) => {
                    error!("Failed to connect: {}. Retrying...", e);
                }
            }

            // 指数退避重连
            if *self.should_run.read().await {
                let delay = std::cmp::min(2_u64.pow(retry_count), max_retry_delay);
                info!("Reconnecting in {} seconds...", delay);
                sleep(Duration::from_secs(delay)).await;
                retry_count += 1;
            }
        }
    }

    /// 建立 WebSocket 连接
    async fn connect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        *self.state.write().await = OcppClientState::Connecting;

        let url = format!("{}/{}", self.url, self.charger_id);
        info!("Connecting to: {}", url);

        let (ws_stream, _) = connect_async(&url).await?;
        *self.ws_stream.write().await = Some(ws_stream);
        *self.state.write().await = OcppClientState::Connected;

        Ok(())
    }

    /// 接收消息循环
    async fn receive_loop(&self) {
        loop {
            if !*self.should_run.read().await {
                break;
            }

            let message = {
                let mut ws_lock = self.ws_stream.write().await;
                if let Some(ws) = ws_lock.as_mut() {
                    match ws.next().await {
                        Some(Ok(msg)) => Some(msg),
                        Some(Err(e)) => {
                            error!("WebSocket error: {}", e);
                            *self.state.write().await = OcppClientState::Error;
                            break;
                        }
                        None => {
                            warn!("WebSocket connection closed");
                            *self.state.write().await = OcppClientState::Disconnected;
                            break;
                        }
                    }
                } else {
                    None
                }
            };

            if let Some(msg) = message {
                if let Err(e) = self.handle_message(msg).await {
                    error!("Failed to handle message: {}", e);
                }
            }
        }
    }

    /// 处理接收到的消息
    async fn handle_message(
        &self,
        msg: Message,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match msg {
            Message::Text(text) => {
                debug!("Received: {}", text);

                let json: serde_json::Value = serde_json::from_str(&text)?;
                if let Some(array) = json.as_array() {
                    if array.is_empty() {
                        return Ok(());
                    }

                    let msg_type = array[0].as_u64().unwrap_or(0);

                    match msg_type {
                        3 => self.handle_call_result(&text).await?,
                        4 => self.handle_call_error(&text).await?,
                        2 => self.handle_call(&text).await?,
                        _ => warn!("Unknown message type: {}", msg_type),
                    }
                }
            }
            Message::Close(_) => {
                info!("Received close frame");
                *self.state.write().await = OcppClientState::Disconnected;
            }
            Message::Ping(data) => {
                debug!("Received ping, sending pong");
                self.send_raw(Message::Pong(data)).await?;
            }
            Message::Pong(_) => {
                debug!("Received pong");
            }
            _ => {}
        }

        Ok(())
    }

    /// 处理 CallResult 消息
    async fn handle_call_result(
        &self,
        text: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result: CallResult = serde_json::from_str(text)?;
        info!("Received CallResult for: {}", result.unique_id);

        // 移除待响应请求
        if let Some(pending) = self
            .pending_requests
            .write()
            .await
            .remove(&result.unique_id)
        {
            debug!(
                "Request {} completed in {:?}",
                pending.action,
                pending.sent_at.elapsed()
            );

            // 特殊处理 BootNotification 响应
            if pending.action == "BootNotification" {
                if let Ok(response) =
                    serde_json::from_value::<BootNotificationResponse>(result.payload)
                {
                    *self.heartbeat_interval.write().await = response.interval as u64;
                    *self.state.write().await = OcppClientState::Registered;
                    info!(
                        "BootNotification accepted, heartbeat interval: {}s",
                        response.interval
                    );
                }
            }
            // 处理 StartTransaction 响应
            else if pending.action == "StartTransaction" {
                if let Ok(response) =
                    serde_json::from_value::<StartTransactionResponse>(result.payload)
                {
                    *self.transaction_id.write().await = Some(response.transaction_id);
                    info!("Transaction started with ID: {}", response.transaction_id);
                }
            }
        }

        Ok(())
    }

    /// 处理 CallError 消息
    async fn handle_call_error(
        &self,
        text: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let error: CallError = serde_json::from_str(text)?;
        error!(
            "Received CallError for {}: {} - {}",
            error.unique_id, error.error_code, error.error_description
        );

        self.pending_requests.write().await.remove(&error.unique_id);
        Ok(())
    }

    /// 处理 Call 消息（服务器请求）
    async fn handle_call(
        &self,
        text: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let call: Call = serde_json::from_str(text)?;
        info!("Received Call: {}", call.action);

        // 根据不同的 action 处理请求
        let response = match call.action.as_str() {
            "RemoteStartTransaction" => self.handle_remote_start(&call).await?,
            "RemoteStopTransaction" => self.handle_remote_stop(&call).await?,
            "Reset" => self.handle_reset(&call).await?,
            "ChangeConfiguration" => self.handle_change_configuration(&call).await?,
            "GetConfiguration" => self.handle_get_configuration(&call).await?,
            _ => {
                warn!("Unsupported action: {}", call.action);
                // 返回错误
                let error = CallError::new(
                    call.unique_id.clone(),
                    "NotSupported".to_string(),
                    format!("Action {} is not supported", call.action),
                );
                serde_json::to_value(error)?
            }
        };

        // 发送响应
        let result = CallResult::new(call.unique_id, response);
        self.send_message(&result).await?;

        Ok(())
    }

    /// 处理 RemoteStartTransaction
    async fn handle_remote_start(
        &self,
        _call: &Call,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        // 简单返回接受
        Ok(serde_json::json!({
            "status": "Accepted"
        }))
    }

    /// 处理 RemoteStopTransaction
    async fn handle_remote_stop(
        &self,
        _call: &Call,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        Ok(serde_json::json!({
            "status": "Accepted"
        }))
    }

    /// 处理 Reset
    async fn handle_reset(
        &self,
        _call: &Call,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        Ok(serde_json::json!({
            "status": "Accepted"
        }))
    }

    /// 处理 ChangeConfiguration
    async fn handle_change_configuration(
        &self,
        _call: &Call,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        Ok(serde_json::json!({
            "status": "Accepted"
        }))
    }

    /// 处理 GetConfiguration
    async fn handle_get_configuration(
        &self,
        _call: &Call,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        Ok(serde_json::json!({
            "configurationKey": [],
            "unknownKey": []
        }))
    }

    /// 发送原始消息
    async fn send_raw(&self, msg: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut ws_lock = self.ws_stream.write().await;
        if let Some(ws) = ws_lock.as_mut() {
            ws.send(msg).await?;
            Ok(())
        } else {
            Err("WebSocket not connected".into())
        }
    }

    /// 发送 OCPP 消息
    async fn send_message<T: Serialize>(
        &self,
        msg: &T,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json = serde_json::to_string(msg)?;
        debug!("Sending: {}", json);
        self.send_raw(Message::Text(json)).await
    }

    /// 发送 Call 消息并记录待响应
    async fn send_call(
        &self,
        action: &str,
        payload: serde_json::Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let call = Call::new(action, payload);
        let unique_id = call.unique_id.clone();

        // 记录待响应请求
        self.pending_requests.write().await.insert(
            unique_id.clone(),
            PendingRequest {
                action: action.to_string(),
                sent_at: std::time::Instant::now(),
            },
        );

        self.send_message(&call).await?;
        Ok(unique_id)
    }

    /// 发送 BootNotification
    pub async fn send_boot_notification(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let request = BootNotificationRequest {
            charge_point_vendor: "Simulator".to_string(),
            charge_point_model: "EV-SIM-V1".to_string(),
            charge_point_serial_number: Some(self.charger_id.clone()),
            charge_box_serial_number: Some(format!("{}-BOX", self.charger_id)),
            firmware_version: Some("1.0.0".to_string()),
            iccid: None,
            imsi: None,
            meter_type: Some("AC".to_string()),
            meter_serial_number: Some(format!("{}-METER", self.charger_id)),
        };

        let payload = serde_json::to_value(request)?;
        self.send_call("BootNotification", payload).await?;
        Ok(())
    }

    /// 启动心跳
    fn start_heartbeat(&self) {
        let client = self.clone();
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(*client.heartbeat_interval.read().await));

            loop {
                ticker.tick().await;

                if !*client.should_run.read().await
                    || *client.state.read().await != OcppClientState::Registered
                {
                    break;
                }

                if let Err(e) = client.send_heartbeat().await {
                    error!("Failed to send heartbeat: {}", e);
                }
            }
        });
    }

    /// 发送心跳
    pub async fn send_heartbeat(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let request = HeartbeatRequest {};
        let payload = serde_json::to_value(request)?;
        self.send_call("Heartbeat", payload).await?;
        Ok(())
    }

    /// 发送状态通知
    pub async fn send_status_notification(
        &self,
        connector_id: i32,
        status: ChargePointStatus,
        error_code: ChargePointErrorCode,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let request = StatusNotificationRequest {
            connector_id,
            error_code,
            status,
            info: None,
            timestamp: Some(Utc::now()),
            vendor_id: None,
            vendor_error_code: None,
        };

        let payload = serde_json::to_value(request)?;
        self.send_call("StatusNotification", payload).await?;
        Ok(())
    }

    /// 开始交易
    pub async fn start_transaction(
        &self,
        connector_id: i32,
        id_tag: String,
        meter_start: i32,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let request = StartTransactionRequest {
            connector_id,
            id_tag,
            meter_start,
            timestamp: Utc::now(),
            reservation_id: None,
        };

        let payload = serde_json::to_value(request)?;
        self.send_call("StartTransaction", payload).await?;
        Ok(())
    }

    /// 停止交易
    pub async fn stop_transaction(
        &self,
        meter_stop: i32,
        reason: Option<Reason>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let transaction_id = self
            .transaction_id
            .read()
            .await
            .ok_or("No active transaction")?;

        let request = StopTransactionRequest {
            transaction_id,
            meter_stop,
            timestamp: Utc::now(),
            reason,
            id_tag: None,
            transaction_data: None,
        };

        let payload = serde_json::to_value(request)?;
        self.send_call("StopTransaction", payload).await?;

        *self.transaction_id.write().await = None;
        Ok(())
    }

    /// 发送电表数据
    pub async fn send_meter_values(
        &self,
        connector_id: i32,
        sampled_values: Vec<SampledValue>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let meter_value = MeterValue {
            timestamp: Utc::now(),
            sampled_value: sampled_values,
        };

        let request = MeterValuesRequest {
            connector_id,
            transaction_id: *self.transaction_id.read().await,
            meter_value: vec![meter_value],
        };

        let payload = serde_json::to_value(request)?;
        self.send_call("MeterValues", payload).await?;
        Ok(())
    }

    /// 获取客户端状态
    pub async fn get_state(&self) -> OcppClientState {
        self.state.read().await.clone()
    }

    /// 是否已连接
    pub async fn is_connected(&self) -> bool {
        matches!(
            *self.state.read().await,
            OcppClientState::Connected | OcppClientState::Registered
        )
    }
}

impl Clone for OcppClient {
    fn clone(&self) -> Self {
        Self {
            charger_id: self.charger_id.clone(),
            url: self.url.clone(),
            state: Arc::clone(&self.state),
            ws_stream: Arc::clone(&self.ws_stream),
            pending_requests: Arc::clone(&self.pending_requests),
            heartbeat_interval: Arc::clone(&self.heartbeat_interval),
            should_run: Arc::clone(&self.should_run),
            transaction_id: Arc::clone(&self.transaction_id),
        }
    }
}

use serde::Serialize;
