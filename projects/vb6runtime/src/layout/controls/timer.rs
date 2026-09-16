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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> LayoutConfig {
        LayoutConfig {
            dpi: 96,
            ..Default::default()
        }
    }

    #[test]
    fn returns_default_style() {
        let props = TimerProperties::default();
        let config = test_config();
        let style = build_timer_style(&props, &config);
        assert_eq!(style, LayoutStyle::default());
    }

    #[test]
    fn ignores_properties() {
        let props = TimerProperties {
            interval: 5000,
            ..Default::default()
        };
        let config = test_config();
        let style = build_timer_style(&props, &config);
        assert_eq!(style, LayoutStyle::default());
    }

    #[test]
    fn no_visual_output() {
        let props = TimerProperties::default();
        let config = test_config();
        let style = build_timer_style(&props, &config);
        assert!(style.background_color.is_none());
        assert!(style.color.is_none());
        assert!(style.border.is_none());
    }
}
