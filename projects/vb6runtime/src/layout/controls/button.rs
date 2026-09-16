//! Style builder for CommandButton controls.
//!
//! Maps CommandButton properties to [`LayoutStyle`]:
//! - `back_color` → `background_color`
//! - `fore_color` → `color`
//! - `font` → `font_family`, `font_size`, `font_weight`, `font_style`, `text_decoration`

use vb6parse::language::CommandButtonProperties;

use super::super::LayoutConfig;
use super::super::color::color_to_css;
use super::super::font_points_to_px;
use super::super::model::style::LayoutStyle;

/// Build CSS style for a CommandButton control.
pub fn build_button_style(props: &CommandButtonProperties, config: &LayoutConfig) -> LayoutStyle {
    let mut style = LayoutStyle {
        background_color: Some(color_to_css(&props.back_color)),
        display: Some("inline-block".to_string()),
        border: Some("1px solid".to_string()),
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

    style.display = Some("inline-block".to_string());
    style.border = Some("1px solid".to_string());

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
        let props = CommandButtonProperties {
            font: None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_button_style(&props, &config);
        assert!(style.background_color.is_some());
        assert_eq!(style.display, Some("inline-block".to_string()));
        assert_eq!(style.border, Some("1px solid".to_string()));
        assert!(style.font_family.is_none());
        assert!(style.font_size.is_none());
    }

    #[test]
    fn font_properties_set() {
        let props = CommandButtonProperties {
            font: Some(vb6parse::language::Font {
                name: "Arial".into(),
                size: 12.0,
                weight: 700,
                italic: true,
                underline: true,
                ..Default::default()
            }),
            ..Default::default()
        };
        let config = test_config();
        let style = build_button_style(&props, &config);
        assert_eq!(style.font_family, Some("Arial".to_string()));
        assert_eq!(style.font_size, Some(16.0));
        assert_eq!(style.font_weight, Some("bold".to_string()));
        assert_eq!(style.font_style, Some("italic".to_string()));
        assert_eq!(style.text_decoration, Some("underline".to_string()));
    }

    #[test]
    fn no_font_defaults() {
        let props = CommandButtonProperties {
            font: None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_button_style(&props, &config);
        assert!(style.font_weight.is_none());
        assert!(style.font_style.is_none());
        assert!(style.text_decoration.is_none());
        assert!(style.font_family.is_none());
        assert!(style.font_size.is_none());
    }

    #[test]
    fn background_color_set() {
        let props = CommandButtonProperties {
            back_color: Color::RGB {
                red: 200,
                green: 100,
                blue: 50,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_button_style(&props, &config);
        assert_eq!(style.background_color, Some(CssColor::Rgb(200, 100, 50)));
    }

    #[test]
    fn border_always_set() {
        let props = CommandButtonProperties::default();
        let config = test_config();
        let style = build_button_style(&props, &config);
        assert_eq!(style.border, Some("1px solid".to_string()));
    }
}
