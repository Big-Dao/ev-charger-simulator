/// 充电桩管理器 - 管理所有充电桩实例
use crate::charger::{Charger, ChargerConfig};
use crate::state::ChargerState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct ChargerScriptEntry {
    pub name: String,
    pub code: String,
    pub auto_start: bool,
    pub running: bool,
    pub last_success: Option<bool>,
    pub last_message: Option<String>,
}

pub struct ChargerManager {
    chargers: Arc<RwLock<HashMap<String, Arc<RwLock<Charger>>>>>,
    scripts: Arc<RwLock<HashMap<String, ChargerScriptEntry>>>,
}

impl ChargerManager {
    /// 创建充电桩管理器
    pub fn new() -> Self {
        Self {
            chargers: Arc::new(RwLock::new(HashMap::new())),
            scripts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 添加充电桩
    pub async fn add_charger(&self, config: ChargerConfig) -> Result<String, String> {
        let charger_id = config.id.clone();
        let charger = Arc::new(RwLock::new(Charger::new(config)));

        let mut chargers = self.chargers.write().await;

        if chargers.contains_key(&charger_id) {
            return Err(format!("Charger {} already exists", charger_id));
        }

        chargers.insert(charger_id.clone(), charger);

        tracing::info!("Added charger: {}", charger_id);
        Ok(charger_id)
    }

    /// 移除充电桩
    pub async fn remove_charger(&self, charger_id: &str) -> Result<(), String> {
        // 先停止充电桩
        if let Some(charger) = self.chargers.read().await.get(charger_id) {
            let mut charger = charger.write().await;
            let _ = charger.stop().await;
        }

        let mut chargers = self.chargers.write().await;
        chargers
            .remove(charger_id)
            .ok_or_else(|| format!("Charger {} not found", charger_id))?;

        // 清除脚本信息
        let mut scripts = self.scripts.write().await;
        scripts.remove(charger_id);

        tracing::info!("Removed charger: {}", charger_id);
        Ok(())
    }

    /// 设置充电桩脚本
    pub async fn set_charger_script(
        &self,
        charger_id: &str,
        name: String,
        code: String,
        auto_start: bool,
    ) -> Result<(), String> {
        // 确保充电桩存在
        self.get_charger(charger_id).await?;

        let mut scripts = self.scripts.write().await;
        scripts.insert(
            charger_id.to_string(),
            ChargerScriptEntry {
                name,
                code,
                auto_start,
                running: false,
                last_success: None,
                last_message: None,
            },
        );

        Ok(())
    }

    /// 清除充电桩脚本
    pub async fn clear_charger_script(&self, charger_id: &str) -> Result<(), String> {
        let mut scripts = self.scripts.write().await;
        if scripts.remove(charger_id).is_none() {
            return Err(format!("Script for charger {} not found", charger_id));
        }
        Ok(())
    }

    /// 获取充电桩脚本
    pub async fn get_charger_script(&self, charger_id: &str) -> Option<ChargerScriptEntry> {
        self.scripts.read().await.get(charger_id).cloned()
    }

    /// 更新脚本运行状态
    pub async fn update_script_run_state(
        &self,
        charger_id: &str,
        running: bool,
        last_success: Option<bool>,
        last_message: Option<String>,
    ) {
        let mut scripts = self.scripts.write().await;
        if let Some(entry) = scripts.get_mut(charger_id) {
            entry.running = running;
            if let Some(value) = last_success {
                entry.last_success = Some(value);
            }
            if let Some(msg) = last_message {
                entry.last_message = Some(msg);
            }
        }
    }

    /// 获取所有脚本列表
    pub async fn list_scripts(&self) -> Vec<(String, ChargerScriptEntry)> {
        self.scripts
            .read()
            .await
            .iter()
            .map(|(id, entry)| (id.clone(), entry.clone()))
            .collect()
    }

    /// 获取充电桩
    async fn get_charger(&self, charger_id: &str) -> Result<Arc<RwLock<Charger>>, String> {
        self.chargers
            .read()
            .await
            .get(charger_id)
            .cloned()
            .ok_or_else(|| format!("Charger {} not found", charger_id))
    }

    /// 获取充电桩配置
    pub async fn get_charger_config(&self, charger_id: &str) -> Result<ChargerConfig, String> {
        let charger = self.get_charger(charger_id).await?;
        let charger = charger.read().await;
        Ok(charger.config.clone())
    }

    /// 获取所有充电桩配置
    pub async fn get_all_charger_configs(&self) -> Vec<ChargerConfig> {
        let chargers = self.chargers.read().await;
        let mut configs = Vec::new();
        for charger in chargers.values() {
            let charger = charger.read().await;
            configs.push(charger.config.clone());
        }
        configs
    }

    /// 更新充电桩配置
    pub async fn update_charger_config(&self, charger_id: &str, config: ChargerConfig) -> Result<(), String> {
        if config.id != charger_id {
            return Err("Cannot change charger ID".to_string());
        }

        let charger = self.get_charger(charger_id).await?;
        let mut charger = charger.write().await;
        
        // 如果充电桩正在运行，先停止
        let was_running = charger.is_running().await;
        if was_running {
            charger.stop().await?;
        }

        // 更新配置
        charger.config = config;

        // 如果之前在运行，重新启动
        if was_running {
            charger.start().await?;
        }

        tracing::info!("Updated config for charger: {}", charger_id);
        Ok(())
    }

    /// 启动充电桩
    pub async fn start_charger(&self, charger_id: &str) -> Result<(), String> {
        let charger = self.get_charger(charger_id).await?;
        let mut charger = charger.write().await;
        charger.start().await
    }

    /// 停止充电桩
    pub async fn stop_charger(&self, charger_id: &str) -> Result<(), String> {
        let charger = self.get_charger(charger_id).await?;
        let mut charger = charger.write().await;
        charger.stop().await
    }

    /// 启动所有充电桩
    pub async fn start_all(&self) -> Result<usize, String> {
        let charger_ids: Vec<String> = self.chargers.read().await.keys().cloned().collect();
        let mut success_count = 0;

        for charger_id in charger_ids {
            if let Ok(_) = self.start_charger(&charger_id).await {
                success_count += 1;
            }
        }

        tracing::info!("Started {} chargers", success_count);
        Ok(success_count)
    }

    /// 停止所有充电桩
    pub async fn stop_all(&self) -> Result<usize, String> {
        let charger_ids: Vec<String> = self.chargers.read().await.keys().cloned().collect();
        let mut success_count = 0;

        for charger_id in charger_ids {
            if let Ok(_) = self.stop_charger(&charger_id).await {
                success_count += 1;
            }
        }

        tracing::info!("Stopped {} chargers", success_count);
        Ok(success_count)
    }

    /// 获取充电桩状态
    pub async fn get_charger_state(&self, charger_id: &str) -> Result<ChargerState, String> {
        let charger = self.get_charger(charger_id).await?;
        let charger = charger.read().await;
        let mut state = charger.get_state().await;

        if let Some(script) = self.get_charger_script(charger_id).await {
            state.script_name = Some(script.name);
            state.script_running = script.running;
            state.script_last_success = script.last_success;
            state.script_last_message = script.last_message;
        }

        Ok(state)
    }

    /// 获取所有充电桩列表
    pub async fn get_charger_list(&self) -> Vec<ChargerState> {
        let chargers = self.chargers.read().await;
        let mut states = Vec::new();

        for (charger_id, charger) in chargers.iter() {
            let charger = charger.read().await;
            let mut state = charger.get_state().await;
            if let Some(script) = self.get_charger_script(charger_id).await {
                state.script_name = Some(script.name);
                state.script_running = script.running;
                state.script_last_success = script.last_success;
                state.script_last_message = script.last_message;
            }
            states.push(state);
        }

        states
    }

    /// 获取充电桩数量
    pub async fn get_charger_count(&self) -> usize {
        self.chargers.read().await.len()
    }

    /// 开始充电
    pub async fn start_charging(&self, charger_id: &str) -> Result<(), String> {
        let charger = self.get_charger(charger_id).await?;
        let mut charger = charger.write().await;
        charger.start_charging().await
    }

    /// 停止充电
    pub async fn stop_charging(&self, charger_id: &str) -> Result<(), String> {
        let charger = self.get_charger(charger_id).await?;
        let mut charger = charger.write().await;
        charger.stop_charging().await
    }

    /// 设置充电功率
    pub async fn set_power(&self, charger_id: &str, power: f64) -> Result<(), String> {
        let charger = self.get_charger(charger_id).await?;
        let charger = charger.read().await;
        charger.set_power(power).await
    }

    /// 批量创建充电桩
    pub async fn create_batch(
        &self,
        count: usize,
        base_config: ChargerConfig,
    ) -> Result<Vec<String>, String> {
        let mut charger_ids = Vec::new();

        for i in 0..count {
            let mut config = base_config.clone();
            config.id = format!("CP{:06}", i + 1);
            config.name = format!("充电桩 #{}", i + 1);

            match self.add_charger(config).await {
                Ok(id) => charger_ids.push(id),
                Err(e) => tracing::warn!("Failed to create charger {}: {}", i, e),
            }
        }

        tracing::info!("Created {} chargers in batch", charger_ids.len());
        Ok(charger_ids)
    }
}

impl Default for ChargerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manager() {
        let manager = ChargerManager::new();

        // 添加充电桩
        let config = ChargerConfig::default();
        let charger_id = manager.add_charger(config).await.unwrap();

        assert_eq!(manager.get_charger_count().await, 1);

        // 启动充电桩
        assert!(manager.start_charger(&charger_id).await.is_ok());

        // 获取状态
        let state = manager.get_charger_state(&charger_id).await.unwrap();
        assert!(state.connected);

        // 停止充电桩
        assert!(manager.stop_charger(&charger_id).await.is_ok());

        // 移除充电桩
        assert!(manager.remove_charger(&charger_id).await.is_ok());
        assert_eq!(manager.get_charger_count().await, 0);
    }
}
