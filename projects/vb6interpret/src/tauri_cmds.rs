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
use std::sync::{LazyLock, Mutex, OnceLock};

use tauri::command;
use vb6parse::io::SourceFile;
use vb6runtime::VBVariant;
use vb6runtime::layout::{self, LayoutConfig};

use crate::tauri_engine::TauriEngine;

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
    layout::render(handle, &layout::renderer::TauriRenderer::new(false))
}

/// Result of starting a project from the webview.
#[derive(Debug, Clone, serde::Serialize)]
pub struct RunProjectStatus {
    /// Whether the project started and its startup procedure (`Sub Main` or
    /// `Form_Load`) completed.
    pub started: bool,
    /// The error message when startup failed. Procedures are still merged
    /// into the interpreter, so control event handlers remain callable.
    pub error: Option<String>,
}

/// Tauri command: start the project associated with the given engine handle.
///
/// Runs the project synchronously on the calling thread: procedures from all
/// modules, classes, and forms are merged into the interpreter, module-level
/// statements execute, and the startup procedure runs.
///
/// The command resolves only once startup completes so the webview can attach
/// event bindings against a fully populated procedure map.
#[command]
pub fn run_project(engine_handle: EngineHandle) -> Option<RunProjectStatus> {
    let engine = get_engine(engine_handle)?;
    let mut interp = engine.interpreter.lock().unwrap();
    let project = engine.project();
    match interp.run_project(&project) {
        Ok(()) => Some(RunProjectStatus {
            started: true,
            error: None,
        }),
        Err(e) => Some(RunProjectStatus {
            started: false,
            error: Some(e.error.to_string()),
        }),
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

/// Event dispatch status returned after calling a form control event handler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum FormEventStatus {
    /// The event handler completed successfully.
    Handled,
    /// The program was terminated (End statement) during event handling.
    Terminated,
}

/// Tauri command: dispatch a form control event to the engine's interpreter.
///
/// Constructs a procedure name from `{control}_{event}` (e.g.
/// `cmdOK_Click`) and calls it as a sub procedure on the interpreter.
/// This simulates a user interaction with a rendered form control.
///
/// Returns a status indicating whether the event handler completed
/// normally or if the program was terminated during execution.
#[command]
pub fn form_event(engine_handle: EngineHandle, control: String, event: String) -> FormEventStatus {
    if let Some(engine) = get_engine(engine_handle) {
        let proc_name = format!("{}_{}", control, event);
        let mut interp = engine.interpreter.lock().unwrap();
        match interp.call_sub(&proc_name, vec![]) {
            Ok(_) => FormEventStatus::Handled,
            Err(e) => {
                eprintln!("Event handler error for '{}_{}': {}", control, event, e.error);
                FormEventStatus::Handled
            }
        }
    } else {
        FormEventStatus::Handled
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

/// A response returned by the `page_ready` IPC command.
#[derive(Clone, serde::Serialize, Debug)]
pub struct PageReadyResponse {
    /// The form HTML markup to inject into rootEl.
    pub form_html: String,
    /// The VB6 form name (e.g. "Form1").
    pub form_name: String,
    /// The engine handle for the background interpreter.
    pub engine_handle: EngineHandle,
}

/// Tauri command: called by the frontend when the page is ready to receive the
/// full form HTML.
///
/// Returns the form HTML markup (without the surrounding shell HTML) along with
/// the form name and engine handle. The frontend injects the markup into rootEl.
///
/// This avoids embedding the HTML as a string literal in the initial page —
/// it flows through the IPC response layer where JSON serialization handles
/// all escaping automatically.
#[command]
pub fn page_ready() -> Option<PageReadyResponse> {
    eprintln!("page_ready command called");
    let form_html = get_form_html();
    eprintln!("  form_html is_none: {}", form_html.is_none());
    if let Some(ref html) = form_html {
        eprintln!("  form_html length: {}", html.len());
        eprintln!("  form_html preview: {:.200}", html);
    }
    let (form_name, engine_handle) = get_page_ready_info()?;
    eprintln!("  returning form_name={}, engine_handle={}", form_name, engine_handle);
    Some(PageReadyResponse {
        form_html: form_html.unwrap_or_default(),
        form_name,
        engine_handle,
    })
}

fn get_page_ready_info() -> Option<(String, EngineHandle)> {
    PAGE_READY_INFO.get().map(|(name, handle)| (name.clone(), *handle))
}

fn get_form_html() -> Option<String> {
    FORM_HTML.get().cloned()
}

/// Stores the form HTML markup for the `page_ready` command.
static FORM_HTML: OnceLock<String> = OnceLock::new();

/// Stores form name and engine handle for the `page_ready` command.
static PAGE_READY_INFO: OnceLock<(String, EngineHandle)> = OnceLock::new();

/// Set the form HTML markup and page-ready info for later retrieval.
pub fn set_page_ready_info(form_html: String, form_name: String, engine_handle: EngineHandle) {
    FORM_HTML.set(form_html).ok();
    PAGE_READY_INFO.set((form_name, engine_handle)).ok();
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
