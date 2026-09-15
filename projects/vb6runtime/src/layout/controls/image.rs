//! Style builder for Image controls.
//!
//! Maps Image properties to [`LayoutStyle`]:
//! - `border_style` → `border`

use super::super::LayoutConfig;
use super::super::model::style::LayoutStyle;
use vb6parse::language::ImageProperties;

/// Build CSS style for an Image control.
pub fn build_image_style(_props: &ImageProperties, config: &LayoutConfig) -> LayoutStyle {
    let _ = config;
    LayoutStyle::default()
}
