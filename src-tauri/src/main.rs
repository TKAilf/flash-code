// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config_manager;
mod discord_notifier;
mod icon_position;
mod image_comparison;
mod monitor;
mod screen_capture;
mod window_collection;
mod window_utils;
use std::env;

use log::warn;
use tauri::Manager;
use tauri_plugin_log::LogTarget;
use window_utils::{get_or_create_config_file_path, ConfigState, MonitorState};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // アプリのシステム設定フォルダパスを取得
            if let Some(config_dir) = app.path_resolver().app_config_dir() {
                let config_file = get_or_create_config_file_path(&config_dir, "appsettings.json");
                app.manage(ConfigState {
                    path: config_file.clone(),
                });
            } else {
                warn!("アプリのシステム設定フォルダパスが取得できませんでした。");
            }

            app.manage(MonitorState::new());

            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .targets(vec![LogTarget::LogDir])
                .log_name({
                    #[cfg(debug_assertions)]
                    {
                        "flash_code_debug"
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        "flash_code"
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::start_monitoring,
            commands::stop_monitoring,
            commands::get_taskbar_apps,
            commands::update_webhook_url,
            commands::update_threshold,
            commands::update_interval,
            commands::get_webhook_url,
            commands::get_threshold,
            commands::get_interval
        ])
        .run(tauri::generate_context!())
        .expect("tauri::Builder::default() | run: error while running tauri application");
}
