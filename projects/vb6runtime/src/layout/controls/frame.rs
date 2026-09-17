//! Style builder for Frame controls.
//!
//! Maps Frame properties to [`LayoutStyle`]:
//! - `back_color` → `background_color`
//! - `fore_color` → `color`
//! - `font` → `font_family`, `font_size`, `font_weight`, `font_style`, `text_decoration`

use vb6parse::language::{BorderStyle, FrameProperties};

use super::super::LayoutConfig;
use super::super::color::color_to_css;
use super::super::font_points_to_px;
use super::super::model::style::LayoutStyle;

/// Build CSS style for a Frame control.
pub fn build_frame_style(props: &FrameProperties, config: &LayoutConfig) -> LayoutStyle {
    let mut style = LayoutStyle {
        background_color: Some(color_to_css(&props.back_color)),
        color: Some(color_to_css(&props.fore_color)),
        ..LayoutStyle::default()
    };

    // Frames always have a border in VB6 unless BorderStyle is None.
    // The `<fieldset>` element supplies the caption notch on the top edge.
    style.border = match props.border_style {
        BorderStyle::None => Some("none".to_string()),
        BorderStyle::FixedSingle => Some("1px solid rgb(120, 120, 120)".to_string()),
    };

    if let Some(ref font) = props.font {
        style.font_family = Some(font.name.clone());
        style.font_size = Some(font_points_to_px(font.size, config.dpi));
        style.font_weight = font_weight_css(font.weight);
        style.font_style = if font.italic {
            Some("italic".to_string())
        } else {
            Some("normal".to_string())
        };
        style.text_decoration = if font.underline {
            Some("underline".to_string())
        } else {
            Some("none".to_string())
        };
    }

    style
}

fn font_weight_css(weight: i32) -> Option<String> {
    match weight {
        100 => Some("100".to_string()),
        200 => Some("200".to_string()),
        300 => Some("300".to_string()),
        400 => Some("normal".to_string()),
        500 => Some("500".to_string()),
        600 => Some("600".to_string()),
        700 => Some("bold".to_string()),
        800 => Some("800".to_string()),
        900 => Some("900".to_string()),
        _ => None,
    }
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
    fn basic_style_default() {
        let props = FrameProperties {
            font: None,
            border_style: BorderStyle::FixedSingle,
            ..Default::default()
        };
        let config = test_config();
        let style = build_frame_style(&props, &config);
        assert!(style.background_color.is_some());
        assert!(style.color.is_some());
        assert!(style.font_family.is_none());
        assert_eq!(style.border, Some("1px solid rgb(120, 120, 120)".to_string()));
    }

    #[test]
    fn border_none_is_none() {
        let props = FrameProperties {
            border_style: BorderStyle::None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_frame_style(&props, &config);
        assert_eq!(style.border, Some("none".to_string()));
    }

    #[test]
    fn font_properties_set() {
        let props = FrameProperties {
            font: Some(vb6parse::language::Font {
                name: "Arial".into(),
                size: 10.0,
                weight: 700,
                italic: true,
                underline: true,
                ..Default::default()
            }),
            ..Default::default()
        };
        let config = test_config();
        let style = build_frame_style(&props, &config);
        assert_eq!(style.font_family, Some("Arial".to_string()));
        assert!((style.font_size.unwrap() - 13.33).abs() < 0.01);
        assert_eq!(style.font_weight, Some("bold".to_string()));
        assert_eq!(style.font_style, Some("italic".to_string()));
        assert_eq!(style.text_decoration, Some("underline".to_string()));
    }

    #[test]
    fn font_properties_not_set() {
        let props = FrameProperties {
            font: None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_frame_style(&props, &config);
        assert!(style.font_family.is_none());
        assert!(style.font_size.is_none());
        assert!(style.font_weight.is_none());
        assert!(style.font_style.is_none());
        assert!(style.text_decoration.is_none());
    }

    #[test]
    fn back_color_custom() {
        let props = FrameProperties {
            back_color: Color::RGB {
                red: 100,
                green: 150,
                blue: 200,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_frame_style(&props, &config);
        assert_eq!(style.background_color, Some(CssColor::Rgb(100, 150, 200)));
    }

    #[test]
    fn fore_color_custom() {
        let props = FrameProperties {
            fore_color: Color::RGB {
                red: 255,
                green: 0,
                blue: 0,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_frame_style(&props, &config);
        assert_eq!(style.color, Some(CssColor::Rgb(255, 0, 0)));
    }
}
