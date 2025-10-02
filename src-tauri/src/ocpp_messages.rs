// OCPP 1.6J 消息定义
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// OCPP 消息类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Call = 2,
    CallResult = 3,
    CallError = 4,
}

/// OCPP Call 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    #[serde(rename = "0")]
    pub message_type: u8, // 2
    #[serde(rename = "1")]
    pub unique_id: String,
    #[serde(rename = "2")]
    pub action: String,
    #[serde(rename = "3")]
    pub payload: serde_json::Value,
}

/// OCPP CallResult 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallResult {
    #[serde(rename = "0")]
    pub message_type: u8, // 3
    #[serde(rename = "1")]
    pub unique_id: String,
    #[serde(rename = "2")]
    pub payload: serde_json::Value,
}

/// OCPP CallError 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallError {
    #[serde(rename = "0")]
    pub message_type: u8, // 4
    #[serde(rename = "1")]
    pub unique_id: String,
    #[serde(rename = "2")]
    pub error_code: String,
    #[serde(rename = "3")]
    pub error_description: String,
    #[serde(rename = "4")]
    pub error_details: serde_json::Value,
}

// ========== BootNotification ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    pub charge_point_vendor: String,
    pub charge_point_model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_point_serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_box_serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_serial_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegistrationStatus {
    Accepted,
    Pending,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    pub status: RegistrationStatus,
    pub current_time: DateTime<Utc>,
    pub interval: i32, // heartbeat interval in seconds
}

// ========== Heartbeat ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatResponse {
    pub current_time: DateTime<Utc>,
}

// ========== StatusNotification ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargePointStatus {
    Available,
    Preparing,
    Charging,
    SuspendedEVSE,
    SuspendedEV,
    Finishing,
    Reserved,
    Unavailable,
    Faulted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargePointErrorCode {
    ConnectorLockFailure,
    EVCommunicationError,
    GroundFailure,
    HighTemperature,
    InternalError,
    LocalListConflict,
    NoError,
    OtherError,
    OverCurrentFailure,
    PowerMeterFailure,
    PowerSwitchFailure,
    ReaderFailure,
    ResetFailure,
    UnderVoltage,
    OverVoltage,
    WeakSignal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest {
    pub connector_id: i32,
    pub error_code: ChargePointErrorCode,
    pub status: ChargePointStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_error_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusNotificationResponse {}

// ========== StartTransaction ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionRequest {
    pub connector_id: i32,
    pub id_tag: String,
    pub meter_start: i32,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AuthorizationStatus {
    Accepted,
    Blocked,
    Expired,
    Invalid,
    ConcurrentTx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdTagInfo {
    pub status: AuthorizationStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id_tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionResponse {
    pub transaction_id: i32,
    pub id_tag_info: IdTagInfo,
}

// ========== StopTransaction ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Reason {
    EmergencyStop,
    EVDisconnected,
    HardReset,
    Local,
    Other,
    PowerLoss,
    Reboot,
    Remote,
    SoftReset,
    UnlockCommand,
    DeAuthorized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopTransactionRequest {
    pub transaction_id: i32,
    pub meter_stop: i32,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_data: Option<Vec<MeterValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopTransactionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
}

// ========== MeterValues ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReadingContext {
    InterruptionBegin,
    InterruptionEnd,
    SampleClock,
    SamplePeriodic,
    TransactionBegin,
    TransactionEnd,
    Trigger,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ValueFormat {
    Raw,
    SignedData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Measurand {
    #[serde(rename = "Energy.Active.Export.Register")]
    EnergyActiveExportRegister,
    #[serde(rename = "Energy.Active.Import.Register")]
    EnergyActiveImportRegister,
    #[serde(rename = "Energy.Reactive.Export.Register")]
    EnergyReactiveExportRegister,
    #[serde(rename = "Energy.Reactive.Import.Register")]
    EnergyReactiveImportRegister,
    #[serde(rename = "Energy.Active.Export.Interval")]
    EnergyActiveExportInterval,
    #[serde(rename = "Energy.Active.Import.Interval")]
    EnergyActiveImportInterval,
    #[serde(rename = "Energy.Reactive.Export.Interval")]
    EnergyReactiveExportInterval,
    #[serde(rename = "Energy.Reactive.Import.Interval")]
    EnergyReactiveImportInterval,
    #[serde(rename = "Power.Active.Export")]
    PowerActiveExport,
    #[serde(rename = "Power.Active.Import")]
    PowerActiveImport,
    #[serde(rename = "Power.Offered")]
    PowerOffered,
    #[serde(rename = "Power.Reactive.Export")]
    PowerReactiveExport,
    #[serde(rename = "Power.Reactive.Import")]
    PowerReactiveImport,
    #[serde(rename = "Power.Factor")]
    PowerFactor,
    #[serde(rename = "Current.Import")]
    CurrentImport,
    #[serde(rename = "Current.Export")]
    CurrentExport,
    #[serde(rename = "Current.Offered")]
    CurrentOffered,
    #[serde(rename = "Voltage")]
    Voltage,
    #[serde(rename = "Frequency")]
    Frequency,
    #[serde(rename = "Temperature")]
    Temperature,
    #[serde(rename = "SoC")]
    SoC,
    #[serde(rename = "RPM")]
    RPM,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Phase {
    L1,
    L2,
    L3,
    N,
    #[serde(rename = "L1-N")]
    L1N,
    #[serde(rename = "L2-N")]
    L2N,
    #[serde(rename = "L3-N")]
    L3N,
    #[serde(rename = "L1-L2")]
    L1L2,
    #[serde(rename = "L2-L3")]
    L2L3,
    #[serde(rename = "L3-L1")]
    L3L1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Location {
    Cable,
    EV,
    Inlet,
    Outlet,
    Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UnitOfMeasure {
    Wh,
    #[serde(rename = "kWh")]
    KWh,
    #[serde(rename = "varh")]
    Varh,
    #[serde(rename = "kvarh")]
    Kvarh,
    W,
    #[serde(rename = "kW")]
    KW,
    #[serde(rename = "VA")]
    VA,
    #[serde(rename = "kVA")]
    KVA,
    #[serde(rename = "var")]
    Var,
    #[serde(rename = "kvar")]
    Kvar,
    A,
    V,
    K,
    Celsius,
    Fahrenheit,
    Percent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampledValue {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<Measurand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<UnitOfMeasure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterValue {
    pub timestamp: DateTime<Utc>,
    pub sampled_value: Vec<SampledValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    pub connector_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<i32>,
    pub meter_value: Vec<MeterValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeterValuesResponse {}

// ========== Authorize ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    pub id_tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    pub id_tag_info: IdTagInfo,
}

// ========== 辅助函数 ==========

impl Call {
    pub fn new(action: &str, payload: serde_json::Value) -> Self {
        Self {
            message_type: MessageType::Call as u8,
            unique_id: uuid::Uuid::new_v4().to_string(),
            action: action.to_string(),
            payload,
        }
    }
}

impl CallResult {
    pub fn new(unique_id: String, payload: serde_json::Value) -> Self {
        Self {
            message_type: MessageType::CallResult as u8,
            unique_id,
            payload,
        }
    }
}

impl CallError {
    pub fn new(unique_id: String, error_code: String, error_description: String) -> Self {
        Self {
            message_type: MessageType::CallError as u8,
            unique_id,
            error_code,
            error_description,
            error_details: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
}
