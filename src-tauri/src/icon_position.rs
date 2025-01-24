use std::collections::HashSet;

use log::{error, info, warn};
use windows::{
    core::VARIANT,
    Win32::{
        Foundation::{HWND, RECT},
        System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
        UI::Accessibility::{
            CUIAutomation, IUIAutomation, TreeScope_Children, TreeScope_Descendants,
            UIA_ButtonControlTypeId, UIA_ClassNamePropertyId, UIA_ControlTypePropertyId,
        },
    },
};

use crate::{window_collection::get_window_title, window_utils::ComWrapper};

/// 指定されたウィンドウハンドルに対応する UI 要素の位置（境界矩形）を取得します。
///
/// # 概要
/// 指定されたウィンドウハンドルのタイトルを基に、タスクバーから該当するボタンを検索し、
/// 一致する場合はそのボタンの境界矩形を取得します。
///
/// # 引数
/// - `hwnd`: 取得対象のウィンドウハンドル。
///
/// # 戻り値
/// - `Option<RECT>`:
///   - 成功時: UI 要素の位置を表す `RECT`。
///   - 失敗時: `None`。
///
/// # 注意事項
/// - `ComWrapper` により COM ライブラリの初期化と終了処理が管理されます。
/// - ウィンドウタイトルがタスクバーのボタンと一致しない場合、`None` が返されます。
///
/// # 使用例
/// ```rust
/// use windows::Win32::Foundation::HWND;
/// use my_crate::icon_position::get_icon_rect;
///
/// let hwnd = HWND(0); // 適切なハンドルを指定
/// if let Some(rect) = get_icon_rect(hwnd) {
///     println!("位置情報: {:?}", rect);
/// } else {
///     println!("一致する UI 要素が見つかりませんでした。");
/// }
/// ```
///
pub fn get_icon_rect(hwnd: HWND) -> Option<RECT> {
    info!("get_icon_rectを呼び出しました。");
    unsafe {
        let _ = match ComWrapper::new() {
            Ok(com_wrapper) => com_wrapper,
            Err(e) => {
                error!("COM ライブラリの初期化に失敗しました。エラーコード: {}", e);
                return None;
            }
        };
        let uiautomation: IUIAutomation =
            match CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER) {
                Ok(uiautomation) => uiautomation,
                Err(e) => {
                    error!(
                        "CoCreateInstanceの呼び出しに失敗しました。エラーコード: {}",
                        e
                    );
                    return None;
                }
            };
        info!("CoCreateInstanceの呼び出しに成功しました。");

        let desktop = match uiautomation.GetRootElement() {
            Ok(desktop) => desktop,
            Err(e) => {
                error!(
                    "GetRootElementの呼び出しに失敗しました。エラーコード: {}",
                    e
                );
                return None;
            }
        };
        info!("GetRootElementの呼び出しに成功しました。");

        let class_name_variant = VARIANT::from("Shell_TrayWnd");
        let condition_taskbar = match uiautomation
            .CreatePropertyCondition(UIA_ClassNamePropertyId, &class_name_variant)
        {
            Ok(condition_taskbar) => condition_taskbar,
            Err(e) => {
                error!(
                    "CreatePropertyConditionの呼び出しに失敗しました。エラーコード: {}",
                    e
                );
                return None;
            }
        };
        let taskbar = match desktop.FindFirst(TreeScope_Children, &condition_taskbar) {
            Ok(taskbar) => taskbar,
            Err(e) => {
                error!("FindFirstの呼び出しに失敗しました。エラーコード: {}", e);
                return None;
            }
        };
        info!("FindFirstの呼び出しに成功しました。");

        let control_type_variant = VARIANT::from(UIA_ButtonControlTypeId.0);
        let condition_button = match uiautomation
            .CreatePropertyCondition(UIA_ControlTypePropertyId, &control_type_variant)
        {
            Ok(condition_button) => condition_button,
            Err(e) => {
                error!(
                    "CreatePropertyConditionの呼び出しに失敗しました。エラーコード: {}",
                    e
                );
                return None;
            }
        };
        let buttons_array = match taskbar.FindAll(TreeScope_Descendants, &condition_button) {
            Ok(buttons_array) => buttons_array,
            Err(e) => {
                error!("FindAllの呼び出しに失敗しました。エラーコード: {}", e);
                return None;
            }
        };
        info!("FindAllの呼び出しに成功しました。");

        let window_title = match get_window_title(hwnd) {
            Some(title) => title,
            None => {
                error!("ウィンドウタイトルの取得に失敗しました。");
                return None;
            }
        };
        let window_words = split_string_into_words(&window_title);
        info!("ウィンドウの単語セットは{:?}です", window_words);

        let length = match buttons_array.Length() {
            Ok(length) => length,
            Err(e) => {
                error!("Lengthの呼び出しに失敗しました。エラーコード: {}", e);
                return None;
            }
        };
        info!("ボタンは{}個あります", length);

        for i in 0..length {
            let button = match buttons_array.GetElement(i) {
                Ok(button) => button,
                Err(e) => {
                    error!("GetElementの呼び出しに失敗しました。エラーコード: {}", e);
                    return None;
                }
            };

            let botton_name = match button.CurrentName() {
                Ok(name) => name,
                Err(e) => {
                    error!("CurrentNameの呼び出しに失敗しました。エラーコード: {}", e);
                    return None;
                }
            };
            info!("ボタン名は{}です", botton_name);

            let botton_words = split_string_into_words(&botton_name.to_string());
            info!("ボタンの単語セットは{:?}です", botton_words);

            let common_words: HashSet<_> = window_words.intersection(&botton_words).collect();
            info!("共通の単語セットは{:?}です", common_words);

            if !common_words.is_empty() {
                let mut score: f32 = 0.0;
                for word in &common_words {
                    if word.len() >= 2 {
                        score += 1.0;
                    } else if word.len() == 1 {
                        score += 0.5;
                    }
                }

                if 1.0 <= score {
                    let rect = match button.CurrentBoundingRectangle() {
                        Ok(rect) => rect,
                        Err(e) => {
                            error!("CurrentBoundingRectangleの呼び出しに失敗しました。エラーコード: {}", e);
                            return None;
                        }
                    };
                    info!("CurrentBoundingRectangleの呼び出しに成功しました。監視対象の名前は{:?}です。", botton_name);
                    return Some(rect);
                }
            }
        }
        warn!("一致するボタンが見つかりませんでした。");
        None
    }
}

fn split_string_into_words(s: &str) -> HashSet<String> {
    s.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase())
        .collect()
}
