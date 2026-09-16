//! Style builder for Label controls.
//!
//! Maps Label properties to [`LayoutStyle`]:
//! - `back_color` → `background_color`
//! - `fore_color` → `color`
//! - `font` → `font_family`, `font_size`, `font_weight`, `font_style`, `text_decoration`
//! - `alignment` → `text_align`
//! - `word_wrap` → `white_space`
//! - `back_style` → `background_color` (Transparent → None)

use vb6parse::language::{Alignment, BackStyle, LabelProperties, WordWrap};

use super::super::LayoutConfig;
use super::super::color::color_to_css;
use super::super::font_points_to_px;
use super::super::model::style::LayoutStyle;

/// Build CSS style for a Label control.
pub fn build_label_style(props: &LabelProperties, config: &LayoutConfig) -> LayoutStyle {
    LayoutStyle {
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
        ..LayoutStyle::default()
    }
}

/// Convert a vb6parse font weight to a CSS font-weight string.
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

/// Convert VB6 alignment to a CSS text-align string.
fn alignment_css(alignment: Alignment) -> Option<String> {
    match alignment {
        Alignment::RightJustify => Some("right".to_string()),
        Alignment::Center => Some("center".to_string()),
        Alignment::LeftJustify => Some("left".to_string()),
    }
}
