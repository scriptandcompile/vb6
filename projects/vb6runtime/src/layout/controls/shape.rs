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

#[cfg(test)]
mod tests {
    use super::super::super::model::style::CssColor;
    use super::*;
    use vb6parse::language::Color;

    fn test_config() -> LayoutConfig {
        LayoutConfig {
            dpi: 96,
            ..Default::default()
        }
    }

    #[test]
    fn rectangle_shape() {
        let props = ShapeProperties {
            shape: Shape::Rectangle,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert!(style.border_radius.is_none());
        assert!(style.border.is_some());
    }

    #[test]
    fn rounded_rectangle_has_border_radius() {
        let props = ShapeProperties {
            shape: Shape::RoundedRectangle,
            border_width: 5,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.border_radius, Some(5.0));
    }

    #[test]
    fn oval_has_infinite_border_radius() {
        let props = ShapeProperties {
            shape: Shape::Oval,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.border_radius, Some(f32::INFINITY));
    }

    #[test]
    fn circle_has_infinite_border_radius() {
        let props = ShapeProperties {
            shape: Shape::Circle,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.border_radius, Some(f32::INFINITY));
    }

    #[test]
    fn transparent_back_style() {
        let props = ShapeProperties {
            back_style: BackStyle::Transparent,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.background_color, None);
    }

    #[test]
    fn opaque_back_style() {
        let props = ShapeProperties {
            back_style: BackStyle::Opaque,
            back_color: Color::RGB {
                red: 128,
                green: 128,
                blue: 128,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.background_color, Some(CssColor::Rgb(128, 128, 128)));
    }

    #[test]
    fn fill_color_set() {
        let props = ShapeProperties {
            fill_color: Color::RGB {
                red: 255,
                green: 0,
                blue: 0,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.fill_color, Some(CssColor::Rgb(255, 0, 0)));
    }

    #[test]
    fn border_width_applied() {
        let props = ShapeProperties {
            border_width: 3,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.border, Some("3px solid".to_string()));
    }

    #[test]
    fn round_square_has_border_radius() {
        let props = ShapeProperties {
            shape: Shape::RoundSquare,
            border_width: 10,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.border_radius, Some(10.0));
    }
}
