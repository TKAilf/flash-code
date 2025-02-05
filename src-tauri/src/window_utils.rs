use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf, ptr::null_mut};
use tauri::async_runtime::{JoinHandle, Mutex};
use tauri::State;
use windows::{
    core::HRESULT,
    Win32::{
        Foundation::HWND,
        Graphics::Gdi::{
            CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, ReleaseDC,
            SelectObject, HBITMAP, HDC, HGDIOBJ,
        },
        System::Com::{CoInitialize, CoUninitialize},
    },
};

use crate::monitor::monitor_app_icon;

/// アプリケーション情報を格納する構造体。
///
/// # フィールド
/// - `name`: ウィンドウのタイトル。
/// - `hwnd`: ウィンドウハンドル（`isize` 型）。
/// - `process_id`: プロセス ID。
/// - `thread_id`: スレッド ID。
/// - `icon`: アプリケーションのアイコンを Base64 形式でエンコードした文字列。
///
#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,         // ウィンドウのタイトル
    pub hwnd: isize,          // ウィンドウハンドル(isize型)
    pub process_id: u32,      // プロセスID
    pub thread_id: u32,       // スレッドID
    pub icon: Option<String>, // アイコンのBase64データ
}

/// 設定ファイルのパスを管理する構造体。
///
/// # フィールド
/// - `path`: 設定ファイルのパス。
///
#[derive(Debug, Clone)]
pub struct ConfigState {
    pub path: PathBuf,
}

/// アプリケーション監視の状態を管理する構造体。
///
/// # 機能
/// - 各監視タスクを非同期で管理。
/// - 全監視タスクの停止処理。
///
/// # フィールド
/// - `tasks`: タスク名をキーとし、対応する非同期タスクのハンドルを格納するマップ。
///
pub struct MonitorState {
    pub tasks: Mutex<HashMap<String, JoinHandle<()>>>,
}
impl MonitorState {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(HashMap::new()),
        }
    }

    pub async fn stop_all(&self) {
        let mut tasks = self.tasks.lock().await;
        for (_, handle) in tasks.drain() {
            handle.abort();
        }
    }

    pub async fn monitor_target<'a>(
        &self,
        app_info: AppInfo,
        interval: u64,
        threshold: f32,
        config_state: State<'a, ConfigState>,
    ) {
        info!("monitor_targetを呼び出しました。");
        let app_name = app_info.name.clone();
        let config_path = config_state.path.clone();
        let handle = tauri::async_runtime::spawn(async move {
            monitor_app_icon(app_info, interval, threshold, config_path).await;
        });
        self.tasks.lock().await.insert(app_name, handle);
    }
}

/// GDI ハンドル (HDC) を安全に管理する RAII ラッパー構造体。
///
/// # 機能
/// - HDC の取得および解放を安全に管理。
///
/// # メソッド
/// - `new`: 指定したウィンドウハンドルから HDC を取得。
/// - `as_hdc`: 内部の HDC を返す。
///
pub struct HdcWrapper(HDC);
impl HdcWrapper {
    pub unsafe fn new(hwnd: HWND) -> Option<Self> {
        let hdc = GetDC(hwnd);
        if hdc.0.is_null() {
            None
        } else {
            Some(Self(hdc))
        }
    }

    pub fn as_hdc(&self) -> HDC {
        self.0
    }
}

impl Drop for HdcWrapper {
    fn drop(&mut self) {
        unsafe {
            ReleaseDC(HWND(null_mut()), self.0);
        }
    }
}

/// メモリデバイスコンテキスト (HDC) を安全に管理する RAII ラッパー構造体。
///
/// # 概要
/// メモリデバイスコンテキストは、画像描画やビットマップ操作などのオフスクリーン描画に使用されます。
/// この構造体は、メモリデバイスコンテキストの生成および解放を安全に管理します。
///
/// # メソッド
/// - `new`: 指定されたスクリーンデバイスコンテキストから新しいメモリデバイスコンテキストを作成します。
/// - `as_hdc`: 内部で保持している HDC を返します。
///
/// # 注意事項
/// - 必要なくなったメモリデバイスコンテキストは自動的に解放されます。
/// - 内部の HDC は、この構造体のライフサイクル内でのみ有効です。
///
/// # 使用例
/// ```rust
/// use windows::Win32::Graphics::Gdi::{CreateCompatibleDC, HDC};
/// use my_crate::window_utils::HdcMemWrapper;
///
/// unsafe {
///     let screen_hdc = HDC(0); // スクリーン HDC を適切に取得
///     if let Some(mem_hdc) = HdcMemWrapper::new(screen_hdc) {
///         println!("メモリデバイスコンテキストが作成されました: {:?}", mem_hdc.as_hdc());
///     } else {
///         println!("メモリデバイスコンテキストの作成に失敗しました。");
///     }
/// }
/// ```
///
pub struct HdcMemWrapper(HDC);
impl HdcMemWrapper {
    pub unsafe fn new(screen_hdc: HDC) -> Option<Self> {
        let mem_hdc = CreateCompatibleDC(screen_hdc);
        if mem_hdc.0.is_null() {
            None
        } else {
            Some(Self(mem_hdc))
        }
    }

