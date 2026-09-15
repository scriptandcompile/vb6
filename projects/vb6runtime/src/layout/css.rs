//! Converts a `LayoutStyle` into a CSS declaration string.
//!
//! Used by both `WebSysRenderer` and `TauriRenderer` to produce
//! the `style` attribute for HTML elements.
//!
//! All `Some` fields in [`LayoutStyle`] produce CSS declarations; `None` fields are omitted.
//! Font sizes are suffixed with `px`. Color values are formatted as `rgb(r, g, b)` or named.

use super::model::style::LayoutStyle;

/// Build a CSS style declaration string from a [`LayoutStyle`].
///
/// Produces a string like `"font-family: Arial; font-size: 12px; color: rgb(255, 0, 0);"`.
/// Fields that are `None` are omitted. Font sizes are automatically suffixed with `px`.
/// Empty style produces an empty string.
pub fn style_to_css(style: &LayoutStyle) -> String {
    let mut parts: Vec<String> = Vec::new();

    if let Some(ref v) = style.background_color {
        parts.push(format!("background-color: {}", v.to_css_string()));
    }
    if let Some(ref v) = style.color {
        parts.push(format!("color: {}", v.to_css_string()));
    }
    if let Some(ref v) = style.font_family {
        parts.push(format!("font-family: {}", escape_font_family(v)));
    }
    if let Some(v) = style.font_size {
        parts.push(format!("font-size: {:.1}px", v));
    }
    if let Some(ref v) = style.font_weight {
        parts.push(format!("font-weight: {}", v));
    }
    if let Some(ref v) = style.font_style {
        parts.push(format!("font-style: {}", v));
    }
    if let Some(ref v) = style.text_decoration {
        parts.push(format!("text-decoration: {}", v));
    }
    if let Some(ref v) = style.display {
        parts.push(format!("display: {}", v));
    }
    if let Some(ref v) = style.overflow {
        parts.push(format!("overflow: {}", v));
    }
    if let Some(ref v) = style.white_space {
        parts.push(format!("white-space: {}", v));
    }
    if let Some(ref v) = style.text_align {
        parts.push(format!("text-align: {}", v));
    }
    if let Some(ref v) = style.vertical_align {
        parts.push(format!("vertical-align: {}", v));
    }
    if let Some(ref v) = style.border {
        parts.push(format!("border: {}", v));
    }
    if let Some(ref v) = style.border_style {
        parts.push(format!("border-style: {}", v));
    }
    if let Some(ref v) = style.align {
        parts.push(format!("align: {}", v));
    }
    if let Some(v) = style.border_radius {
        parts.push(format!("border-radius: {:.1}px", v));
    }
    if let Some(v) = style.border_width {
        parts.push(format!("border-width: {:.1}px", v));
    }
    if let Some(ref v) = style.fill_color {
        parts.push(format!("fill-color: {}", v.to_css_string()));
    }
    if let Some(ref v) = style.line_color {
        parts.push(format!("line-color: {}", v.to_css_string()));
    }
    if let Some(v) = style.line_width {
        parts.push(format!("line-width: {:.1}px", v));
    }
    if let Some(v) = style.line_x1 {
        parts.push(format!("line-x1: {:.1}px", v));
    }
    if let Some(v) = style.line_y1 {
        parts.push(format!("line-y1: {:.1}px", v));
    }
    if let Some(v) = style.line_x2 {
        parts.push(format!("line-x2: {:.1}px", v));
    }
    if let Some(v) = style.line_y2 {
        parts.push(format!("line-y2: {:.1}px", v));
    }
    if let Some(ref v) = style.cursor {
        parts.push(format!("cursor: {}", v));
    }

    parts.join("; ")
}

