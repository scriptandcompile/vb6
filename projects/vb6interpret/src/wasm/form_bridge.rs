//! WASM bridge for VB6 form rendering.
//!
//! This module exposes form-loading, rendering, and DOM manipulation functions
//! to the browser via `wasm_bindgen`. It sits alongside the existing module
//! execution bridge ([`super::run_bridge`]) and environment state bindings
//! ([`super::state_bridge`]).
//!
//! # Public API
//!
//! - [`show_form`] — Parse form bytes, load into the layout engine, and render
//!   the DOM tree into `#vb6-container`.
//! - [`hide_form`] — Hide a loaded form by handle.
//! - [`show_form_by_handle`] — Show (unhide) a previously hidden form by handle.
//! - [`unload_form`] — Remove a form from the store and clear the DOM container.
//! - [`update_form`] — Re-render a loaded form after state changes.

use wasm_bindgen::prelude::*;
use web_sys::window;

use vb6runtime::layout::{self, LayoutConfig};

/// Parse form bytes (as produced by a VB6 `.frm` file), load into the layout
/// engine, and render the DOM tree into `#vb6-container`.
///
/// Returns a handle that can be used with [`hide_form`], [`show_form_by_handle`],
/// [`unload_form`], and [`update_form`].
///
/// # Errors
///
/// Returns a `JsValue` error string if:
/// - The bytes cannot be parsed as a VB6 Form file.
/// - The `#vb6-container` element does not exist in the DOM.
/// - The form cannot be retrieved from the layout store after loading.
#[wasm_bindgen]
pub fn show_form(form_bytes: &[u8]) -> Result<JsValue, JsError> {
    let source_file = vb6parse::io::SourceFile::decode_with_replacement("form.frm", form_bytes)
        .map_err(|e| JsError::new(&e.kind.to_string()))?;
    let form_file = vb6parse::FormFile::parse(&source_file)
        .map(|r| r.ok().unwrap())
        .map_err(|e| {
            let message = e
                .first()
                .map(|em| em.kind.to_string())
                .unwrap_or_else(|| "Failed to parse form file".to_string());
            JsError::new(&message)
        })?;

    let config = LayoutConfig::default();
    let handle = layout::load_form(&form_file.form, &config);

    let doc = window()
        .ok_or("no window")?
        .document()
        .ok_or("no document")?;

    let container = doc
        .get_element_by_id("vb6-container")
        .ok_or("#vb6-container element not found")?;
    container.set_inner_html("");

    let renderer = layout::renderer::WebSysRenderer::new(doc);
    let model =
        layout::get_form(handle, |f| f.root_node.clone()).ok_or("form not found after loading")?;
    let dom_root = renderer.render_node(&model);
    container
        .append_child(&dom_root)
        .map_err(|_| "failed to append form to container")?;

    Ok(handle.into())
}

/// Hide a loaded form by handle.
///
/// Sets the form's visibility to false and re-renders the DOM.
#[wasm_bindgen]
pub fn hide_form(handle: u32) -> Result<(), JsError> {
    layout::get_form_mut(handle, |form| {
        form.visible = false;
    })
    .ok_or("unknown form handle")?;

    update_form(handle)
}

/// Show (unhide) a previously hidden form by handle.
///
/// Sets the form's visibility to true and re-renders the DOM.
#[wasm_bindgen]
pub fn show_form_by_handle(handle: u32) -> Result<(), JsError> {
    layout::get_form_mut(handle, |form| {
        form.visible = true;
    })
    .ok_or("unknown form handle")?;

    update_form(handle)
}

/// Remove a form from the layout store and clear the DOM container.
///
/// After calling this, the handle is no longer valid.
#[wasm_bindgen]
pub fn unload_form(handle: u32) -> Result<(), JsError> {
    layout::form_store::remove(handle);

    let doc = window()
        .ok_or("no window")?
        .document()
        .ok_or("no document")?;

    let container = doc
        .get_element_by_id("vb6-container")
        .ok_or("#vb6-container element not found")?;
    container.set_inner_html("");

    Ok(())
}

/// Re-render a loaded form after state changes.
///
/// Clears the DOM container and re-renders the form from the current
/// layout model state. This should be called after mutating form properties
/// via [`layout::get_form_mut`].
#[wasm_bindgen]
pub fn update_form(handle: u32) -> Result<(), JsError> {
    let doc = window()
        .ok_or("no window")?
        .document()
        .ok_or("no document")?;

    let container = doc
        .get_element_by_id("vb6-container")
        .ok_or("#vb6-container element not found")?;

    let model = layout::get_form(handle, |f| f.root_node.clone()).ok_or("unknown form handle")?;

    container.set_inner_html("");
    let renderer = layout::renderer::WebSysRenderer::new(doc);
    let dom_root = renderer.render_node(&model);
    container
        .append_child(&dom_root)
        .map_err(|_| "failed to append form to container")?;

    Ok(())
}
