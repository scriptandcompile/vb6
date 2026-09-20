//! `LayoutStyle` struct — CSS-compatible style properties.
//!
//! This struct is immutable once computed during the converter pass.
//! Mutable state (visibility, value, enabled) lives in `LayoutNode` / `LayoutLeaf`.
//!
//! Also provides CSS utility functions for converting VB6 font/alignment properties
//! to CSS values.

use vb6parse::language::{Alignment, Style, VB_BUTTON_FACE, VB_WINDOW_BACKGROUND, VB_WINDOW_TEXT};

use super::super::LayoutConfig;
use super::super::color::color_to_css;

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

    /// Background image as a base64 data URL (for PictureBox controls).
    pub background_image: Option<String>,
}

/// Convert a VB6 font weight to a CSS font-weight string.
///
/// Maps VB6 font weights (100–900) to their CSS equivalents, with
/// special handling for 400 (→ "normal") and 700 (→ "bold").
/// Returns `None` for unrecognized weights.
///
/// # Examples
///
/// ```
/// use vb6runtime::layout::model::style::font_weight_css;
/// assert_eq!(font_weight_css(700), Some("bold".to_string()));
/// assert_eq!(font_weight_css(400), Some("normal".to_string()));
/// assert_eq!(font_weight_css(500), Some("500".to_string()));
/// assert_eq!(font_weight_css(100), Some("100".to_string()));
/// ```
#[must_use]
pub fn font_weight_css(weight: i32) -> Option<String> {
    match weight {
        100 => Some("100".to_string()),
        200 => Some("200".to_string()),
        300 => Some("300".to_string()),
        400 => Some("normal".to_string()),
        500 => Some("500".to_string()),
        600 => Some("600".to_string()),
        700 => Some("bold".to_string()),
        800 => Some("800".to_string()),
        900 => Some("900".to_string()),
        _ => None,
    }
}

/// Convert an italic boolean to a CSS font-style string.
///
/// # Examples
///
/// ```
/// use vb6runtime::layout::model::style::font_style_css;
/// assert_eq!(font_style_css(true), "italic");
/// assert_eq!(font_style_css(false), "normal");
/// ```
#[must_use]
pub fn font_style_css(italic: bool) -> String {
    if italic {
        "italic".to_string()
    } else {
        "normal".to_string()
    }
}

/// Convert a VB6 [`Alignment`] to a CSS `text-align` string.
///
/// # Examples
///
/// ```
/// use vb6parse::language::Alignment;
/// use vb6runtime::layout::model::style::alignment_css;
/// assert_eq!(alignment_css(Alignment::LeftJustify), Some("left".to_string()));
/// assert_eq!(alignment_css(Alignment::Center), Some("center".to_string()));
/// assert_eq!(alignment_css(Alignment::RightJustify), Some("right".to_string()));
/// ```
#[must_use]
pub fn alignment_css(alignment: Alignment) -> Option<String> {
    match alignment {
        Alignment::RightJustify => Some("right".to_string()),
        Alignment::Center => Some("center".to_string()),
        Alignment::LeftJustify => Some("left".to_string()),
    }
}

/// The standard 3D bevel box-shadow string used across VB6 controls.
const THREE_D_BOX_SHADOW: &str =
    "inset -1px -1px 0 rgb(128, 128, 128), inset 1px 1px 0 rgb(255, 255, 255)";

/// Helper: compute the default border radius for a Shape based on its type.
fn shape_border_radius(props: &vb6parse::language::ShapeProperties) -> Option<f32> {
    match props.shape {
        vb6parse::language::Shape::RoundedRectangle | vb6parse::language::Shape::RoundSquare => {
            Some(props.border_width as f32)
        }
        vb6parse::language::Shape::Oval | vb6parse::language::Shape::Circle => Some(f32::INFINITY),
        _ => None,
    }
}

// ============================================================================
// Default style functions — VB6 system defaults
// ============================================================================
//
// Each function returns the LayoutStyle that a "bare" control (all properties at
// their VB6 system defaults) would produce.  The diff_against() method (Step 2)
// and style builders (Step 3) use these to zero-out matching fields, so that
// controls at VB6 defaults produce zero inline CSS.

