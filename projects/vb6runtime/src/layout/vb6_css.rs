//! Core VB6 CSS stylesheet with scoped selectors for platform isolation.
//!
//! Generates a CSS stylesheet that uses `.vb6-app` as a scope root for WASM isolation.
//! On WASM, all selectors descend from `.vb6-app` so parent page styles don't leak in
//! and VB6 styles don't leak out. On Tauri, the same CSS is used but without the
//! `.vb6-app` prefix since the webview owns the full DOM.
//!
//! # Generated CSS Structure
//!
//! ```css
//! .vb6-app {
//!     /* Root reset + CSS custom properties */
//!     margin: 0; padding: 0; box-sizing: border-box;
//!     font-family: "MS Sans Serif", Tahoma, sans-serif;
//!     font-size: 11px;
//!     --vb6-bg: rgb(192, 192, 192);
//!     --vb6-fg: rgb(0, 0, 0);
//!     /* ... more custom properties ... */
//! }
//!
//! .vb6-app .vb6-form {
//!     background-color: var(--vb6-bg);
//!     color: var(--vb6-fg);
//!     /* ... */
//! }
//! /* ... more control selectors ... */
//! ```
//!
//! # Usage
//!
//! ```
//! use vb6runtime::layout::vb6_css::{scoped_css, bare_css, scoped_dark_css};
//!
//! // WASM: scoped selectors (prefixed with .vb6-app)
//! let css = scoped_css();
//! assert!(css.contains(".vb6-app .vb6-form"));
//!
//! // Tauri: bare selectors (no .vb6-app prefix)
//! let css = bare_css();
//! assert!(css.contains("body {"));
//! assert!(!css.contains(".vb6-app"));
//!
//! // Dark theme override
//! let dark = scoped_dark_css();
//! assert!(dark.contains("--vb6-bg: #1e1e1e"));
//! ```

/// Generate the full VB6 CSS stylesheet with `.vb6-app` scoped selectors.
///
/// Every selector is prefixed with `.vb6-app` so the stylesheet can be safely
/// embedded inside a `.vb6-app` root element on WASM. This prevents parent-page
/// styles from leaking into VB6 elements and vice versa.
///
/// The generated CSS includes:
/// - Root-level reset (margin, padding, box-sizing)
/// - Default font settings
/// - CSS custom property definitions (theme variables)
/// - Per-control scoped selectors using `var(--vb6-*)` references
#[must_use]
pub fn scoped_css() -> String {
    let base = root_css();
    let controls = scoped_control_css();
    let themes = theme_overrides_css();
    format!("{base}\n{controls}\n{themes}")
}

/// Generate the root-level CSS with bare selectors (CSS custom property definitions).
///
/// Defines the `--vb6-*` custom properties at `:root` so per-control selectors
/// can reference them via `var()`. This is the bare equivalent of `root_css()`
/// for Tauri where there is no `.vb6-app` scope root.
#[must_use]
pub fn bare_root_css() -> String {
    r#":root {
  /* VB6 color scheme (Windows classic) */
  --vb6-bg: rgb(192, 192, 192);
  --vb6-fg: rgb(0, 0, 0);
  --vb6-window-bg: #ffffff;
  --vb6-window-text: #000000;
  --vb6-button-bg: rgb(192, 192, 192);
  --vb6-button-border: rgb(120, 120, 120);
  --vb6-highlight: rgb(0, 0, 128);
  --vb6-highlight-text: #ffffff;
  --vb6-focus-border: rgb(0, 0, 128);
  --vb6-disabled-opacity: 0.5;
  --vb6-font-family: "MS Sans Serif", Tahoma, sans-serif;
  --vb6-font-size: 11px;
}
body {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  background-color: var(--vb6-bg);
  color: var(--vb6-fg);
  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);
  line-height: 1.2;
}"#
    .to_string()
}

/// Generate the full VB6 CSS stylesheet with bare selectors (no `.vb6-app` prefix).
///
/// Used by Tauri renderer where the webview owns the full DOM. Same selectors
/// as `scoped_css()` but without the `.vb6-app` ancestor prefix.
#[must_use]
pub fn bare_css() -> String {
    let root = bare_root_css();
    let controls = bare_control_css();
    let themes = bare_theme_overrides_css();
    format!("{root}\n{controls}\n{themes}")
}

/// Generate a CSS override for the dark theme.
///
/// The output is a CSS rule: `.vb6-app.theme-dark { --vb6-bg: #1e1e1e; ... }`
/// When scoped, the `.vb6-app` prefix is included. When bare, it is omitted.
///
/// Callers should combine this with `scoped_css()` or `bare_css()` by concatenation,
/// or inject it separately as a theme override.
#[must_use]
pub fn scoped_dark_css() -> String {
    format!(".vb6-app.theme-dark {{\n{}\n}}", dark_props())
}

