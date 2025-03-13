use log::{error, info};
use tauri::State;

use crate::window_utils::{AppInfo, ConfigState, MonitorState};

/// 監視を開始するコマンド。
///
/// # 概要
/// タスクバー上のアプリケーションを監視します。  
/// 各アプリは指定された監視間隔（ミリ秒）および画像比較のしきい値を用いてモニタリングされます。
///
/// # 引数
/// * `monitor_state` - `MonitorState`の状態。監視の管理に使用されます。
/// * `config_state` - `ConfigState`の状態。アプリケーションの設定を提供します。
/// * `app_handle` - Tauriの`AppHandle`。
/// * `apps` - 監視対象のアプリケーション情報のリスト。
///
/// # 使用例
/// ```rust
/// use tauri::State;
/// use flash_code::{MonitorState, ConfigState, AppInfo};
///
/// #[tauri::command]
/// async fn start_monitoring_example(
///     monitor_state: State<'_, MonitorState>,
///     config_state: State<'_, ConfigState>,
///     app_handle: tauri::AppHandle,
///     apps: Vec<AppInfo>,
/// ) {
///     start_monitoring(monitor_state, config_state, apps).await.unwrap();
/// }
/// ```
///
#[tauri::command]
pub async fn start_monitoring(
    monitor_state: State<'_, MonitorState>,
    config_state: State<'_, ConfigState>,
    app_handle: tauri::AppHandle,
    apps: Vec<AppInfo>,
) -> Result<(), String> {
    info!("start_monitoringを呼び出しました。");
    monitor_state.stop_all(apps.clone()).await;
    for app in apps {
        let interval_str = match get_interval(config_state.clone()).await {
            Ok(val) => val,
            Err(e) => {
                error!("監視間隔の取得に失敗しました: {}", e);
                return Err(e);
            }
        };
        let interval = match interval_str.parse::<u64>() {
            Ok(val) => val,
            Err(e) => {
                error!("監視間隔の文字列の解析に失敗しました: {}", e);
                return Err(format!("監視間隔のパースに失敗しました: {}", e));
            }
        };
        let threshold_str = match get_threshold(config_state.clone()).await {
            Ok(val) => val,
            Err(e) => {
                error!("しきい値の取得に失敗しました: {}", e);
                return Err(e);
            }
        };
        let threshold = match threshold_str.parse::<f32>() {
            Ok(val) => val,
            Err(e) => {
                error!("しきい値の文字列の解析に失敗しました: {}", e);
                return Err(format!("しきい値のパースに失敗しました: {}", e));
            }
        };
        monitor_state
            .monitor_target(
                app,
                interval,
                threshold,
                config_state.clone(),
                app_handle.clone(),
            )
            .await;
    }
    Ok(())
}

/// 監視を停止するコマンド。
///
/// # 概要
/// 現在実行中のすべての監視プロセスを停止します。  
/// 停止が正常に完了した場合、成功の結果を返します。
///
/// # 引数
/// * `monitor_state` - 監視タスクの管理を行う `MonitorState`。
///
/// # 戻り値
/// `Result`:
/// - `Ok(())`: 正常にすべての監視が停止した場合。
/// - `Err(String)`: 停止処理中にエラーが発生した場合。
///
/// # 注意点
/// - この関数ではエラー処理を実施しません。エラーは呼び出し元で処理する必要があります。
///
/// # 使用例
/// ```rust
/// use tauri::State;
/// use flash_code::MonitorState;
///
/// #[tauri::command]
/// async fn stop_monitoring_example(monitor_state: State<'_, MonitorState>) {
///     if let Err(e) = stop_monitoring(monitor_state).await {
///         eprintln!("監視の停止に失敗しました: {}", e);
///     } else {
///         println!("すべての監視が正常に停止しました。");
///     }
/// }
/// ```
///
#[tauri::command]
pub async fn stop_monitoring(
    monitor_state: State<'_, MonitorState>,
    apps: Vec<AppInfo>,
) -> Result<(), String> {
    monitor_state.stop_all(apps).await;
    Ok(())
}

/// タスクバーに表示されているアプリ情報を取得するコマンド。
///
/// # 概要
/// タスクバーに現在表示されているすべてのアプリケーションの情報を取得します。
///
/// # 戻り値
/// タスクバーに表示されている`AppInfo`オブジェクトのベクターを返します。
///
/// # 使用例
/// ```rust
/// use flash_code::AppInfo;
///
/// #[tauri::command]
/// pub fn get_taskbar_apps_example() -> Vec<AppInfo> {
///     get_taskbar_apps()
/// }
/// ```
///
#[tauri::command]
pub fn get_taskbar_apps() -> Vec<AppInfo> {
    crate::window_collection::get_taskbar_apps()
}

/// Webhook URLを更新する関数。
///
/// # 概要
/// 設定ファイル（appsettings.json）の `DISCORD_WEBHOOK_URL` 項目を更新します。  
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
#[tauri::command]
pub async fn update_webhook_url(
    config_state: State<'_, ConfigState>,
    url: String,
) -> Result<(), String> {
    match crate::config_manager::update_webhook_url(config_state, url).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Webhook URLの更新に失敗しました: {}", e);
            Err(e)
        }
    }
}

