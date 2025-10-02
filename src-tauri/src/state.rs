/// 充电桩状态定义
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ChargerStatus {
    /// 可用状态
    Available,
    /// 准备中
    Preparing,
    /// 充电中
    Charging,
    /// 挂起/暂停
    SuspendedEV,
    /// 暂停（设备端）
    SuspendedEVSE,
    /// 结束中
    Finishing,
    /// 预约
    Reserved,
    /// 不可用
    Unavailable,
    /// 故障
    Faulted,
}

impl Default for ChargerStatus {
    fn default() -> Self {
        Self::Available
    }
}

impl ChargerStatus {
    /// 状态转换验证
    pub fn can_transition_to(&self, target: &ChargerStatus) -> bool {
        use ChargerStatus::*;
        match (self, target) {
            // Available 可以转到任何状态
            (Available, _) => true,
            // Preparing 可以转到 Charging 或回到 Available
            (Preparing, Charging) | (Preparing, Available) | (Preparing, Faulted) => true,
            // Charging 可以暂停或结束
            (Charging, SuspendedEV)
            | (Charging, SuspendedEVSE)
            | (Charging, Finishing)
            | (Charging, Faulted) => true,
            // 暂停状态可以恢复充电或结束
            (SuspendedEV, Charging) | (SuspendedEV, Finishing) => true,
            (SuspendedEVSE, Charging) | (SuspendedEVSE, Finishing) => true,
            // Finishing 只能回到 Available
            (Finishing, Available) => true,
            // Reserved 可以转到 Preparing 或 Available
            (Reserved, Preparing) | (Reserved, Available) => true,
            // Faulted 可以恢复到 Available
            (Faulted, Available) => true,
            // 其他转换不允许
            _ => false,
        }
    }
}

/// 充电桩错误代码
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ErrorCode {
    ConnectorLockFailure,
    EVCommunicationError,
    GroundFailure,
    HighTemperature,
    InternalError,
    LocalListConflict,
    NoError,
    OtherError,
    OverCurrentFailure,
    OverVoltage,
    PowerMeterFailure,
    PowerSwitchFailure,
    ReaderFailure,
    ResetFailure,
    UnderVoltage,
    WeakSignal,
}

/// 充电桩运行状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargerState {
    /// 充电桩 ID
    pub charger_id: String,
    /// 当前状态
    pub status: ChargerStatus,
    /// 错误代码
    pub error_code: ErrorCode,
    /// 连接状态
    pub connected: bool,
    /// 当前充电功率 (kW)
    pub current_power: f64,
    /// 累计充电电量 (kWh)
    pub total_energy: f64,
    /// 充电会话 ID
    pub transaction_id: Option<i32>,
    /// 最后更新时间
    pub last_updated: chrono::DateTime<chrono::Utc>,
    /// 当前关联的脚本名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,
    /// 脚本是否正在运行
    pub script_running: bool,
    /// 最近一次脚本执行是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_last_success: Option<bool>,
    /// 最近一次脚本执行返回的信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_last_message: Option<String>,
}

impl ChargerState {
    pub fn new(charger_id: String) -> Self {
        Self {
            charger_id,
            status: ChargerStatus::Available,
            error_code: ErrorCode::NoError,
            connected: false,
            current_power: 0.0,
            total_energy: 0.0,
            transaction_id: None,
            last_updated: chrono::Utc::now(),
            script_name: None,
            script_running: false,
            script_last_success: None,
            script_last_message: None,
        }
    }

    /// 更新状态
    pub fn update_status(&mut self, new_status: ChargerStatus) -> Result<(), String> {
        if self.status.can_transition_to(&new_status) {
            self.status = new_status;
            self.last_updated = chrono::Utc::now();
            Ok(())
        } else {
            Err(format!(
                "Invalid state transition from {:?} to {:?}",
                self.status, new_status
            ))
        }
    }

    /// 更新功率和电量
    pub fn update_power(&mut self, power: f64, energy_increment: f64) {
        self.current_power = power;
        self.total_energy += energy_increment;
        self.last_updated = chrono::Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transition() {
        let mut state = ChargerState::new("TEST001".to_string());

        // Available -> Preparing
        assert!(state.update_status(ChargerStatus::Preparing).is_ok());

        // Preparing -> Charging
        assert!(state.update_status(ChargerStatus::Charging).is_ok());

        // Charging -> Available (不允许)
        assert!(state.update_status(ChargerStatus::Available).is_err());

        // Charging -> Finishing
        assert!(state.update_status(ChargerStatus::Finishing).is_ok());

        // Finishing -> Available
        assert!(state.update_status(ChargerStatus::Available).is_ok());
    }
}
