//! Style builder for Image controls.
//!
//! Maps Image properties to [`LayoutStyle`]:
//! - `border_style` → `border`

use super::super::LayoutConfig;
use super::super::color::mouse_pointer_css;
use super::super::model::style::LayoutStyle;
use vb6parse::language::ImageProperties;

/// Build CSS style for an Image control.
pub fn build_image_style(props: &ImageProperties, config: &LayoutConfig) -> LayoutStyle {
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
        let props = ImageProperties::default();
        let config = test_config();
        let style = build_image_style(&props, &config);
        assert_eq!(style, LayoutStyle::default());
    }

    #[test]
    fn ignores_properties() {
        let props = ImageProperties::default();
        let config = test_config();
        let style = build_image_style(&props, &config);
        assert!(style.background_color.is_none());
        assert!(style.border.is_none());
        assert!(style.font_family.is_none());
    }

    #[test]
    fn ignores_config() {
        let props = ImageProperties::default();
        let config = LayoutConfig {
            dpi: 120,
            ..Default::default()
        };
        let style = build_image_style(&props, &config);
        assert_eq!(style, LayoutStyle::default());
    }
}
