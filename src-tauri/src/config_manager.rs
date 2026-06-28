use log::{error, info, warn};
use serde_json::Value;
use std::fs;
use tauri::State;

use crate::window_utils::{get_or_create_config_file_path, initilize_config_file, ConfigState};

const KEY_NOT_FOUND_PREFIX: &str = "CONFIG_KEY_NOT_FOUND:";

/// Discord Webhook URL を設定ファイルへ保存します。
///
/// # 概要
/// `appsettings.json` の `DISCORD_WEBHOOK_URL` を更新します。
/// 空文字も有効な値として保存され、その場合 Discord 通知側で送信がスキップされます。
///
/// # 引数
/// * `config_state` - Tauri state に保持されている設定ファイルパス。
/// * `url` - 保存する Discord Webhook URL。
///
/// # 戻り値
/// * `Ok(())` - 設定ファイルの更新に成功した場合。
/// * `Err(String)` - 設定ファイルの読み込み、JSON 解析、または書き込みに失敗した場合。
///
/// # 注意
/// Webhook URL は秘密情報に近いため、更新ログでは値をマスクします。
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

/// 画像差分のしきい値を設定ファイルへ保存します。
///
/// # 概要
/// `appsettings.json` の `THRESHOLD` を更新します。
/// この関数では文字列として保存するだけで、数値範囲の検証は監視開始時に行います。
///
/// # 引数
/// * `config_state` - Tauri state に保持されている設定ファイルパス。
/// * `threshold` - 保存するしきい値。想定値は `0.0` から `1.0` までの文字列です。
///
/// # 戻り値
/// * `Ok(())` - 設定ファイルの更新に成功した場合。
/// * `Err(String)` - 設定ファイルの読み込み、JSON 解析、または書き込みに失敗した場合。
pub async fn update_threshold(
    config_state: State<'_, ConfigState>,
    threshold: String,
) -> Result<(), String> {
    update_config_value(config_state, "THRESHOLD", &threshold, "Threshold updated").await
}

/// 監視間隔を設定ファイルへ保存します。
///
/// # 概要
/// `appsettings.json` の `INTERVAL` を更新します。
/// この関数では文字列として保存するだけで、最小値などの検証は監視開始時に行います。
///
/// # 引数
/// * `config_state` - Tauri state に保持されている設定ファイルパス。
/// * `interval` - 保存する監視間隔。単位はミリ秒です。
///
/// # 戻り値
/// * `Ok(())` - 設定ファイルの更新に成功した場合。
/// * `Err(String)` - 設定ファイルの読み込み、JSON 解析、または書き込みに失敗した場合。
pub async fn update_interval(
    config_state: State<'_, ConfigState>,
    interval: String,
) -> Result<(), String> {
    update_config_value(config_state, "INTERVAL", &interval, "Interval updated").await
}

/// LINE 通知の有効状態を設定ファイルへ保存します。
///
/// # 概要
/// `appsettings.json` の `LINE_ENABLED` を更新します。
/// LINE 通知は任意機能のため、`"true"` の場合だけ送信対象になります。
/// 現在の設定ファイルは既存実装に合わせて文字列値を保存するため、boolean ではなく文字列として扱います。
///
/// # 引数
/// * `config_state` - Tauri state に保持されている設定ファイルパス。
/// * `enabled` - `"true"` または `"false"` を想定した文字列。
///
/// # 戻り値
/// * `Ok(())` - 設定ファイルの更新に成功した場合。
/// * `Err(String)` - 設定ファイルの読み込み、JSON 解析、または書き込みに失敗した場合。
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

/// LINE Channel Access Token を設定ファイルへ保存します。
///
/// # 概要
/// `appsettings.json` の `LINE_CHANNEL_ACCESS_TOKEN` を更新します。
/// LINE 通知は `LINE_ENABLED` が `"true"` で、かつ token と target が両方設定されている場合だけ送信されます。
///
/// # 引数
/// * `config_state` - Tauri state に保持されている設定ファイルパス。
/// * `token` - 保存する LINE Messaging API の Channel Access Token。
///
/// # 戻り値
/// * `Ok(())` - 設定ファイルの更新に成功した場合。
/// * `Err(String)` - 設定ファイルの読み込み、JSON 解析、または書き込みに失敗した場合。
///
/// # 注意
/// Channel Access Token は秘密情報のため、更新ログでは値をマスクします。
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

