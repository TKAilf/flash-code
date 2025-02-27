use std::{fs, path::PathBuf};

use log::{error, info, warn};
use reqwest::Client;
use serde_json::Value;

pub async fn send_line_notification(app_name: &str, config_path: PathBuf) {
    let config_data = match fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(e) => {
            error!("appsettings.jsonの読み込みに失敗しました: {:?}", e);
            return;
        }
    };

    let config: Value = match serde_json::from_str(&config_data) {
        Ok(json) => json,
        Err(e) => {
            error!("appsettings.jsonの解析に失敗しました: {:?}", e);
            return;
        }
    };

    let channel_access_token = match config["LINE_CHANNEL_ACCESS_TOKEN"].as_str() {
        Some(token) => token.to_string(),
        None => {
            error!("設定ファイルにLINE_CHANNEL_ACCESS_TOKENが設定されていません。");
            return;
        }
    };

    let target = match config["LINE_TARGET"].as_str() {
        Some(target) => target,
        None => {
            error!("LINE_TARGETが設定されていません。");
            return;
        }
    };

    let message_text = format!(
        "アプリケーション「{}」のアイコンに変化がありました。",
        app_name
    );

    let payload = serde_json::json!({
        "to": target,
        "messages": [
            {
                "type": "text",
                "text": message_text
            }
        ]
    });

    let client = Client::new();
    let response = client
        .post("https://api.line.me/v2/bot/message/push")
        .header("Authorization", format!("Bearer {}", channel_access_token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                info!("通知を送信しました。");
            } else {
                warn!(
                    "通知の送信に失敗しました。ステータスコード: {}",
                    response.status()
                );
            }
        }
        Err(e) => {
            error!("通知の送信中にエラーが発生しました: {:?}", e);
        }
    }
}
