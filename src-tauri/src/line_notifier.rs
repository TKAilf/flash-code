use std::{fs, path::PathBuf};

use log::{error, info, warn};
use reqwest::Client;
use serde_json::Value;

pub async fn send_line_notification(app_name: &str, config_path: PathBuf) {
    let config_data = match fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read appsettings.json: {:?}", e);
            return;
        }
    };

    let config: Value = match serde_json::from_str(&config_data) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to parse appsettings.json: {:?}", e);
            return;
        }
    };

    if config["LINE_ENABLED"].as_str().unwrap_or("false") != "true" {
        info!("LINE notification is disabled. Skipping notification.");
        return;
    }

    let channel_access_token = config["LINE_CHANNEL_ACCESS_TOKEN"].as_str().unwrap_or("");
    let target = config["LINE_TARGET"].as_str().unwrap_or("");

    if channel_access_token.trim().is_empty() || target.trim().is_empty() {
        info!("LINE notification settings are incomplete. Skipping notification.");
        return;
    }

    let message_text = format!("Application \"{}\" taskbar icon changed.", app_name);

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
                info!("LINE notification sent.");
            } else {
                warn!(
                    "LINE notification failed with status code: {}",
                    response.status()
                );
            }
        }
        Err(e) => {
            error!("Failed to send LINE notification: {:?}", e);
        }
    }
}
