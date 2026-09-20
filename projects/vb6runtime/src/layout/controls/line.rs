//! Style builder for Line controls.
//!
//! Maps Line properties to [`LayoutStyle`]:
//! - `border_color` → `line_color`
//! - `border_width` → `line_width`
//! - Coordinates stored in `line_*` fields for SVG rendering

use super::super::LayoutConfig;
use super::super::color::color_to_css;
use super::super::model::style::LayoutStyle;
use super::super::scale::scale_mode_to_pixels;
use vb6parse::language::{LineProperties, ScaleMode};

/// Build CSS style for a Line control.
pub fn build_line_style(props: &LineProperties, config: &LayoutConfig) -> LayoutStyle {
    let mut style = LayoutStyle {
        line_color: Some(color_to_css(&props.border_color)),
        line_width: Some(props.border_width as f32),
        line_x1: Some(scale_mode_to_pixels(props.x1, ScaleMode::Twip, config.dpi)),
        line_y1: Some(scale_mode_to_pixels(props.y1, ScaleMode::Twip, config.dpi)),
        line_x2: Some(scale_mode_to_pixels(props.x2, ScaleMode::Twip, config.dpi)),
        line_y2: Some(scale_mode_to_pixels(props.y2, ScaleMode::Twip, config.dpi)),
        ..LayoutStyle::default()
    };

    // Diff against VB6 defaults — set matching fields to None
    let defaults = LayoutStyle::default_line(props, config);
    style.diff_against(&defaults);

    style
}

#[cfg(test)]
mod tests {
    use super::super::super::model::style::CssColor;
    use super::*;
    use vb6parse::language::Color;

    fn test_config() -> LayoutConfig {
        LayoutConfig {
            dpi: 96,
            ..Default::default()
        }
    }

    #[test]
    fn line_color_set() {
        let props = LineProperties {
            border_color: Color::RGB {
                red: 255,
                green: 0,
                blue: 0,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_line_style(&props, &config);
        assert_eq!(style.line_color, Some(CssColor::Rgb(255, 0, 0)));
    }

    #[test]
    fn line_width_set() {
        let props = LineProperties {
            border_width: 2,
            ..Default::default()
        };
        let config = test_config();
        let style = build_line_style(&props, &config);
        assert_eq!(style.line_width, Some(2.0));
    }

    #[test]
    fn coordinates_set() {
        let props = LineProperties {
            x1: 1440,
            y1: 720,
            x2: 2880,
            y2: 1440,
            ..Default::default()
        };
        let config = test_config();
        let style = build_line_style(&props, &config);
        // 1440 twips at 96 DPI = 96 pixels
        assert_eq!(style.line_x1, Some(96.0));
        assert_eq!(style.line_y1, Some(48.0));
        assert_eq!(style.line_x2, Some(192.0));
        assert_eq!(style.line_y2, Some(96.0));
    }

    #[test]
    fn line_color_system() {
        let props = LineProperties {
            border_color: Color::System { index: 0x07 },
            ..Default::default()
        };
        let config = test_config();
        let style = build_line_style(&props, &config);
        assert!(matches!(style.line_color, Some(CssColor::Named(_))));
    }

    #[test]
    fn zero_width_line() {
        let props = LineProperties {
            border_width: 0,
            ..Default::default()
        };
        let config = test_config();
        let style = build_line_style(&props, &config);
        assert_eq!(style.line_width, Some(0.0));
    }
}