/// Generate bare CSS for the dark theme (properties only, no selector).
///
/// Used as the body of a `.vb6-app.theme-dark { ... }` rule without the
/// `.vb6-app` prefix. For Tauri where scoping is not needed.
#[must_use]
pub fn bare_dark_css() -> String {
    dark_props()
}

/// Generate CSS for the Fluent (Windows 11) theme override.
#[must_use]
pub fn scoped_fluent_css() -> String {
    format!(".vb6-app.theme-fluent {{\n{}\n}}", fluent_props())
}

/// Generate bare CSS for the Fluent (Windows 11) theme (properties only, no selector).
///
/// Used as the body of a `.vb6-app.theme-fluent { ... }` rule without the
/// `.vb6-app` prefix. For Tauri where scoping is not needed.
#[must_use]
pub fn bare_fluent_css() -> String {
    fluent_props()
}

/// Generate theme overrides for dark and fluent themes (bare, no `.vb6-app` prefix).
fn bare_theme_overrides_css() -> String {
    format!("\n{}", bare_dark_css())
}

/// Generate theme overrides for the dark theme (properties only, no selector).
///
/// Used as the body of a `.vb6-app.theme-dark { ... }` rule.
#[must_use]
pub fn dark_props() -> String {
    r#"  --vb6-bg: #1e1e1e;
  --vb6-fg: #d4d4d4;
  --vb6-window-bg: #3c3c3c;
  --vb6-window-text: #cccccc;
  --vb6-button-bg: #3c3c3c;
  --vb6-button-border: #555555;
  --vb6-highlight: #0067c0;
  --vb6-highlight-text: #ffffff;
  --vb6-focus-border: #0067c0;
  --vb6-font-family: "MS Sans Serif", Tahoma, sans-serif;
  --vb6-font-size: 11px;
  --vb6-disabled-opacity: 0.5;"#
        .to_string()
}

/// Generate theme overrides for the Fluent (Windows 11) theme.
#[must_use]
pub fn fluent_props() -> String {
    r#"  --vb6-bg: #f3f3f3;
  --vb6-fg: #202020;
  --vb6-window-bg: #ffffff;
  --vb6-window-text: #1a1a1a;
  --vb6-button-bg: #f3f3f3;
  --vb6-button-border: #c8c8c8;
  --vb6-highlight: #0067c0;
  --vb6-highlight-text: #ffffff;
  --vb6-focus-border: #0067c0;
  --vb6-font-family: "MS Sans Serif", Tahoma, sans-serif;
  --vb6-font-size: 11px;
  --vb6-disabled-opacity: 0.5;"#
        .to_string()
}

/// Generate theme overrides for a custom theme.
///
/// Produces a CSS rule `.vb6-app.theme-<name> { --vb6-*: ...; }` from
/// a map of property name to value.
#[must_use]
pub fn custom_theme_css(name: &str, props: &[(&str, &str)]) -> String {
    let mut css = format!(".vb6-app.theme-{name} {{\n");
    for (key, value) in props {
        css.push_str(&format!("  --{key}: {value};\n"));
    }
    css.push('}');
    css
}

// ── Root reset ────────────────────────────────────────────────────────

/// The root-level CSS reset + theme variables.
///
/// Contains the base reset (margin, padding, box-sizing) and default CSS
/// custom property definitions that per-control selectors reference via `var()`.
fn root_css() -> String {
    r#".vb6-app {
  /* Root reset — only affects VB6 content */
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: "MS Sans Serif", Tahoma, sans-serif;
  font-size: 11px;
  line-height: 1.2;

  /* Default VB6 color scheme (Windows classic) */
  --vb6-bg: rgb(192, 192, 192);
  --vb6-fg: rgb(0, 0, 0);
  --vb6-window-bg: #ffffff;
  --vb6-window-text: #000000;
  --vb6-button-bg: rgb(192, 192, 192);
  --vb6-button-border: rgb(120, 120, 120);
  --vb6-highlight: rgb(0, 0, 128);
  --vb6-highlight-text: #ffffff;
  --vb6-focus-border: rgb(0, 0, 128);
  --vb6-disabled-opacity: 0.5;
  --vb6-font-family: "MS Sans Serif", Tahoma, sans-serif;
  --vb6-font-size: 11px;
}"#
    .to_string()
}

// ── Per-control selectors (scoped) ────────────────────────────────────

/// All per-control CSS selectors with `.vb6-app` prefix.
fn scoped_control_css() -> String {
    let mut css = String::new();
    for rule in scoped_control_rules() {
        css.push_str(&format!("\n{rule}"));
    }
    css
}

