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
use std::{env, fs};

use log::{info, warn};
use tauri::Manager;
use tauri_plugin_log::LogTarget;
use window_utils::{ConfigState, MonitorState};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // アプリのシステム設定フォルダパスを取得
            if let Some(config_dir) = app.path_resolver().app_config_dir() {
                let config_file = config_dir.join("appsettings.json");

                // 設定ファイルの存在を確認なければ作成
                if !config_file.exists() {
                    info!("設定ファイルが存在しません。デフォルトの設定ファイルを生成します。");

                    if let Ok(_) = fs::create_dir_all(&config_dir) {
                        info!("設定ファイルのディレクトリを作成しました。");
                    } else {
                        warn!("設定ファイルのディレクトリの作成に失敗しました。");
                    };

                    if let Ok(_) = fs::write(
                        &config_file,
                        r#"{
    "DISCORD_WEBHOOK_URL": "ここにDiscordBotURLを入力してください",
    "THRESHOLD": "0.034",
    "INTERVAL": "3000"
}"#,
                    ) {
                        info!("設定ファイルを作成しました。");
                    } else {
                        warn!("設定ファイルの作成に失敗しました。");
                    };
                } else {
                    info!("設定ファイルが存在します。");
                }
                app.manage(ConfigState { path: config_file });
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
