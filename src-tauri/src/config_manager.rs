use log::{error, info, warn};
use serde_json::Value;
use std::fs;
use tauri::State;

use crate::window_utils::{get_or_create_config_file_path, initilize_config_file, ConfigState};

pub async fn update_webhook_url(
    config_state: State<'_, ConfigState>,
    url: String,
) -> Result<(), String> {
    update_config_value(
        config_state,
        "DISCORD_WEBHOOK_URL",
        &url,
        "Webhook URLを更新しました",
    )
    .await
}

pub async fn update_threshold(
    config_state: State<'_, ConfigState>,
    threshold: String,
) -> Result<(), String> {
    update_config_value(
        config_state,
        "THRESHOLD",
        &threshold,
        "しきい値を更新しました",
    )
    .await
}

pub async fn update_interval(
    config_state: State<'_, ConfigState>,
    interval: String,
) -> Result<(), String> {
    update_config_value(
        config_state,
        "INTERVAL",
        &interval,
        "監視間隔を更新しました",
    )
    .await
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

/// 非同期関数 get_config_value
///
/// # 概要
/// 指定された設定ファイルから、与えられたキーに対応する値を取得します。  
/// もし設定ファイルの読み込みに失敗した場合、親ディレクトリから設定ファイルを再生成し、  
/// 再試行するループ処理を実装しています。
///
/// # 引数
/// * `config_state` - 設定ファイルのパスなどの状態を保持する `ConfigState` のラッパー（Tauri の State）
/// * `key` - 取得対象の設定キー
///
/// # 戻り値
/// * `Ok(String)` - 設定ファイルに指定されたキーが存在し、その値が文字列として取得できた場合
/// * `Err(String)` - 設定ファイルの読み込みに失敗した場合や、指定されたキーが存在しない場合のエラーメッセージ
///
/// # 処理の概要
/// 1. 設定ファイルのパス（`config_state.path`）からファイル内容を読み込み、JSONとして解析する。
/// 2. もし読み込みに失敗した場合、親ディレクトリから `appsettings.json` を再生成するため  
///    `get_or_create_config_file_path` を呼び出し、再試行のためループ先頭に戻る。
/// 3. JSON 内に指定されたキーが存在する場合、その値を文字列として返す。存在しなければエラーを返す.
///
/// # 例
/// ```rust
/// #[tauri::command]
/// async fn example(config_state: State<'_, ConfigState>) {
///     match get_config_value(config_state, "THRESHOLD").await {
///         Ok(value) => println!("しきい値: {}", value),
///         Err(e) => eprintln!("取得に失敗: {}", e),
///     }
/// }
/// ```
///
async fn get_config_value(
    config_state: State<'_, ConfigState>,
    key: &str,
) -> Result<String, String> {
    loop {
        let config_path = config_state.path.to_string_lossy().to_string();
        let json_value: Value = match read_config_file(&config_path) {
            Ok(value) => value,
            Err(e) => {
                error!("設定ファイルの読み込みに失敗しました: {:?}", e);
                if let Some(parent_dir) = config_state.path.parent() {
                    let config_file =
                        get_or_create_config_file_path(&parent_dir, "appsettings.json");
                    initilize_config_file(&config_file);
                    // ファイルを再生成した後、ループの先頭に戻り再試行する
                    continue;
                } else {
                    warn!("親ディレクトリが見つかりません。");
                    return Err(e);
                }
            }
        };

        if let Some(value_str) = json_value[key].as_str() {
            return Ok(value_str.to_string());
        } else {
            return Err("指定されたキーが見つかりません".to_string());
        }
    }
}

async fn update_config_value(
    config_state: State<'_, ConfigState>,
    key: &str,
    value: &str,
    success_log: &str,
) -> Result<(), String> {
    let config_path = config_state.path.to_string_lossy().to_string();
    let mut json_value: Value = match read_config_file(&config_path) {
        Ok(value) => value,
        Err(e) => {
            error!("設定ファイルの読み込みに失敗しました: {:?}", e);
            return Err(e);
        }
    };

    if let Some(obj) = json_value.as_object_mut() {
        obj.insert(key.to_string(), Value::String(value.to_string()));
    }

    match write_config_file(&config_path, &json_value) {
        Ok(_) => {
            info!("{}: {}", success_log, value);
            Ok(())
        }
        Err(e) => {
            error!("設定ファイルの書き込みに失敗しました: {:?}", e);
            Err(e)
        }
    }
}

/// 設定ファイルを読み込み、`serde_json::Value` として返す関数。
///
/// # 概要
/// 指定されたパスの設定ファイル（JSON形式）を読み込み、  
/// パースして `serde_json::Value` 型として返します。
///
/// # 引数
/// * `config_path` - 読み込む設定ファイルのパス。
///
/// # 戻り値
/// `Result<Value, String>`:
/// - `Ok(Value)`: 正常にファイルが読み込まれ、JSONとして解析された場合。
/// - `Err(String)`: ファイルの読み込みや解析に失敗した場合、エラーメッセージを含む文字列を返します。
///
/// # 注意点
/// - ファイルが存在しない、または読み取り権限がない場合はエラーとなります。
/// - ファイル内容が正しいJSON形式でない場合、解析エラーが返されます。
///
/// # 使用例
/// ```rust
/// use serde_json::Value;
///
/// let config_path = "path/to/appsettings.json";
/// match read_config_file(config_path) {
///     Ok(config) => println!("設定ファイルを読み込みました: {:?}", config),
///     Err(e) => eprintln!("設定ファイルの読み込みに失敗しました: {}", e),
/// }
/// ```
///
fn read_config_file(config_path: &str) -> Result<Value, String> {
    let config_data = match fs::read_to_string(&config_path).map_err(|e| {
        error!("設定ファイルの読み込みに失敗しました: {:?}", e);
        format!("読み込みエラー: {:?}", e)
    }) {
        Ok(content) => content,
        Err(e) => {
            error!("設定ファイルの読み込みに失敗しました: {:?}", e);
            return Err(e);
        }
    };

    serde_json::from_str(&config_data).map_err(|e| {
        error!("設定ファイルの解析に失敗しました: {:?}", e);
        format!("解析エラー: {:?}", e)
    })
}

/// 設定ファイルにJSONデータを書き込む関数。
///
/// # 概要
/// 指定されたパスに、与えられた `serde_json::Value` をJSON形式で書き込みます。
///
/// # 引数
/// * `config_path` - 書き込む設定ファイルのパス。
/// * `json_value` - ファイルに書き込むJSONデータ。
///
/// # 戻り値
/// `Result<(), String>`:
/// - `Ok(())`: 正常にファイルが書き込まれた場合。
/// - `Err(String)`: ファイルのシリアライズまたは書き込みに失敗した場合、エラーメッセージを含む文字列を返します。
///
/// # 注意点
/// - ファイルの書き込み権限がない場合はエラーとなります。
/// - 書き込むJSONデータがシリアライズできない場合、エラーが発生します。
///
/// # 使用例
/// ```rust
/// use serde_json::json;
///
/// let config_path = "path/to/appsettings.json";
/// let json_data = json!({
///     "DISCORD_WEBHOOK_URL": "https://example.com/webhook"
/// });
///
/// match write_config_file(config_path, &json_data) {
///     Ok(_) => println!("設定ファイルを正常に書き込みました。"),
///     Err(e) => eprintln!("設定ファイルの書き込みに失敗しました: {}", e),
/// }
/// ```
///
fn write_config_file(config_path: &str, json_value: &Value) -> Result<(), String> {
    let updated_content = match serde_json::to_string_pretty(&json_value).map_err(|e| {
        error!("設定ファイルのシリアライズに失敗しました: {:?}", e);
        format!("シリアライズエラー: {:?}", e)
    }) {
        Ok(content) => content,
        Err(e) => {
            error!("設定ファイルのシリアライズに失敗しました: {:?}", e);
            return Err(e);
        }
    };

    match fs::write(&config_path, updated_content).map_err(|e| {
        error!("設定ファイルの書き込みに失敗しました: {:?}", e);
        format!("書き込みエラー: {:?}", e)
    }) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
