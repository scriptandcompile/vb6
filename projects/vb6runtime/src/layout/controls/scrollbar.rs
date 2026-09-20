//! Style builder for HScrollBar and VScrollBar controls.
//!
//! Maps ScrollBar properties to [`LayoutStyle`].
//! Note: ScrollBars are primarily rendered via native `<input type="range">` elements,
//! so the style builder provides minimal styling.

use super::super::LayoutConfig;
use super::super::color::mouse_pointer_css;
use super::super::model::style::LayoutStyle;
use vb6parse::language::{ScrollBarProperties, TextDirection};

/// Build CSS style for a ScrollBar control.
pub fn build_scrollbar_style(props: &ScrollBarProperties, config: &LayoutConfig) -> LayoutStyle {
    let mut style = LayoutStyle {
        cursor: mouse_pointer_css(props.mouse_pointer),
        direction: if matches!(props.right_to_left, TextDirection::RightToLeft) {
            Some("rtl".to_string())
        } else {
            None
        },
        ..LayoutStyle::default()
    };
    style.box_shadow = match props.appearance {
        vb6parse::language::Appearance::ThreeD => Some(
            "inset -1px -1px 0 rgb(128, 128, 128), inset 1px 1px 0 rgb(255, 255, 255)".to_string(),
        ),
        vb6parse::language::Appearance::Flat => None,
    };

    // Diff against VB6 defaults — set matching fields to None
    let defaults = LayoutStyle::default_scrollbar(config);
    style.diff_against(&defaults);

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
        assert_eq!(
            style.box_shadow,
            Some(
                "inset -1px -1px 0 rgb(128, 128, 128), inset 1px 1px 0 rgb(255, 255, 255)"
                    .to_string()
            )
        );
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
        assert!(style.box_shadow.is_some());
    }

    #[test]
    fn three_d_appearance_sets_box_shadow() {
        let props = ScrollBarProperties {
            appearance: vb6parse::language::Appearance::ThreeD,
            ..Default::default()
        };
        let config = test_config();
        let style = build_scrollbar_style(&props, &config);
        assert_eq!(
            style.box_shadow.as_deref(),
            Some("inset -1px -1px 0 rgb(128, 128, 128), inset 1px 1px 0 rgb(255, 255, 255)")
        );
    }

    #[test]
    fn flat_appearance_clears_box_shadow() {
        let props = ScrollBarProperties {
            appearance: vb6parse::language::Appearance::Flat,
            ..Default::default()
        };
        let config = test_config();
        let style = build_scrollbar_style(&props, &config);
        assert!(style.box_shadow.is_none());
    }
}
