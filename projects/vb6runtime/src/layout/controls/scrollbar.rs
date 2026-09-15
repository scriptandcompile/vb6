//! Style builder for HScrollBar and VScrollBar controls.
//!
//! Maps ScrollBar properties to [`LayoutStyle`].
//! Note: ScrollBars are primarily rendered via native `<input type="range">` elements,
//! so the style builder provides minimal styling.

use super::super::LayoutConfig;
use super::super::model::style::LayoutStyle;
use vb6parse::language::ScrollBarProperties;

/// Build CSS style for a ScrollBar control.
pub fn build_scrollbar_style(_props: &ScrollBarProperties, config: &LayoutConfig) -> LayoutStyle {
    let _ = config;
    LayoutStyle::default()
}
