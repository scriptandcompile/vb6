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
//!   the DOM tree into the container identified by `container_id`.
//! - [`get_form_procedures`] — Get event procedure bindings for a loaded form.
//! - [`hide_form`] — Hide a loaded form by handle.
//! - [`show_form_by_handle`] — Show (unhide) a previously hidden form by handle.
//! - [`unload_form`] — Remove a form from the store and clear the DOM container.
//! - [`update_form`] — Re-render a loaded form after state changes.

use wasm_bindgen::prelude::*;
use web_sys::window;

use vb6runtime::layout::renderer::Renderer;
use vb6runtime::layout::theme::CssInjector;
use vb6runtime::layout::{self, LayoutConfig};

use crate::project::LoadedForm;
use serde_wasm_bindgen::to_value;

const DEFAULT_CONTAINER_ID: &str = "vb6-container";

/// Format a [`NodeId`] as a string.
fn node_id_to_string(node_id: &vb6runtime::layout::model::NodeId) -> String {
    format!("{}-{:?}-{}", node_id.name, node_id.kind, node_id.index)
}

/// Parse form bytes (as produced by a VB6 `.frm` file), load into the layout
/// engine, and render the DOM tree into the container identified by `container_id`.
///
/// Returns a handle that can be used with [`hide_form`], [`show_form_by_handle`],
/// [`unload_form`], and [`update_form`].
///
/// # Arguments
///
/// * `form_bytes` — Raw bytes of a VB6 `.frm` file.
/// * `container_id` — The `id` attribute of the DOM element to render into.
///   Defaults to `"vb6-container"` if empty.
///
/// # Errors
///
/// Returns a `JsValue` error string if:
/// - The bytes cannot be parsed as a VB6 Form file.
/// - The container element does not exist in the DOM.
/// - The form cannot be retrieved from the layout store after loading.
#[wasm_bindgen]
pub fn show_form(form_bytes: &[u8], container_id: &str) -> Result<JsValue, JsError> {
    let container_id = container_id.trim();
    let container_id: &str = if container_id.is_empty() {
        DEFAULT_CONTAINER_ID
    } else {
        container_id
    };

    let source_file = vb6parse::io::SourceFile::decode_with_replacement("form.frm", form_bytes)
        .map_err(|e| JsError::new(&e.kind.to_string()))?;
    let form_file = vb6parse::FormFile::parse(&source_file)
        .ok_or_errors()
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

    let win = window().ok_or_else(|| JsError::new("no window"))?;
    let doc = win.document().ok_or_else(|| JsError::new("no document"))?;

    let css_injector = layout::renderer::WebSysRenderer::new(doc.clone());
    css_injector.inject_css(&layout::vb6_css::scoped_css());

    let container = doc
        .get_element_by_id(container_id)
        .ok_or_else(|| JsError::new(&format!("#{container_id} element not found")))?;
    container.set_inner_html("");

    let renderer = layout::renderer::WebSysRenderer::new(doc);
    let model = layout::get_form(handle, |f| f.root_node.clone())
        .ok_or_else(|| JsError::new("form not found after loading"))?;
    let dom_root = renderer.render_node(&model);
    container
        .append_child(&dom_root)
        .map_err(|_| JsError::new("failed to append form to container"))?;

    Ok(handle.into())
}

/// Get event procedure bindings for a loaded form.
///
/// Returns a JSON array of bindings in the same format as the Tauri
/// `form_event_bindings` command. Each binding contains:
/// - `node_id`: The unique node ID of the control in the layout tree.
/// - `control`: The VB6 control name (e.g. `"cmdOK"`).
/// - `event`: The VB6 event name (e.g. `"Click"`).
/// - `procedure`: The full procedure name (e.g. `"cmdOK_Click"`).
///
/// # Arguments
///
/// * `form_handle` — The handle returned by [`show_form`].
///
/// # Errors
///
/// Returns a `JsValue` error if the form handle is unknown.
#[wasm_bindgen]
pub fn get_form_procedures(form_handle: u32) -> Result<JsValue, JsError> {
    layout::get_form(form_handle, |form| {
        let bindings: Vec<_> = form
            .event_procedures
            .iter()
            .map(|p| {
                serde_json::json!({
                    "node_id": node_id_to_string(&p.node_id),
                    "control": p.control,
                    "event": p.event,
                    "procedure": p.procedure,
                })
            })
            .collect();
        let js_val = to_value(&bindings)
            .map_err(|_| JsError::new("failed to serialize event procedures"))?;
        Ok(js_val)
    })
    .ok_or_else(|| JsError::new("unknown form handle"))
    .and_then(|r| r)
}