/// Escape a font family name for use in CSS by wrapping it in quotes if it
/// contains special characters (spaces, commas, etc.).
fn escape_font_family(name: &str) -> String {
    if name.contains(|c: char| c.is_whitespace() || c == ',' || c == '"') {
        format!("\"{}\"", name.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        name.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::model::style::CssColor;

    #[test]
    fn empty_style() {
        let style = LayoutStyle::default();
        let css = style_to_css(&style);
        assert_eq!(css, "");
    }

    #[test]
    fn style_with_fonts() {
        let style = LayoutStyle {
            font_family: Some("Arial".into()),
            font_size: Some(12.0),
            ..LayoutStyle::default()
        };
        let css = style_to_css(&style);
        assert!(css.contains("font-family: Arial"));
        assert!(css.contains("font-size: 12.0px"));
    }

    #[test]
    fn style_with_colors() {
        let style = LayoutStyle {
            background_color: Some(CssColor::Rgb(255, 0, 0)),
            color: Some(CssColor::Rgb(0, 0, 0)),
            ..LayoutStyle::default()
        };
        let css = style_to_css(&style);
        assert!(css.contains("background-color: rgb(255, 0, 0)"));
        assert!(css.contains("color: rgb(0, 0, 0)"));
    }

    #[test]
    fn style_with_border() {
        let style = LayoutStyle {
            border: Some("1px solid rgb(0, 0, 0)".into()),
            ..LayoutStyle::default()
        };
        let css = style_to_css(&style);
        assert!(css.contains("border: 1px solid rgb(0, 0, 0)"));
    }

    #[test]
    fn style_css_properties_order() {
        let style = LayoutStyle::default();
        let css = style_to_css(&style);
        assert!(css.trim().is_empty());
    }

    #[test]
    fn style_with_multiple_properties() {
        let style = LayoutStyle {
            font_family: Some("MS Sans Serif".into()),
            font_size: Some(11.0),
            font_weight: Some("bold".into()),
            font_style: Some("italic".into()),
            color: Some(CssColor::Rgb(0, 0, 0)),
            background_color: Some(CssColor::Named("Window".into())),
            text_align: Some("center".into()),
            border: Some("1px solid rgb(120, 120, 120)".into()),
            overflow: Some("hidden".into()),
            white_space: Some("pre-wrap".into()),
            ..LayoutStyle::default()
        };

        let css = style_to_css(&style);
        assert!(css.contains("font-family: \"MS Sans Serif\""));
        assert!(css.contains("font-size: 11.0px"));
        assert!(css.contains("font-weight: bold"));
        assert!(css.contains("font-style: italic"));
        assert!(css.contains("color: rgb(0, 0, 0)"));
        assert!(css.contains("background-color: Window"));
        assert!(css.contains("text-align: center"));
        assert!(css.contains("border: 1px solid rgb(120, 120, 120)"));
        assert!(css.contains("overflow: hidden"));
        assert!(css.contains("white-space: pre-wrap"));
    }

    #[test]
    fn style_with_shape_properties() {
        let style = LayoutStyle {
            border_radius: Some(8.0),
            border_width: Some(2.0),
            fill_color: Some(CssColor::Rgb(255, 255, 0)),
            ..LayoutStyle::default()
        };
        let css = style_to_css(&style);
        assert!(css.contains("border-radius: 8.0px"));
        assert!(css.contains("border-width: 2.0px"));
        assert!(css.contains("fill-color: rgb(255, 255, 0)"));
    }

    #[test]
    fn style_with_line_properties() {
        let style = LayoutStyle {
            line_color: Some(CssColor::Rgb(255, 0, 0)),
            line_width: Some(1.5),
            line_x1: Some(0.0),
            line_y1: Some(0.0),
            line_x2: Some(100.0),
            line_y2: Some(50.0),
            ..LayoutStyle::default()
        };
        let css = style_to_css(&style);
        assert!(css.contains("line-color: rgb(255, 0, 0)"));
        assert!(css.contains("line-width: 1.5px"));
        assert!(css.contains("line-x1: 0.0px"));
        assert!(css.contains("line-y1: 0.0px"));
        assert!(css.contains("line-x2: 100.0px"));
        assert!(css.contains("line-y2: 50.0px"));
    }

    #[test]
    fn style_with_none_fields_omitted() {
        let style = LayoutStyle {
            font_family: Some("Arial".into()),
            ..LayoutStyle::default()
        };
        // All other fields remain None
        let css = style_to_css(&style);
        assert!(css.contains("font-family: Arial"));
        assert!(!css.contains("font-size"));
        assert!(!css.contains("font-weight"));
        assert!(!css.contains("color"));
        assert!(!css.contains("background-color"));
    }

    #[test]
    fn font_family_escaping() {
        assert_eq!(escape_font_family("Arial"), "Arial");
        assert_eq!(escape_font_family("MS Sans Serif"), "\"MS Sans Serif\"");
        assert_eq!(escape_font_family("Times, New Roman"), "\"Times, New Roman\"");
    }
}