    pub fn as_hdc(&self) -> HDC {
        self.0
    }
}

impl Drop for HdcMemWrapper {
    fn drop(&mut self) {
        unsafe {
            let _ = DeleteDC(self.0);
        }
    }
}

/// ビットマップ (HBITMAP) を安全に管理する RAII ラッパー構造体。
///
/// # 概要
/// ビットマップリソースの作成および解放を安全に管理します。
/// 主にオフスクリーン描画や画像データの操作に使用されます。
///
/// # メソッド
/// - `new_hdc_base`: 指定されたスクリーンデバイスコンテキストとサイズから新しいビットマップを作成します。
/// - `new_hbitmap_base`: 既存の HBITMAP をラップします。
/// - `as_hbitmap`: 内部で保持している HBITMAP を返します。
/// - `into_inner`: 内部の HBITMAP を取得し、ラッパー構造体から解放します。
///
/// # 注意事項
/// - 内部の HBITMAP は、この構造体のライフサイクル内でのみ安全に使用されます。
/// - `into_inner` を呼び出すと、HBITMAP のライフサイクル管理が呼び出し元に移行します。
///
/// # 使用例
/// ```rust
/// use windows::Win32::Graphics::Gdi::{CreateCompatibleBitmap, HDC};
/// use my_crate::window_utils::HBitmapWrapper;
///
/// unsafe {
///     let screen_hdc = HDC(0); // スクリーン HDC を適切に取得
///     if let Some(bitmap) = HBitmapWrapper::new_hdc_base(screen_hdc, 100, 100) {
///         println!("ビットマップが作成されました: {:?}", bitmap.as_hbitmap());
///     } else {
///         println!("ビットマップの作成に失敗しました。");
///     }
/// }
/// ```
///
pub struct HBitmapWrapper(HBITMAP);
impl HBitmapWrapper {
    pub unsafe fn new_hdc_base(screen_hdc: HDC, width: i32, height: i32) -> Option<Self> {
        let hbm = CreateCompatibleBitmap(screen_hdc, width, height);
        if hbm.0.is_null() {
            None
        } else {
            Some(Self(hbm))
        }
    }

    pub fn new_hbitmap_base(hbm: HBITMAP) -> Self {
        Self(hbm)
    }

    pub fn as_hbitmap(&self) -> HBITMAP {
        self.0
    }

    pub fn into_inner(self) -> HBITMAP {
        let hbm = self.0;
        // Prevent Drop from deleting the HBITMAP
        std::mem::forget(self);
        hbm
    }
}

impl Drop for HBitmapWrapper {
    fn drop(&mut self) {
        unsafe {
            let _ = DeleteObject(self.0);
        }
    }
}

/// GDI オブジェクトの選択を安全に管理する RAII ラッパー構造体。
///
/// # 概要
/// デバイスコンテキスト (HDC) に対する GDI オブジェクトの選択を管理します。
/// GDI オブジェクトを選択する際、以前のオブジェクトを記憶し、
/// ライフサイクル終了時に元のオブジェクトを復元します。
///
/// # メソッド
/// - `new`: 指定した GDI オブジェクトをデバイスコンテキストに選択します。
///
/// # 注意事項
/// - この構造体がスコープを外れる際、自動的に元の GDI オブジェクトが復元されます。
///
/// # 使用例
/// ```rust
/// use windows::Win32::Graphics::Gdi::{SelectObject, HGDIOBJ, HDC};
/// use my_crate::window_utils::GdiObjectSelector;
///
/// unsafe {
///     let hdc = HDC(0); // 適切な HDC を取得
///     let new_obj = HGDIOBJ(0); // 新しい GDI オブジェクトを指定
///     if let Some(selector) = GdiObjectSelector::new(hdc, new_obj) {
///         println!("GDI オブジェクトを選択しました。");
///     } else {
///         println!("GDI オブジェクトの選択に失敗しました。");
///     }
/// }
/// ```
///
pub struct GdiObjectSelector {
    hdc: HDC,
    old_obj: HGDIOBJ,
}
impl GdiObjectSelector {
    pub unsafe fn new(hdc: HDC, new_obj: HGDIOBJ) -> Option<Self> {
        let old_obj = SelectObject(hdc, new_obj);
        if old_obj.0.is_null() {
            None
        } else {
            Some(Self { hdc, old_obj })
        }
    }
}