/// Hide a loaded form by handle.
///
/// Sets the form's visibility to false and re-renders the DOM.
/// Uses the same container as the original [`show_form`] call.
///
/// # Note
///
/// This function uses the default container `"vb6-container"` for
/// backward compatibility. For multi-container support, use
/// [`hide_form_with_container`].
#[wasm_bindgen]
pub fn hide_form(handle: u32) -> Result<(), JsError> {
    hide_form_with_container(handle, DEFAULT_CONTAINER_ID)
}

/// Hide a loaded form by handle, targeting a specific container.
///
/// Sets the form's visibility to false and re-renders the DOM in the
/// container identified by `container_id`.
#[wasm_bindgen]
pub fn hide_form_with_container(handle: u32, container_id: &str) -> Result<(), JsError> {
    layout::get_form_mut(handle, |form| {
        form.visible = false;
    })
    .ok_or_else(|| JsError::new("unknown form handle"))?;

    update_form_with_container(handle, container_id)
}

/// Show (unhide) a previously hidden form by handle.
///
/// Sets the form's visibility to true and re-renders the DOM.
/// Uses the same container as the original [`show_form`] call.
#[wasm_bindgen]
pub fn show_form_by_handle(handle: u32) -> Result<(), JsError> {
    show_form_by_handle_with_container(handle, DEFAULT_CONTAINER_ID)
}

/// Show (unhide) a previously hidden form by handle, targeting a specific container.
///
/// Sets the form's visibility to true and re-renders the DOM in the
/// container identified by `container_id`.
#[wasm_bindgen]
pub fn show_form_by_handle_with_container(handle: u32, container_id: &str) -> Result<(), JsError> {
    layout::get_form_mut(handle, |form| {
        form.visible = true;
    })
    .ok_or_else(|| JsError::new("unknown form handle"))?;

    update_form_with_container(handle, container_id)
}

/// Remove a form from the layout store and clear the DOM container.
///
/// After calling this, the handle is no longer valid.
///
/// # Arguments
///
/// * `handle` — The form handle returned by [`show_form`].
/// * `container_id` — The container to clear. Defaults to `"vb6-container"` if empty.
#[wasm_bindgen]
pub fn unload_form(handle: u32, container_id: &str) -> Result<(), JsError> {
    layout::form_store::remove(handle);

    let container_id = container_id.trim();
    let container_id: &str = if container_id.is_empty() {
        DEFAULT_CONTAINER_ID
    } else {
        container_id
    };

    let win = window().ok_or_else(|| JsError::new("no window"))?;
    let doc = win.document().ok_or_else(|| JsError::new("no document"))?;

    let container = doc
        .get_element_by_id(container_id)
        .ok_or_else(|| JsError::new(&format!("#{container_id} element not found")))?;
    container.set_inner_html("");

    Ok(())
}

/// Re-render a loaded form after state changes.
///
/// Clears the DOM container and re-renders the form from the current
/// layout model state. This should be called after mutating form properties
/// via [`layout::get_form_mut`].
///
/// Uses the same container as the original [`show_form`] call.
#[wasm_bindgen]
pub fn update_form(handle: u32) -> Result<(), JsError> {
    update_form_with_container(handle, DEFAULT_CONTAINER_ID)
}