/// LINE の送信先 ID を設定ファイルへ保存します。
///
/// # 概要
/// `appsettings.json` の `LINE_TARGET` を更新します。
/// 送信先 ID が空の場合、LINE 通知は送信されません。
///
/// # 引数
/// * `config_state` - Tauri state に保持されている設定ファイルパス。
/// * `target` - 保存する LINE のユーザー ID、グループ ID、またはルーム ID。
///
/// # 戻り値
/// * `Ok(())` - 設定ファイルの更新に成功した場合。
/// * `Err(String)` - 設定ファイルの読み込み、JSON 解析、または書き込みに失敗した場合。
pub async fn update_line_target(
    config_state: State<'_, ConfigState>,
    target: String,
) -> Result<(), String> {
    update_config_value(config_state, "LINE_TARGET", &target, "LINE target updated").await
}

/// Discord Webhook URL を設定ファイルから取得します。
///
/// # 戻り値
/// * `Ok(String)` - `DISCORD_WEBHOOK_URL` の値。
/// * `Err(String)` - 設定ファイルの読み込み、JSON 解析、またはキー取得に失敗した場合。
pub async fn get_webhook_url(config_state: State<'_, ConfigState>) -> Result<String, String> {
    get_config_value(config_state, "DISCORD_WEBHOOK_URL").await
}

/// 画像差分のしきい値を設定ファイルから取得します。
///
/// # 戻り値
/// * `Ok(String)` - `THRESHOLD` の値。
/// * `Err(String)` - 設定ファイルの読み込み、JSON 解析、またはキー取得に失敗した場合。
pub async fn get_threshold(config_state: State<'_, ConfigState>) -> Result<String, String> {
    get_config_value(config_state, "THRESHOLD").await
}

/// 監視間隔を設定ファイルから取得します。
///
/// # 戻り値
/// * `Ok(String)` - `INTERVAL` の値。
/// * `Err(String)` - 設定ファイルの読み込み、JSON 解析、またはキー取得に失敗した場合。
pub async fn get_interval(config_state: State<'_, ConfigState>) -> Result<String, String> {
    get_config_value(config_state, "INTERVAL").await
}

/// LINE 通知の有効状態を設定ファイルから取得します。
///
/// # 概要
/// 既存ユーザーの設定ファイルに `LINE_ENABLED` が存在しない場合は、後方互換のため `"false"` を返します。
/// 設定ファイルの読み込みや JSON 解析に失敗した場合は、デフォルト値ではなくエラーを返します。
///
/// # 戻り値
/// * `Ok(String)` - `"true"` または `"false"`。
/// * `Err(String)` - 設定ファイルの読み込み、または JSON 解析に失敗した場合。
pub async fn get_line_enabled(config_state: State<'_, ConfigState>) -> Result<String, String> {
    get_config_value_or_default(config_state, "LINE_ENABLED", "false").await
}

/// LINE Channel Access Token が設定済みかどうかを取得します。
///
/// # 概要
/// token 自体はフロントエンドへ返さず、設定済みかどうかだけを返します。
/// 既存ユーザーの設定ファイルに `LINE_CHANNEL_ACCESS_TOKEN` が存在しない場合は未設定扱いにします。
///
/// # 戻り値
/// * `Ok(true)` - 空白以外の token が保存されている場合。
/// * `Ok(false)` - token が空、またはキーが存在しない場合。
/// * `Err(String)` - 設定ファイルの読み込み、または JSON 解析に失敗した場合。
pub async fn get_line_channel_access_token_configured(
    config_state: State<'_, ConfigState>,
) -> Result<bool, String> {
    let token = get_config_value_or_default(config_state, "LINE_CHANNEL_ACCESS_TOKEN", "").await?;
    Ok(!token.trim().is_empty())
}

