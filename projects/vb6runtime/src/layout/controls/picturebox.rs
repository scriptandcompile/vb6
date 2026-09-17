//! Style builder for PictureBox controls.
//!
//! Maps PictureBox properties to [`LayoutStyle`]:
//! - `back_color` → `background_color`
//! - `fore_color` → `color`
//! - `font` → `font_family`, `font_size`, `font_weight`, `font_style`, `text_decoration`
//! - `border_style` → `border`

use vb6parse::language::{BorderStyle, PictureBoxProperties};

use super::super::LayoutConfig;
use super::super::color::color_to_css;
use super::super::font_points_to_px;
use super::super::model::style::LayoutStyle;

/// Build CSS style for a PictureBox control.
pub fn build_picturebox_style(props: &PictureBoxProperties, config: &LayoutConfig) -> LayoutStyle {
    let mut style = LayoutStyle {
        background_color: Some(color_to_css(&props.back_color)),
        color: Some(color_to_css(&props.fore_color)),
        ..LayoutStyle::default()
    };

    // A border is provided by the .vb6-picturebox class; an explicit
    // BorderStyle::None must override it so borderless pictureboxes stay borderless.
    style.border = match props.border_style {
        BorderStyle::None => Some("none".to_string()),
        BorderStyle::FixedSingle => None,
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

    style.overflow = Some("hidden".to_string());

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
    use vb6parse::language::BorderStyle;
    use vb6parse::language::Color;

    fn test_config() -> LayoutConfig {
        LayoutConfig {
            dpi: 96,
            ..Default::default()
        }
    }

    #[test]
    fn basic_style_default() {
        let props = PictureBoxProperties {
            font: None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_picturebox_style(&props, &config);
        assert!(style.background_color.is_some());
        assert!(style.color.is_some());
        assert!(style.font_family.is_none());
        assert_eq!(style.overflow, Some("hidden".to_string()));
    }

    #[test]
    fn border_via_css_class() {
        // PictureBox border is handled by CSS class .vb6-picturebox
        // which sets border: 1px solid var(--vb6-button-border), not by inline style.
        let props = PictureBoxProperties {
            border_style: BorderStyle::FixedSingle,
            ..Default::default()
        };
        let config = test_config();
        let style = build_picturebox_style(&props, &config);
        assert!(style.border.is_none());
    }

    #[test]
    fn border_none_is_none() {
        let props = PictureBoxProperties {
            border_style: BorderStyle::None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_picturebox_style(&props, &config);
        assert_eq!(style.border, Some("none".to_string()));
    }

    #[test]
    fn overflow_always_hidden() {
        let props = PictureBoxProperties::default();
        let config = test_config();
        let style = build_picturebox_style(&props, &config);
        assert_eq!(style.overflow, Some("hidden".to_string()));
    }

    #[test]
    fn font_properties_applied() {
        let props = PictureBoxProperties {
            font: Some(vb6parse::language::Font {
                name: "Courier New".into(),
                size: 12.0,
                weight: 400,
                italic: false,
                underline: false,
                ..Default::default()
            }),
            ..Default::default()
        };
        let config = test_config();
        let style = build_picturebox_style(&props, &config);
        assert_eq!(style.font_family, Some("Courier New".to_string()));
        assert_eq!(style.font_size, Some(16.0));
    }

    #[test]
    fn custom_colors() {
        let props = PictureBoxProperties {
            back_color: Color::RGB {
                red: 0,
                green: 128,
                blue: 255,
            },
            fore_color: Color::RGB {
                red: 255,
                green: 255,
                blue: 255,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_picturebox_style(&props, &config);
        assert_eq!(style.background_color, Some(CssColor::Rgb(0, 128, 255)));
        assert_eq!(style.color, Some(CssColor::Rgb(255, 255, 255)));
    }
}
