//! Style builder for HScrollBar and VScrollBar controls.
//!
//! Maps ScrollBar properties to [`LayoutStyle`].
//! Note: ScrollBars are primarily rendered via native `<input type="range">` elements,
//! so the style builder provides minimal styling.

use super::super::LayoutConfig;
use super::super::color::mouse_pointer_css;
use super::super::model::style::LayoutStyle;
use vb6parse::language::ScrollBarProperties;

/// Build CSS style for a ScrollBar control.
pub fn build_scrollbar_style(props: &ScrollBarProperties, config: &LayoutConfig) -> LayoutStyle {
    let style = LayoutStyle {
        cursor: mouse_pointer_css(props.mouse_pointer),
        ..LayoutStyle::default()
    };
    let _ = config;
    style
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
        let props = ScrollBarProperties::default();
        let config = test_config();
        let style = build_scrollbar_style(&props, &config);
        assert_eq!(style, LayoutStyle::default());
    }

    #[test]
    fn ignores_properties() {
        let props = ScrollBarProperties::default();
        let config = test_config();
        let style = build_scrollbar_style(&props, &config);
        assert!(style.background_color.is_none());
        assert!(style.border.is_none());
    }

    #[test]
    fn ignores_config() {
        let props = ScrollBarProperties::default();
        let config = LayoutConfig {
            dpi: 192,
            ..Default::default()
        };
        let style = build_scrollbar_style(&props, &config);
        assert_eq!(style, LayoutStyle::default());
    }
}
