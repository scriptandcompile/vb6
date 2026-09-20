//! Style builder for Image controls.
//!
//! Maps Image properties to [`LayoutStyle`]:
//! - `border_style` → `border`
//! - `stretch` → `object_fit`

use super::super::LayoutConfig;
use super::super::color::mouse_pointer_css;
use super::super::model::style::LayoutStyle;
use vb6parse::language::ImageProperties;

/// Build CSS style for an Image control.
pub fn build_image_style(props: &ImageProperties, config: &LayoutConfig) -> LayoutStyle {
    let mut style = LayoutStyle {
        cursor: mouse_pointer_css(props.mouse_pointer),
        box_shadow: match props.appearance {
            vb6parse::language::Appearance::ThreeD => Some(
                "inset -1px -1px 0 rgb(128, 128, 128), inset 1px 1px 0 rgb(255, 255, 255)"
                    .to_string(),
            ),
            vb6parse::language::Appearance::Flat => None,
        },
        border: match props.border_style {
            vb6parse::language::BorderStyle::None => Some("none".to_string()),
            vb6parse::language::BorderStyle::FixedSingle => {
                Some("1px solid rgb(120, 120, 120)".to_string())
            }
        },
        object_fit: if props.stretch {
            Some("fill".to_string())
        } else {
            Some("contain".to_string())
        },
        ..LayoutStyle::default()
    };

    // Diff against VB6 defaults — set matching fields to None
    let defaults = LayoutStyle::default_image(config);
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
        let props = ImageProperties::default();
        let config = test_config();
        let style = build_image_style(&props, &config);
        // All values match default → zeroed by diff_against
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn image_threed_appearance() {
        let props = ImageProperties {
            appearance: vb6parse::language::Appearance::ThreeD,
            ..Default::default()
        };
        let config = test_config();
        let style = build_image_style(&props, &config);
        // ThreeD is the VB6 default → box_shadow zeroed by diff_against
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn image_flat_appearance() {
        let props = ImageProperties {
            appearance: vb6parse::language::Appearance::Flat,
            ..Default::default()
        };
        let config = test_config();
        let style = build_image_style(&props, &config);
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn ignores_properties() {
        let props = ImageProperties::default();
        let config = test_config();
        let style = build_image_style(&props, &config);
        assert!(style.background_color.is_none());
        // BorderStyle::None → "none" matches default → zeroed by diff_against
        assert_eq!(style.border, None);
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
        // ThreeD is the VB6 default → box_shadow zeroed by diff_against
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn stretch_sets_object_fit() {
        let props = ImageProperties {
            stretch: true,
            ..Default::default()
        };
        let config = test_config();
        let style = build_image_style(&props, &config);
        assert_eq!(style.object_fit, Some("fill".to_string()));
    }

    #[test]
    fn no_stretch_sets_object_fit_contain() {
        let props = ImageProperties {
            stretch: false,
            ..Default::default()
        };
        let config = test_config();
        let style = build_image_style(&props, &config);
        // contain is the VB6 default → zeroed by diff_against
        assert_eq!(style.object_fit, None);
    }

    #[test]
    fn stretch_sets_object_fit_fill() {
        let props = ImageProperties {
            stretch: true,
            ..Default::default()
        };
        let config = test_config();
        let style = build_image_style(&props, &config);
        // fill differs from default contain → preserved
        assert_eq!(style.object_fit, Some("fill".to_string()));
    }

    #[test]
    fn fixed_single_border() {
        let props = ImageProperties {
            border_style: vb6parse::language::BorderStyle::FixedSingle,
            ..Default::default()
        };
        let config = test_config();
        let style = build_image_style(&props, &config);
        assert_eq!(
            style.border,
            Some("1px solid rgb(120, 120, 120)".to_string())
        );
    }
}
