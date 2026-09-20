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
use vb6parse::language::{BackStyle, DrawStyle, Shape, ShapeProperties};

/// Build CSS style for a Shape control.
pub fn build_shape_style(props: &ShapeProperties, _config: &LayoutConfig) -> LayoutStyle {
    let mut style = LayoutStyle {
        background_color: match props.back_style {
            BackStyle::Transparent => None,
            BackStyle::Opaque => Some(color_to_css(&props.back_color)),
        },
        border: Some(format!(
            "{}px solid {}",
            props.border_width,
            color_to_css(&props.border_color).to_css_string()
        )),
        border_style: draw_style_to_css_border_style(props.border_style),
        fill_color: match props.fill_style {
            DrawStyle::Transparent => None,
            _ => Some(color_to_css(&props.fill_color)),
        },
        border_radius: match props.shape {
            Shape::RoundedRectangle | Shape::RoundSquare => Some(props.border_width as f32),
            Shape::Oval | Shape::Circle => Some(f32::INFINITY),
            _ => None,
        },
        ..LayoutStyle::default()
    };
    style.border_width = Some(props.border_width as f32);

    // Diff against VB6 defaults — set matching fields to None
    let defaults = LayoutStyle::default_shape(props, _config);
    style.diff_against(&defaults);

    style
}

/// Map VB6 DrawStyle to CSS border-style value.
fn draw_style_to_css_border_style(style: DrawStyle) -> Option<String> {
    match style {
        DrawStyle::Transparent | DrawStyle::InsideSolid => Some("none"),
        DrawStyle::Solid => Some("solid"),
        DrawStyle::Dash => Some("dashed"),
        DrawStyle::DashDot | DrawStyle::DashDotDot => Some("dashed"),
        DrawStyle::Dot => Some("dotted"),
    }
    .map(String::from)
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
    fn border_width_applied() {
        let props = ShapeProperties {
            border_width: 3,
            border_color: Color::RGB {
                red: 128,
                green: 128,
                blue: 128,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(
            style.border.as_deref(),
            Some("3px solid rgb(128, 128, 128)")
        );
        assert_eq!(style.border_width, Some(3.0));
    }

    #[test]
    fn border_color_applied() {
        let props = ShapeProperties {
            border_color: Color::RGB {
                red: 255,
                green: 0,
                blue: 0,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.border.as_deref(), Some("1px solid rgb(255, 0, 0)"));
    }

    #[test]
    fn border_style_solid() {
        let props = ShapeProperties {
            border_style: DrawStyle::Solid,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.border_style.as_deref(), Some("solid"));
    }

    #[test]
    fn border_style_dash() {
        let props = ShapeProperties {
            border_style: DrawStyle::Dash,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.border_style.as_deref(), Some("dashed"));
    }

    #[test]
    fn border_style_dot() {
        let props = ShapeProperties {
            border_style: DrawStyle::Dot,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.border_style.as_deref(), Some("dotted"));
    }

    #[test]
    fn border_style_dashdot() {
        let props = ShapeProperties {
            border_style: DrawStyle::DashDot,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.border_style.as_deref(), Some("dashed"));
    }

    #[test]
    fn border_style_transparent() {
        let props = ShapeProperties {
            border_style: DrawStyle::Transparent,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.border_style.as_deref(), Some("none"));
    }

    #[test]
    fn fill_style_transparent_clears_fill_color() {
        let props = ShapeProperties {
            fill_color: Color::RGB {
                red: 255,
                green: 0,
                blue: 0,
            },
            fill_style: DrawStyle::Transparent,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.fill_color, None);
    }

    #[test]
    fn fill_style_opaque_uses_fill_color() {
        let props = ShapeProperties {
            fill_color: Color::RGB {
                red: 0,
                green: 255,
                blue: 0,
            },
            fill_style: DrawStyle::Solid,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        assert_eq!(style.fill_color, Some(CssColor::Rgb(0, 255, 0)));
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
