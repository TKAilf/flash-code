use log::{error, info, warn};
use serde_json::Value;
use std::fs;
use tauri::State;

use crate::window_utils::{get_or_create_config_file_path, initilize_config_file, ConfigState};

const KEY_NOT_FOUND_PREFIX: &str = "CONFIG_KEY_NOT_FOUND:";

pub async fn update_webhook_url(
    config_state: State<'_, ConfigState>,
    url: String,
) -> Result<(), String> {
    update_config_value(
        config_state,
        "DISCORD_WEBHOOK_URL",
        &url,
        "Webhook URL updated",
    )
    .await
}

pub async fn update_threshold(
    config_state: State<'_, ConfigState>,
    threshold: String,
) -> Result<(), String> {
    update_config_value(config_state, "THRESHOLD", &threshold, "Threshold updated").await
}

pub async fn update_interval(
    config_state: State<'_, ConfigState>,
    interval: String,
) -> Result<(), String> {
    update_config_value(config_state, "INTERVAL", &interval, "Interval updated").await
}

pub async fn update_line_enabled(
    config_state: State<'_, ConfigState>,
    enabled: String,
) -> Result<(), String> {
    update_config_value(
        config_state,
        "LINE_ENABLED",
        &enabled,
        "LINE enabled flag updated",
    )
    .await
}

pub async fn update_line_channel_access_token(
    config_state: State<'_, ConfigState>,
    token: String,
) -> Result<(), String> {
    update_config_value(
        config_state,
        "LINE_CHANNEL_ACCESS_TOKEN",
        &token,
        "LINE channel access token updated",
    )
    .await
}

pub async fn update_line_target(
    config_state: State<'_, ConfigState>,
    target: String,
) -> Result<(), String> {
    update_config_value(config_state, "LINE_TARGET", &target, "LINE target updated").await
}

pub async fn get_webhook_url(config_state: State<'_, ConfigState>) -> Result<String, String> {
    get_config_value(config_state, "DISCORD_WEBHOOK_URL").await
}

pub async fn get_threshold(config_state: State<'_, ConfigState>) -> Result<String, String> {
    get_config_value(config_state, "THRESHOLD").await
}

pub async fn get_interval(config_state: State<'_, ConfigState>) -> Result<String, String> {
    get_config_value(config_state, "INTERVAL").await
}

pub async fn get_line_enabled(config_state: State<'_, ConfigState>) -> Result<String, String> {
    get_config_value_or_default(config_state, "LINE_ENABLED", "false").await
}

pub async fn get_line_channel_access_token_configured(
    config_state: State<'_, ConfigState>,
) -> Result<bool, String> {
    let token = get_config_value_or_default(config_state, "LINE_CHANNEL_ACCESS_TOKEN", "").await?;
    Ok(!token.trim().is_empty())
}

pub async fn get_line_target(config_state: State<'_, ConfigState>) -> Result<String, String> {
    get_config_value_or_default(config_state, "LINE_TARGET", "").await
}

async fn get_config_value(
    config_state: State<'_, ConfigState>,
    key: &str,
) -> Result<String, String> {
    loop {
        let config_path = config_state.path.to_string_lossy().to_string();
        let json_value = match read_config_file(&config_path) {
            Ok(value) => value,
            Err(e) => {
                error!("Failed to read config file: {:?}", e);
                if let Some(parent_dir) = config_state.path.parent() {
                    let config_file =
                        get_or_create_config_file_path(parent_dir, "appsettings.json");
                    initilize_config_file(&config_file);
                    continue;
                }

                warn!("Config parent directory was not found.");
                return Err(e);
            }
        };

        if let Some(value_str) = json_value[key].as_str() {
            return Ok(value_str.to_string());
        }

        return Err(format!("{}{}", KEY_NOT_FOUND_PREFIX, key));
    }
}

async fn get_config_value_or_default(
    config_state: State<'_, ConfigState>,
    key: &str,
    default_value: &str,
) -> Result<String, String> {
    match get_config_value(config_state, key).await {
        Ok(value) => Ok(value),
        Err(e) if e.starts_with(KEY_NOT_FOUND_PREFIX) => Ok(default_value.to_string()),
        Err(e) => Err(e),
    }
}

async fn update_config_value(
    config_state: State<'_, ConfigState>,
    key: &str,
    value: &str,
    success_log: &str,
) -> Result<(), String> {
    let config_path = config_state.path.to_string_lossy().to_string();
    let mut json_value = match read_config_file(&config_path) {
        Ok(value) => value,
        Err(e) => {
            error!("Failed to read config file: {:?}", e);
            return Err(e);
        }
    };

    if let Some(obj) = json_value.as_object_mut() {
        obj.insert(key.to_string(), Value::String(value.to_string()));
    }

    match write_config_file(&config_path, &json_value) {
        Ok(_) => {
            let log_value = if key == "LINE_CHANNEL_ACCESS_TOKEN" || key == "DISCORD_WEBHOOK_URL" {
                "<redacted>"
            } else {
                value
            };
            info!("{}: {}", success_log, log_value);
            Ok(())
        }
        Err(e) => {
            error!("Failed to write config file: {:?}", e);
            Err(e)
        }
    }
}

fn read_config_file(config_path: &str) -> Result<Value, String> {
    let config_data = fs::read_to_string(config_path).map_err(|e| {
        error!("Failed to read config file: {:?}", e);
        format!("read error: {:?}", e)
    })?;

    serde_json::from_str(&config_data).map_err(|e| {
        error!("Failed to parse config file: {:?}", e);
        format!("parse error: {:?}", e)
    })
}

fn write_config_file(config_path: &str, json_value: &Value) -> Result<(), String> {
    let updated_content = serde_json::to_string_pretty(json_value).map_err(|e| {
        error!("Failed to serialize config file: {:?}", e);
        format!("serialize error: {:?}", e)
    })?;

    fs::write(config_path, updated_content).map_err(|e| {
        error!("Failed to write config file: {:?}", e);
        format!("write error: {:?}", e)
    })
}
