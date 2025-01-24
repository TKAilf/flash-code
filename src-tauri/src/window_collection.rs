use base64::{engine::general_purpose, Engine};
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
use log::{error, info, warn};
use std::{ffi::c_void, io::Cursor, mem::MaybeUninit, ptr::null_mut};
use windows::Win32::{
    Foundation::{BOOL, HANDLE, HWND, LPARAM, WPARAM},
    Graphics::Gdi::{
        CreateDIBSection, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, HGDIOBJ,
    },
    System::Threading::GetCurrentProcessId,
    UI::WindowsAndMessaging::{
        DrawIconEx, EnumWindows, GetAncestor, GetClassLongPtrW, GetWindowLongPtrW, GetWindowTextW,
        GetWindowThreadProcessId, IsWindowVisible, SendMessageW, DI_NORMAL, GA_ROOTOWNER,
        GCLP_HICON, GWL_EXSTYLE, HICON, ICON_BIG, ICON_SMALL, WM_GETICON,
        WS_EX_NOREDIRECTIONBITMAP, WS_EX_TOOLWINDOW,
    },
};

struct EnumData {
    apps: *mut Vec<AppInfo>,
    current_process_id: u32,
}

use crate::window_utils::{AppInfo, GdiObjectSelector, HBitmapWrapper, HdcMemWrapper, HdcWrapper};

/// タスクバーに表示されているウィンドウを列挙し、情報を返す。
///
/// # 戻り値
/// - `Vec<AppInfo>`:
///   - タスクバーに表示されている各ウィンドウの情報を含むリスト。
///
/// # 注意事項
/// - 非表示のウィンドウやツールウィンドウは除外されます。
/// - 自プロセスのウィンドウも除外対象となります。
///
pub fn get_taskbar_apps() -> Vec<AppInfo> {
    let mut apps: Vec<AppInfo> = Vec::new();

    unsafe {
        let current_process_id = GetCurrentProcessId();
        let mut enum_data = EnumData {
            apps: &mut apps as *mut _,
            current_process_id: current_process_id,
        };

        let _ = EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut enum_data as *mut _ as isize),
        );
    }

    apps
}

/// ウィンドウを列挙するためのコールバック関数。
///
/// # 引数
/// - `hwnd`: 列挙されたウィンドウのハンドル。
/// - `lparam`: 列挙データを含む追加パラメータ。
///
/// # 戻り値
/// - `BOOL`:
///   - 継続時: `TRUE`。
///   - 中止時: `FALSE`。
///
/// # 注意事項
/// - 指定されたウィンドウが条件に合わない場合、処理はスキップされます。
///
unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let enum_data = &mut *(lparam.0 as *mut EnumData);
    let apps = &mut *enum_data.apps;
    let current_process_id = enum_data.current_process_id;

    if IsWindowVisible(hwnd).as_bool() {
        // ウィンドウのタイトルを取得
        let title = match get_window_title(hwnd) {
            Some(title) => title,
            None => return BOOL(1), // 次のウィンドウへ
        };

        // ウィンドウが自プロセスのものであるか確認
        let mut process_id: u32 = 0;
        if GetWindowThreadProcessId(hwnd, Some(&mut process_id)) == 0 {
            warn!("プロセスIDの取得に失敗しました。");
            return BOOL(1); // 次のウィンドウへ
        }
        if current_process_id == process_id {
            warn!(
                "タイトル：{:?}は自プロセスのウィンドウであるため、スキップします。",
                title
            );
            return BOOL(1); // 次のウィンドウへ
        }

        // ルートオーナーウィンドウを取得
        let howner = GetAncestor(hwnd, GA_ROOTOWNER);
        if howner != hwnd {
            warn!(
                "タイトル：{:?}はオーナーウィンドウであるため、スキップします。",
                title
            );
            return BOOL(1); // 次のウィンドウへ
        }

        // 拡張ウィンドウ情報を取得
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
        // ツールウィンドウか確認し、結果を取得
        let is_toolwindow =
            (ex_style & WS_EX_TOOLWINDOW.0) != 0 || (ex_style & WS_EX_NOREDIRECTIONBITMAP.0) != 0;
        if is_toolwindow {
            warn!(
                "タイトル：{:?}はツールウィンドウであるため、スキップします。",
                title
            );
            return BOOL(1); // 次のウィンドウへ
        }

        let mut process_id = 0;
        let thread_id = GetWindowThreadProcessId(hwnd, Some(&mut process_id));
        info!(
            "タイトル：{:?}、プロセスID：{:?}、スレッドID：{:?}",
            title, process_id, thread_id
        );

        let icon_base64 = get_window_icon_base64(hwnd);

        let title_clone = title.clone();
        apps.push(AppInfo {
            name: title,
            hwnd: hwnd.0 as isize,
            process_id: process_id,
            thread_id: thread_id,
            icon: icon_base64,
        });
        info!("タイトル：{:?}、追加しました。", title_clone);
    }
    BOOL(1)
}