impl Drop for GdiObjectSelector {
    fn drop(&mut self) {
        unsafe {
            let _ = SelectObject(self.hdc, self.old_obj);
        }
    }
}

/// COM ライブラリの初期化と解放を安全に管理する RAII ラッパー構造体。
///
/// # 概要
/// COM ライブラリをスレッド単位で初期化し、スコープ終了時に自動的に解放します。
///
/// # メソッド
/// - `new`: COM ライブラリを初期化します。
///
/// # 注意事項
/// - COM ライブラリはスレッド単位で管理されます。
/// - この構造体がスコープを外れる際、自動的に COM ライブラリが解放されます。
///
/// # 使用例
/// ```rust
/// use my_crate::window_utils::ComWrapper;
///
/// unsafe {
///     if let Ok(wrapper) = ComWrapper::new() {
///         println!("COM ライブラリが初期化されました。");
///     } else {
///         println!("COM ライブラリの初期化に失敗しました。");
///     }
/// }
/// ```
///
pub struct ComWrapper;
impl ComWrapper {
    pub unsafe fn new() -> Result<Self, HRESULT> {
        let hr = CoInitialize(None);
        if hr.is_err() {
            Err(hr)
        } else {
            Ok(ComWrapper)
        }
    }
}

impl Drop for ComWrapper {
    fn drop(&mut self) {
        unsafe {
            let _ = CoUninitialize();
        }
    }
}

/// 設定ディレクトリ内に指定された設定ファイルが存在するか確認し、
/// 存在しなければ設定ファイルのディレクトリを作成し、デフォルト内容で初期化します。
///
/// # 引数
/// * `config_dir` - 設定ファイルが格納されるディレクトリへのパス。
/// * `file_name` - 設定ファイルの名前（例: "appsettings.json"）。
///
/// # 戻り値
/// 設定ファイルの完全なパス (PathBuf) を返します。
///
/// # 処理の概要
/// 1. `config_dir` と `file_name` を結合して、設定ファイルのパスを生成します。
/// 2. そのパスにファイルが存在しなければ、
///    - 指定ディレクトリを作成し、
///    - デフォルト設定の内容で設定ファイルを初期化（書き込み）します。
/// 3. 既に設定ファイルが存在する場合は、その旨をログに出力し、パスを返します。
///
/// # 例
/// ```rust
/// use std::path::Path;
/// let config_dir = Path::new("/path/to/config");
/// let config_file = get_or_create_config_file_path(&config_dir, "appsettings.json");
/// println!("Config file path: {:?}", config_file);
/// ```
///
pub fn get_or_create_config_file_path(config_dir: &Path, file_name: &str) -> PathBuf {
    let config_file = config_dir.join(file_name);
    // 設定ファイルの存在を確認し、なければ作成する処理
    if !config_file.exists() {
        info!("設定ファイルが存在しません。デフォルトの設定ファイルを生成します。");

        match fs::create_dir_all(config_dir) {
            Ok(_) => info!("設定ファイルのディレクトリを作成しました。"),
            Err(e) => warn!("設定ファイルのディレクトリの作成に失敗しました: {:?}", e),
        };

        initilize_config_file(&config_file);
    } else {
        info!("設定ファイルが存在します。");
    }

    config_file
}

/// 指定されたパスにデフォルトの設定ファイル ("appsettings.json") を初期化（書き込み）します。
///
/// 書き込み内容は以下のJSON形式で、各項目はユーザーが後で更新する前提です:
/// {
///     "DISCORD_WEBHOOK_URL": "ここにDiscordBotURLを入力してください",
///     "THRESHOLD": "0.050",
///     "INTERVAL": "3000"
/// }
///
/// # 引数
/// * `config_file` - 初期化する設定ファイルへのパス。
///
/// # 例
/// ```rust
/// use std::path::Path;
/// let config_file = Path::new("/path/to/appsettings.json");
/// initilize_config_file(&config_file);
/// ```
///
pub fn initilize_config_file(config_file: &Path) {
    match fs::write(
        config_file,
        r#"{
"DISCORD_WEBHOOK_URL": "DiscordBotURLを入力してください",
"THRESHOLD": "0.050",
"INTERVAL": "3000"
}"#,
    ) {
        Ok(_) => info!("設定ファイルを初期化しました。"),
        Err(e) => warn!("設定ファイルの初期化に失敗しました: {:?}", e),
    }
}