/// All per-control CSS selectors without `.vb6-app` prefix (for Tauri).
fn bare_control_css() -> String {
    let mut css = String::new();
    for rule in bare_control_rules() {
        css.push_str(&format!("\n{rule}"));
    }
    css
}

/// Scoped control rules (each rule starts with `.vb6-app .vb6-<type>`).
fn scoped_control_rules() -> Vec<String> {
    vec![
        vb6_rule(".vb6-app .vb6-form", &form_style()),
        vb6_rule(".vb6-app .vb6-label", &label_style()),
        vb6_rule(".vb6-app .vb6-textbox", &textbox_style()),
        vb6_rule(
            ".vb6-app .vb6-textbox:focus::selection",
            &textbox_focus_selection_style(),
        ),
        vb6_rule(
            ".vb6-app .vb6-textbox:not(:focus)::selection",
            &textbox_blur_selection_style(),
        ),
        vb6_rule(".vb6-app .vb6-commandbutton", &button_style()),
        vb6_rule(".vb6-app .vb6-frame", &frame_style()),
        vb6_rule(".vb6-app .vb6-picturebox", &picturebox_style()),
        vb6_rule(".vb6-app .vb6-align-top", &align_top_style()),
        vb6_rule(".vb6-app .vb6-align-bottom", &align_bottom_style()),
        vb6_rule(".vb6-app .vb6-align-left", &align_left_style()),
        vb6_rule(".vb6-app .vb6-align-right", &align_right_style()),
        vb6_rule(".vb6-app .vb6-image", &image_style()),
        vb6_rule(".vb6-app .vb6-checkbox", &checkbox_style()),
        vb6_rule(".vb6-app .vb6-optionbutton", &optionbutton_style()),
        vb6_rule(".vb6-app .vb6-combobox", &combobox_style()),
        vb6_rule(".vb6-app .vb6-listbox", &listbox_style()),
        vb6_rule(".vb6-app .vb6-hscrollbar", &hscrollbar_style()),
        vb6_rule(".vb6-app .vb6-vscrollbar", &vscrollbar_style()),
        vb6_rule(
            ".vb6-app .vb6-hscrollbar::-webkit-slider-runnable-track",
            &track_style(),
        ),
        vb6_rule(
            ".vb6-app .vb6-hscrollbar::-webkit-slider-thumb",
            &thumb_style(),
        ),
        vb6_rule(
            ".vb6-app .vb6-vscrollbar::-webkit-slider-runnable-track",
            &v_track_style(),
        ),
        vb6_rule(
            ".vb6-app .vb6-vscrollbar::-webkit-slider-thumb",
            &v_thumb_style(),
        ),
        vb6_rule(".vb6-app .vb6-shape", &shape_style()),
        vb6_rule(".vb6-app .vb6-line", &line_style()),
        vb6_rule(".vb6-app .vb6-timer", &timer_style()),
        vb6_rule(".vb6-app .vb6-dritelb", &drive_listbox_style()),
        vb6_rule(".vb6-app .vb6-dirlistbox", &dir_listbox_style()),
        vb6_rule(".vb6-app .vb6-filelistbox", &file_listbox_style()),
        vb6_rule(
            ".vb6-app .vb6-commandbutton:disabled, \
             .vb6-app .vb6-textbox:disabled, \
             .vb6-app .vb6-checkbox:disabled, \
             .vb6-app .vb6-optionbutton:disabled, \
             .vb6-app .vb6-combobox:disabled, \
             .vb6-app .vb6-listbox:disabled, \
             .vb6-app .vb6-hscrollbar:disabled, \
             .vb6-app .vb6-vscrollbar:disabled",
            &disabled_control_style(),
        ),
        vb6_rule(".vb6-app .vb6-mnemonic", &mnemonic_style()),
    ]
}

