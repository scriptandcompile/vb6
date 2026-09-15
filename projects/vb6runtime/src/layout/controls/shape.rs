//! Style builder for Shape controls.
//!
//! Maps Shape properties to [`LayoutStyle`]:
//! - `back_color` + `back_style` → `background_color`
//! - `border_color` → `border` color
//! - `border_width` → `border_width`
//! - `fill_color` → `fill_color`
//! - `shape` → `border_radius` (for rounded shapes)

use super::super::LayoutConfig;
use super::super::color::color_to_css;
use super::super::model::style::LayoutStyle;
use vb6parse::language::{BackStyle, Shape, ShapeProperties};

/// Build CSS style for a Shape control.
pub fn build_shape_style(props: &ShapeProperties, _config: &LayoutConfig) -> LayoutStyle {
    LayoutStyle {
        background_color: match props.back_style {
            BackStyle::Transparent => None,
            BackStyle::Opaque => Some(color_to_css(&props.back_color)),
        },
        border: Some(format!("{}px solid", props.border_width)),
        fill_color: Some(color_to_css(&props.fill_color)),
        border_radius: match props.shape {
            Shape::RoundedRectangle | Shape::RoundSquare => Some(props.border_width as f32),
            Shape::Oval | Shape::Circle => Some(f32::INFINITY),
            _ => None,
        },
        ..LayoutStyle::default()
    }
}
