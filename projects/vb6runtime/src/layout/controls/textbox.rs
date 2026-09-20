//! Style builder for TextBox controls.
//!
//! Maps TextBox properties to [`LayoutStyle`]:
//! - `back_color` → `background_color`
//! - `fore_color` → `color`
//! - `font` → `font_family`, `font_size`, `font_weight`, `font_style`, `text_decoration`
//! - `alignment` → `text_align`
//! - `scroll_bars` + `multi_line` → `overflow`
//! - `border_style` → `border`

use vb6parse::language::{BorderStyle, TextBoxProperties, TextDirection};

use super::super::LayoutConfig;
use super::super::color::{color_to_css, mouse_pointer_css};
use super::super::font_points_to_px;
use super::super::model::style::{LayoutStyle, alignment_css, font_weight_css};

/// Build CSS style for a TextBox control.
pub fn build_textbox_style(props: &TextBoxProperties, config: &LayoutConfig) -> LayoutStyle {
    let mut style = LayoutStyle {
        background_color: Some(color_to_css(&props.back_color)),
        color: Some(color_to_css(&props.fore_color)),
        ..LayoutStyle::default()
    };

    // A border is provided by the .vb6-textbox class; an explicit
    // BorderStyle::None must override it so borderless textboxes stay borderless.
    style.border = match props.border_style {
        BorderStyle::None => Some("none".to_string()),
        BorderStyle::FixedSingle => None,
    };

    style.multi_line = props.multi_line == vb6parse::language::MultiLine::MultiLine;

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

    style.text_align = alignment_css(props.alignment);

    if props.multi_line == vb6parse::language::MultiLine::MultiLine {
        style.overflow = Some("auto".to_string());
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
    let defaults = LayoutStyle::default_textbox(config);
    style.diff_against(&defaults);

    style
}

#[cfg(test)]
mod tests {
    use super::super::super::model::style::CssColor;
    use super::*;
    use vb6parse::language::Color;
    use vb6parse::language::MultiLine;
    use vb6parse::language::{Alignment, BorderStyle};

    fn test_config() -> LayoutConfig {
        LayoutConfig {
            dpi: 96,
            ..Default::default()
        }
    }

    #[test]
    fn textbox_threed_appearance() {
        let props = TextBoxProperties {
            appearance: vb6parse::language::Appearance::ThreeD,
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        // ThreeD is the VB6 default → box_shadow zeroed by diff_against
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn textbox_flat_appearance() {
        let props = TextBoxProperties {
            appearance: vb6parse::language::Appearance::Flat,
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn basic_style_default() {
        let props = TextBoxProperties {
            font: None,
            border_style: BorderStyle::None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        // Default colors match VB6 system defaults → zeroed by diff_against
        assert!(style.background_color.is_none());
        assert!(style.color.is_none());
        assert!(style.font_family.is_none());
        assert_eq!(style.overflow, None);
        // BorderStyle::None differs from default FixedSingle → preserved
        assert_eq!(style.border, Some("none".to_string()));
    }

    #[test]
    fn multiline_overflow() {
        let props = TextBoxProperties {
            multi_line: MultiLine::MultiLine,
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        assert_eq!(style.overflow, Some("auto".to_string()));
        assert!(style.multi_line);
    }

    #[test]
    fn singleline_no_overflow() {
        let props = TextBoxProperties {
            multi_line: MultiLine::SingleLine,
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        assert_eq!(style.overflow, None);
        assert!(!style.multi_line);
    }

    #[test]
    fn border_via_css_class() {
        // TextBox border is handled by CSS class .vb6-textbox
        // which sets border: 1px solid var(--vb6-button-border), not by inline style.
        let props = TextBoxProperties {
            border_style: BorderStyle::FixedSingle,
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        assert!(style.border.is_none());
    }

    #[test]
    fn border_none() {
        let props = TextBoxProperties {
            border_style: BorderStyle::None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        assert_eq!(style.border, Some("none".to_string()));
    }

    #[test]
    fn text_alignment_left() {
        let props = TextBoxProperties {
            alignment: Alignment::LeftJustify,
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        // LeftJustify is the VB6 default → zeroed by diff_against
        assert_eq!(style.text_align, None);
    }

    #[test]
    fn text_alignment_center_preserved() {
        let props = TextBoxProperties {
            alignment: Alignment::Center,
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        // Center differs from default LeftJustify → preserved
        assert_eq!(style.text_align, Some("center".to_string()));
    }

    #[test]
    fn text_alignment_center() {
        let props = TextBoxProperties {
            alignment: Alignment::Center,
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        assert_eq!(style.text_align, Some("center".to_string()));
    }

    #[test]
    fn text_alignment_right() {
        let props = TextBoxProperties {
            alignment: Alignment::RightJustify,
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        assert_eq!(style.text_align, Some("right".to_string()));
    }

    #[test]
    fn font_properties_applied() {
        let props = TextBoxProperties {
            font: Some(vb6parse::language::Font {
                name: "Times New Roman".into(),
                size: 14.0,
                weight: 400,
                italic: false,
                underline: false,
                ..Default::default()
            }),
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        assert_eq!(style.font_family, Some("Times New Roman".to_string()));
        assert!((style.font_size.unwrap() - 18.67).abs() < 0.01);
        assert_eq!(style.font_style, Some("normal".to_string()));
        assert_eq!(style.text_decoration, Some("none".to_string()));
    }

    #[test]
    fn custom_colors() {
        let props = TextBoxProperties {
            back_color: Color::RGB {
                red: 255,
                green: 255,
                blue: 0,
            },
            fore_color: Color::RGB {
                red: 0,
                green: 0,
                blue: 255,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_textbox_style(&props, &config);
        assert_eq!(style.background_color, Some(CssColor::Rgb(255, 255, 0)));
        assert_eq!(style.color, Some(CssColor::Rgb(0, 0, 255)));
    }
}
