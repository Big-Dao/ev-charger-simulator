// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod charger;
mod commands;
mod config_loader;
mod manager;
mod ocpp_client;
mod ocpp_messages;
mod protocol;
mod script_engine;
mod state;

use std::sync::Arc;
use tracing_subscriber;

use tauri::Manager;

fn main() {
    // 初始化日志系统
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting EV Charger Simulator...");

    // 创建充电桩管理器（使用 Arc 包装以便共享）
    let manager = Arc::new(manager::ChargerManager::new());

    // 创建脚本引擎 (Deno Core)
    let script_engine = script_engine::ScriptEngine::new(manager.clone())
        .expect("Failed to initialize script engine");

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::get_charger_list,
            commands::start_charger,
            commands::stop_charger,
            commands::start_all_chargers,
            commands::stop_all_chargers,
            commands::get_charger_status,
            commands::get_charger_config,
            commands::update_charger_config,
            commands::send_charger_command,
            commands::add_charger,
            commands::remove_charger,
            commands::create_batch_chargers,
            commands::get_statistics,
            // 脚本引擎命令
            commands::execute_script,
            commands::stop_script,
            commands::is_script_running,
            commands::set_charger_script,
            commands::clear_charger_script,
            commands::start_charger_script,
            commands::stop_charger_script,
            // 预设脚本命令
            commands::get_preset_scripts,
            commands::read_preset_script,
        ])
        .manage(manager)
        .manage(script_engine)
        .setup(|app| {
            if let Some(manager_state) = app.try_state::<Arc<manager::ChargerManager>>() {
                let manager = manager_state.inner().clone();
                if let Err(err) =
                    tauri::async_runtime::block_on(config_loader::initialize_from_file(manager))
                {
                    tracing::warn!("Failed to initialize chargers from config: {}", err);
                }
            } else {
                tracing::warn!("Charger manager state is not available during setup");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
