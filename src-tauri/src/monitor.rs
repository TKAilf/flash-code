// monitor.rs
use crate::{
    discord_notifier::send_discord_notification, image_comparison::has_significant_difference,
    screen_capture::capture_icon_image, window_utils::AppInfo,
};
use log::{error, info};
use std::{path::PathBuf, time::Duration};
use tokio::time::sleep;
use windows::Win32::Foundation::HWND;

/// 監視対象のアプリケーションアイコンを定期的にチェックする非同期関数。
///
/// # 概要
/// 指定したアプリケーションのアイコンを一定間隔でキャプチャし、
/// 初期状態のアイコンと比較して変化があった場合に通知を送信します。
///
/// # 引数
/// - `app_info`: 監視対象アプリケーションの情報（`AppInfo`）。
/// - `interval_ms`: チェック間隔（ミリ秒）。
/// - `threshold`: 画像比較のしきい値（`0.0〜1.0`）。
/// - `config_path`: 通知送信に必要な設定ファイルのパス。
///
/// # 使用例
/// ```rust
/// use my_crate::monitor::monitor_app_icon;
/// use std::path::PathBuf;
///
/// let app_info = AppInfo { /* 初期化 */ };
/// let interval_ms = 5000;
/// let threshold = 0.05;
/// let config_path = PathBuf::from("path/to/config.json");
///
/// monitor_app_icon(app_info, interval_ms, threshold, config_path).await;
/// ```
///
pub async fn monitor_app_icon(
    app_info: AppInfo,
    interval_ms: u64,
    threshold: f32,
    config_path: PathBuf,
) {
    info!("monitor_app_iconを呼び出しました。");
    // 初期状態のアイコン画像を取得
    let initial_image =
        capture_icon_image(HWND(app_info.hwnd as *mut _)).expect("初期画像の取得に失敗しました");

    info!("アイコンの監視ループを開始します。");
    loop {
        // 一定時間待機
        sleep(Duration::from_millis(interval_ms)).await;

        // 現在のアイコン画像を取得
        let current_image = match capture_icon_image(HWND(app_info.hwnd as *mut _)) {
            Some(img) => img,
            None => {
                error!("アイコンの取得に失敗しました。");
                continue;
            }
        };
        info!("アイコンの取得に成功しました。");

        // 画像比較
        if has_significant_difference(&initial_image, &current_image, threshold) {
            info!("アイコンに変化がありました。");
            // 変化が検知された場合の処理
            send_discord_notification(&app_info.name, config_path).await;
            break;
        }
        info!("アイコンに変化はありませんでした。");
    }
}