/// ウィンドウのアイコンを Base64 エンコードして返す。
///
/// # 引数
/// - `hwnd`: ウィンドウハンドル。
///
/// # 戻り値
/// - `Option<String>`:
///   - 成功時: Base64 エンコードされたアイコン文字列。
///   - 失敗時: `None`。
///
/// # 注意事項
/// - アイコンが存在しない場合やエンコード失敗時は `None` を返します。
///
unsafe fn get_window_icon_base64(hwnd: HWND) -> Option<String> {
    // アイコンを取得
    let hicon: HICON = match get_icon_handle(hwnd) {
        Some(hicon) => hicon,
        None => return None,
    };

    // デバイスコンテキストを取得
    let hdc_wrap_screen: HdcWrapper = HdcWrapper::new(hwnd)?;

    // メモリデバイスコンテキストを作成
    let hdc_wrap_mem: HdcMemWrapper = HdcMemWrapper::new(hdc_wrap_screen.as_hdc())?;

    let (ppv_bits, hbm_result) = get_bitmap_handle(&hdc_wrap_mem)?;

    let hbm_wrapper = HBitmapWrapper::new_hbitmap_base(hbm_result);

    let _gdi_selector = match GdiObjectSelector::new(
        hdc_wrap_mem.as_hdc(),
        HGDIOBJ::from(hbm_wrapper.as_hbitmap()),
    ) {
        Some(gdi_selector) => gdi_selector,
        None => {
            error!("GdiObjectSelectorの作成に失敗しました。");
            return None;
        }
    };

    // アイコンを描画
    if DrawIconEx(
        hdc_wrap_mem.as_hdc(),
        0,
        0,
        hicon,
        32,
        32,
        0,
        None,
        DI_NORMAL,
    )
    .is_err()
    {
        warn!("DrawIconExの呼び出しに失敗しました。");
        return None;
    }
    info!("DrawIconExの呼び出しに成功しました。");

    let mut pixels_vec = {
        let slice = std::slice::from_raw_parts(ppv_bits as *const u8, (32 * 32 * 4) as usize);
        slice.to_vec()
    };

    // BGRからRGBに変換
    for i in (0..pixels_vec.len()).step_by(4) {
        pixels_vec.swap(i, i + 2); // BとRを入れ替える
    }

    // ImageBufferに変換
    let image_buffer = match ImageBuffer::<Rgba<u8>, _>::from_raw(32, 32, pixels_vec) {
        Some(image_buffer) => {
            info!("ImageBufferの生成に成功しました。");
            image_buffer
        }
        None => {
            warn!("ImageBufferの生成に失敗しました。");
            return None;
        }
    };

    // PNGとしてエンコード
    let mut png_buffer = Vec::new();
    {
        let dynamic_image = DynamicImage::ImageRgba8(image_buffer);
        if dynamic_image
            .write_to(&mut Cursor::new(&mut png_buffer), ImageFormat::Png)
            .is_err()
        {
            warn!("PNGのエンコードに失敗しました。");
            return None;
        }
        info!("PNGのエンコードに成功しました。");
    }

    let base64 = general_purpose::STANDARD.encode(&png_buffer);

    Some(base64)
}

/// ウィンドウのアイコンハンドルを取得する関数。
///
/// # 概要
/// 指定されたウィンドウハンドルに関連付けられたアイコンハンドルを取得します。
/// 大きいアイコン、小さいアイコン、クラスアイコンの順に取得を試みます。
///
/// # 引数
/// - `hwnd`: 対象となるウィンドウのハンドル。
///
/// # 戻り値
/// - `Option<HICON>`:
///   - 成功時: アイコンハンドル。
///   - 失敗時: `None`。
///
/// # 注意事項
/// - ウィンドウに関連付けられたアイコンが存在しない場合、`None` を返します。
/// - エラーが発生した場合、ログに情報が記録されます。
///
/// # 使用例
/// ```rust
/// use windows::Win32::Foundation::HWND;
/// use my_crate::window_collection::get_icon_handle;
///
/// unsafe {
///     let hwnd = HWND(0); // 適切なウィンドウハンドルを指定
///     if let Some(hicon) = get_icon_handle(hwnd) {
///         println!("アイコンハンドルを取得しました: {:?}", hicon);
///     } else {
///         println!("アイコンが見つかりませんでした。");
///     }
/// }
/// ```
///
unsafe fn get_icon_handle(hwnd: HWND) -> Option<HICON> {
    // 大きいアイコンを取得
    let hicon_raw = SendMessageW(hwnd, WM_GETICON, WPARAM(ICON_BIG as usize), LPARAM(0));
    let hicon_ptr = hicon_raw.0 as *mut c_void;
    if !hicon_ptr.is_null() {
        info!("大きいアイコンを取得しました。");
        return Some(HICON(hicon_ptr));
    }

    // 小さいアイコンを取得
    let hicon_raw = SendMessageW(hwnd, WM_GETICON, WPARAM(ICON_SMALL as usize), LPARAM(0));
    let hicon_ptr = hicon_raw.0 as *mut c_void;
    if !hicon_ptr.is_null() {
        info!("小さいアイコンを取得しました。");
        return Some(HICON(hicon_ptr));
    }

    // クラスのアイコンを取得
    let hicon_raw = GetClassLongPtrW(hwnd, GCLP_HICON);
    let hicon = HICON(hicon_raw as *mut c_void);
    if !hicon.is_invalid() {
        info!("クラスのアイコンを取得しました。");
        return Some(hicon);
    }

    // アイコンがない場合はNoneを返す
    info!("アイコンが見つかりませんでした。");
    None
}

