//! Twip-to-pixel and ScaleMode conversion.
//!
//! Imports `ScaleMode` from vb6parse — no local redefinition.
//!
//! Default DPI is 96 (standard web/screen). Make DPI configurable
//! via the layout engine's configuration.

use vb6parse::language::ScaleMode;

/// Convert twips to pixels at the given DPI.
///
/// 1 twip = 1/1440 inch. At 96 DPI: 1440 twips = 96 pixels, i.e. 15 twips = 1 pixel.
///
/// # Arguments
/// * `twips` — value in twips
/// * `dpi` — dots per inch (default: 96)
///
/// # Examples
/// ```
/// use vb6runtime::layout::scale::twips_to_pixels;
///
/// // 15 twips = 1 pixel at 96 DPI
/// assert_eq!(twips_to_pixels(15, 96), 1.0);
/// // 1440 twips = 96 pixels at 96 DPI (1 inch)
/// assert_eq!(twips_to_pixels(1440, 96), 96.0);
/// ```
pub fn twips_to_pixels(twips: i32, dpi: u32) -> f32 {
    twips as f32 * dpi as f32 / 1440.0
}

/// Convert a value in a given ScaleMode to pixels at the given DPI.
///
/// # Arguments
/// * `value` — value in the given scale mode's units
/// * `scale_mode` — the [`ScaleMode`] of the value
/// * `dpi` — dots per inch
///
/// # Fallback
/// For `ScaleMode::User`, `ScaleMode::Character`,
/// `ScaleMode::ContainerPosition`, and `ScaleMode::ContainerSize` — falls back
/// to twip conversion (these have no fixed pixel mapping).
///
/// # Examples
/// ```
/// use vb6parse::language::ScaleMode;
/// use vb6runtime::layout::scale::scale_mode_to_pixels;
///
/// // Pixel mode: 1:1 mapping
/// assert_eq!(scale_mode_to_pixels(100, ScaleMode::Pixel, 96), 100.0);
///
/// // Point mode: 1 point = 4/3 pixels at 96 DPI
/// assert_eq!(scale_mode_to_pixels(3, ScaleMode::Point, 96), 4.0);
///
/// // Inch mode: 1 inch = 96 pixels at 96 DPI
/// assert_eq!(scale_mode_to_pixels(1, ScaleMode::Inches, 96), 96.0);
///
/// // Centimeter mode: 1 cm ≈ 37.8 pixels at 96 DPI
/// let result = scale_mode_to_pixels(1, ScaleMode::Centimeter, 96);
/// assert!((result - 37.7959).abs() < 0.01);
/// ```
pub fn scale_mode_to_pixels(value: i32, scale_mode: ScaleMode, dpi: u32) -> f32 {
    match scale_mode {
        ScaleMode::Twip => twips_to_pixels(value, dpi),
        ScaleMode::Pixel => value as f32,
        ScaleMode::Point => value as f32 * dpi as f32 / 72.0,
        ScaleMode::Inches => value as f32 * dpi as f32,
        ScaleMode::Millimeter => value as f32 * dpi as f32 / 25.4,
        ScaleMode::Centimeter => value as f32 * dpi as f32 / 2.54,
        ScaleMode::HiMetric => value as f32 * dpi as f32 / 25.4, // HiMetric = millimeter
        // Fallback: no fixed pixel mapping — treat as twips
        ScaleMode::User
        | ScaleMode::Character
        | ScaleMode::ContainerPosition
        | ScaleMode::ContainerSize => twips_to_pixels(value, dpi),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twips_to_pixels_96dpi() {
        // 15 twips = 1 pixel at 96 DPI
        assert_eq!(twips_to_pixels(15, 96), 1.0);
        assert_eq!(twips_to_pixels(150, 96), 10.0);
        assert_eq!(twips_to_pixels(1440, 96), 96.0);
    }

    #[test]
    fn twips_to_pixels_custom_dpi() {
        // At 120 DPI: 1440 twips = 120 pixels
        assert_eq!(twips_to_pixels(1440, 120), 120.0);
    }

    #[test]
    fn twips_to_pixels_zero() {
        assert_eq!(twips_to_pixels(0, 96), 0.0);
    }

    #[test]
    fn twips_to_pixels_negative() {
        assert_eq!(twips_to_pixels(-15, 96), -1.0);
    }

    #[test]
    fn scale_mode_pixel() {
        assert_eq!(scale_mode_to_pixels(100, ScaleMode::Pixel, 96), 100.0);
    }

    #[test]
    fn scale_mode_point() {
        // 1 point = 1/72 inch; at 96 DPI: 96/72 = 4/3 pixels per point
        // 3 points = 4 pixels
        assert_eq!(scale_mode_to_pixels(3, ScaleMode::Point, 96), 4.0);
    }

    #[test]
    fn scale_mode_inch() {
        assert_eq!(scale_mode_to_pixels(1, ScaleMode::Inches, 96), 96.0);
    }

    #[test]
    fn scale_mode_centimeter() {
        // 1 cm = 96/2.54 pixels ≈ 37.7959
        let result = scale_mode_to_pixels(1, ScaleMode::Centimeter, 96);
        assert!((result - 37.7959).abs() < 0.01);
    }

    #[test]
    fn scale_mode_millimeter() {
        // 1 mm = 96/25.4 pixels ≈ 3.7795
        let result = scale_mode_to_pixels(1, ScaleMode::Millimeter, 96);
        assert!((result - 3.7795).abs() < 0.01);
    }

    #[test]
    fn scale_mode_himetric() {
        // HiMetric = millimeter
        let result = scale_mode_to_pixels(1, ScaleMode::HiMetric, 96);
        assert!((result - 3.7795).abs() < 0.01);
    }

    #[test]
    fn scale_mode_twip() {
        // Twip mode at 96 DPI: 15 twips = 1 pixel
        assert_eq!(scale_mode_to_pixels(15, ScaleMode::Twip, 96), 1.0);
        assert_eq!(scale_mode_to_pixels(150, ScaleMode::Twip, 96), 10.0);
    }

    #[test]
    fn scale_mode_twip_custom_dpi() {
        // At 120 DPI: 1440 twips = 120 pixels
        assert_eq!(scale_mode_to_pixels(1440, ScaleMode::Twip, 120), 120.0);
    }

    #[test]
    fn scale_mode_user_fallback() {
        // Unknown ScaleMode falls back to twip conversion
        let result = scale_mode_to_pixels(150, ScaleMode::User, 96);
        assert_eq!(result, 10.0);
    }

    #[test]
    fn scale_mode_container_position_fallback() {
        assert_eq!(
            scale_mode_to_pixels(15, ScaleMode::ContainerPosition, 96),
            1.0
        );
    }

    #[test]
    fn scale_mode_container_size_fallback() {
        assert_eq!(scale_mode_to_pixels(15, ScaleMode::ContainerSize, 96), 1.0);
    }

    #[test]
    fn scale_mode_character_fallback() {
        assert_eq!(scale_mode_to_pixels(15, ScaleMode::Character, 96), 1.0);
    }

    #[test]
    fn scale_mode_inch_custom_dpi() {
        // At 120 DPI: 1 inch = 120 pixels
        assert_eq!(scale_mode_to_pixels(1, ScaleMode::Inches, 120), 120.0);
    }

    #[test]
    fn scale_mode_point_custom_dpi() {
        // At 120 DPI: 1 point = 120/72 = 5/3 pixels
        // 6 points = 10 pixels
        assert_eq!(scale_mode_to_pixels(6, ScaleMode::Point, 120), 10.0);
    }

    #[test]
    fn scale_mode_zero_value() {
        // Zero should be zero for all modes
        assert_eq!(scale_mode_to_pixels(0, ScaleMode::Twip, 96), 0.0);
        assert_eq!(scale_mode_to_pixels(0, ScaleMode::Pixel, 96), 0.0);
        assert_eq!(scale_mode_to_pixels(0, ScaleMode::Point, 96), 0.0);
        assert_eq!(scale_mode_to_pixels(0, ScaleMode::Inches, 96), 0.0);
    }

    #[test]
    fn twips_to_pixels_large_value() {
        // 1,440,000 twips = 96,000 pixels at 96 DPI (1000 inches = 96,000 pixels)
        assert_eq!(twips_to_pixels(1440000, 96), 96000.0);
    }
}
