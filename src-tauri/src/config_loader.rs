use crate::charger::ChargerConfig;
use crate::manager::ChargerManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
struct ChargersFile {
    chargers: Vec<ChargerEntry>,
    batch_config: Option<BatchConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
struct ChargerEntry {
    id: Option<String>,
    name: Option<String>,
    protocol_type: Option<String>,
    server_url: Option<String>,
    max_power: Option<f64>,
    script_path: Option<String>,
    enabled: Option<bool>,
}

impl Default for ChargerEntry {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            protocol_type: None,
            server_url: None,
            max_power: None,
            script_path: None,
            enabled: Some(true),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
struct BatchConfig {
    count: usize,
    protocol_type: Option<String>,
    server_url: Option<String>,
    max_power: Option<f64>,
    script_path: Option<String>,
    enabled: bool,
    name_prefix: Option<String>,
    id_prefix: Option<String>,
    start_index: Option<usize>,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            count: 0,
            protocol_type: None,
            server_url: None,
            max_power: None,
            script_path: None,
            enabled: false,
            name_prefix: None,
            id_prefix: None,
            start_index: None,
        }
    }
}

pub async fn initialize_from_file(manager: Arc<ChargerManager>) -> Result<(), String> {
    let Some(path) = locate_config_file() else {
        tracing::info!("No chargers.json configuration file found, skipping auto initialization");
        return Ok(());
    };

    tracing::info!("Loading charger configuration from {}", path.display());
    let file_content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read charger config {}: {}", path.display(), e))?;

    let config: ChargersFile = serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse charger config: {}", e))?;

    let mut created = Vec::new();

    for entry in config.chargers {
        if entry.enabled.unwrap_or(true) {
            let mut charger_config = ChargerConfig::default();
            if let Some(id) = entry.id {
                charger_config.id = id;
            }
            if let Some(name) = entry.name {
                charger_config.name = name;
            }
            if let Some(protocol) = entry.protocol_type {
                charger_config.protocol_type = protocol;
            }
            if let Some(url) = entry.server_url {
                charger_config.server_url = url;
            }
            if let Some(power) = entry.max_power {
                charger_config.max_power = power;
            }
            charger_config.script_path = entry.script_path;
            charger_config.enabled = true;

            match manager.add_charger(charger_config.clone()).await {
                Ok(id) => created.push(id),
                Err(err) => tracing::warn!("Failed to add charger from config: {}", err),
            }
        }
    }

    if let Some(batch) = config.batch_config {
        if batch.enabled && batch.count > 0 {
            let mut base_config = ChargerConfig::default();
            if let Some(protocol) = batch.protocol_type {
                base_config.protocol_type = protocol;
            }
            if let Some(url) = batch.server_url {
                base_config.server_url = url;
            }
            if let Some(power) = batch.max_power {
                base_config.max_power = power;
            }
            base_config.script_path = batch.script_path;
            base_config.enabled = true;

            let mut existing_ids: HashMap<String, ()> = HashMap::new();
            for id in created.iter() {
                existing_ids.insert(id.clone(), ());
            }

            let start_index = batch.start_index.unwrap_or(1);
            let id_prefix = batch.id_prefix.unwrap_or_else(|| "CP".to_string());
            let name_prefix = batch.name_prefix.unwrap_or_else(|| "充电桩".to_string());

            let mut successful = Vec::new();
            let mut counter = start_index;

            while successful.len() < batch.count {
                let candidate_id = format!("{}{:06}", id_prefix, counter);
                counter += 1;

                if existing_ids.contains_key(&candidate_id) {
                    continue;
                }

                let mut config = base_config.clone();
                config.id = candidate_id.clone();
                config.name = format!("{} #{}", name_prefix, counter - 1);

                match manager.add_charger(config).await {
                    Ok(id) => {
                        existing_ids.insert(id.clone(), ());
                        successful.push(id);
                    }
                    Err(err) => tracing::warn!("Failed to create batch charger: {}", err),
                }
            }

            tracing::info!(
                "Created {} chargers via batch configuration",
                successful.len()
            );
        }
    }

    Ok(())
}

fn locate_config_file() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("CHARGER_CONFIG_PATH") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    let candidates = [
        PathBuf::from("config/chargers.json"),
        PathBuf::from("../config/chargers.json"),
        PathBuf::from("../../config/chargers.json"),
    ];

    for candidate in candidates {
        if candidate.exists() {
            return Some(clean_path(candidate));
        }
    }

    None
}

fn clean_path(path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        match std::fs::canonicalize(&path) {
            Ok(p) => p,
            Err(_) => path,
        }
    }
}

/// 保存充电桩配置到文件
pub async fn save_to_file(manager: Arc<ChargerManager>) -> Result<(), String> {
    let config_path = locate_config_file()
        .or_else(|| Some(PathBuf::from("config/chargers.json")))
        .unwrap();

    // 确保配置目录存在
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    // 获取所有充电桩配置
    let charger_configs = manager.get_all_charger_configs().await;
    
    // 转换为 ChargerEntry
    let chargers: Vec<ChargerEntry> = charger_configs
        .into_iter()
        .map(|config| ChargerEntry {
            id: Some(config.id),
            name: Some(config.name),
            protocol_type: Some(config.protocol_type),
            server_url: Some(config.server_url),
            max_power: Some(config.max_power),
            script_path: config.script_path,
            enabled: Some(config.enabled),
        })
        .collect();

    // 读取现有文件的 batch_config，如果文件存在的话
    let batch_config = if config_path.exists() {
        fs::read_to_string(&config_path)
            .ok()
            .and_then(|content| serde_json::from_str::<ChargersFile>(&content).ok())
            .and_then(|file| file.batch_config)
    } else {
        None
    };

    let chargers_file = ChargersFile {
        chargers,
        batch_config: batch_config.or_else(|| Some(BatchConfig::default())),
    };

    // 序列化为 JSON
    let json_content = serde_json::to_string_pretty(&chargers_file)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    // 写入文件
    fs::write(&config_path, json_content)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    tracing::info!("Saved charger configuration to {}", config_path.display());
    Ok(())
}
