use image::{DynamicImage, ImageBuffer, Rgba};
use log::info;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    BitBlt, GetDIBits, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_USAGE, HBITMAP,
    HGDIOBJ, SRCCOPY,
};

use crate::icon_position::get_icon_rect;
use crate::window_utils::{HBitmapWrapper, HdcMemWrapper, HdcWrapper};

/// 指定したウィンドウのアイコン領域をキャプチャして画像として返す。
///
/// # 概要
/// 指定されたウィンドウのアイコンの位置とサイズを取得し、
/// その領域をキャプチャして `DynamicImage` オブジェクトとして返します。
///
/// # 引数
/// - `hwnd`: ウィンドウハンドル。
///
/// # 戻り値
/// - `Option<DynamicImage>`:
///   - 成功時: キャプチャした画像。
///   - 失敗時: `None`。
///
/// # 注意事項
/// - キャプチャ対象のウィンドウが存在しない場合や領域の取得に失敗した場合、`None` を返します。
///
pub fn capture_icon_image(hwnd: HWND) -> Option<DynamicImage> {
    info!("capture_icon_imageを呼び出しました。");
    // アイコンの位置とサイズを取得
    let rect = get_icon_rect(hwnd)?;
    info!("アイコンの位置とサイズを表示します。rect: {:?}", rect);

    let icon_width = rect.right - rect.left;
    let icon_height = rect.bottom - rect.top;
    // アイコン領域をキャプチャ
    let hbitmap = capture_screen_area(rect.left, rect.top, icon_width, icon_height)?;
    let image = hbitmap_to_image(hbitmap, icon_width, icon_height)?;
    Some(image)
}

/// 指定した画面領域をキャプチャし、ビットマップを取得する。
///
/// # 引数
/// - `x`: キャプチャ開始位置のX座標。
/// - `y`: キャプチャ開始位置のY座標。
/// - `width`: キャプチャする幅。
/// - `height`: キャプチャする高さ。
///
/// # 戻り値
/// - `Option<HBITMAP>`:
///   - 成功時: ビットマップハンドル。
///   - 失敗時: `None`。
///
/// # 注意事項
/// - キャプチャ失敗時はエラーメッセージがログに記録されます。
///
fn capture_screen_area(x: i32, y: i32, width: i32, height: i32) -> Option<HBITMAP> {
    info!("screen_capture.rs : capture_screen_areaを呼び出しました。");
    unsafe {
        let hdc_screen: HdcWrapper = HdcWrapper::new(HWND(null_mut()))?;
        let hdc_mem: HdcMemWrapper = HdcMemWrapper::new(hdc_screen.as_hdc())?;
        let hbm_screen: HBitmapWrapper =
            HBitmapWrapper::new_hdc_base(hdc_screen.as_hdc(), width, height)?;
        let old_obj: HGDIOBJ = SelectObject(
            hdc_mem.as_hdc(),
            HGDIOBJ(hbm_screen.as_hbitmap().0 as *mut _),
        );
        if old_obj.0.is_null() {
            return None;
        }

        let result = BitBlt(
            hdc_mem.as_hdc(),
            0,
            0,
            width,
            height,
            hdc_screen.as_hdc(),
            x,
            y,
            SRCCOPY,
        );
        SelectObject(hdc_mem.as_hdc(), old_obj);

        if result.is_err() {
            None
        } else {
            Some(hbm_screen.into_inner())
        }
    }
}

/// ビットマップ (HBITMAP) を画像 (DynamicImage) に変換する。
///
/// # 引数
/// - `hbitmap`: 変換対象のビットマップハンドル。
/// - `width`: 画像の幅。
/// - `height`: 画像の高さ。
///
/// # 戻り値
/// - `Option<DynamicImage>`:
///   - 成功時: 変換された画像。
///   - 失敗時: `None`。
///
/// # 注意事項
/// - ビットマップデータの取得やRGB変換に失敗した場合、`None` を返します。
///
fn hbitmap_to_image(hbitmap: HBITMAP, width: i32, height: i32) -> Option<DynamicImage> {
    info!("screen_capture.rs : hbitmap_to_imageを呼び出しました。");
    unsafe {
        let hdc_screen: HdcWrapper = HdcWrapper::new(HWND(null_mut()))?;

        let mut bitmap_info_uninit = MaybeUninit::<BITMAPINFO>::uninit();
        let bitmap_info_ptr = bitmap_info_uninit.as_mut_ptr();

        (*bitmap_info_ptr).bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        (*bitmap_info_ptr).bmiHeader.biWidth = width;
        (*bitmap_info_ptr).bmiHeader.biHeight = -height; // 正の値で上下反転、負の値でそのまま
        (*bitmap_info_ptr).bmiHeader.biPlanes = 1;
        (*bitmap_info_ptr).bmiHeader.biBitCount = 32;
        (*bitmap_info_ptr).bmiHeader.biCompression = BI_RGB.0;
        (*bitmap_info_ptr).bmiHeader.biSizeImage = 0;
        (*bitmap_info_ptr).bmiHeader.biXPelsPerMeter = 0;
        (*bitmap_info_ptr).bmiHeader.biYPelsPerMeter = 0;
        (*bitmap_info_ptr).bmiHeader.biClrUsed = 0;
        (*bitmap_info_ptr).bmiHeader.biClrImportant = 0;

        // 初期化が完了したら、初期化済みとして扱う
        let mut bitmap_info = bitmap_info_uninit.assume_init();

        let mut pixels = vec![0u8; (width * height * 4) as usize];

        let result = GetDIBits(
            hdc_screen.as_hdc(),
            hbitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as *mut _),
            &mut bitmap_info,
            DIB_USAGE(0),
        );

        if result == 0 {
            return None;
        }

        // BGRからRGBに変換
        for i in (0..pixels.len()).step_by(4) {
            pixels.swap(i, i + 2); // BとRを入れ替える
        }

        let image_buffer =
            ImageBuffer::<Rgba<u8>, _>::from_raw(width as u32, height as u32, pixels)?;
        Some(DynamicImage::ImageRgba8(image_buffer))
    }
}