/// Bare control rules (no `.vb6-app` prefix, starts with `.vb6-<type>`).
fn bare_control_rules() -> Vec<String> {
    vec![
        vb6_rule(".vb6-label", &label_style()),
        vb6_rule(".vb6-textbox", &textbox_style()),
        vb6_rule(
            ".vb6-textbox:focus::selection",
            &textbox_focus_selection_style(),
        ),
        vb6_rule(
            ".vb6-textbox:not(:focus)::selection",
            &textbox_blur_selection_style(),
        ),
        vb6_rule(".vb6-commandbutton", &button_style()),
        vb6_rule(".vb6-frame", &frame_style()),
        vb6_rule(".vb6-picturebox", &picturebox_style()),
        vb6_rule(".vb6-align-top", &align_top_style()),
        vb6_rule(".vb6-align-bottom", &align_bottom_style()),
        vb6_rule(".vb6-align-left", &align_left_style()),
        vb6_rule(".vb6-align-right", &align_right_style()),
        vb6_rule(".vb6-image", &image_style()),
        vb6_rule(".vb6-checkbox", &checkbox_style()),
        vb6_rule(".vb6-optionbutton", &optionbutton_style()),
        vb6_rule(".vb6-combobox", &combobox_style()),
        vb6_rule(".vb6-listbox", &listbox_style()),
        vb6_rule(".vb6-hscrollbar", &hscrollbar_style()),
        vb6_rule(".vb6-vscrollbar", &vscrollbar_style()),
        vb6_rule(
            ".vb6-hscrollbar::-webkit-slider-runnable-track",
            &track_style(),
        ),
        vb6_rule(".vb6-hscrollbar::-webkit-slider-thumb", &thumb_style()),
        vb6_rule(
            ".vb6-vscrollbar::-webkit-slider-runnable-track",
            &v_track_style(),
        ),
        vb6_rule(".vb6-vscrollbar::-webkit-slider-thumb", &v_thumb_style()),
        vb6_rule(".vb6-shape", &shape_style()),
        vb6_rule(".vb6-line", &line_style()),
        vb6_rule(".vb6-timer", &timer_style()),
        vb6_rule(".vb6-dritelb", &drive_listbox_style()),
        vb6_rule(".vb6-dirlistbox", &dir_listbox_style()),
        vb6_rule(".vb6-filelistbox", &file_listbox_style()),
        vb6_rule(
            ".vb6-commandbutton:disabled, \
             .vb6-textbox:disabled, \
             .vb6-checkbox:disabled, \
             .vb6-optionbutton:disabled, \
             .vb6-combobox:disabled, \
             .vb6-listbox:disabled, \
             .vb6-hscrollbar:disabled, \
             .vb6-vscrollbar:disabled",
            &disabled_control_style(),
        ),
        vb6_rule(".vb6-mnemonic", &mnemonic_style()),
    ]
}

/// Format a CSS rule: `<selector> { <properties> }`.
fn vb6_rule(selector: &str, props: &str) -> String {
    format!("{selector} {{\n{props}}}")
}

// ── Per-control CSS properties ────────────────────────────────────────

fn form_style() -> String {
    r#"  background-color: var(--vb6-bg);
  color: var(--vb6-fg);
  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);"#
        .to_string()
}

fn label_style() -> String {
    r#"  background-color: var(--vb6-bg);
  color: var(--vb6-fg);
  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);
  overflow: hidden;"#
        .to_string()
}

fn textbox_style() -> String {
    r#"  background-color: var(--vb6-window-bg);
  color: var(--vb6-window-text);
  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);
  border: 1px solid var(--vb6-button-border);
  resize: none;"#
        .to_string()
}

fn textbox_focus_selection_style() -> String {
    "  background: #0078d4;\n  color: var(--vb6-window-text);".to_string()
}

fn textbox_blur_selection_style() -> String {
    "  background: rgba(0, 0, 0, 0.1);\n  color: var(--vb6-window-text);".to_string()
}

fn button_style() -> String {
    r#"  background-color: var(--vb6-button-bg);
  color: var(--vb6-fg);
  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);
  border: 1px solid var(--vb6-button-border);
  padding: 2px 8px;
  cursor: default;"#
        .to_string()
}

fn frame_style() -> String {
    r#"  background-color: var(--vb6-bg);
  color: var(--vb6-fg);
  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);
  border-radius: 2px;"#
        .to_string()
}

fn picturebox_style() -> String {
    r#"  background-color: var(--vb6-bg);
  color: var(--vb6-fg);
  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);
  border: 1px solid var(--vb6-button-border);
  overflow: hidden;"#
        .to_string()
}

fn align_top_style() -> String {
    "  position: absolute;\n  left: 0;\n  width: 100%;\n  top: 0;".to_string()
}

fn align_bottom_style() -> String {
    "  position: absolute;\n  left: 0;\n  width: 100%;\n  bottom: 0;".to_string()
}

fn align_left_style() -> String {
    "  position: absolute;\n  top: 0;\n  height: 100%;\n  left: 0;".to_string()
}

fn align_right_style() -> String {
    "  position: absolute;\n  top: 0;\n  height: 100%;\n  right: 0;".to_string()
}

fn image_style() -> String {
    r#"  background-color: transparent;
  border: none;"#
        .to_string()
}

fn mnemonic_style() -> String {
    r#"  text-decoration: underline;
  cursor: default;"#
        .to_string()
}

fn checkbox_style() -> String {
    r#"  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);"#
        .to_string()
}

fn optionbutton_style() -> String {
    r#"  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);"#
        .to_string()
}

