//! Tauri command handlers for form rendering and engine management.
//!
//! These commands are available when the `tauri` feature is enabled.
//! They wire the VB6 form parser, layout engine, Tauri renderer, and
//! the background [`TauriEngine`][crate::TauriEngine] into Tauri IPC
//! entry points.
//!
//! # Engine Store
//!
//! The [`ENGINE_STORE`] is a process-global map of engine handles to
//! [`TauriEngine`] instances. Handles are incrementing u32 values
//! assigned on each call to `spawn_engine`. Tauri commands receive a
//! handle, look up the engine, and dispatch operations.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use tauri::command;
use vb6parse::io::SourceFile;
use vb6runtime::VBVariant;
use vb6runtime::layout::{self, LayoutConfig};

use crate::tauri_engine::{TauriCommand, TauriEngine};

/// Handle type for referencing a spawned engine in the store.
///
/// Handles are incrementing u32 values starting from 0.
/// They remain valid for the lifetime of the process.
pub type EngineHandle = u32;

/// The global engine store.
///
/// Initialized lazily on first use to avoid unnecessary static allocation.
static ENGINE_STORE: LazyLock<Mutex<HashMap<EngineHandle, TauriEngine>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Next engine handle to assign.
static NEXT_ENGINE_HANDLE: LazyLock<Mutex<EngineHandle>> = LazyLock::new(|| Mutex::new(0));

// ---------------------------------------------------------------------------
// Engine management helpers (used by Tauri commands)
// ---------------------------------------------------------------------------

/// Spawn a new [`TauriEngine`] for the given project and store it in the
/// global engine store.
///
/// Returns the handle assigned to the new engine.
pub fn spawn_engine(project: crate::project::LoadedProject) -> EngineHandle {
    let (engine, _resp_rx) = TauriEngine::spawn(project);
    let mut handle = NEXT_ENGINE_HANDLE.lock().unwrap();
    let current = *handle;
    *handle += 1;
    drop(handle);

    ENGINE_STORE.lock().unwrap().insert(current, engine);
    current
}

/// Retrieve a clone of the engine from the store by handle.
pub(crate) fn get_engine(handle: EngineHandle) -> Option<TauriEngine> {
    ENGINE_STORE.lock().unwrap().get(&handle).cloned()
}

/// Remove an engine from the store by handle, stopping it first.
pub(crate) fn remove_engine(handle: EngineHandle) -> Option<TauriEngine> {
    ENGINE_STORE.lock().unwrap().remove(&handle)
}

// ---------------------------------------------------------------------------
// Tauri IPC commands
// ---------------------------------------------------------------------------

/// Tauri command: load a VB6 form from raw bytes.
///
/// Parses the form file data, converts it into the layout model,
/// and returns a [`FormHandle`][vb6runtime::layout::FormHandle] that can
/// be used to update the rendered output.
///
/// The form bytes are decoded using Windows-1252 encoding (with
/// replacement for invalid characters) to match VB6's native encoding.
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
#[command]
pub fn update_form(handle: u32) -> String {
    let html = layout::render(handle, &layout::renderer::TauriRenderer::new(false));
    format!("<div id='root'>{}</div>", html)
}

/// Tauri command: start running the project associated with the given
/// engine handle.
///
/// Sends a [`RunProject`][TauriCommand::RunProject] command to the
/// background thread. The frontend should poll the response channel
/// for [`Running`][crate::TauriResponse::Running],
/// [`Finished`][crate::TauriResponse::Finished], or
/// [`Error`][crate::TauriResponse::Error] responses.
#[command]
pub fn run_project(engine_handle: EngineHandle) -> bool {
    if let Some(engine) = get_engine(engine_handle) {
        engine.cmd_tx.send(TauriCommand::RunProject).is_ok()
    } else {
        false
    }
}

/// Tauri command: stop the engine identified by the given handle.
///
/// Sends a [`Stop`][TauriCommand::Stop] command to the background
/// thread and removes the engine from the global store.
#[command]
pub fn stop_engine(engine_handle: EngineHandle) -> bool {
    if let Some(engine) = remove_engine(engine_handle) {
        engine.stop();
        true
    } else {
        false
    }
}

