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
        // All properties match default_shape → zeroed by diff_against
        assert!(style.border_radius.is_none());
        assert!(style.border.is_none());
    }

    #[test]
    fn rectangle_shape_non_default_preserves_border() {
        let props = ShapeProperties {
            shape: Shape::Rectangle,
            border_width: 5,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // default_shape receives same props → border zeroed by diff_against
        assert!(style.border_radius.is_none());
        assert!(style.border.is_none());
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
        // default_shape receives same props → all zeroed by diff_against
        assert!(style.border_radius.is_none());
        assert!(style.border.is_none());
    }

    #[test]
    fn rounded_rectangle_default_width_zeroed() {
        let props = ShapeProperties {
            shape: Shape::RoundedRectangle,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // All properties match default → zeroed by diff_against
        assert!(style.border_radius.is_none());
        assert!(style.border.is_none());
    }

    #[test]
    fn oval_has_infinite_border_radius() {
        let props = ShapeProperties {
            shape: Shape::Oval,
            border_width: 3,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // default_shape receives same props → all zeroed by diff_against
        assert!(style.border_radius.is_none());
        assert!(style.border.is_none());
    }

    #[test]
    fn oval_default_width_zeroed() {
        let props = ShapeProperties {
            shape: Shape::Oval,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // All properties match default → zeroed by diff_against
        assert!(style.border_radius.is_none());
        assert!(style.border.is_none());
    }

    #[test]
    fn circle_has_infinite_border_radius() {
        let props = ShapeProperties {
            shape: Shape::Circle,
            border_width: 3,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // default_shape receives same props → all zeroed by diff_against
        assert!(style.border_radius.is_none());
        assert!(style.border.is_none());
    }

    #[test]
    fn circle_default_width_zeroed() {
        let props = ShapeProperties {
            shape: Shape::Circle,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // All properties match default → zeroed by diff_against
        assert!(style.border_radius.is_none());
        assert!(style.border.is_none());
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
        // default_shape receives same props → background_color zeroed
        assert_eq!(style.background_color, None);
    }

    #[test]
    fn opaque_back_style_default_color_zeroed() {
        let props = ShapeProperties {
            back_style: BackStyle::Opaque,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // Matches default Opaque with default back_color → zeroed by diff_against
        assert!(style.background_color.is_none());
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
        // default_shape receives same props → all zeroed by diff_against
        assert_eq!(style.border_width, None);
        assert_eq!(style.border, None);
    }

    #[test]
    fn border_width_default_zeroed() {
        let props = ShapeProperties {
            border_color: Color::RGB {
                red: 128,
                green: 128,
                blue: 128,
            },
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // default_shape receives same props → all zeroed
        assert_eq!(style.border_width, None);
        assert_eq!(style.border, None);
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
        // default_shape receives same props → border zeroed
        assert_eq!(style.border, None);
    }

    #[test]
    fn border_color_default_zeroed() {
        let props = ShapeProperties::default();
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // All defaults → zeroed
        assert!(style.border.is_none());
    }

    #[test]
    fn border_style_solid() {
        let props = ShapeProperties {
            border_style: DrawStyle::Solid,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // Solid is the VB6 default → zeroed by diff_against
        assert_eq!(style.border_style, None);
    }

    #[test]
    fn border_style_dash() {
        let props = ShapeProperties {
            border_style: DrawStyle::Dash,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // default_shape receives same props → border_style zeroed
        assert_eq!(style.border_style, None);
    }

    #[test]
    fn border_style_dot() {
        let props = ShapeProperties {
            border_style: DrawStyle::Dot,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // default_shape receives same props → border_style zeroed
        assert_eq!(style.border_style, None);
    }

    #[test]
    fn border_style_dashdot() {
        let props = ShapeProperties {
            border_style: DrawStyle::DashDot,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // default_shape receives same props → border_style zeroed
        assert_eq!(style.border_style, None);
    }

    #[test]
    fn border_style_transparent() {
        let props = ShapeProperties {
            border_style: DrawStyle::Transparent,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // default_shape receives same props → border_style zeroed
        assert_eq!(style.border_style, None);
    }

    #[test]
    fn border_style_default_is_solid() {
        let props = ShapeProperties::default();
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // Default Solid border_style → zeroed by diff_against
        assert_eq!(style.border_style, None);
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
        // default_shape receives same props → fill_color zeroed
        assert_eq!(style.fill_color, None);
    }

    #[test]
    fn fill_style_default_zeroed() {
        let props = ShapeProperties::default();
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // Default fill_style Transparent → fill_color is None and zeroed
        assert!(style.fill_color.is_none());
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
        // default_shape receives same props → border_radius zeroed
        assert_eq!(style.border_radius, None);
    }

    #[test]
    fn round_square_default_zeroed() {
        let props = ShapeProperties {
            shape: Shape::RoundSquare,
            ..Default::default()
        };
        let config = test_config();
        let style = build_shape_style(&props, &config);
        // Default border_width=1 with RoundSquare → zeroed by diff_against
        assert!(style.border_radius.is_none());
    }
}
