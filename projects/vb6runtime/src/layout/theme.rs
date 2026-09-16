//! CSS custom property theming + injection API.
//!
//! Provides [`Vb6Theme`] for programmatic overrides and built-in themes
//! (default, dark). Themes generate CSS custom property declarations that
//! cascade to all VB6 controls via the `.vb6-app` scope root.

//!
//! # Usage
//!
//! ```
//! use vb6runtime::layout::theme::{Vb6Theme, default_theme, dark_theme};
//!
//! let theme = default_theme();
//! let css = theme.to_css();
//! assert!(css.contains("--vb6-bg"));
//! assert!(css.starts_with(".vb6-app {"));
//!
//! let dark = dark_theme();
//! let css = dark.to_css();
//! assert!(css.contains("--vb6-bg: #1e1e1e"));
//! ```

/// VB6 theme — a set of CSS custom property overrides.
///
/// Each field corresponds to a CSS custom property used by the VB6 layout engine.
/// Only non-`None` fields are included in the generated CSS.
///
/// # CSS Custom Properties
///
/// | Field | CSS Property | Default |
/// |-------|-------------|---------|
/// | `bg` | `--vb6-bg` | `rgb(192, 192, 192)` |
/// | `fg` | `--vb6-fg` | `rgb(0, 0, 0)` |
/// | `window_bg` | `--vb6-window-bg` | `#ffffff` |
/// | `window_text` | `--vb6-window-text` | `#000000` |
/// | `button_bg` | `--vb6-button-bg` | `rgb(192, 192, 192)` |
/// | `button_border` | `--vb6-button-border` | `rgb(120, 120, 120)` |
/// | `highlight` | `--vb6-highlight` | `rgb(0, 0, 128)` |
/// | `highlight_text` | `--vb6-highlight-text` | `#ffffff` |
/// | `focus_border` | `--vb6-focus-border` | `rgb(0, 0, 128)` |
/// | `disabled_opacity` | `--vb6-disabled-opacity` | `0.5` |
/// | `font_family` | `--vb6-font-family` | `"MS Sans Serif", Tahoma, sans-serif` |
/// | `font_size` | `--vb6-font-size` | `11px` |
///
/// # Theming Strategy
///
/// Themes are applied by injecting CSS into the host environment:
/// - **WASM**: `<style>` tag injected into `<head>` with `.vb6-app { --vb6-*: ... }`
/// - **Tauri**: `<style>` tag injected into the webview via `webview.eval()`
///
/// The `.vb6-app` wrapper ensures CSS is scoped and doesn't leak into the parent page.
#[derive(Debug, Clone, Default)]
pub struct Vb6Theme {
    /// Background color for forms and controls.
    pub bg: Option<String>,
    /// Foreground / text color.
    pub fg: Option<String>,
    /// Background color for window content (TextBox, etc.).
    pub window_bg: Option<String>,
    /// Text color for window content.
    pub window_text: Option<String>,
    /// Background color for buttons.
    pub button_bg: Option<String>,
    /// Border color for buttons and controls.
    pub button_border: Option<String>,
    /// Highlight color (selection, active items).
    pub highlight: Option<String>,
    /// Text color for highlighted items.
    pub highlight_text: Option<String>,
    /// Focus indicator border color.
    pub focus_border: Option<String>,
    /// Opacity value for disabled controls.
    pub disabled_opacity: Option<String>,
    /// Font family for all controls.
    pub font_family: Option<String>,
    /// Font size for all controls.
    pub font_size: Option<String>,
}

impl Vb6Theme {
    /// Generate CSS custom property declarations from the theme.
    ///
    /// The output is a CSS rule targeting `.vb6-app { ... }` with all
    /// non-`None` theme fields as custom property declarations.
    ///
    /// # Examples
    /// ```
    /// use vb6runtime::layout::theme::default_theme;
    ///
    /// let theme = default_theme();
    /// let css = theme.to_css();
    /// assert!(css.starts_with(".vb6-app {"));
    /// assert!(css.contains("--vb6-bg: rgb(192, 192, 192)"));
    /// ```
    pub fn to_css(&self) -> String {
        let mut props = Vec::new();

        if let Some(ref v) = self.bg {
            props.push(format!("  --vb6-bg: {v};"));
        }
        if let Some(ref v) = self.fg {
            props.push(format!("  --vb6-fg: {v};"));
        }
        if let Some(ref v) = self.window_bg {
            props.push(format!("  --vb6-window-bg: {v};"));
        }
        if let Some(ref v) = self.window_text {
            props.push(format!("  --vb6-window-text: {v};"));
        }
        if let Some(ref v) = self.button_bg {
            props.push(format!("  --vb6-button-bg: {v};"));
        }
        if let Some(ref v) = self.button_border {
            props.push(format!("  --vb6-button-border: {v};"));
        }
        if let Some(ref v) = self.highlight {
            props.push(format!("  --vb6-highlight: {v};"));
        }
        if let Some(ref v) = self.highlight_text {
            props.push(format!("  --vb6-highlight-text: {v};"));
        }
        if let Some(ref v) = self.focus_border {
            props.push(format!("  --vb6-focus-border: {v};"));
        }
        if let Some(ref v) = self.disabled_opacity {
            props.push(format!("  --vb6-disabled-opacity: {v};"));
        }
        if let Some(ref v) = self.font_family {
            props.push(format!("  --vb6-font-family: {v};"));
        }
        if let Some(ref v) = self.font_size {
            props.push(format!("  --vb6-font-size: {v};"));
        }

        format!(".vb6-app {{\n{}\n}}", props.join("\n"))
    }
}