/// Re-render a loaded form after state changes, targeting a specific container.
///
/// Clears the DOM container identified by `container_id` and re-renders
/// the form from the current layout model state.
#[wasm_bindgen]
pub fn update_form_with_container(handle: u32, container_id: &str) -> Result<(), JsError> {
    let win = window().ok_or_else(|| JsError::new("no window"))?;
    let doc = win.document().ok_or_else(|| JsError::new("no document"))?;

    let container = doc
        .get_element_by_id(container_id)
        .ok_or_else(|| JsError::new(&format!("#{container_id} element not found")))?;

    let model = layout::get_form(handle, |f| f.root_node.clone())
        .ok_or_else(|| JsError::new("unknown form handle"))?;

    container.set_inner_html("");
    let renderer = layout::renderer::WebSysRenderer::new(doc);
    let dom_root = renderer.render_node(&model);
    container
        .append_child(&dom_root)
        .map_err(|_| JsError::new("failed to append form to container"))?;

    Ok(())
}

/// Internal helper: render a single form into a container element.
///
/// Kept for future use — will be wired into a full form rendering pipeline
/// that supports loading forms from raw `.frm` bytes with a configurable
/// layout configuration and renderer instance.
#[allow(dead_code)]
fn render_form_into_container(
    bytes: &[u8],
    file_name: &str,
    config: &LayoutConfig,
    container: &web_sys::Element,
    renderer: &layout::renderer::WebSysRenderer,
) -> Result<u32, JsError> {
    let source_file = vb6parse::io::SourceFile::decode_with_replacement(file_name, bytes)
        .map_err(|e| JsError::new(&e.kind.to_string()))?;
    let form_file = vb6parse::FormFile::parse(&source_file)
        .ok_or_errors()
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
        file_name: file_name.to_string(),
        parsed: form_file.clone(),
        raw_bytes: bytes.to_vec(),
    };
    let bindings = loaded_form.event_bindings();
    let event_procedures: Vec<_> = bindings
        .into_iter()
        .map(|((control, event), procedure)| (control, event, procedure))
        .collect();

    let handle = layout::load_form(&form_file.form, event_procedures, config);
    let model = layout::get_form(handle, |f| f.root_node.clone())
        .ok_or_else(|| JsError::new("form not found after loading"))?;
    let dom_root = renderer.render_node(&model);
    container
        .append_child(&dom_root)
        .map_err(|_| JsError::new("failed to append form to container"))?;

    Ok(handle)
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
///
/// # Note
///
/// This function uses the default container `"vb6-container"`. For
/// multi-container support, use [`show_project_forms_with_container`].
///
/// Kept for future use — will be wired into a full project loading pipeline
/// that handles multiple forms from a `.vbp` project.
#[allow(dead_code)]
pub fn show_project_forms(form_files: Vec<(String, Vec<u8>)>) -> Result<Vec<u32>, JsError> {
    show_project_forms_with_container(form_files, DEFAULT_CONTAINER_ID)
}

/// Load multiple forms from raw `.frm` file bytes and render each one into
/// a container identified by `container_id`.
///
/// Each `(name, bytes)` pair is parsed as a VB6 form file.  All loaded
/// forms are rendered in sequence inside the container, producing a stack
/// of overlapping form DOM trees.
///
/// Returns a vector of handles — one per successfully loaded form — in the
/// same order as the input.  Any form that fails to parse halts and returns
/// an error for the first failing entry.
///
/// Kept for future use — will be wired into a full project loading pipeline
/// that handles multiple forms from a `.vbp` project with per-window containers.
#[allow(dead_code)]
pub fn show_project_forms_with_container(
    form_files: Vec<(String, Vec<u8>)>,
    container_id: &str,
) -> Result<Vec<u32>, JsError> {
    let config = LayoutConfig::default();

    let win = window().ok_or_else(|| JsError::new("no window"))?;
    let doc = win.document().ok_or_else(|| JsError::new("no document"))?;

    let css_injector = layout::renderer::WebSysRenderer::new(doc.clone());
    css_injector.inject_css(&layout::vb6_css::scoped_css());

    let container = doc
        .get_element_by_id(container_id)
        .ok_or_else(|| JsError::new(&format!("#{container_id} element not found")))?;
    container.set_inner_html("");

    let renderer = layout::renderer::WebSysRenderer::new(doc);
    let mut handles = Vec::with_capacity(form_files.len());

    for (file_name, bytes) in form_files {
        let handle =
            render_form_into_container(&bytes, &file_name, &config, &container, &renderer)?;
        handles.push(handle);
    }

    Ok(handles)
}
