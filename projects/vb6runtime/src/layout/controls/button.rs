//! Style builder for CommandButton controls.
//!
//! Maps CommandButton properties to [`LayoutStyle`]:
//! - `back_color` → `background_color`
//! - `fore_color` → `color`
//! - `font` → `font_family`, `font_size`, `font_weight`, `font_style`, `text_decoration`

use vb6parse::language::{CommandButtonProperties, Style, TextDirection};

use super::super::LayoutConfig;
use super::super::color::{color_to_css, mouse_pointer_css};
use super::super::font_points_to_px;
use super::super::model::style::{font_weight_css, LayoutStyle};

/// Build CSS style for a CommandButton control.
pub fn build_button_style(props: &CommandButtonProperties, config: &LayoutConfig) -> LayoutStyle {
    let mut style = LayoutStyle {
        display: Some("inline-block".to_string()),
        ..LayoutStyle::default()
    };
    style.background_color = match props.style {
        // VB6 only honors BackColor when the button is in Graphical style;
        // Standard buttons use the system ButtonFace color from the theme.
        Style::Standard => None,
        Style::Graphical => Some(color_to_css(&props.back_color)),
    };
    style.box_shadow = match props.appearance {
        vb6parse::language::Appearance::ThreeD => Some(
            "inset -1px -1px 0 rgb(128, 128, 128), inset 1px 1px 0 rgb(255, 255, 255)".to_string(),
        ),
        vb6parse::language::Appearance::Flat => None,
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

    style.cursor = mouse_pointer_css(props.mouse_pointer);
    style.direction = if matches!(props.right_to_left, TextDirection::RightToLeft) {
        Some("rtl".to_string())
    } else {
        None
    };

    // Diff against VB6 defaults — set matching fields to None
    let defaults = LayoutStyle::default_button(config, props.style, props.appearance);
    style.diff_against(&defaults);

    style
}

#[cfg(test)]
mod tests {

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
        // Standard button: background handled by CSS class → zeroed
        // display is always set by builder but matches default → zeroed
        assert!(style.background_color.is_none());
        assert_eq!(style.display, None);
        assert!(style.border.is_none());
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
    fn background_color_via_css_variable() {
        // Button background is handled by CSS class .vb6-commandbutton
        // (background-color: var(--vb6-button-bg)) for Standard-style buttons,
        // matching VB6 where BackColor only applies to Graphical buttons.
        let props = CommandButtonProperties {
            style: Style::Standard,
            back_color: Color::RGB {
                red: 200,
                green: 100,
                blue: 50,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_button_style(&props, &config);
        assert!(style.background_color.is_none());
    }

    #[test]
    fn graphical_button_uses_back_color() {
        // VB6 honors CommandButton.BackColor when Style = Graphical.
        let props = CommandButtonProperties {
            style: Style::Graphical,
            back_color: Color::RGB {
                red: 200,
                green: 100,
                blue: 50,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_button_style(&props, &config);
        assert_eq!(
            style.background_color,
            Some(super::super::super::model::style::CssColor::Rgb(
                200, 100, 50
            ))
        );
    }

    #[test]
    fn standard_button_ignores_back_color() {
        let props = CommandButtonProperties {
            style: Style::Standard,
            back_color: Color::RGB {
                red: 0,
                green: 128,
                blue: 0,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_button_style(&props, &config);
        assert!(style.background_color.is_none());
    }

    #[test]
    fn three_d_appearance_adds_bevel() {
        let props = CommandButtonProperties {
            appearance: vb6parse::language::Appearance::ThreeD,
            ..Default::default()
        };
        let config = test_config();
        let style = build_button_style(&props, &config);
        // ThreeD is the VB6 default → box_shadow zeroed by diff_against
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn flat_appearance_no_bevel() {
        let props = CommandButtonProperties {
            appearance: vb6parse::language::Appearance::Flat,
            ..Default::default()
        };
        let config = test_config();
        let style = build_button_style(&props, &config);
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn border_via_css_class() {
        // Button border is handled by CSS class .vb6-commandbutton
        // which sets border: 1px solid var(--vb6-button-border), not by inline style.
        let props = CommandButtonProperties::default();
        let config = test_config();
        let style = build_button_style(&props, &config);
        assert!(style.border.is_none());
    }
}