/// ビットマップハンドルを取得する関数。
///
/// # 概要
/// 指定されたメモリデバイスコンテキストに基づいて、32x32ピクセルのビットマップを作成し、
/// ピクセルデータへのポインタとビットマップハンドルを返します。
///
/// # 引数
/// - `hdc_mem`: メモリデバイスコンテキストのラッパー。
///
/// # 戻り値
/// - `Option<(*mut c_void, HBITMAP)>`:
///   - 成功時: ピクセルデータへのポインタとビットマップハンドルのタプル。
///   - 失敗時: `None`。
///
/// # 注意事項
/// - ビットマップ作成に失敗した場合、`None` を返します。
/// - ピクセルデータが取得できない場合も `None` を返します。
///
/// # 使用例
/// ```rust
/// use windows::Win32::Graphics::Gdi::HDC;
/// use my_crate::window_collection::{get_bitmap_handle, HdcMemWrapper};
///
/// unsafe {
///     let hdc_mem = HdcMemWrapper::new(HDC(0)).unwrap();
///     if let Some((ppv_bits, hbitmap)) = get_bitmap_handle(&hdc_mem) {
///         println!("ビットマップハンドルを取得しました: {:?}", hbitmap);
///     } else {
///         println!("ビットマップの作成に失敗しました。");
///     }
/// }
/// ```
///
unsafe fn get_bitmap_handle(hdc_wrap_mem: &HdcMemWrapper) -> Option<(*mut c_void, HBITMAP)> {
    let mut ppv_bits: *mut c_void = null_mut();
    let mut bitmap_info_uninit = MaybeUninit::<BITMAPINFO>::uninit();
    let bitmap_info_ptr = bitmap_info_uninit.as_mut_ptr();

    (*bitmap_info_ptr).bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    (*bitmap_info_ptr).bmiHeader.biWidth = 32;
    (*bitmap_info_ptr).bmiHeader.biHeight = -32;
    (*bitmap_info_ptr).bmiHeader.biPlanes = 1;
    (*bitmap_info_ptr).bmiHeader.biBitCount = 32;
    (*bitmap_info_ptr).bmiHeader.biCompression = BI_RGB.0;

    let bitmap_info = bitmap_info_uninit.assume_init();

    let hbm_result = match CreateDIBSection(
        hdc_wrap_mem.as_hdc(),
        &bitmap_info as *const _,
        DIB_RGB_COLORS,
        &mut ppv_bits,
        HANDLE(null_mut()),
        0,
    ) {
        Ok(hbm) => hbm,
        Err(e) => {
            error!(
                "CreateDIBSectionの呼び出しに失敗しました。エラーコード: {}",
                e
            );
            return None;
        }
    };
    info!("CreateDIBSectionの呼び出しに成功しました。");
    if ppv_bits.is_null() {
        return None;
    }
    Some((ppv_bits, hbm_result))
}

/// ウィンドウのタイトルを取得する関数。
///
/// # 概要
/// 指定されたウィンドウハンドルに対応するウィンドウタイトルを取得し、
/// UTF-16 から文字列に変換して返します。
///
/// # 引数
/// - `hwnd`: タイトルを取得する対象のウィンドウハンドル。
///
/// # 戻り値
/// - `Option<String>`:
///   - 成功時: ウィンドウのタイトル。
///   - 失敗時: `None`。
///
/// # 注意事項
/// - タイトルが空の場合、または取得に失敗した場合は `None` を返します。
///
/// # 使用例
/// ```rust
/// use windows::Win32::Foundation::HWND;
/// use my_crate::window_collection::get_window_title;
///
/// let hwnd = HWND(0); // 適切なウィンドウハンドルを指定
/// if let Some(title) = get_window_title(hwnd) {
///     println!("ウィンドウタイトル: {}", title);
/// } else {
///     println!("ウィンドウタイトルを取得できませんでした。");
/// }
/// ```
///
pub fn get_window_title(hwnd: HWND) -> Option<String> {
    let mut buffer = [0u16; 256];
    let length = unsafe { GetWindowTextW(hwnd, &mut buffer) } as usize;
    if length > 0 {
        Some(String::from_utf16_lossy(&buffer[..length]))
    } else {
        None
    }
}