/// Tauri command: dispatch a form control event to the engine's interpreter.
///
/// Constructs a procedure name from `{control}_{event}` (e.g.
/// `cmdOK_Click`) and calls it as a sub procedure on the interpreter.
/// This simulates a user interaction with a rendered form control.
#[command]
pub fn form_event(engine_handle: EngineHandle, control: String, event: String) {
    if let Some(engine) = get_engine(engine_handle) {
        let proc_name = format!("{}_{}", control, event);
        let mut interp = engine.interpreter.lock().unwrap();
        let _ = interp.call_sub(&proc_name, vec![]);
    }
}

/// Tauri command: get the current output text from the engine's interpreter.
///
/// Returns the concatenated `Debug.Print` / `Print` output captured so
/// far. The frontend may call this periodically to poll for new output.
#[command]
pub fn get_output(engine_handle: EngineHandle) -> String {
    if let Some(engine) = get_engine(engine_handle) {
        let interp = engine.interpreter.lock().unwrap();
        interp.output_text()
    } else {
        String::new()
    }
}

/// Tauri command: set a global variable in the engine's interpreter.
#[command]
pub fn set_variable(engine_handle: EngineHandle, name: String, value: String) -> bool {
    if let Some(engine) = get_engine(engine_handle) {
        let mut interp = engine.interpreter.lock().unwrap();
        interp.set_global(&name, VBVariant::String(value));
        true
    } else {
        false
    }
}

/// Tauri command: get the value of a global variable from the engine's interpreter.
#[command]
pub fn get_variable(engine_handle: EngineHandle, name: String) -> Option<String> {
    if let Some(engine) = get_engine(engine_handle) {
        let interp = engine.interpreter.lock().unwrap();
        if let Some(value) = interp.global(&name) {
            return Some(variant_to_string(value));
        }
    }
    None
}

/// A single form event binding returned to the frontend.
///
/// Represents one control event handler mapping: a `{control, event}` pair
/// pointing to a procedure name in the VB6 code.
#[derive(Clone, serde::Serialize, Debug)]
pub struct FormEventBinding {
    /// The VB6 control name (e.g. "cmdOK").
    pub control: String,
    /// The event name (e.g. "Click", "Change").
    pub event: String,
    /// The full procedure name (e.g. "cmdOK_Click").
    pub procedure: String,
}

/// Tauri command: get all event bindings for a form in the given engine.
///
/// Returns a list of `{control, event, procedure}` tuples that the frontend
/// can use to attach DOM event listeners to rendered controls. The bindings
/// are extracted from the loaded form's parsed structure using the
/// `LoadedForm::event_bindings()` method.
#[command]
pub fn form_event_bindings(engine_handle: EngineHandle, form_name: String) -> Vec<FormEventBinding> {
    if let Some(engine) = get_engine(engine_handle) {
        let project = engine.project();
        for loaded_form in &project.forms {
            if loaded_form.name == form_name {
                let bindings = loaded_form.event_bindings();
                return bindings
                    .into_iter()
                    .map(|((control, event), procedure)| FormEventBinding {
                        control,
                        event,
                        procedure,
                    })
                    .collect();
            }
        }
    }
    Vec::new()
}

// ---------------------------------------------------------------------------
// Variant serialization helper
// ---------------------------------------------------------------------------

fn variant_to_string(value: &VBVariant) -> String {
    match value {
        VBVariant::String(s) => s.clone(),
        VBVariant::Double(d) => d.to_string(),
        VBVariant::Boolean(b) => b.to_string(),
        VBVariant::Integer(i) => i.to_string(),
        VBVariant::Long(l) => l.to_string(),
        VBVariant::Byte(b) => b.to_string(),
        VBVariant::Single(s) => s.to_string(),
        VBVariant::Currency(c) => c.to_string(),
        VBVariant::Date(d) => d.to_string(),
        VBVariant::Empty | VBVariant::Null | VBVariant::Nothing => String::new(),
        VBVariant::Error(e) => format!("Error {}: {}", e.number, e.description),
        VBVariant::Object(_) => "[Object]".to_string(),
        VBVariant::Array(_) => "[Array]".to_string(),
    }
}