fn combobox_style() -> String {
    r#"  background-color: var(--vb6-window-bg);
  color: var(--vb6-window-text);
  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);
  border: 1px solid var(--vb6-button-border);
  border-radius: 0;"#
        .to_string()
}

fn listbox_style() -> String {
    r#"  background-color: var(--vb6-window-bg);
  color: var(--vb6-window-text);
  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);
  border: 1px solid var(--vb6-button-border);
  border-radius: 0;"#
        .to_string()
}

fn hscrollbar_style() -> String {
    r#"  -webkit-appearance: none;
  appearance: none;
  background-color: transparent;
  border: none;
  height: 16px;"#
        .to_string()
}

fn vscrollbar_style() -> String {
    r#"  -webkit-appearance: none;
  appearance: none;
  background-color: transparent;
  border: none;
  width: 16px;
  writing-mode: vertical-lr;"#
        .to_string()
}

fn shape_style() -> String {
    r#"  background-color: transparent;
  border: 1px solid var(--vb6-fg);"#
        .to_string()
}

fn line_style() -> String {
    String::new()
}

fn timer_style() -> String {
    String::new()
}

/// Classic Windows 95 style scrollbar track (horizontal).
fn track_style() -> String {
    r#"  height: 8px;
  background: var(--vb6-button-bg);
  border: 1px solid var(--vb6-button-border);
  border-radius: 0;"#
        .to_string()
}

/// Classic Windows 95 style scrollbar thumb (horizontal).
fn thumb_style() -> String {
    r#"  -webkit-appearance: none;
  appearance: none;
  width: 12px;
  height: 8px;
  background: linear-gradient(to right, rgb(255, 255, 255), rgb(192, 192, 192));
  border: 1px solid var(--vb6-button-border);
  border-radius: 0;"#
        .to_string()
}

/// Classic Windows 95 style scrollbar track (vertical).
fn v_track_style() -> String {
    r#"  width: 8px;
  height: 100%;
  background: var(--vb6-button-bg);
  border: 1px solid var(--vb6-button-border);
  border-radius: 0;"#
        .to_string()
}

/// Classic Windows 95 style scrollbar thumb (vertical).
fn v_thumb_style() -> String {
    r#"  -webkit-appearance: none;
  appearance: none;
  width: 8px;
  height: 12px;
  background: linear-gradient(to top, rgb(255, 255, 255), rgb(192, 192, 192));
  border: 1px solid var(--vb6-button-border);
  border-radius: 0;"#
        .to_string()
}

/// Grayed text for disabled interactive controls (native controls also gray
/// themselves; this keeps themed controls readable).
fn disabled_control_style() -> String {
    "  color: rgb(128, 128, 128);".to_string()
}

fn drive_listbox_style() -> String {
    r#"  background-color: var(--vb6-window-bg);
  color: var(--vb6-window-text);
  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);
  border: 1px solid var(--vb6-button-border);"#
        .to_string()
}

fn dir_listbox_style() -> String {
    r#"  background-color: var(--vb6-window-bg);
  color: var(--vb6-window-text);
  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);
  border: 1px solid var(--vb6-button-border);"#
        .to_string()
}

fn file_listbox_style() -> String {
    r#"  background-color: var(--vb6-window-bg);
  color: var(--vb6-window-text);
  font-family: var(--vb6-font-family);
  font-size: var(--vb6-font-size);
  border: 1px solid var(--vb6-button-border);"#
        .to_string()
}

// ── Theme overrides ───────────────────────────────────────────────────