impl LayoutStyle {
    /// VB6 system defaults for a TextBox.
    ///
    /// A bare TextBox (all properties at VB6 defaults) produces this exact style.
    /// If `build_textbox_style(props, config) == this`, zero inline CSS is emitted.
    ///
    /// Default properties:
    /// - back_color = vbWindowBackground → Canvas
    /// - fore_color = vbWindowText → ButtonText
    /// - font = None (inherits from form / CSS class)
    /// - border_style = FixedSingle (handled by .vb6-textbox CSS class, so `None`)
    /// - multi_line = SingleLine (so `overflow` is `None`)
    /// - appearance = ThreeD (standard 3D bevel)
    /// - mouse_pointer = Default (cursor is `None`)
    /// - right_to_left = LeftToRight (direction is `None`)
    /// - alignment = LeftJustify → "left"
    pub fn default_textbox(config: &LayoutConfig) -> Self {
        let _ = config;
        LayoutStyle {
            background_color: Some(color_to_css(&VB_WINDOW_BACKGROUND)),
            color: Some(color_to_css(&VB_WINDOW_TEXT)),
            text_align: Some("left".to_string()),
            box_shadow: Some(THREE_D_BOX_SHADOW.to_string()),
            ..LayoutStyle::default()
        }
    }

    /// VB6 system defaults for a CommandButton.
    ///
    /// Default properties:
    /// - style = Standard (background handled by .vb6-commandbutton CSS class)
    /// - appearance = ThreeD (standard 3D bevel)
    /// - font = None (inherits from form / CSS class)
    /// - mouse_pointer = Default (cursor is `None`)
    /// - right_to_left = LeftToRight (direction is `None`)
    pub fn default_button(
        config: &LayoutConfig,
        style: Style,
        appearance: vb6parse::language::Appearance,
    ) -> Self {
        let _ = config;
        let mut result = LayoutStyle {
            display: Some("inline-block".to_string()),
            ..LayoutStyle::default()
        };
        // Graphical buttons use the button's own back_color as default.
        // Standard buttons delegate background to the .vb6-commandbutton CSS class.
        if matches!(style, Style::Graphical) {
            result.background_color = Some(color_to_css(&VB_BUTTON_FACE));
        }
        if matches!(appearance, vb6parse::language::Appearance::ThreeD) {
            result.box_shadow = Some(THREE_D_BOX_SHADOW.to_string());
        }
        result
    }

    /// VB6 system defaults for a Label.
    ///
    /// Default properties:
    /// - back_style = Transparent (background_color is `None`)
    /// - fore_color = vbWindowText → ButtonText
    /// - font = None (inherits from form / CSS class)
    /// - word_wrap = Wrapping → "pre-wrap"
    /// - border_style = FixedSingle (handled by .vb6-label CSS class)
    /// - appearance = ThreeD (standard 3D bevel)
    /// - mouse_pointer = Default (cursor is `None`)
    /// - right_to_left = LeftToRight (direction is `None`)
    /// - alignment = LeftJustify → "left"
    pub fn default_label(config: &LayoutConfig) -> Self {
        let _ = config;
        LayoutStyle {
            color: Some(color_to_css(&VB_WINDOW_TEXT)),
            white_space: Some("pre-wrap".to_string()),
            text_align: Some("left".to_string()),
            box_shadow: Some(THREE_D_BOX_SHADOW.to_string()),
            ..LayoutStyle::default()
        }
    }