/// LINE の送信先 ID を設定ファイルから取得します。
///
/// # 概要
/// 既存ユーザーの設定ファイルに `LINE_TARGET` が存在しない場合は、後方互換のため空文字を返します。
///
/// # 戻り値
/// * `Ok(String)` - `LINE_TARGET` の値。未設定の場合は空文字。
/// * `Err(String)` - 設定ファイルの読み込み、または JSON 解析に失敗した場合。
pub async fn get_line_target(config_state: State<'_, ConfigState>) -> Result<String, String> {
    get_config_value_or_default(config_state, "LINE_TARGET", "").await
}

/// 指定されたキーの設定値を取得します。
///
/// # 概要
/// 設定ファイルを読み込み、指定キーの文字列値を返します。
/// 設定ファイルが読み込めない場合は、親ディレクトリが存在する限り初期設定ファイルを作成して再試行します。
///
/// # 引数
/// * `config_state` - Tauri state に保持されている設定ファイルパス。
/// * `key` - 取得対象の設定キー。
///
/// # 戻り値
/// * `Ok(String)` - 指定キーに対応する文字列値。
/// * `Err(String)` - 読み込み、JSON 解析、またはキー取得に失敗した場合。
///
/// # 注意
/// キーが存在しない場合は `CONFIG_KEY_NOT_FOUND:<key>` 形式のエラーを返します。
/// これは、後方互換用のデフォルト値を返す処理と、設定ファイル破損などの実エラーを区別するためです。
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

/// 指定キーの設定値を取得し、キー欠落時だけデフォルト値を返します。
///
/// # 概要
/// 既存ユーザーの古い設定ファイルに新しい任意設定キーが存在しない場合の後方互換用ヘルパーです。
/// 設定ファイルの読み込み失敗や JSON 解析失敗はデフォルト値で握りつぶさず、エラーとして返します。
///
/// # 引数
/// * `config_state` - Tauri state に保持されている設定ファイルパス。
/// * `key` - 取得対象の設定キー。
/// * `default_value` - キーが存在しない場合だけ返すデフォルト値。
///
/// # 戻り値
/// * `Ok(String)` - 設定値、またはキー欠落時のデフォルト値。
/// * `Err(String)` - 設定ファイルの読み込み、または JSON 解析に失敗した場合。
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

/// 指定されたキーの設定値を更新します。
///
/// # 概要
/// 設定ファイルを JSON として読み込み、指定キーを文字列値で更新して保存します。
/// 存在しないキーも追加されます。
///
/// # 引数
/// * `config_state` - Tauri state に保持されている設定ファイルパス。
/// * `key` - 更新対象の設定キー。
/// * `value` - 保存する文字列値。
/// * `success_log` - 更新成功時に出力するログメッセージ。
///
/// # 戻り値
/// * `Ok(())` - 設定ファイルの更新に成功した場合。
/// * `Err(String)` - 設定ファイルの読み込み、JSON 解析、または書き込みに失敗した場合。
///
/// # 注意
/// `DISCORD_WEBHOOK_URL` と `LINE_CHANNEL_ACCESS_TOKEN` はログ出力時に値をマスクします。
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

/// 設定ファイルを読み込み、JSON として解析します。
///
/// # 引数
/// * `config_path` - 読み込む設定ファイルのパス。
///
/// # 戻り値
/// * `Ok(Value)` - 解析済み JSON。
/// * `Err(String)` - ファイル読み込み、または JSON 解析に失敗した場合。
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

/// JSON 値を設定ファイルへ整形して書き込みます。
///
/// # 引数
/// * `config_path` - 書き込み先の設定ファイルパス。
/// * `json_value` - 保存する JSON 値。
///
/// # 戻り値
/// * `Ok(())` - 書き込みに成功した場合。
/// * `Err(String)` - JSON の整形、またはファイル書き込みに失敗した場合。
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