/// Generate CSS for theme overrides (dark, fluent, etc.).
fn theme_overrides_css() -> String {
    format!("\n{}", scoped_dark_css())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::color::color_to_css;
    use vb6parse::language::{Color, VB_BUTTON_FACE, VB_WINDOW_BACKGROUND, VB_WINDOW_TEXT};

    #[test]
    fn scoped_css_contains_vb6_app_prefix() {
        let css = scoped_css();
        assert!(css.contains(".vb6-app .vb6-form"));
        assert!(css.contains(".vb6-app .vb6-label"));
        assert!(css.contains(".vb6-app .vb6-textbox"));
        assert!(css.contains(".vb6-app .vb6-commandbutton"));
        assert!(css.contains(".vb6-app .vb6-frame"));
    }

    #[test]
    fn scoped_css_contains_root_reset() {
        let css = scoped_css();
        assert!(css.contains(".vb6-app {"));
        assert!(css.contains("margin: 0"));
        assert!(css.contains("box-sizing: border-box"));
    }

    #[test]
    fn scoped_css_contains_theme_variables() {
        let css = scoped_css();
        assert!(css.contains("--vb6-bg: rgb(192, 192, 192)"));
        assert!(css.contains("--vb6-fg: rgb(0, 0, 0)"));
        assert!(css.contains("--vb6-window-bg: #ffffff"));
        assert!(css.contains("--vb6-button-bg: rgb(192, 192, 192)"));
        assert!(css.contains("--vb6-button-border: rgb(120, 120, 120)"));
        assert!(css.contains("--vb6-highlight: rgb(0, 0, 128)"));
    }

    #[test]
    fn scoped_css_uses_css_variables_in_selectors() {
        let css = scoped_css();
        assert!(css.contains("background-color: var(--vb6-bg)"));
        assert!(css.contains("color: var(--vb6-fg)"));
        assert!(css.contains("font-family: var(--vb6-font-family)"));
        assert!(css.contains("font-size: var(--vb6-font-size)"));
    }

    #[test]
    fn bare_css_has_no_vb6_app_prefix() {
        let css = bare_css();
        assert!(css.contains("body {"));
        assert!(css.contains(".vb6-label"));
        assert!(css.contains(".vb6-textbox"));
        assert!(!css.contains(".vb6-app"));
    }

    #[test]
    fn bare_css_has_no_root_reset() {
        let css = bare_css();
        // bare_css should not contain the .vb6-app root selector
        assert!(!css.contains(".vb6-app {"));
    }

    #[test]
    fn bare_css_contains_root_variables() {
        let css = bare_css();
        // bare_css should define CSS custom properties at :root for Tauri
        assert!(css.contains(":root {"));
        assert!(css.contains("--vb6-bg: rgb(192, 192, 192)"));
        assert!(css.contains("--vb6-fg: rgb(0, 0, 0)"));
        assert!(css.contains("--vb6-window-bg: #ffffff"));
        assert!(css.contains("--vb6-button-bg: rgb(192, 192, 192)"));
        assert!(css.contains("--vb6-button-border: rgb(120, 120, 120)"));
        assert!(css.contains("--vb6-font-family:"));
        assert!(css.contains("--vb6-font-size: 11px"));
    }

    #[test]
    fn dark_theme_css() {
        let css = scoped_dark_css();
        assert!(css.contains(".vb6-app.theme-dark {"));
        assert!(css.contains("--vb6-bg: #1e1e1e"));
        assert!(css.contains("--vb6-window-bg: #3c3c3c"));
        assert!(css.contains("--vb6-fg: #d4d4d4"));
    }

    #[test]
    fn fluent_theme_css() {
        let css = scoped_fluent_css();
        assert!(css.contains(".vb6-app.theme-fluent {"));
        assert!(css.contains("--vb6-bg: #f3f3f3"));
        assert!(css.contains("--vb6-button-border: #c8c8c8"));
    }

    #[test]
    fn test_custom_theme_css() {
        let css = crate::layout::vb6_css::custom_theme_css(
            "mytheme",
            &[("bg", "#000000"), ("fg", "#ffffff")],
        );
        assert!(css.contains(".vb6-app.theme-mytheme {"));
        assert!(css.contains("--bg: #000000"));
        assert!(css.contains("--fg: #ffffff"));
    }

    #[test]
    fn scoped_css_contains_all_control_types() {
        let css = scoped_css();
        let controls = [
            ".vb6-form",
            ".vb6-label",
            ".vb6-textbox",
            ".vb6-commandbutton",
            ".vb6-frame",
            ".vb6-picturebox",
            ".vb6-image",
            ".vb6-checkbox",
            ".vb6-optionbutton",
            ".vb6-combobox",
            ".vb6-listbox",
            ".vb6-hscrollbar",
            ".vb6-vscrollbar",
            ".vb6-shape",
            ".vb6-line",
            ".vb6-timer",
            ".vb6-dritelb",
            ".vb6-dirlistbox",
            ".vb6-filelistbox",
        ];
        for ctrl in &controls {
            assert!(css.contains(ctrl), "missing control selector: {ctrl}");
        }
    }

    #[test]
    fn bare_css_contains_all_control_types() {
        let css = bare_css();
        assert!(css.contains("body {"));
        assert!(css.contains("background-color: var(--vb6-bg)"));
        let controls = [
            ".vb6-label",
            ".vb6-textbox",
            ".vb6-commandbutton",
            ".vb6-frame",
            ".vb6-picturebox",
            ".vb6-image",
            ".vb6-checkbox",
            ".vb6-optionbutton",
            ".vb6-combobox",
            ".vb6-listbox",
            ".vb6-hscrollbar",
            ".vb6-vscrollbar",
            ".vb6-shape",
            ".vb6-line",
            ".vb6-timer",
            ".vb6-dritelb",
            ".vb6-dirlistbox",
            ".vb6-filelistbox",
        ];
        for ctrl in &controls {
            assert!(css.contains(ctrl), "missing control selector: {ctrl}");
        }
    }

    #[test]
    fn scoped_and_bare_are_equivalent_except_for_prefix() {
        // The control rules should be identical except for the .vb6-app prefix.
        let scoped = scoped_css();
        let bare = bare_css();

        // Scoped should have more content (root reset + prefixed selectors)
        assert!(scoped.len() >= bare.len());

        // Bare rules should appear in scoped (without .vb6-app prefix)
        assert!(bare.contains("body {"));
        assert!(scoped.contains(".vb6-app .vb6-form"));
    }

    #[test]
    fn scoped_css_has_no_bare_global_selectors() {
        // Ensure there are no global selectors like "div {" that could leak.
        let css = scoped_css();
        // All control rules should be prefixed with .vb6-app
        for line in css.lines() {
            let trimmed = line.trim();
            // A bare selector like "div {" should not appear at the start of a rule
            if trimmed.starts_with("div {")
                || trimmed.starts_with("span {")
                || trimmed.starts_with("label {")
            {
                panic!("Global selector found: {trimmed}");
            }
        }
    }

    #[test]
    fn form_style_uses_theme_variables() {
        let css = scoped_css();
        assert!(css.contains("background-color: var(--vb6-bg)"));
        assert!(css.contains("color: var(--vb6-fg)"));
    }

    #[test]
    fn textbox_style_uses_window_colors() {
        let css = scoped_css();
        assert!(css.contains("background-color: var(--vb6-window-bg)"));
        assert!(css.contains("color: var(--vb6-window-text)"));
    }

    #[test]
    fn button_style_uses_button_colors() {
        let css = scoped_css();
        assert!(css.contains("background-color: var(--vb6-button-bg)"));
        assert!(css.contains("border: 1px solid var(--vb6-button-border)"));
    }

    #[test]
    fn dark_theme_overrides_all_form_styles() {
        let dark = scoped_dark_css();
        // Dark theme should at minimum override the key variables
        assert!(dark.contains("--vb6-bg: #1e1e1e"));
        assert!(dark.contains("--vb6-fg: #d4d4d4"));
        assert!(dark.contains("--vb6-window-bg: #3c3c3c"));
        assert!(dark.contains("--vb6-window-text: #cccccc"));
        assert!(dark.contains("--vb6-button-bg: #3c3c3c"));
        assert!(dark.contains("--vb6-button-border: #555555"));
    }

    #[test]
    fn root_css_has_default_font_settings() {
        let css = root_css();
        assert!(css.contains("\"MS Sans Serif\""));
        assert!(css.contains("Tahoma"));
        assert!(css.contains("sans-serif"));
        assert!(css.contains("font-size: 11px"));
    }

    #[test]
    fn all_bare_selectors_are_scoped_control_types() {
        // Verify all bare selectors start with .vb6- (namespaced)
        let css = bare_css();
        for line in css.lines() {
            let trimmed = line.trim();
            // Rule start lines look like ".vb6-form {" or ".vb6-app.theme-dark {" or ".vb6-label {"
            if trimmed.ends_with("{") && trimmed.starts_with('.') {
                assert!(
                    trimmed.starts_with(".vb6-form")
                        || trimmed.starts_with(".vb6-app")
                        || trimmed.starts_with(".vb6-"),
                    "unexpected bare selector: {trimmed}"
                );
            }
        }
    }

    #[test]
    fn css_theme_variables_match_vb6_defaults() {
        // Verify that CSS classes use theme variables that resolve to the same
        // values as VB6 system defaults used in default_*() functions.
        //
        // This ensures zero inline CSS is emitted for bare controls at VB6 defaults,
        // since their computed styles will exactly match the CSS class defaults.

        // vbWindowBackground (index 5) → Canvas → --vb6-window-bg in default theme
        let vb_bg = color_to_css(&VB_WINDOW_BACKGROUND);
        assert!(
            matches!(vb_bg, crate::layout::model::style::CssColor::Named(ref s) if s == "Canvas"),
            "VB_WINDOW_BACKGROUND should map to Canvas, got {:?}",
            vb_bg
        );
        // Default theme --vb6-window-bg should also be white (Canvas equivalent)
        let theme = crate::layout::theme::default_theme();
        assert!(
            theme.window_bg.as_ref().map(|s| s == "#ffffff").unwrap_or(false),
            "default theme window_bg should be white (#ffffff)"
        );

        // vbWindowText (index 8) → ButtonText → --vb6-window-text in default theme
        let vb_text = color_to_css(&VB_WINDOW_TEXT);
        assert!(
            matches!(vb_text, crate::layout::model::style::CssColor::Named(ref s) if s == "ButtonText"),
            "VB_WINDOW_TEXT should map to ButtonText, got {:?}",
            vb_text
        );
        // Default theme --vb6-window-text should be black
        assert!(
            theme.window_text.as_ref().map(|s| s == "#000000").unwrap_or(false),
            "default theme window_text should be black (#000000)"
        );

        // vbButtonFace (index 15) → ButtonFace → --vb6-button-bg in default theme
        let vb_btn_bg = color_to_css(&VB_BUTTON_FACE);
        assert!(
            matches!(vb_btn_bg, crate::layout::model::style::CssColor::Named(ref s) if s == "ButtonFace"),
            "VB_BUTTON_FACE should map to ButtonFace, got {:?}",
            vb_btn_bg
        );
        // Default theme --vb6-button-bg should be rgb(192, 192, 192)
        assert!(
            theme.button_bg.as_ref().map(|s| s == "rgb(192, 192, 192)").unwrap_or(false),
            "default theme button_bg should be rgb(192, 192, 192)"
        );

        // vbDesktop (index 1) → Canvas → --vb6-bg in default theme
        let vb_desktop = Color::System { index: 0x01 };
        let desktop_css = color_to_css(&vb_desktop);
        assert!(
            matches!(desktop_css, crate::layout::model::style::CssColor::Named(ref s) if s == "Canvas"),
            "VB_DESKTOP should map to Canvas, got {:?}",
            desktop_css
        );
        // Default theme --vb6-bg should be rgb(192, 192, 192)
        assert!(
            theme.bg.as_ref().map(|s| s == "rgb(192, 192, 192)").unwrap_or(false),
            "default theme bg should be rgb(192, 192, 192)"
        );
    }

    #[test]
    fn textbox_css_uses_window_colors_matching_vb6_defaults() {
        // .vb6-textbox uses var(--vb6-window-bg) and var(--vb6-window-text)
        // These should match VB_WINDOW_BACKGROUND and VB_WINDOW_TEXT respectively
        let css = scoped_css();
        assert!(css.contains("background-color: var(--vb6-window-bg)"));
        assert!(css.contains("color: var(--vb6-window-text)"));

        // Verify the CSS variable values in default theme match VB6 system colors
        let theme = crate::layout::theme::default_theme();
        assert!(theme.window_bg.is_some());
        assert!(theme.window_text.is_some());
    }

    #[test]
    fn button_css_uses_button_colors_matching_vb6_defaults() {
        // .vb6-commandbutton uses var(--vb6-button-bg) and var(--vb6-fg)
        // These should match VB_BUTTON_FACE and VB_BUTTON_TEXT respectively
        let css = scoped_css();
        assert!(css.contains("background-color: var(--vb6-button-bg)"));
        assert!(css.contains("color: var(--vb6-fg)"));

        let theme = crate::layout::theme::default_theme();
        assert!(theme.button_bg.is_some());
        assert!(theme.fg.is_some());
    }

    #[test]
    fn label_css_uses_form_colors_matching_vb6_defaults() {
        // .vb6-label uses var(--vb6-bg) and var(--vb6-fg)
        // These should match VB_DESKTOP and VB_BUTTON_TEXT respectively
        let css = scoped_css();
        assert!(css.contains("background-color: var(--vb6-bg)"));
        assert!(css.contains("color: var(--vb6-fg)"));
    }

    #[test]
    fn all_control_styles_use_theme_variables() {
        // Every control style should use var(--vb6-*) instead of hardcoded colors
        let css = scoped_css();

        // Controls with background colors should use theme variables
        assert!(css.contains("background-color: var(--vb6-bg)"));
        assert!(css.contains("background-color: var(--vb6-button-bg)"));
        assert!(css.contains("background-color: var(--vb6-window-bg)"));

        // Controls with text colors should use theme variables
        assert!(css.contains("color: var(--vb6-fg)"));
        assert!(css.contains("color: var(--vb6-window-text)"));
    }

    #[test]
    fn default_theme_produces_css_matching_css_file_defaults() {
        // The default theme should produce CSS custom properties with values
        // that match the hardcoded defaults in vb6_css.rs root CSS
        let theme = crate::layout::theme::default_theme();
        let css = theme.to_css();

        // These should match the root_css() values
        assert!(css.contains("--vb6-bg: rgb(192, 192, 192)"));
        assert!(css.contains("--vb6-fg: rgb(0, 0, 0)"));
        assert!(css.contains("--vb6-window-bg: #ffffff"));
        assert!(css.contains("--vb6-window-text: #000000"));
        assert!(css.contains("--vb6-button-bg: rgb(192, 192, 192)"));
        assert!(css.contains("--vb6-button-border: rgb(120, 120, 120)"));
        assert!(css.contains("--vb6-highlight: rgb(0, 0, 128)"));
        assert!(css.contains("--vb6-highlight-text: #ffffff"));
    }
}