/// しきい値を更新する関数。
///
/// # 概要
/// 設定ファイル（appsettings.json）の `THRESHOLD` 項目を更新します。  
/// 新しいしきい値を指定し、設定ファイルに保存します。
///
/// # 引数
/// * `config_state` - 設定ファイルのパスを管理する `ConfigState`。
/// * `threshold` - 新しいしきい値（文字列形式）。
///
/// # 戻り値
/// `Result`:
/// - `Ok(())`: しきい値が正常に更新された場合。
/// - `Err(String)`: 設定ファイルの読み書きまたは解析に失敗した場合、そのエラー内容を含むメッセージ。
///
/// # 注意点
/// - しきい値の形式が正しいかどうかはこの関数では検証されません。
///
/// # 使用例
/// ```rust
/// use tauri::State;
/// use flash_code::ConfigState;
///
/// #[tauri::command]
/// async fn update_threshold_example(config_state: State<'_, ConfigState>, threshold: String) {
///     match update_threshold(config_state, threshold).await {
///         Ok(_) => println!("しきい値が正常に更新されました。"),
///         Err(e) => eprintln!("しきい値の更新に失敗しました: {}", e),
///     }
/// }
/// ```
///
#[tauri::command]
pub async fn update_threshold(
    config_state: State<'_, ConfigState>,
    threshold: String,
) -> Result<(), String> {
    match crate::config_manager::update_threshold(config_state, threshold).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("しきい値の更新に失敗しました: {}", e);
            Err(e)
        }
    }
}

/// 監視間隔を更新する関数。
///
/// # 概要
/// 設定ファイル（appsettings.json）の `INTERVAL` 項目を更新します。  
/// 新しい監視間隔（ミリ秒単位）を指定し、設定ファイルに保存します。
///
/// # 引数
/// * `config_state` - 設定ファイルのパスを管理する `ConfigState`。
/// * `interval` - 新しい監視間隔（文字列形式、ミリ秒単位）。
///
/// # 戻り値
/// `Result`:
/// - `Ok(())`: 監視間隔が正常に更新された場合。
/// - `Err(String)`: 設定ファイルの読み書きまたは解析に失敗した場合、そのエラー内容を含むメッセージ。
///
/// # 注意点
/// - 監視間隔の形式が正しいかどうかはこの関数では検証されません。
///
/// # 使用例
/// ```rust
/// use tauri::State;
/// use flash_code::ConfigState;
///
/// #[tauri::command]
/// async fn update_interval_example(config_state: State<'_, ConfigState>, interval: String) {
///     match update_interval(config_state, interval).await {
///         Ok(_) => println!("監視間隔が正常に更新されました。"),
///         Err(e) => eprintln!("監視間隔の更新に失敗しました: {}", e),
///     }
/// }
/// ```
///
#[tauri::command]
pub async fn update_interval(
    config_state: State<'_, ConfigState>,
    interval: String,
) -> Result<(), String> {
    match crate::config_manager::update_interval(config_state, interval).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("監視間隔の更新に失敗しました: {}", e);
            Err(e)
        }
    }
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
#[tauri::command]
pub async fn get_webhook_url(config_state: State<'_, ConfigState>) -> Result<String, String> {
    match crate::config_manager::get_webhook_url(config_state).await {
        Ok(url) => Ok(url),
        Err(e) => {
            error!("Webhook URLの取得に失敗しました: {}", e);
            Err(e)
        }
    }
}

/// 設定ファイルからしきい値を取得する関数。
///
/// # 概要
/// 設定ファイル（例: appsettings.json）に保存されている `THRESHOLD` 値を取得します。
///
/// # 引数
/// * `config_state` - 設定ファイルのパスや状態を管理する `ConfigState`。
///
/// # 戻り値
/// `Result<String, String>`
/// - `Ok(String)`: 取得に成功した場合、しきい値が文字列として返されます。
/// - `Err(String)`: 取得に失敗した場合、エラー内容が文字列で返されます。
///
/// # 使用例
/// ```rust
/// #[tauri::command]
/// async fn example(config_state: State<'_, ConfigState>) {
///     match get_threshold(config_state).await {
///         Ok(threshold) => println!("しきい値: {}", threshold),
///         Err(e) => eprintln!("取得に失敗: {}", e),
///     }
/// }
/// ```
#[tauri::command]
pub async fn get_threshold(config_state: State<'_, ConfigState>) -> Result<String, String> {
    match crate::config_manager::get_threshold(config_state).await {
        Ok(url) => Ok(url),
        Err(e) => {
            error!("しきい値の取得に失敗しました: {}", e);
            Err(e)
        }
    }
}

/// 設定ファイルから監視間隔を取得する関数。
///
/// # 概要
/// 設定ファイル（例: appsettings.json）に保存されている `INTERVAL` 値を取得します。
///
/// # 引数
/// * `config_state` - 設定ファイルのパスや状態を管理する `ConfigState`。
///
/// # 戻り値
/// `Result<String, String>`
/// - `Ok(String)`: 取得に成功した場合、監視間隔が文字列として返されます。
/// - `Err(String)`: 取得に失敗した場合、エラー内容が文字列で返されます。
///
/// # 使用例
/// ```rust
/// #[tauri::command]
/// async fn example(config_state: State<'_, ConfigState>) {
///     match get_interval(config_state).await {
///         Ok(interval) => println!("監視間隔: {}", interval),
///         Err(e) => eprintln!("取得に失敗: {}", e),
///     }
/// }
/// ```
///
#[tauri::command]
pub async fn get_interval(config_state: State<'_, ConfigState>) -> Result<String, String> {
    match crate::config_manager::get_interval(config_state).await {
        Ok(url) => Ok(url),
        Err(e) => {
            error!("監視間隔の取得に失敗しました: {}", e);
            Err(e)
        }
    }
}
