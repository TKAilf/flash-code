use image::{DynamicImage, GenericImageView, Rgba};
use log::info;

/// 2つの画像間のピクセルごとの差分を計算し、
/// 画像全体の正規化された差分値と比較対象画像中のオレンジ色ピクセルの比率の両方が
/// 指定された閾値を超える場合に、有意な差分があると判定する関数。
///
/// # 概要
/// 指定された2つの画像 (`img1` と `img2`) をピクセル単位で比較します。
/// 画像サイズが異なる場合は、自動的に差分があると判断します。
/// それ以外の場合、各ピクセルのRGB値の絶対差の合計から画像全体の差分値を算出し、
/// その値を画像の最大差分値で正規化します（0.0〜1.0の範囲）。
/// さらに、`img2` 中のオレンジ色ピクセルの比率も計算し、
/// 正規化された差分値が `diff_threshold` を超え、かつオレンジ色ピクセルの比率が 0.3 を超える場合に
/// 有意な差分があると判定します。
///
/// # 引数
/// - `img1`: 最初の画像 (`DynamicImage`)。
/// - `img2`: 比較対象の画像 (`DynamicImage`)。
/// - `diff_threshold`: 正規化された差分値のしきい値（`0.0〜1.0`）。
///   この値以上の場合、画像間の全体的な差分が大きいと見なされます。
///
/// # 戻り値
/// - `bool`:
///   - `true`: 画像全体の正規化された差分値が `diff_threshold` を超え、かつオレンジ色ピクセルの比率が 0.3 を超える場合。
///   - `false`: それ以外の場合。
///
/// # 使用例
/// ```rust
/// use image::{open, DynamicImage};
/// use my_crate::image_comparison::has_significant_difference;
///
/// let img1 = open("image1.png").unwrap();
/// let img2 = open("image2.png").unwrap();
/// let diff_threshold = 0.05; // 画像全体の差分のしきい値
///
/// if has_significant_difference(&img1, &img2, diff_threshold) {
///     println!("画像に有意な差分があります。");
/// } else {
///     println!("画像の差分はしきい値以下です。");
/// }
/// ```
///
pub fn has_significant_difference(
    img1: &DynamicImage,
    img2: &DynamicImage,
    diff_threshold: f32,
) -> bool {
    // 画像サイズが異なる場合は差分ありと判断
    if img1.dimensions() != img2.dimensions() {
        return true;
    }
    let (width, height) = img1.dimensions();
    let total_pixels = (width as u64) * (height as u64);

    // 画像全体の差分計算
    let mut total_diff = 0u64;
    let mut orange_count = 0u64;
    for y in 0..height {
        for x in 0..width {
            let pixel1 = img1.get_pixel(x, y);
            let pixel2 = img2.get_pixel(x, y);
            let diff_r = (pixel1[0] as i32 - pixel2[0] as i32).abs() as u64;
            let diff_g = (pixel1[1] as i32 - pixel2[1] as i32).abs() as u64;
            let diff_b = (pixel1[2] as i32 - pixel2[2] as i32).abs() as u64;
            total_diff += diff_r + diff_g + diff_b;

            // 2枚目の画像のピクセルがオレンジ色かどうか
            if is_orange(pixel2) {
                orange_count += 1;
            }
        }
    }
    // 差分の正規化（0.0〜1.0）
    let max_diff = (255u64 * 3) * total_pixels;
    let normalized_diff = total_diff as f32 / max_diff as f32;

    // オレンジ色ピクセルの比率
    let orange_ratio = orange_count as f32 / total_pixels as f32;

    info!("正規化された差分値: {}", normalized_diff);
    info!("オレンジピクセルの比率: {}", orange_ratio);
    info!("しきい値: {}", diff_threshold);

    // 閾値との比較
    normalized_diff > diff_threshold && orange_ratio > 0.25
}

fn is_orange(pixel: Rgba<u8>) -> bool {
    // RGB値を0.0〜1.0の範囲に正規化
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;

    // 一般的なオレンジ色の値 (255, 165, 0) を正規化
    let orange_r = 1.0;
    let orange_g = 165.0 / 255.0;
    let orange_b = 0.0;

    // ユークリッド距離の計算
    let distance =
        ((r - orange_r).powi(2) + (g - orange_g).powi(2) + (b - orange_b).powi(2)).sqrt();

    // 距離の閾値（調整可能、ここでは0.7とする）
    let threshold = 0.7;

    distance < threshold
}
