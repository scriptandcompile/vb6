//! Tauri command handlers for form rendering.
//!
//! These commands are available when the `tauri` feature is enabled.
//! They wire the VB6 form parser, layout engine, and Tauri renderer
//! into Tauri IPC entry points.

#[cfg(feature = "tauri")]
use tauri::command;

#[cfg(feature = "tauri")]
use vb6parse::io::SourceFile;

#[cfg(feature = "tauri")]
use vb6runtime::layout::{self, LayoutConfig};

/// Tauri command: load a VB6 form from raw bytes.
///
/// Parses the form file data, converts it into the layout model,
/// and returns a [`FormHandle`][vb6runtime::layout::FormHandle] that can
/// be used to update the rendered output.
///
/// The form bytes are decoded using Windows-1252 encoding (with
/// replacement for invalid characters) to match VB6's native encoding.
#[cfg(feature = "tauri")]
#[command]
pub fn load_form(form_data: Vec<u8>) -> u32 {
    let source_file =
        SourceFile::decode_with_replacement("form.frm", &form_data).expect("failed to decode form");
    let form_file = vb6parse::FormFile::parse(&source_file).unwrap_or_fail();
    layout::load_form(&form_file.form, &LayoutConfig::default())
}

/// Tauri command: render a form to an HTML fragment string.
///
/// Uses the [`TauriRenderer`][vb6runtime::layout::renderer::TauriRenderer]
/// to produce a self-contained HTML string from the layout model.
/// This is called whenever the form's state changes and the webview
/// needs to reflect the update.
#[cfg(feature = "tauri")]
#[command]
pub fn update_form(handle: u32) -> String {
    let html = layout::render(handle, &layout::renderer::TauriRenderer::new(false));
    format!("<div id='root'>{}</div>", html)
}
