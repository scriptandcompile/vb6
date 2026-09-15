//! VB6 Color to CSS color mapping.
//!
//! Converts `vb6parse::language::Color` (RGB or System index) to
//! the layout engine's `CssColor` type.

use vb6parse::language::Color;

use super::model::style::CssColor;

/// Map a VB6 system color index to a CSS system color name.
///
/// Uses the system color indices defined in vb6parse.
/// Unknown indices fall back to "Canvas" (generic background).
fn system_color_to_css(index: u8) -> CssColor {
    match index {
        0x00 => CssColor::Named("ActiveBorder".to_string()),
        0x01 => CssColor::Named("Canvas".to_string()),
        0x02 => CssColor::Named("ActiveCaption".to_string()),
        0x03 => CssColor::Named("InactiveCaption".to_string()),
        0x04 => CssColor::Named("Menu".to_string()),
        0x05 => CssColor::Named("Canvas".to_string()),
        0x06 => CssColor::Named("ButtonFace".to_string()),
        0x07 => CssColor::Named("WindowText".to_string()),
        0x08 => CssColor::Named("ButtonText".to_string()),
        0x09 => CssColor::Named("CaptionText".to_string()),
        0x0A => CssColor::Named("ActiveBorder".to_string()),
        0x0B => CssColor::Named("InactiveBorder".to_string()),
        0x0C => CssColor::Named("Canvas".to_string()),
        0x0D => CssColor::Named("Highlight".to_string()),
        0x0E => CssColor::Named("HighlightText".to_string()),
        0x0F => CssColor::Named("ButtonFace".to_string()),
        0x10 => CssColor::Named("ButtonShadow".to_string()),
        0x11 => CssColor::Named("GrayText".to_string()),
        0x12 => CssColor::Named("ButtonText".to_string()),
        0x13 => CssColor::Named("InactiveCaptionText".to_string()),
        0x14 => CssColor::Named("Highlight".to_string()),
        0x15 => CssColor::Named("ButtonShadow".to_string()),
        0x16 => CssColor::Named("ButtonHighlight".to_string()),
        0x17 => CssColor::Named("WindowText".to_string()),
        0x18 => CssColor::Named("Canvas".to_string()),
        _ => CssColor::Named("Canvas".to_string()),
    }
}