    /// VB6 system defaults for a Frame.
    ///
    /// Default properties:
    /// - back_color = vbWindowBackground → Canvas
    /// - fore_color = vbWindowText → ButtonText
    /// - font = None (inherits from form / CSS class)
    /// - border_style = FixedSingle → handled by CSS class
    /// - appearance = ThreeD (standard 3D bevel)
    /// - mouse_pointer = Default (cursor is `None`)
    /// - right_to_left = LeftToRight (direction is `None`)
    /// - clip_controls = Unbounded (overflow is `None`)
    pub fn default_frame(config: &LayoutConfig) -> Self {
        let _ = config;
        LayoutStyle {
            background_color: Some(color_to_css(&VB_WINDOW_BACKGROUND)),
            color: Some(color_to_css(&VB_WINDOW_TEXT)),
            box_shadow: Some(THREE_D_BOX_SHADOW.to_string()),
            ..LayoutStyle::default()
        }
    }

    /// VB6 system defaults for a PictureBox.
    ///
    /// Default properties:
    /// - back_color = vbWindowBackground → Canvas
    /// - fore_color = vbWindowText → ButtonText
    /// - font = None (inherits from form / CSS class)
    /// - border_style = FixedSingle (handled by .vb6-picturebox CSS class)
    /// - appearance = ThreeD (standard 3D bevel)
    /// - mouse_pointer = Default (cursor is `None`)
    /// - right_to_left = LeftToRight (direction is `None`)
    /// - align = None → "none"
    /// - clip_controls = Unbounded (overflow is `None`)
    /// - font_transparent = Opaque (background_color stays set)
    /// - picture = None (no background image)
    pub fn default_picturebox(config: &LayoutConfig) -> Self {
        let _ = config;
        LayoutStyle {
            background_color: Some(color_to_css(&VB_WINDOW_BACKGROUND)),
            color: Some(color_to_css(&VB_WINDOW_TEXT)),
            box_shadow: Some(THREE_D_BOX_SHADOW.to_string()),
            align: Some("none".to_string()),
            ..LayoutStyle::default()
        }
    }

    /// VB6 system defaults for an Image control.
    ///
    /// Default properties:
    /// - stretch = False → object_fit "contain"
    /// - border_style = None → "none"
    /// - appearance = ThreeD (standard 3D bevel)
    /// - mouse_pointer = Default (cursor is `None`)
    pub fn default_image(config: &LayoutConfig) -> Self {
        let _ = config;
        LayoutStyle {
            object_fit: Some("contain".to_string()),
            border: Some("none".to_string()),
            box_shadow: Some(THREE_D_BOX_SHADOW.to_string()),
            ..LayoutStyle::default()
        }
    }

    /// VB6 system defaults for a CheckBox.
    ///
    /// Default properties:
    /// - back_color = vbWindowBackground → Canvas
    /// - fore_color = vbWindowText → ButtonText
    /// - font = None (inherits from form / CSS class)
    /// - appearance = ThreeD (standard 3D bevel)
    /// - mouse_pointer = Default (cursor is `None`)
    /// - right_to_left = LeftToRight (direction is `None`)
    /// - alignment = LeftJustify → "left"
    pub fn default_checkbox(config: &LayoutConfig) -> Self {
        let _ = config;
        LayoutStyle {
            background_color: Some(color_to_css(&VB_WINDOW_BACKGROUND)),
            color: Some(color_to_css(&VB_WINDOW_TEXT)),
            box_shadow: Some(THREE_D_BOX_SHADOW.to_string()),
            text_align: Some("left".to_string()),
            ..LayoutStyle::default()
        }
    }

    /// VB6 system defaults for an OptionButton (radio button).
    ///
    /// Same defaults as CheckBox since the builders are identical.
    pub fn default_optionbutton(config: &LayoutConfig) -> Self {
        Self::default_checkbox(config)
    }

    /// VB6 system defaults for a ComboBox.
    ///
    /// Default properties:
    /// - back_color = vbWindowBackground → Canvas
    /// - fore_color = vbWindowText → ButtonText
    /// - font = None (inherits from form / CSS class)
    /// - appearance = ThreeD (standard 3D bevel)
    /// - mouse_pointer = Default (cursor is `None`)
    /// - right_to_left = LeftToRight (direction is `None`)
    pub fn default_combobox(config: &LayoutConfig) -> Self {
        let _ = config;
        LayoutStyle {
            background_color: Some(color_to_css(&VB_WINDOW_BACKGROUND)),
            color: Some(color_to_css(&VB_WINDOW_TEXT)),
            box_shadow: Some(THREE_D_BOX_SHADOW.to_string()),
            ..LayoutStyle::default()
        }
    }

