//! `LayoutStyle` struct — CSS-compatible style properties.
//!
//! This struct is immutable once computed during the converter pass.
//! Mutable state (visibility, value, enabled) lives in `LayoutNode` / `LayoutLeaf`.

/// A CSS color value.
///
/// Supports both RGB triples and named CSS colors (e.g. "Window", "WindowText").
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CssColor {
    /// An explicit RGB color: `rgb(r, g, b)`.
    Rgb(u8, u8, u8),
    /// A named CSS color (e.g. "Window", "WindowText").
    Named(String),
}

impl CssColor {
    /// Returns the color formatted as a CSS `rgb(r, g, b)` or named value.
    #[must_use]
    pub fn to_css_string(&self) -> String {
        match self {
            Self::Rgb(r, g, b) => format!("rgb({r}, {g}, {b})"),
            Self::Named(name) => name.clone(),
        }
    }
}

impl Default for CssColor {
    fn default() -> Self {
        Self::Rgb(0, 0, 0)
    }
}

/// CSS-compatible style properties derived from control-specific VB6 properties.
///
/// This struct is immutable once computed during the converter pass.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutStyle {
    /// Background color of the control.
    pub background_color: Option<CssColor>,
    /// Foreground / text color of the control.
    pub color: Option<CssColor>,

    /// Font family name (e.g. "MS Sans Serif", "Arial").
    pub font_family: Option<String>,
    /// Font size in pixels.
    pub font_size: Option<f32>,
    /// Font weight ('normal', 'bold', or numeric '400', '700').
    pub font_weight: Option<String>,
    /// Font style ('normal' or 'italic').
    pub font_style: Option<String>,
    /// Text decoration ('underline', 'none', etc.).
    pub text_decoration: Option<String>,

    /// CSS display property ('block', 'inline-block', 'flex').
    pub display: Option<String>,
    /// CSS overflow property ('visible', 'hidden', 'auto').
    pub overflow: Option<String>,
    /// CSS white-space property ('nowrap', 'normal', 'pre-wrap').
    pub white_space: Option<String>,
    /// CSS text-align property ('left', 'center', 'right', 'justify').
    pub text_align: Option<String>,
    /// CSS vertical-align property.
    pub vertical_align: Option<String>,

    /// CSS border shorthand (e.g. "1px solid rgb(0, 0, 0)").
    pub border: Option<String>,
    /// CSS border-style property ('solid', 'dashed', 'none').
    pub border_style: Option<String>,
    /// CSS box-shadow shorthand (e.g. raised 3D bevel for buttons).
    pub box_shadow: Option<String>,
    /// Whether a control renders as a multi-line editor (e.g. `<textarea>`).
    pub multi_line: bool,
    /// Radio-group name used to group OptionButton controls within a container.
    pub group: Option<String>,

    /// Alignment for controls with the VB6 Align property ('top', 'bottom', 'left', 'right', 'none').
    pub align: Option<String>,

    /// Border radius in pixels, for rounded shapes.
    pub border_radius: Option<f32>,
    /// Border width in pixels.
    pub border_width: Option<f32>,
    /// Fill color for shape controls.
    pub fill_color: Option<CssColor>,

    /// Line color for the Line control.
    pub line_color: Option<CssColor>,
    /// Line width in pixels for the Line control.
    pub line_width: Option<f32>,
    /// X1 coordinate for the Line control (in pixels).
    pub line_x1: Option<f32>,
    /// Y1 coordinate for the Line control (in pixels).
    pub line_y1: Option<f32>,
    /// X2 coordinate for the Line control (in pixels).
    pub line_x2: Option<f32>,
    /// Y2 coordinate for the Line control (in pixels).
    pub line_y2: Option<f32>,

    /// CSS cursor property ('default', 'pointer', 'wait').
    pub cursor: Option<String>,
    /// CSS direction property ('rtl' for right-to-left).
    pub direction: Option<String>,

    /// CSS object-fit property for Image controls ('fill', 'contain', 'cover', 'none', 'scale-down').
    pub object_fit: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_default_all_none() {
        let s = LayoutStyle::default();
        assert!(s.background_color.is_none());
        assert!(s.color.is_none());
        assert!(s.font_family.is_none());
        assert!(s.font_size.is_none());
        assert!(s.font_weight.is_none());
        assert!(s.font_style.is_none());
        assert!(s.text_decoration.is_none());
        assert!(s.display.is_none());
        assert!(s.overflow.is_none());
        assert!(s.white_space.is_none());
        assert!(s.text_align.is_none());
        assert!(s.vertical_align.is_none());
        assert!(s.border.is_none());
        assert!(s.border_style.is_none());
        assert!(s.box_shadow.is_none());
        assert!(!s.multi_line);
        assert!(s.group.is_none());
        assert!(s.align.is_none());
        assert!(s.border_radius.is_none());
        assert!(s.border_width.is_none());
        assert!(s.fill_color.is_none());
        assert!(s.line_color.is_none());
        assert!(s.line_width.is_none());
        assert!(s.line_x1.is_none());
        assert!(s.line_y1.is_none());
        assert!(s.line_x2.is_none());
        assert!(s.line_y2.is_none());
        assert!(s.cursor.is_none());
        assert!(s.direction.is_none());
        assert!(s.object_fit.is_none());
    }

    #[test]
    fn style_builder_pattern() {
        let s = LayoutStyle {
            font_family: Some("Arial".into()),
            font_size: Some(12.0),
            color: Some(CssColor::Rgb(255, 0, 0)),
            ..LayoutStyle::default()
        };
        assert_eq!(s.font_family.as_deref(), Some("Arial"));
    }

    #[test]
    fn css_color_rgb_to_string() {
        let c = CssColor::Rgb(255, 128, 0);
        assert_eq!(c.to_css_string(), "rgb(255, 128, 0)");
    }

    #[test]
    fn css_color_named_to_string() {
        let c = CssColor::Named("Window".into());
        assert_eq!(c.to_css_string(), "Window");
    }
}