/// Returns the default (Windows classic) VB6 theme.
///
/// Uses the classic gray (`rgb(192, 192, 192)`) background with dark text,
/// matching the default VB6 appearance.
#[must_use]
pub fn default_theme() -> Vb6Theme {
    Vb6Theme {
        bg: Some("rgb(192, 192, 192)".into()),
        fg: Some("rgb(0, 0, 0)".into()),
        window_bg: Some("#ffffff".into()),
        window_text: Some("#000000".into()),
        button_bg: Some("rgb(192, 192, 192)".into()),
        button_border: Some("rgb(120, 120, 120)".into()),
        highlight: Some("rgb(0, 0, 128)".into()),
        highlight_text: Some("#ffffff".into()),
        focus_border: Some("rgb(0, 0, 128)".into()),
        disabled_opacity: Some("0.5".into()),
        font_family: Some("MS Sans Serif, Tahoma, sans-serif".into()),
        font_size: Some("11px".into()),
    }
}

/// Returns the dark theme.
///
/// Uses dark backgrounds (`#1e1e1e`) with light text (`#d4d4d4`),
/// similar to modern IDE dark themes.
#[must_use]
pub fn dark_theme() -> Vb6Theme {
    Vb6Theme {
        bg: Some("#1e1e1e".into()),
        fg: Some("#d4d4d4".into()),
        window_bg: Some("#3c3c3c".into()),
        window_text: Some("#cccccc".into()),
        button_bg: Some("#3c3c3c".into()),
        button_border: Some("#555555".into()),
        highlight: Some("#0067c0".into()),
        highlight_text: Some("#ffffff".into()),
        focus_border: Some("#0067c0".into()),
        disabled_opacity: Some("0.5".into()),
        font_family: Some("MS Sans Serif, Tahoma, sans-serif".into()),
        font_size: Some("11px".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_theme_produces_valid_css() {
        let theme = default_theme();
        let css = theme.to_css();
        assert!(css.contains("--vb6-bg"));
        assert!(css.contains("rgb(192, 192, 192)"));
        assert!(css.starts_with(".vb6-app {"));
        assert!(css.ends_with("}"));
    }

    #[test]
    fn dark_theme_overrides_bg() {
        let theme = dark_theme();
        let css = theme.to_css();
        assert!(css.contains("--vb6-bg: #1e1e1e"));
        assert!(css.contains("--vb6-window-bg: #3c3c3c"));
        assert!(css.contains("--vb6-fg: #d4d4d4"));
    }

    #[test]
    fn empty_theme_produces_empty_rule() {
        let theme = Vb6Theme {
            bg: None,
            fg: None,
            window_bg: None,
            window_text: None,
            button_bg: None,
            button_border: None,
            highlight: None,
            highlight_text: None,
            focus_border: None,
            disabled_opacity: None,
            font_family: None,
            font_size: None,
        };
        let css = theme.to_css();
        assert!(css.starts_with(".vb6-app {\n\n}"));
    }

    #[test]
    fn partial_theme_includes_only_set_fields() {
        let theme = Vb6Theme {
            bg: Some("#ff0000".into()),
            ..Default::default()
        };
        let css = theme.to_css();
        assert!(css.contains("--vb6-bg: #ff0000"));
        assert!(!css.contains("--vb6-fg"));
        assert!(!css.contains("--vb6-window-bg"));
    }

    #[test]
    fn default_theme_has_all_fields() {
        let theme = default_theme();
        let css = theme.to_css();
        assert!(css.contains("--vb6-bg"));
        assert!(css.contains("--vb6-fg"));
        assert!(css.contains("--vb6-window-bg"));
        assert!(css.contains("--vb6-window-text"));
        assert!(css.contains("--vb6-button-bg"));
        assert!(css.contains("--vb6-button-border"));
        assert!(css.contains("--vb6-highlight"));
        assert!(css.contains("--vb6-highlight-text"));
        assert!(css.contains("--vb6-focus-border"));
        assert!(css.contains("--vb6-disabled-opacity"));
        assert!(css.contains("--vb6-font-family"));
        assert!(css.contains("--vb6-font-size"));
    }
}