    /// VB6 system defaults for a ListBox.
    ///
    /// Default properties:
    /// - back_color = vbWindowBackground → Canvas
    /// - fore_color = vbWindowText → ButtonText
    /// - font = None (inherits from form / CSS class)
    /// - appearance = ThreeD (standard 3D bevel)
    /// - mouse_pointer = Default (cursor is `None`)
    /// - right_to_left = LeftToRight (direction is `None`)
    pub fn default_listbox(config: &LayoutConfig) -> Self {
        let _ = config;
        LayoutStyle {
            background_color: Some(color_to_css(&VB_WINDOW_BACKGROUND)),
            color: Some(color_to_css(&VB_WINDOW_TEXT)),
            overflow: Some("auto".to_string()),
            box_shadow: Some(THREE_D_BOX_SHADOW.to_string()),
            ..LayoutStyle::default()
        }
    }

    /// VB6 system defaults for a ScrollBar (HScrollBar / VScrollBar).
    ///
    /// Default properties:
    /// - appearance = ThreeD (standard 3D bevel)
    /// - mouse_pointer = Default (cursor is `None`)
    /// - right_to_left = LeftToRight (direction is `None`)
    pub fn default_scrollbar(config: &LayoutConfig) -> Self {
        let _ = config;
        LayoutStyle {
            box_shadow: Some(THREE_D_BOX_SHADOW.to_string()),
            ..LayoutStyle::default()
        }
    }

    /// VB6 system defaults for a Shape control.
    ///
    /// Default properties:
    /// - shape = Rectangle (border_radius is `None`)
    /// - back_style = Transparent (background_color is `None`)
    /// - border_style = Solid
    /// - border_width = 1
    /// - border_color = vbBlack
    /// - fill_style = Transparent (fill_color is `None`)
    ///
    /// Note: border, border_width, border_style are always set for shapes
    /// and cannot be delegated to CSS classes.
    pub fn default_shape(
        props: &vb6parse::language::ShapeProperties,
        _config: &LayoutConfig,
    ) -> Self {
        let _ = _config;
        LayoutStyle {
            background_color: match props.back_style {
                vb6parse::language::BackStyle::Transparent => None,
                vb6parse::language::BackStyle::Opaque => Some(color_to_css(&props.back_color)),
            },
            border: Some(format!(
                "{}px solid {}",
                props.border_width,
                color_to_css(&props.border_color).to_css_string()
            )),
            border_style: draw_style_to_css_border_style(props.border_style),
            fill_color: match props.fill_style {
                vb6parse::language::DrawStyle::Transparent => None,
                _ => Some(color_to_css(&props.fill_color)),
            },
            border_radius: shape_border_radius(props),
            border_width: Some(props.border_width as f32),
            ..LayoutStyle::default()
        }
    }

    /// VB6 system defaults for a Line control.
    ///
    /// Default properties:
    /// - border_color = vbBlack
    /// - border_width = 0
    /// - coordinates are instance-specific → always `None` (set by renderer)
    pub fn default_line(
        props: &vb6parse::language::LineProperties,
        _config: &LayoutConfig,
    ) -> Self {
        let _ = _config;
        LayoutStyle {
            line_color: Some(color_to_css(&props.border_color)),
            line_width: Some(props.border_width as f32),
            ..LayoutStyle::default()
        }
    }
}

/// Map VB6 DrawStyle to CSS border-style value.
fn draw_style_to_css_border_style(style: vb6parse::language::DrawStyle) -> Option<String> {
    match style {
        vb6parse::language::DrawStyle::Transparent | vb6parse::language::DrawStyle::InsideSolid => {
            Some("none")
        }
        vb6parse::language::DrawStyle::Solid => Some("solid"),
        vb6parse::language::DrawStyle::Dash => Some("dashed"),
        vb6parse::language::DrawStyle::DashDot | vb6parse::language::DrawStyle::DashDotDot => {
            Some("dashed")
        }
        vb6parse::language::DrawStyle::Dot => Some("dotted"),
    }
    .map(String::from)
}

