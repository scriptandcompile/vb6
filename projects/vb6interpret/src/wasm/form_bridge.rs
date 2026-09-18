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

use crate::project::LoadedForm;

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

    // Compute event bindings from the form
    let loaded_form = LoadedForm {
        name: form_file.attributes.name.clone(),
        file_name: "form.frm".to_string(),
        parsed: form_file.clone(),
        raw_bytes: form_bytes.to_vec(),
    };
    let bindings = loaded_form.event_bindings();
    let event_procedures: Vec<_> = bindings
        .into_iter()
        .map(|((control, event), procedure)| (control, event, procedure))
        .collect();

    let config = LayoutConfig::default();
    let handle = layout::load_form(&form_file.form, event_procedures, &config);

    let doc = window()
        .ok_or("no window")?
        .document()
        .ok_or("no document")?;

    let css_injector = layout::renderer::WebSysRenderer::new(doc.clone());
    css_injector.inject_css(&layout::vb6_css::scoped_css());

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

/// Load multiple forms from raw `.frm` file bytes and render each one into
/// `#vb6-container`.
///
/// Each `(name, bytes)` pair is parsed as a VB6 form file.  All loaded
/// forms are rendered in sequence inside the container, producing a stack
/// of overlapping form DOM trees.
///
/// Returns a vector of handles — one per successfully loaded form — in the
/// same order as the input.  Any form that fails to parse halts and returns
/// an error for the first failing entry.
#[wasm_bindgen]
pub fn show_project_forms(form_files: Vec<(String, Vec<u8>)>) -> Result<Vec<u32>, JsError> {
    let config = LayoutConfig::default();

    let doc = window()
        .ok_or("no window")?
        .document()
        .ok_or("no document")?;

    let css_injector = layout::renderer::WebSysRenderer::new(doc.clone());
    css_injector.inject_css(&layout::vb6_css::scoped_css());

    let container = doc
        .get_element_by_id("vb6-container")
        .ok_or("#vb6-container element not found")?;
    container.set_inner_html("");

    let renderer = layout::renderer::WebSysRenderer::new(doc);
    let mut handles = Vec::with_capacity(form_files.len());

    for (file_name, bytes) in form_files {
        let source_file = vb6parse::io::SourceFile::decode_with_replacement(&file_name, &bytes)
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

        // Compute event bindings for this form
        let loaded_form = LoadedForm {
            name: form_file.attributes.name.clone(),
            file_name: file_name.clone(),
            parsed: form_file.clone(),
            raw_bytes: bytes.clone(),
        };
        let bindings = loaded_form.event_bindings();
        let event_procedures: Vec<_> = bindings
            .into_iter()
            .map(|((control, event), procedure)| (control, event, procedure))
            .collect();

        let handle = layout::load_form(&form_file.form, event_procedures, &config);
        let model = layout::get_form(handle, |f| f.root_node.clone())
            .ok_or("form not found after loading")?;
        let dom_root = renderer.render_node(&model);
        container
            .append_child(&dom_root)
            .map_err(|_| "failed to append form to container")?;

        handles.push(handle);
    }

    Ok(handles)
}
