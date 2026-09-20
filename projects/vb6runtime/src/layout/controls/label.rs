//! Style builder for Label controls.
//!
//! Maps Label properties to [`LayoutStyle`]:
//! - `back_color` → `background_color`
//! - `fore_color` → `color`
//! - `font` → `font_family`, `font_size`, `font_weight`, `font_style`, `text_decoration`
//! - `alignment` → `text_align`
//! - `word_wrap` → `white_space`
//! - `back_style` → `background_color` (Transparent → None)

use vb6parse::language::{BackStyle, LabelProperties, TextDirection, WordWrap};

use super::super::LayoutConfig;
use super::super::color::{color_to_css, mouse_pointer_css};
use super::super::font_points_to_px;
use super::super::model::style::{LayoutStyle, alignment_css, font_weight_css};

/// Build CSS style for a Label control.
pub fn build_label_style(props: &LabelProperties, config: &LayoutConfig) -> LayoutStyle {
    let mut style = LayoutStyle {
        background_color: match props.back_style {
            BackStyle::Transparent => None,
            BackStyle::Opaque => Some(color_to_css(&props.back_color)),
        },
        color: Some(color_to_css(&props.fore_color)),
        font_family: props.font.as_ref().map(|f| f.name.clone()),
        font_size: props
            .font
            .as_ref()
            .map(|f| font_points_to_px(f.size, config.dpi)),
        font_weight: props.font.as_ref().and_then(|f| font_weight_css(f.weight)),
        font_style: props
            .font
            .as_ref()
            .map(|f| if f.italic { "italic" } else { "normal" }.to_string()),
        text_decoration: props
            .font
            .as_ref()
            .map(|f| if f.underline { "underline" } else { "none" }.to_string()),
        text_align: alignment_css(props.alignment),
        white_space: match props.word_wrap {
            WordWrap::NonWrapping => Some("nowrap".to_string()),
            WordWrap::Wrapping => Some("pre-wrap".to_string()),
        },
        cursor: mouse_pointer_css(props.mouse_pointer),
        direction: if matches!(props.right_to_left, TextDirection::RightToLeft) {
            Some("rtl".to_string())
        } else {
            None
        },
        border: match props.border_style {
            vb6parse::language::BorderStyle::None => Some("none".to_string()),
            vb6parse::language::BorderStyle::FixedSingle => {
                Some("1px solid rgb(120, 120, 120)".to_string())
            }
        },
        box_shadow: match props.appearance {
            vb6parse::language::Appearance::ThreeD => Some(
                "inset -1px -1px 0 rgb(128, 128, 128), inset 1px 1px 0 rgb(255, 255, 255)"
                    .to_string(),
            ),
            vb6parse::language::Appearance::Flat => None,
        },
        ..LayoutStyle::default()
    };

    // Diff against VB6 defaults — set matching fields to None
    let defaults = LayoutStyle::default_label(config);
    style.diff_against(&defaults);

    style
}
#[cfg(test)]
mod tests {
    use super::super::super::model::style::CssColor;
    use super::*;
    use vb6parse::language::{Alignment, BorderStyle, Color};

    fn test_config() -> LayoutConfig {
        LayoutConfig {
            dpi: 96,
            ..Default::default()
        }
    }

    #[test]
    fn label_threed_appearance() {
        let props = LabelProperties {
            appearance: vb6parse::language::Appearance::ThreeD,
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        // ThreeD is the VB6 default → box_shadow zeroed by diff_against
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn label_flat_appearance() {
        let props = LabelProperties {
            appearance: vb6parse::language::Appearance::Flat,
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert!(style.box_shadow.is_none());
    }

    fn make_font(name: &str, size: f32) -> vb6parse::language::Font {
        vb6parse::language::Font {
            name: name.to_string(),
            size,
            ..Default::default()
        }
    }

    #[test]
    fn basic_style_default_font_none() {
        let props = LabelProperties {
            font: None,
            back_style: BackStyle::Transparent,
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert!(style.font_family.is_none());
        assert!(style.font_size.is_none());
        assert!(style.font_weight.is_none());
        assert_eq!(style.background_color, None);
        // Fore color matches VB6 default → zeroed by diff_against
        assert!(style.color.is_none());
        // 3D bevel is the default appearance → zeroed by diff_against
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn alignment_center() {
        let props = LabelProperties {
            alignment: Alignment::Center,
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert_eq!(style.text_align, Some("center".to_string()));
    }

    #[test]
    fn alignment_right() {
        let props = LabelProperties {
            alignment: Alignment::RightJustify,
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert_eq!(style.text_align, Some("right".to_string()));
    }

    #[test]
    fn word_wrap_wrapping() {
        let props = LabelProperties {
            word_wrap: WordWrap::Wrapping,
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        // Wrapping → pre-wrap is the VB6 default → zeroed by diff_against
        assert_eq!(style.white_space, None);
    }

    #[test]
    fn word_wrap_non_wrapping_preserved() {
        let props = LabelProperties {
            word_wrap: WordWrap::NonWrapping,
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        // NonWrapping differs from default Wrapping → preserved
        assert_eq!(style.white_space, Some("nowrap".to_string()));
    }

    #[test]
    fn word_wrap_non_wrapping() {
        let props = LabelProperties {
            word_wrap: WordWrap::NonWrapping,
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert_eq!(style.white_space, Some("nowrap".to_string()));
    }

    #[test]
    fn transparent_background() {
        let props = LabelProperties {
            back_style: BackStyle::Transparent,
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert_eq!(style.background_color, None);
    }

    #[test]
    fn opaque_background() {
        let props = LabelProperties {
            back_style: BackStyle::Opaque,
            back_color: Color::RGB {
                red: 255,
                green: 255,
                blue: 255,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert!(matches!(
            style.background_color,
            Some(CssColor::Rgb(255, 255, 255))
        ));
    }

    #[test]
    fn font_properties_applied() {
        let props = LabelProperties {
            font: Some(make_font("Arial", 12.0)),
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert_eq!(style.font_family, Some("Arial".to_string()));
        assert_eq!(style.font_size, Some(16.0));
        assert_eq!(style.font_style, Some("normal".to_string()));
    }

    #[test]
    fn italic_font() {
        let props = LabelProperties {
            font: Some(vb6parse::language::Font {
                name: "Arial".into(),
                size: 12.0,
                italic: true,
                ..Default::default()
            }),
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert_eq!(style.font_style, Some("italic".to_string()));
    }

    #[test]
    fn underline_font() {
        let props = LabelProperties {
            font: Some(vb6parse::language::Font {
                name: "Arial".into(),
                size: 12.0,
                underline: true,
                ..Default::default()
            }),
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert_eq!(style.text_decoration, Some("underline".to_string()));
    }

    #[test]
    fn bold_font_weight() {
        let props = LabelProperties {
            font: Some(vb6parse::language::Font {
                name: "Arial".into(),
                size: 12.0,
                weight: 700,
                ..Default::default()
            }),
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert_eq!(style.font_weight, Some("bold".to_string()));
    }

    #[test]
    fn label_fixed_single_border() {
        let props = LabelProperties {
            border_style: BorderStyle::FixedSingle,
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert_eq!(
            style.border,
            Some("1px solid rgb(120, 120, 120)".to_string())
        );
    }

    #[test]
    fn label_border_none() {
        let props = LabelProperties {
            border_style: BorderStyle::None,
            ..Default::default()
        };
        let config = test_config();
        let style = build_label_style(&props, &config);
        assert_eq!(style.border, Some("none".to_string()));
    }
}
