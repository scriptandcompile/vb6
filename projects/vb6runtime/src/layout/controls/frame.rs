//! Style builder for Frame controls.
//!
//! Maps Frame properties to [`LayoutStyle`]:
//! - `back_color` → `background_color`
//! - `fore_color` → `color`
//! - `font` → `font_family`, `font_size`, `font_weight`, `font_style`, `text_decoration`

use vb6parse::language::{BorderStyle, ClipControls, FrameProperties, TextDirection};

use super::super::LayoutConfig;
use super::super::color::{color_to_css, mouse_pointer_css};
use super::super::font_points_to_px;
use super::super::model::style::{font_weight_css, LayoutStyle};

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

    style.box_shadow = match props.appearance {
        vb6parse::language::Appearance::ThreeD => Some(
            "inset -1px -1px 0 rgb(128, 128, 128), inset 1px 1px 0 rgb(255, 255, 255)".to_string(),
        ),
        vb6parse::language::Appearance::Flat => None,
    };

    style.cursor = mouse_pointer_css(props.mouse_pointer);
    style.direction = if matches!(props.right_to_left, TextDirection::RightToLeft) {
        Some("rtl".to_string())
    } else {
        None
    };

    if matches!(props.clip_controls, ClipControls::Clipped) {
        style.overflow = Some("hidden".to_string());
    }

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
    fn frame_threed_appearance() {
        let props = FrameProperties {
            appearance: vb6parse::language::Appearance::ThreeD,
            ..Default::default()
        };
        let config = test_config();
        let style = build_frame_style(&props, &config);
        assert!(style.box_shadow.is_some());
    }

    #[test]
    fn frame_flat_appearance() {
        let props = FrameProperties {
            appearance: vb6parse::language::Appearance::Flat,
            ..Default::default()
        };
        let config = test_config();
        let style = build_frame_style(&props, &config);
        assert!(style.box_shadow.is_none());
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
        assert_eq!(
            style.border,
            Some("1px solid rgb(120, 120, 120)".to_string())
        );
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

    #[test]
    fn clip_controls_clipped_sets_overflow() {
        let props = FrameProperties {
            clip_controls: vb6parse::language::ClipControls::Clipped,
            ..Default::default()
        };
        let config = test_config();
        let style = build_frame_style(&props, &config);
        assert_eq!(style.overflow, Some("hidden".to_string()));
    }

    #[test]
    fn clip_controls_unbounded_no_overflow() {
        let props = FrameProperties {
            clip_controls: vb6parse::language::ClipControls::Unbounded,
            ..Default::default()
        };
        let config = test_config();
        let style = build_frame_style(&props, &config);
        assert_eq!(style.overflow, None);
    }
}