// ============================================================================
// diff_against helpers and method
// ============================================================================
//
// These helpers let style builders zero-out fields that match VB6 system defaults,
// so that controls at defaults produce zero inline CSS.  The existing style_to_css()
// already skips None fields, so this is the only change needed.

/// Set a field to `None` when it matches the default value.
///
/// Generic helper for `Option<T>` fields where `T: PartialEq`.
/// Returns `None` if `value == default`, otherwise returns `value`.
#[must_use]
fn diff_option<T: PartialEq>(value: Option<T>, default: Option<T>) -> Option<T> {
    if value == default { None } else { value }
}

/// Compare `Option<String>` fields and return `None` when they match.
///
/// Used for string fields like `font_family`, `display`, `border`, etc.
#[must_use]
fn diff_option_string(value: Option<String>, default: Option<String>) -> Option<String> {
    match (&value, &default) {
        (Some(v), Some(d)) if v == d => None,
        _ => value,
    }
}

impl LayoutStyle {
    /// Subtract `defaults` from `self`, setting matching fields to `None`.
    ///
    /// This is called after building a style to reduce it to only the properties
    /// that differ from VB6 system defaults.  The result is suitable for
    /// `style_to_css()` which already omits `None` fields.
    ///
    /// # Example
    ///
    /// ```
    /// use vb6runtime::layout::model::style::LayoutStyle;
    /// use vb6runtime::layout::LayoutConfig;
    ///
    /// let config = LayoutConfig::default();
    /// let defaults = LayoutStyle::default_textbox(&config);
    /// let mut style = defaults.clone();
    /// style.diff_against(&defaults);
    /// // Every field is now None (or false for bools), so style_to_css(&style) == ""
    /// ```
    pub fn diff_against(&mut self, defaults: &Self) {
        self.background_color = diff_option(
            self.background_color.clone(),
            defaults.background_color.clone(),
        );
        self.color = diff_option(self.color.clone(), defaults.color.clone());
        self.font_family =
            diff_option_string(self.font_family.clone(), defaults.font_family.clone());
        self.font_size = diff_option(self.font_size, defaults.font_size);
        self.font_weight =
            diff_option_string(self.font_weight.clone(), defaults.font_weight.clone());
        self.font_style = diff_option_string(self.font_style.clone(), defaults.font_style.clone());
        self.text_decoration = diff_option_string(
            self.text_decoration.clone(),
            defaults.text_decoration.clone(),
        );
        self.display = diff_option_string(self.display.clone(), defaults.display.clone());
        self.overflow = diff_option_string(self.overflow.clone(), defaults.overflow.clone());
        self.white_space =
            diff_option_string(self.white_space.clone(), defaults.white_space.clone());
        self.text_align = diff_option_string(self.text_align.clone(), defaults.text_align.clone());
        self.vertical_align =
            diff_option_string(self.vertical_align.clone(), defaults.vertical_align.clone());
        self.border = diff_option_string(self.border.clone(), defaults.border.clone());
        self.border_style =
            diff_option_string(self.border_style.clone(), defaults.border_style.clone());
        self.box_shadow = diff_option_string(self.box_shadow.clone(), defaults.box_shadow.clone());
        self.multi_line = self.multi_line && !defaults.multi_line;
        self.group = diff_option_string(self.group.clone(), defaults.group.clone());
        self.align = diff_option_string(self.align.clone(), defaults.align.clone());
        self.border_radius = diff_option(self.border_radius, defaults.border_radius);
        self.border_width = diff_option(self.border_width, defaults.border_width);
        self.fill_color = diff_option(self.fill_color.clone(), defaults.fill_color.clone());
        self.line_color = diff_option(self.line_color.clone(), defaults.line_color.clone());
        self.line_width = diff_option(self.line_width, defaults.line_width);
        self.line_x1 = diff_option(self.line_x1, defaults.line_x1);
        self.line_y1 = diff_option(self.line_y1, defaults.line_y1);
        self.line_x2 = diff_option(self.line_x2, defaults.line_x2);
        self.line_y2 = diff_option(self.line_y2, defaults.line_y2);
        self.cursor = diff_option_string(self.cursor.clone(), defaults.cursor.clone());
        self.direction = diff_option_string(self.direction.clone(), defaults.direction.clone());
        self.object_fit = diff_option_string(self.object_fit.clone(), defaults.object_fit.clone());
        self.background_image = diff_option_string(
            self.background_image.clone(),
            defaults.background_image.clone(),
        );
    }
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

