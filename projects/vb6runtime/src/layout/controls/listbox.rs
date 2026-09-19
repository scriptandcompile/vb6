//! Style builder for ListBox controls.
//!
//! Maps ListBox properties to [`LayoutStyle`]:
//! - `back_color` → `background_color`
//! - `fore_color` → `color`
//! - `font` → `font_family`, `font_size`, `font_weight`, `font_style`, `text_decoration`

use vb6parse::language::{ListBoxProperties, TextDirection};

use super::super::LayoutConfig;
use super::super::color::{color_to_css, mouse_pointer_css};
use super::super::font_points_to_px;
use super::super::model::style::LayoutStyle;

/// Build CSS style for a ListBox control.
pub fn build_listbox_style(props: &ListBoxProperties, config: &LayoutConfig) -> LayoutStyle {
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

    style.overflow = Some("auto".to_string());

    style.cursor = mouse_pointer_css(props.mouse_pointer);
    style.direction = if matches!(props.right_to_left, TextDirection::RightToLeft) {
        Some("rtl".to_string())
    } else {
        None
    };

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
        let props = ListBoxProperties {
            font: None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_listbox_style(&props, &config);
        assert!(style.background_color.is_some());
        assert!(style.color.is_some());
        assert!(style.font_family.is_none());
        assert_eq!(style.overflow, Some("auto".to_string()));
    }

    #[test]
    fn overflow_auto() {
        let props = ListBoxProperties::default();
        let config = test_config();
        let style = build_listbox_style(&props, &config);
        assert_eq!(style.overflow, Some("auto".to_string()));
    }

    #[test]
    fn font_properties_set() {
        let props = ListBoxProperties {
            font: Some(vb6parse::language::Font {
                name: "Consolas".into(),
                size: 10.0,
                weight: 400,
                italic: false,
                underline: false,
                ..Default::default()
            }),
            ..Default::default()
        };
        let config = test_config();
        let style = build_listbox_style(&props, &config);
        assert_eq!(style.font_family, Some("Consolas".to_string()));
        assert!((style.font_size.unwrap() - 13.33).abs() < 0.01);
    }

    #[test]
    fn back_color_custom() {
        let props = ListBoxProperties {
            back_color: Color::RGB {
                red: 255,
                green: 255,
                blue: 220,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_listbox_style(&props, &config);
        assert_eq!(style.background_color, Some(CssColor::Rgb(255, 255, 220)));
    }
}
