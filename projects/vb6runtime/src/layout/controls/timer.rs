//! Style builder for Timer controls.
//!
//! Timer controls have no visual output, so this returns an empty [`LayoutStyle`].
//! Timers are handled by the converter to skip them from the DOM tree entirely.

use super::super::LayoutConfig;
use super::super::model::style::LayoutStyle;
use vb6parse::language::TimerProperties;

/// Build CSS style for a Timer control.
///
/// Always returns default (empty) style since Timer has no visual output.
pub fn build_timer_style(_props: &TimerProperties, config: &LayoutConfig) -> LayoutStyle {
    let _ = config;
    LayoutStyle::default()
}
