//! Style builder for Line controls.
//!
//! Maps Line properties to [`LayoutStyle`]:
//! - `border_color` → `line_color`
//! - `border_width` → `line_width`
//! - Coordinates stored in `line_*` fields for SVG rendering

use super::super::LayoutConfig;
use super::super::color::color_to_css;
use super::super::model::style::LayoutStyle;
use super::super::scale::scale_mode_to_pixels;
use vb6parse::language::{LineProperties, ScaleMode};

/// Build CSS style for a Line control.
pub fn build_line_style(props: &LineProperties, config: &LayoutConfig) -> LayoutStyle {
    LayoutStyle {
        line_color: Some(color_to_css(&props.border_color)),
        line_width: Some(props.border_width as f32),
        line_x1: Some(scale_mode_to_pixels(props.x1, ScaleMode::Twip, config.dpi)),
        line_y1: Some(scale_mode_to_pixels(props.y1, ScaleMode::Twip, config.dpi)),
        line_x2: Some(scale_mode_to_pixels(props.x2, ScaleMode::Twip, config.dpi)),
        line_y2: Some(scale_mode_to_pixels(props.y2, ScaleMode::Twip, config.dpi)),
        ..LayoutStyle::default()
    }
}
