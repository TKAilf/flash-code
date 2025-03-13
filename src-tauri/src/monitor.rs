// monitor.rs
use crate::{
    discord_notifier::send_discord_notification, image_comparison::has_significant_difference,
    line_notifier::send_line_notification, screen_capture::capture_icon_image,
    window_utils::AppInfo,
};
use log::{error, info};
use std::{path::PathBuf, time::Duration};
use tauri::Manager;
use tokio::time::sleep;
use windows::Win32::{
    Foundation::{FALSE, HWND, TRUE},
    UI::WindowsAndMessaging::{ShowWindow, SW_MINIMIZE, SW_RESTORE},
};

/// 監視対象のアプリケーションアイコンを定期的にチェックする非同期関数。
///
/// # 概要
/// 指定したアプリケーションのアイコンを一定間隔でキャプチャし、
/// 初期状態のアイコンと比較して変化があった場合に通知を送信します。
///
/// # 引数
/// - `app_info`: 監視対象アプリケーションの情報（`AppInfo`）。
/// - `interval`: チェック間隔（ミリ秒）。
/// - `threshold`: 画像比較のしきい値（`0.0〜1.0`）。
/// - `config_path`: 通知送信に必要な設定ファイルのパス。
/// - `app_handle`: Tauri の AppHandle。`emit_all` を用いて全ウィンドウへ "monitoring_stopped" イベントを発行するために利用。
///
/// # 使用例
/// ```rust
/// use my_crate::monitor::monitor_app_icon;
/// use std::path::PathBuf;
///
/// let app_info = AppInfo { /* 初期化 */ };
/// let interval = 3000;
/// let threshold = 0.050;
/// let config_path = PathBuf::from("path/to/config.json");
/// // Tauri 側で AppHandle を取得する必要があります。
/// // ここでは仮に `app_handle` として取得したものを渡す例です。
/// let app_handle = tauri::AppHandle::current();
///
/// monitor_app_icon(app_info, interval, threshold, config_path, app_handle).await;
/// ```
///
pub async fn monitor_app_icon(
    app_info: AppInfo,
    interval: u64,
    threshold: f32,
    config_path: PathBuf,
    app_handle: tauri::AppHandle,
) {
    info!("monitor_app_iconを呼び出しました。");
    // 対象ウィンドウの最小化
    unsafe {
        let result = ShowWindow(HWND(app_info.hwnd as *mut _), SW_MINIMIZE);
        match result {
            FALSE => {
                error!("ウィンドウの最小化に失敗しました。");
                return;
            }
            TRUE => {
                info!("ウィンドウを最小化しました。");
            }
            _ => {
                error!("予期しないエラー：{:?}", result)
            }
        };
    }
    // 初期状態のアイコン画像を取得
    let initial_image = match capture_icon_image(HWND(app_info.hwnd as *mut _)) {
        Some(img) => img,
        None => {
            error!("初期画像の取得に失敗しました。");
            return;
        }
    };

    info!("アイコンの監視ループを開始します。");
    loop {
        // 一定時間待機
        sleep(Duration::from_millis(interval)).await;

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
            send_discord_notification(&app_info.name, config_path.clone()).await;
            send_line_notification(&app_info.name, config_path.clone()).await;

            unsafe {
                let result = ShowWindow(HWND(app_info.hwnd as *mut _), SW_RESTORE);
                match result {
                    FALSE => {
                        error!("ウィンドウの復元に失敗しました。");
                        return;
                    }
                    TRUE => {
                        info!("ウィンドウを復元しました。");
                    }
                    _ => {
                        error!("予期しないエラー：{:?}", result)
                    }
                };
            }

            match app_handle.emit_all("monitoring_stopped", ()) {
                Ok(_) => info!("monitoring_stoppedイベントを送信しました。"),
                Err(e) => error!("monitoring_stoppedイベントの送信に失敗しました: {:?}", e),
            }
            break;
        }
        info!("アイコンに変化はありませんでした。");
    }
}
