//! Style builder for CheckBox and OptionButton controls.
//!
//! Maps CheckBox / OptionButton properties to [`LayoutStyle`]:
//! - `back_color` → `background_color`
//! - `fore_color` → `color`
//! - `font` → `font_family`, `font_size`, `font_weight`, `font_style`, `text_decoration`

use vb6parse::language::{
    CheckBoxProperties, JustifyAlignment, OptionButtonProperties, TextDirection,
};

use super::super::LayoutConfig;
use super::super::color::{color_to_css, mouse_pointer_css};
use super::super::font_points_to_px;
use super::super::model::style::{font_weight_css, LayoutStyle};

/// Build CSS style for a CheckBox control.
pub fn build_checkbox_style(props: &CheckBoxProperties, config: &LayoutConfig) -> LayoutStyle {
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

    style.text_align = match props.alignment {
        JustifyAlignment::LeftJustify => Some("left".to_string()),
        JustifyAlignment::RightJustify => Some("right".to_string()),
    };

    // Diff against VB6 defaults — set matching fields to None
    let defaults = LayoutStyle::default_checkbox(config);
    style.diff_against(&defaults);

    style
}

/// Build CSS style for an OptionButton (radio button) control.
pub fn build_optionbutton_style(
    props: &OptionButtonProperties,
    config: &LayoutConfig,
) -> LayoutStyle {
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

    style.text_align = match props.alignment {
        JustifyAlignment::LeftJustify => Some("left".to_string()),
        JustifyAlignment::RightJustify => Some("right".to_string()),
    };

    // Diff against VB6 defaults — set matching fields to None
    let defaults = LayoutStyle::default_optionbutton(config);
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

    // CheckBox tests
    #[test]
    fn checkbox_threed_appearance() {
        let props = CheckBoxProperties {
            appearance: vb6parse::language::Appearance::ThreeD,
            ..Default::default()
        };
        let config = test_config();
        let style = build_checkbox_style(&props, &config);
        assert!(style.box_shadow.is_some());
    }

    #[test]
    fn checkbox_flat_appearance() {
        let props = CheckBoxProperties {
            appearance: vb6parse::language::Appearance::Flat,
            ..Default::default()
        };
        let config = test_config();
        let style = build_checkbox_style(&props, &config);
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn checkbox_basic_style_default() {
        let props = CheckBoxProperties {
            font: None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_checkbox_style(&props, &config);
        assert!(style.background_color.is_some());
        assert!(style.color.is_some());
        assert!(style.font_family.is_none());
    }

    #[test]
    fn checkbox_font_properties_set() {
        let props = CheckBoxProperties {
            font: Some(vb6parse::language::Font {
                name: "Tahoma".into(),
                size: 8.0,
                weight: 400,
                italic: false,
                underline: false,
                ..Default::default()
            }),
            ..Default::default()
        };
        let config = test_config();
        let style = build_checkbox_style(&props, &config);
        assert_eq!(style.font_family, Some("Tahoma".to_string()));
        assert!((style.font_size.unwrap() - 10.67).abs() < 0.01);
        assert_eq!(style.font_style, Some("normal".to_string()));
    }

    #[test]
    fn checkbox_back_color_custom() {
        let props = CheckBoxProperties {
            back_color: Color::RGB {
                red: 255,
                green: 200,
                blue: 100,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_checkbox_style(&props, &config);
        assert_eq!(style.background_color, Some(CssColor::Rgb(255, 200, 100)));
    }

    #[test]
    fn checkbox_font_not_set() {
        let props = CheckBoxProperties {
            font: None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_checkbox_style(&props, &config);
        assert!(style.font_family.is_none());
        assert!(style.font_size.is_none());
        assert!(style.font_weight.is_none());
    }

    // OptionButton tests
    #[test]
    fn optionbutton_basic_style_default() {
        let props = OptionButtonProperties {
            font: None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_optionbutton_style(&props, &config);
        assert!(style.background_color.is_some());
        assert!(style.color.is_some());
        assert!(style.font_family.is_none());
    }

    #[test]
    fn optionbutton_font_properties_set() {
        let props = OptionButtonProperties {
            font: Some(vb6parse::language::Font {
                name: "Verdana".into(),
                size: 9.0,
                weight: 700,
                italic: true,
                underline: false,
                ..Default::default()
            }),
            ..Default::default()
        };
        let config = test_config();
        let style = build_optionbutton_style(&props, &config);
        assert_eq!(style.font_family, Some("Verdana".to_string()));
        assert_eq!(style.font_weight, Some("bold".to_string()));
        assert_eq!(style.font_style, Some("italic".to_string()));
    }

    #[test]
    fn optionbutton_back_color_custom() {
        let props = OptionButtonProperties {
            back_color: Color::RGB {
                red: 100,
                green: 200,
                blue: 50,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_optionbutton_style(&props, &config);
        assert_eq!(style.background_color, Some(CssColor::Rgb(100, 200, 50)));
    }

    #[test]
    fn optionbutton_threed_appearance() {
        let props = OptionButtonProperties {
            appearance: vb6parse::language::Appearance::ThreeD,
            ..Default::default()
        };
        let config = test_config();
        let style = build_optionbutton_style(&props, &config);
        assert!(style.box_shadow.is_some());
    }

    #[test]
    fn optionbutton_flat_appearance() {
        let props = OptionButtonProperties {
            appearance: vb6parse::language::Appearance::Flat,
            ..Default::default()
        };
        let config = test_config();
        let style = build_optionbutton_style(&props, &config);
        assert!(style.box_shadow.is_none());
    }
}