/// Convert a VB6 [`Color`] to a CSS [`CssColor`].
///
/// RGB colors are mapped directly. System colors are mapped to the
/// nearest CSS system color name via the vb6parse index.
///
/// # Examples
/// ```
/// use vb6parse::language::Color;
/// use vb6runtime::layout::model::style::CssColor;
/// use vb6runtime::layout::color::color_to_css;
///
/// let rgb = Color::RGB { red: 255, green: 128, blue: 0 };
/// assert_eq!(color_to_css(&rgb), CssColor::Rgb(255, 128, 0));
///
/// let black = Color::RGB { red: 0, green: 0, blue: 0 };
/// assert_eq!(color_to_css(&black), CssColor::Rgb(0, 0, 0));
///
/// let white = Color::RGB { red: 255, green: 255, blue: 255 };
/// assert_eq!(color_to_css(&white), CssColor::Rgb(255, 255, 255));
/// ```
pub fn color_to_css(color: &Color) -> CssColor {
    match color {
        Color::RGB { red, green, blue } => CssColor::Rgb(*red, *green, *blue),
        Color::System { index } => system_color_to_css(*index),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_color_mapping() {
        let vb_color = Color::RGB { red: 255, green: 128, blue: 0 };
        let css = color_to_css(&vb_color);
        assert_eq!(css, CssColor::Rgb(255, 128, 0));
    }

    #[test]
    fn rgb_black() {
        let css = color_to_css(&Color::RGB { red: 0, green: 0, blue: 0 });
        assert_eq!(css, CssColor::Rgb(0, 0, 0));
    }

    #[test]
    fn rgb_white() {
        let css = color_to_css(&Color::RGB { red: 255, green: 255, blue: 255 });
        assert_eq!(css, CssColor::Rgb(255, 255, 255));
    }

    #[test]
    fn rgb_red() {
        let css = color_to_css(&Color::RGB { red: 255, green: 0, blue: 0 });
        assert_eq!(css, CssColor::Rgb(255, 0, 0));
    }

    #[test]
    fn rgb_green() {
        let css = color_to_css(&Color::RGB { red: 0, green: 255, blue: 0 });
        assert_eq!(css, CssColor::Rgb(0, 255, 0));
    }

    #[test]
    fn rgb_blue() {
        let css = color_to_css(&Color::RGB { red: 0, green: 0, blue: 255 });
        assert_eq!(css, CssColor::Rgb(0, 0, 255));
    }

    #[test]
    fn system_color_scrollbars() {
        // index 0 = vbScrollBars
        let vb_color = Color::System { index: 0x00 };
        let css = color_to_css(&vb_color);
        assert!(matches!(css, CssColor::Named(ref s) if s == "ActiveBorder"));
    }

    #[test]
    fn system_color_desktop() {
        // index 1 = vbDesktop
        let vb_color = Color::System { index: 0x01 };
        let css = color_to_css(&vb_color);
        assert!(matches!(css, CssColor::Named(ref s) if s == "Canvas"));
    }

    #[test]
    fn system_color_window() {
        // index 5 = vbWindowBackground
        let vb_color = Color::System { index: 0x05 };
        let css = color_to_css(&vb_color);
        assert!(matches!(css, CssColor::Named(ref s) if s == "Canvas"));
    }

    #[test]
    fn system_color_windowtext() {
        // index 8 = vbWindowText
        let vb_color = Color::System { index: 0x08 };
        let css = color_to_css(&vb_color);
        assert!(matches!(css, CssColor::Named(ref s) if s == "ButtonText"));
    }

    #[test]
    fn system_color_highlight() {
        // index 13 = vbHighlight
        let vb_color = Color::System { index: 0x0D };
        let css = color_to_css(&vb_color);
        assert!(matches!(css, CssColor::Named(ref s) if s == "Highlight"));
    }

    #[test]
    fn system_color_highlight_text() {
        // index 14 = vbHighlightText
        let vb_color = Color::System { index: 0x0E };
        let css = color_to_css(&vb_color);
        assert!(matches!(css, CssColor::Named(ref s) if s == "HighlightText"));
    }

    #[test]
    fn system_color_gray_text() {
        // index 17 = vbGrayText
        let vb_color = Color::System { index: 0x11 };
        let css = color_to_css(&vb_color);
        assert!(matches!(css, CssColor::Named(ref s) if s == "GrayText"));
    }

    #[test]
    fn system_color_button_text() {
        // index 18 = vbButtonText
        let vb_color = Color::System { index: 0x12 };
        let css = color_to_css(&vb_color);
        assert!(matches!(css, CssColor::Named(ref s) if s == "ButtonText"));
    }

    #[test]
    fn system_color_info_background() {
        // index 24 = vbInfoBackground / vbMessageBox
        let vb_color = Color::System { index: 0x18 };
        let css = color_to_css(&vb_color);
        assert!(matches!(css, CssColor::Named(ref s) if s == "Canvas"));
    }

    #[test]
    fn system_color_unknown_fallback() {
        let vb_color = Color::System { index: 0xFF };
        let css = color_to_css(&vb_color);
        // Unknown indices fall back to Canvas
        assert!(matches!(css, CssColor::Named(ref s) if s == "Canvas"));
    }

    #[test]
    fn system_color_zero() {
        let vb_color = Color::System { index: 0x00 };
        let css = color_to_css(&vb_color);
        assert!(matches!(css, CssColor::Named(_)));
    }

    #[test]
    fn predefined_vb_black_is_rgb() {
        // vbBlack = RGB(0,0,0)
        let css = color_to_css(&vb6parse::language::VB_BLACK);
        assert_eq!(css, CssColor::Rgb(0, 0, 0));
    }

    #[test]
    fn predefined_vb_white_is_rgb() {
        // vbWhite = RGB(255,255,255)
        let css = color_to_css(&vb6parse::language::VB_WHITE);
        assert_eq!(css, CssColor::Rgb(255, 255, 255));
    }

    #[test]
    fn predefined_vb_red_is_rgb() {
        let css = color_to_css(&vb6parse::language::VB_RED);
        assert_eq!(css, CssColor::Rgb(255, 0, 0));
    }

    #[test]
    fn predefined_vb_green_is_rgb() {
        let css = color_to_css(&vb6parse::language::VB_GREEN);
        assert_eq!(css, CssColor::Rgb(0, 255, 0));
    }

    #[test]
    fn predefined_vb_blue_is_rgb() {
        let css = color_to_css(&vb6parse::language::VB_BLUE);
        assert_eq!(css, CssColor::Rgb(0, 0, 255));
    }

    #[test]
    fn predefined_vb_yellow_is_rgb() {
        let css = color_to_css(&vb6parse::language::VB_YELLOW);
        assert_eq!(css, CssColor::Rgb(255, 255, 0));
    }

    #[test]
    fn predefined_vb_magenta_is_rgb() {
        let css = color_to_css(&vb6parse::language::VB_MAGENTA);
        assert_eq!(css, CssColor::Rgb(255, 0, 255));
    }

    #[test]
    fn predefined_vb_cyan_is_rgb() {
        let css = color_to_css(&vb6parse::language::VB_CYAN);
        assert_eq!(css, CssColor::Rgb(0, 255, 255));
    }

    #[test]
    fn all_predefined_colors_convert() {
        // Every color in PREDEFINED_COLORS should convert without panicking
        for (_, color) in vb6parse::language::color::PREDEFINED_COLORS {
            let _ = color_to_css(&color);
        }
    }
}