    // -----------------------------------------------------------------------
    // Default style functions — Step 1
    // -----------------------------------------------------------------------

    #[test]
    fn default_textbox_has_expected_colors() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_textbox(&config);
        assert!(defaults.background_color.is_some());
        assert!(defaults.color.is_some());
        // font should NOT be set — inherited from form/CSS class
        assert!(defaults.font_family.is_none());
        assert!(defaults.font_size.is_none());
        // 3D bevel is the default appearance
        assert!(defaults.box_shadow.is_some());
        assert_eq!(defaults.box_shadow.as_deref(), Some(THREE_D_BOX_SHADOW));
        // alignment is left by default
        assert_eq!(defaults.text_align, Some("left".to_string()));
    }

    #[test]
    fn default_button_standard_no_background() {
        // Standard buttons delegate background to the .vb6-commandbutton CSS class
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_button(
            &config,
            Style::Standard,
            vb6parse::language::Appearance::ThreeD,
        );
        assert!(defaults.background_color.is_none());
        assert_eq!(defaults.display, Some("inline-block".to_string()));
        assert!(defaults.box_shadow.is_some());
    }

    #[test]
    fn default_button_graphical_has_button_face() {
        // Graphical buttons use their own back_color as default
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_button(
            &config,
            Style::Graphical,
            vb6parse::language::Appearance::ThreeD,
        );
        assert_eq!(
            defaults.background_color,
            Some(color_to_css(&VB_BUTTON_FACE))
        );
    }

    #[test]
    fn default_button_flat_no_box_shadow() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_button(
            &config,
            Style::Standard,
            vb6parse::language::Appearance::Flat,
        );
        assert!(defaults.box_shadow.is_none());
    }

    #[test]
    fn default_label_no_background() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_label(&config);
        assert!(defaults.background_color.is_none()); // Transparent by default
        assert!(defaults.color.is_some());
        // word wrap → pre-wrap by default
        assert_eq!(defaults.white_space, Some("pre-wrap".to_string()));
        assert!(defaults.box_shadow.is_some());
    }

    #[test]
    fn default_frame_has_window_colors() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_frame(&config);
        assert_eq!(
            defaults.background_color,
            Some(color_to_css(&VB_WINDOW_BACKGROUND))
        );
        assert_eq!(defaults.color, Some(color_to_css(&VB_WINDOW_TEXT)));
        assert!(defaults.box_shadow.is_some());
    }

    #[test]
    fn default_picturebox_has_window_colors() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_picturebox(&config);
        assert_eq!(
            defaults.background_color,
            Some(color_to_css(&VB_WINDOW_BACKGROUND))
        );
        assert_eq!(defaults.color, Some(color_to_css(&VB_WINDOW_TEXT)));
        assert_eq!(defaults.align, Some("none".to_string()));
        assert!(defaults.box_shadow.is_some());
    }

    #[test]
    fn default_image_has_contain() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_image(&config);
        assert_eq!(defaults.object_fit, Some("contain".to_string()));
        assert_eq!(defaults.border, Some("none".to_string()));
        assert!(defaults.box_shadow.is_some());
    }

    #[test]
    fn default_checkbox_has_window_colors() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_checkbox(&config);
        assert_eq!(
            defaults.background_color,
            Some(color_to_css(&VB_WINDOW_BACKGROUND))
        );
        assert_eq!(defaults.color, Some(color_to_css(&VB_WINDOW_TEXT)));
        assert!(defaults.box_shadow.is_some());
        assert_eq!(defaults.text_align, Some("left".to_string()));
    }

    #[test]
    fn default_optionbutton_same_as_checkbox() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_optionbutton(&config);
        let checkbox = LayoutStyle::default_checkbox(&config);
        assert_eq!(defaults, checkbox);
    }

    #[test]
    fn default_combobox_has_window_colors() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_combobox(&config);
        assert_eq!(
            defaults.background_color,
            Some(color_to_css(&VB_WINDOW_BACKGROUND))
        );
        assert_eq!(defaults.color, Some(color_to_css(&VB_WINDOW_TEXT)));
        assert!(defaults.box_shadow.is_some());
    }

    #[test]
    fn default_listbox_has_auto_overflow() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_listbox(&config);
        assert_eq!(
            defaults.background_color,
            Some(color_to_css(&VB_WINDOW_BACKGROUND))
        );
        assert_eq!(defaults.color, Some(color_to_css(&VB_WINDOW_TEXT)));
        assert_eq!(defaults.overflow, Some("auto".to_string()));
        assert!(defaults.box_shadow.is_some());
    }

    #[test]
    fn default_scrollbar_has_box_shadow() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_scrollbar(&config);
        assert!(defaults.box_shadow.is_some());
        assert!(defaults.background_color.is_none());
        assert!(defaults.color.is_none());
    }

    #[test]
    fn default_shape_rectangle_no_radius() {
        let config = LayoutConfig::default();
        let props = vb6parse::language::ShapeProperties::default();
        let defaults = LayoutStyle::default_shape(&props, &config);
        assert!(defaults.border.is_some());
        assert!(defaults.border_width.is_some());
        assert!(defaults.border_radius.is_none()); // Rectangle
    }

    #[test]
    fn default_shape_rounded_rectangle_has_radius() {
        let config = LayoutConfig::default();
        let props = vb6parse::language::ShapeProperties {
            shape: vb6parse::language::Shape::RoundedRectangle,
            border_width: 5,
            ..Default::default()
        };
        let defaults = LayoutStyle::default_shape(&props, &config);
        assert_eq!(defaults.border_radius, Some(5.0));
    }

    #[test]
    fn default_shape_circle_has_infinite_radius() {
        let config = LayoutConfig::default();
        let props = vb6parse::language::ShapeProperties {
            shape: vb6parse::language::Shape::Circle,
            ..Default::default()
        };
        let defaults = LayoutStyle::default_shape(&props, &config);
        assert_eq!(defaults.border_radius, Some(f32::INFINITY));
    }

    #[test]
    fn default_line_has_color_and_width() {
        let config = LayoutConfig::default();
        let props = vb6parse::language::LineProperties::default();
        let defaults = LayoutStyle::default_line(&props, &config);
        assert!(defaults.line_color.is_some());
        assert!(defaults.line_width.is_some());
        // Coordinates are NOT set — they're instance-specific
        assert!(defaults.line_x1.is_none());
        assert!(defaults.line_y1.is_none());
        assert!(defaults.line_x2.is_none());
        assert!(defaults.line_y2.is_none());
    }

    #[test]
    fn default_line_custom_border_width() {
        let config = LayoutConfig::default();
        let props = vb6parse::language::LineProperties {
            border_width: 3,
            ..Default::default()
        };
        let defaults = LayoutStyle::default_line(&props, &config);
        assert_eq!(defaults.line_width, Some(3.0));
    }

    // -----------------------------------------------------------------------
    // diff_against tests
    // -----------------------------------------------------------------------

    #[test]
    fn diff_against_clears_matching_fields() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_textbox(&config);
        let mut style = defaults.clone();
        style.diff_against(&defaults);
        assert!(style.background_color.is_none());
        assert!(style.color.is_none());
        assert!(style.text_align.is_none());
        assert!(style.box_shadow.is_none());
        assert!(!style.multi_line);
    }

    #[test]
    fn diff_against_preserves_non_matching_fields() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_textbox(&config);
        let mut style = LayoutStyle {
            background_color: Some(CssColor::Rgb(255, 255, 0)),
            ..defaults.clone()
        };
        style.diff_against(&defaults);
        assert_eq!(style.background_color, Some(CssColor::Rgb(255, 255, 0)));
        assert!(style.color.is_none());
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn diff_against_clears_matching_strings() {
        let defaults = LayoutStyle {
            font_family: Some("Arial".to_string()),
            display: Some("inline-block".to_string()),
            ..LayoutStyle::default()
        };
        let mut style = defaults.clone();
        style.diff_against(&defaults);
        assert!(style.font_family.is_none());
        assert!(style.display.is_none());
    }

    #[test]
    fn diff_against_preserves_different_strings() {
        let defaults = LayoutStyle {
            font_family: Some("Arial".to_string()),
            ..LayoutStyle::default()
        };
        let mut style = LayoutStyle {
            font_family: Some("Times New Roman".to_string()),
            ..defaults.clone()
        };
        style.diff_against(&defaults);
        assert_eq!(style.font_family, Some("Times New Roman".to_string()));
    }

    #[test]
    fn diff_against_clears_matching_f32() {
        let defaults = LayoutStyle {
            font_size: Some(14.0),
            border_width: Some(3.0),
            ..LayoutStyle::default()
        };
        let mut style = defaults.clone();
        style.diff_against(&defaults);
        assert!(style.font_size.is_none());
        assert!(style.border_width.is_none());
    }

    #[test]
    fn diff_against_handles_multi_line() {
        let defaults = LayoutStyle {
            multi_line: false,
            ..LayoutStyle::default()
        };
        let mut style = LayoutStyle {
            multi_line: true,
            ..defaults.clone()
        };
        style.diff_against(&defaults);
        assert!(style.multi_line);

        let mut style2 = LayoutStyle {
            multi_line: false,
            ..defaults.clone()
        };
        style2.diff_against(&defaults);
        assert!(!style2.multi_line);
    }

    #[test]
    fn diff_against_button_standard_no_background() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_button(
            &config,
            Style::Standard,
            vb6parse::language::Appearance::ThreeD,
        );
        let mut style = defaults.clone();
        style.diff_against(&defaults);
        assert!(style.display.is_none());
        assert!(style.box_shadow.is_none());
        assert!(style.background_color.is_none());
    }

    #[test]
    fn diff_against_button_graphical_clears_background() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_button(
            &config,
            Style::Graphical,
            vb6parse::language::Appearance::ThreeD,
        );
        let mut style = defaults.clone();
        style.diff_against(&defaults);
        assert!(style.background_color.is_none());
        assert!(style.box_shadow.is_none());
    }

    #[test]
    fn diff_against_preserves_custom_button_background() {
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_button(
            &config,
            Style::Graphical,
            vb6parse::language::Appearance::ThreeD,
        );
        let mut style = LayoutStyle {
            background_color: Some(CssColor::Rgb(255, 0, 0)),
            ..defaults.clone()
        };
        style.diff_against(&defaults);
        assert_eq!(style.background_color, Some(CssColor::Rgb(255, 0, 0)));
    }

    #[test]
    fn diff_against_shape_clears_defaults() {
        let props = vb6parse::language::ShapeProperties::default();
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_shape(&props, &config);
        let mut style = defaults.clone();
        style.diff_against(&defaults);
        assert!(style.border.is_none());
        assert!(style.border_style.is_none());
        assert!(style.border_width.is_none());
        assert!(style.border_radius.is_none());
    }

    #[test]
    fn diff_against_line_clears_defaults() {
        let props = vb6parse::language::LineProperties::default();
        let config = LayoutConfig::default();
        let defaults = LayoutStyle::default_line(&props, &config);
        let mut style = defaults.clone();
        style.diff_against(&defaults);
        assert!(style.line_color.is_none());
        assert!(style.line_width.is_none());
    }
}
