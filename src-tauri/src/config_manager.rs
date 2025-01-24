use log::{error, info};
use serde_json::Value;
use std::fs;
use tauri::State;

use crate::window_utils::ConfigState;

/// Webhook URLを更新する関数。
///
/// # 概要
/// 設定ファイル（appsettings.json）の`DISCORD_WEBHOOK_URL`項目を更新します。  
/// 新しいWebhook URLを指定し、設定ファイルに保存します。
///
/// # 引数
/// * `config_state` - 設定ファイルのパスを管理する `ConfigState`。
/// * `url` - 新しいWebhook URL。
///
/// # 戻り値
/// `Result`:
/// - `Ok(())`: Webhook URLが正常に更新された場合。
/// - `Err(String)`: 設定ファイルの読み書きや解析に失敗した場合、そのエラー内容を含むメッセージ。
///
/// # 注意点
/// - URLの形式が正しいかどうかはこの関数では検証されません。
/// - 書き込み権限がない場合や設定ファイルが壊れている場合、エラーが発生します。
///
/// # 使用例
/// ```rust
/// use tauri::State;
/// use flash_code::ConfigState;
///
/// #[tauri::command]
/// async fn update_webhook_url_example(config_state: State<'_, ConfigState>, url: String) {
///     match update_webhook_url(config_state, url).await {
///         Ok(_) => println!("Webhook URLが正常に更新されました。"),
///         Err(e) => eprintln!("Webhook URLの更新に失敗しました: {}", e),
///     }
/// }
/// ```
///
pub async fn update_webhook_url(
    config_state: State<'_, ConfigState>,
    url: String,
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
        obj.insert(
            "DISCORD_WEBHOOK_URL".to_string(),
            Value::String(url.clone()),
        );
    }

    match write_config_file(&config_path, &json_value) {
        Ok(_) => {}
        Err(e) => {
            error!("設定ファイルの書き込みに失敗しました: {:?}", e);
            return Err(e);
        }
    };

    info!("Webhook URLを更新しました: {}", url);
    Ok(())
}

/// Webhook URLを取得する関数。
///
/// # 概要
/// 設定ファイル（appsettings.json）から`DISCORD_WEBHOOK_URL`項目を取得します。
///
/// # 引数
/// * `config_state` - 設定ファイルのパスを管理する `ConfigState`。
///
/// # 戻り値
/// `Result`:
/// - `Ok(String)`: Webhook URLが正常に取得された場合。その値を返します。
/// - `Err(String)`: 設定ファイルの読み取りや解析に失敗した場合、または`DISCORD_WEBHOOK_URL`が存在しない場合にエラーメッセージを返します。
///
/// # 注意点
/// - 設定ファイルが存在しない、または正しく構成されていない場合、エラーとなります。
/// - `DISCORD_WEBHOOK_URL`が設定されていない場合、エラーが返されます。
///
/// # 使用例
/// ```rust
/// use tauri::State;
/// use flash_code::ConfigState;
///
/// #[tauri::command]
/// async fn get_webhook_url_example(config_state: State<'_, ConfigState>) {
///     match get_webhook_url(config_state).await {
///         Ok(url) => println!("Webhook URL: {}", url),
///         Err(e) => eprintln!("Webhook URLの取得に失敗しました: {}", e),
///     }
/// }
/// ```
///
pub async fn get_webhook_url(config_state: State<'_, ConfigState>) -> Result<String, String> {
    let config_path = config_state.path.to_string_lossy().to_string();
    let json_value: Value = match read_config_file(&config_path) {
        Ok(value) => value,
        Err(e) => {
            error!("設定ファイルの読み込みに失敗しました: {:?}", e);
            return Err(e);
        }
    };

    json_value["DISCORD_WEBHOOK_URL"]
        .as_str()
        .map(|url| url.to_string())
        .ok_or_else(|| {
            error!("設定ファイルにDISCORD_WEBHOOK_URLが設定されていません。");
            "Webhook URLが設定されていません".to_string()
        })
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
