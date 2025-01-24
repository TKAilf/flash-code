use image::{DynamicImage, GenericImageView};
use log::info;

/// 2つの画像間のピクセルごとの差分を計算し、しきい値を超えるか判定する関数。
///
/// # 概要
/// 指定された2つの画像をピクセル単位で比較し、差分の割合が指定されたしきい値を超えているかを判定します。
/// 画像サイズが異なる場合は、強制的に差分があると見なします。
///
/// # 引数
/// - `img1`: 最初の画像 (`DynamicImage`)。
/// - `img2`: 比較対象の画像 (`DynamicImage`)。
/// - `threshold`: 差分を評価するしきい値（`0.0〜1.0`）。
///
/// # 戻り値
/// - `bool`:
///   - 差分がしきい値を超える場合: `true`。
///   - 差分がしきい値以下の場合: `false`。
///
/// # 使用例
/// ```rust
/// use image::{open, DynamicImage};
/// use my_crate::image_comparison::has_significant_difference;
///
/// let img1 = open("image1.png").unwrap();
/// let img2 = open("image2.png").unwrap();
/// let threshold = 0.05; // しきい値
///
/// if has_significant_difference(&img1, &img2, threshold) {
///     println!("画像に有意な差分があります。");
/// } else {
///     println!("画像の差分はしきい値以下です。");
/// }
/// ```
///
pub fn has_significant_difference(
    img1: &DynamicImage,
    img2: &DynamicImage,
    threshold: f32,
) -> bool {
    // 画像サイズの確認
    if img1.dimensions() != img2.dimensions() {
        return true;
    }

    let (width, height) = img1.dimensions();
    let mut total_diff = 0u64;

    // 各ピクセルの差分を計算
    for y in 0..height {
        for x in 0..width {
            let pixel1 = img1.get_pixel(x, y);
            let pixel2 = img2.get_pixel(x, y);

            let diff_r = (pixel1[0] as i32 - pixel2[0] as i32).abs() as u64;
            let diff_g = (pixel1[1] as i32 - pixel2[1] as i32).abs() as u64;
            let diff_b = (pixel1[2] as i32 - pixel2[2] as i32).abs() as u64;

            total_diff += diff_r + diff_g + diff_b;
        }
    }

    // 最大差分値の計算
    let max_diff = (255u64 * 3) * (width as u64) * (height as u64);

    // 正規化された差分値(0.0〜1.0)の計算
    let normalized_diff = total_diff as f32 / max_diff as f32;
    info!("正規化された差分値: {}", normalized_diff);
    info!("しきい値: {}", threshold);

    // 閾値との比較
    normalized_diff > threshold
}
