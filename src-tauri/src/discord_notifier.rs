use log::{error, info, warn};
use reqwest::Client;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

/// Discordに通知を送信する非同期関数。
///
/// # 概要
/// 指定されたアプリケーション名に基づき、DiscordのWebhook URLを使用して通知を送信します。
/// 通知内容はアプリケーションのアイコンの変化を示します。
///
/// # 引数
/// - `app_name`:
///   - 変化が検知されたアプリケーション名。通知メッセージ内で使用されます。
/// - `config_path`:
///   - `appsettings.json` のパス。ここからDiscord Webhook URLを取得します。
///
/// # 処理概要
/// 1. 指定された設定ファイルを読み込む。
/// 2. 設定ファイルから`DISCORD_WEBHOOK_URL`を取得。
/// 3. Webhook URLに対して通知メッセージを送信。
/// 4. 送信結果に応じてログを出力。
///
/// # 注意点
/// - 設定ファイルが存在しない、または内容が不正な場合、通知は送信されません。
/// - Webhook URLが設定ファイルに含まれていない場合もエラーとなります。
///
/// # 使用例
/// ```rust
/// use std::path::PathBuf;
/// use my_crate::discord_notifier::send_discord_notification;
///
/// #[tokio::main]
/// async fn main() {
///     let config_path = PathBuf::from("path/to/appsettings.json");
///     send_discord_notification("ExampleApp", config_path).await;
/// }
/// ```
pub async fn send_discord_notification(app_name: &str, config_path: PathBuf) {
    let webhook_url = {
        let config_data = match fs::read_to_string(config_path) {
            Ok(content) => content,
            Err(e) => {
                error!("appsettings.jsonの読み込みに失敗しました: {:?}", e);
                return;
            }
        };
        info!("config_data: {:?}", config_data);

        let config: Value = match serde_json::from_str(&config_data) {
            Ok(json) => json,
            Err(e) => {
                error!("appsettings.jsonの解析に失敗しました: {:?}", e);
                return;
            }
        };
        info!("config: {:?}", config);

        match config["DISCORD_WEBHOOK_URL"].as_str() {
            Some(url) => url.to_string(),
            None => {
                error!("appsettings.jsonにDISCORD_WEBHOOK_URLが設定されていません。");
                return;
            }
        }
    };

    let client = Client::new();
    let content = format!(
        "アプリケーション「{}」のアイコンに変化がありました。",
        app_name
    );
    let payload = serde_json::json!({
        "content": content
    });
    let _ = match client.post(&webhook_url).json(&payload).send().await {
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
    };
}
