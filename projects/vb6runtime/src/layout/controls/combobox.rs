//! Style builder for ComboBox controls.
//!
//! Maps ComboBox properties to [`LayoutStyle`]:
//! - `back_color` → `background_color`
//! - `fore_color` → `color`
//! - `font` → `font_family`, `font_size`, `font_weight`, `font_style`, `text_decoration`

use vb6parse::language::{ComboBoxProperties, TextDirection};

use super::super::LayoutConfig;
use super::super::color::{color_to_css, mouse_pointer_css};
use super::super::font_points_to_px;
use super::super::model::style::{font_weight_css, LayoutStyle};

/// Build CSS style for a ComboBox control.
pub fn build_combobox_style(props: &ComboBoxProperties, config: &LayoutConfig) -> LayoutStyle {
    let mut style = LayoutStyle {
        background_color: Some(color_to_css(&props.back_color)),
        color: Some(color_to_css(&props.fore_color)),
        ..LayoutStyle::default()
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

    // Diff against VB6 defaults — set matching fields to None
    let defaults = LayoutStyle::default_combobox(config);
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
    fn combobox_threed_appearance() {
        let props = ComboBoxProperties {
            appearance: vb6parse::language::Appearance::ThreeD,
            ..Default::default()
        };
        let config = test_config();
        let style = build_combobox_style(&props, &config);
        // ThreeD is the VB6 default → box_shadow zeroed by diff_against
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn combobox_flat_appearance() {
        let props = ComboBoxProperties {
            appearance: vb6parse::language::Appearance::Flat,
            ..Default::default()
        };
        let config = test_config();
        let style = build_combobox_style(&props, &config);
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn basic_style_default() {
        let props = ComboBoxProperties {
            font: None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_combobox_style(&props, &config);
        // Default colors match VB6 system defaults → zeroed by diff_against
        assert!(style.background_color.is_none());
        assert!(style.color.is_none());
        assert!(style.font_family.is_none());
        // ThreeD is the VB6 default → box_shadow zeroed by diff_against
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn font_properties_set() {
        let props = ComboBoxProperties {
            font: Some(vb6parse::language::Font {
                name: "Segoe UI".into(),
                size: 11.0,
                weight: 400,
                italic: false,
                underline: false,
                ..Default::default()
            }),
            ..Default::default()
        };
        let config = test_config();
        let style = build_combobox_style(&props, &config);
        assert_eq!(style.font_family, Some("Segoe UI".to_string()));
        assert!((style.font_size.unwrap() - 14.67).abs() < 0.01);
        assert_eq!(style.font_style, Some("normal".to_string()));
    }

    #[test]
    fn back_color_custom() {
        let props = ComboBoxProperties {
            back_color: Color::RGB {
                red: 240,
                green: 240,
                blue: 240,
            },
            fore_color: Color::RGB {
                red: 30,
                green: 30,
                blue: 30,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_combobox_style(&props, &config);
        assert_eq!(style.background_color, Some(CssColor::Rgb(240, 240, 240)));
        assert_eq!(style.color, Some(CssColor::Rgb(30, 30, 30)));
    }

    #[test]
    fn font_not_set() {
        let props = ComboBoxProperties {
            font: None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_combobox_style(&props, &config);
        assert!(style.font_family.is_none());
        assert!(style.font_size.is_none());
        assert!(style.font_weight.is_none());
    }
}
