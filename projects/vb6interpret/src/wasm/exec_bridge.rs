//! Handle-based interpreter state for form-mode execution.
//!
//! While module-mode code uses one-shot functions that create a new interpreter
//! per call, form-mode needs a persistent interpreter session identified by a
//! handle.  This module provides that handle-based store and the
//! `run_project()` / `call_sub()` / `get_output()` / `dispose_state()` WASM
//! entry points.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use super::project_parser::{
    detect_startup, js_map_to_byte_pairs, parse_classes, parse_forms, parse_modules,
};
use super::{WasmRunOutput, build_debug_state, convert_run_error};
use crate::Interpreter;
use crate::interpreter::Flow;
use crate::project::LoadedProject;
use vb6runtime::state::clock as clock_state;
use vb6runtime::state::file as file_state;

/// Persistent interpreter session for form-based execution.
struct WasmRunState {
    interpreter: Interpreter,
}

static RUN_STATE: LazyLock<Mutex<HashMap<u32, WasmRunState>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static NEXT_STATE_HANDLE: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

/// Run a project (forms + modules + classes) and return a state handle.
///
/// The handle can be used with [`call_sub`] and [`get_output`] to interact
/// with the running interpreter session. Returns a handle that JS must
/// pass to subsequent calls.
#[wasm_bindgen]
pub fn run_project(
    form_bytes: JsValue,
    module_bytes: JsValue,
    class_bytes: JsValue,
    startup: &str,
) -> Result<JsValue, JsError> {
    let forms = parse_forms(js_map_to_byte_pairs(&form_bytes)?)?;
    let modules = parse_modules(js_map_to_byte_pairs(&module_bytes)?)?;
    let classes = parse_classes(js_map_to_byte_pairs(&class_bytes)?)?;

    let startup_object = detect_startup(startup, &forms, &modules);

    let project = LoadedProject {
        project_type: vb6parse::files::project::properties::CompileTargetType::Exe,
        project_name: String::new(),
        forms,
        modules,
        classes,
        file_entries: Vec::new(),
        startup_object,
    };

    let mut h = NEXT_STATE_HANDLE
        .lock()
        .map_err(|_| JsError::new("lock poisoned"))?;
    let handle = *h;
    *h += 1;

    let mut interp = Interpreter::new();
    interp.set_file_backend(Box::new(file_state::memory::MemoryBackend::new()));
    interp.set_clock_backend(Box::new(clock_state::memory::MemoryBackend::new(
        jiff::Timestamp::now(),
    )));

    let result = interp.run_project(&project);
    let new_output = interp.drain_output();
    let steps = interp.steps();
    let terminated = interp.is_terminated();
    let debug = build_debug_state(&interp);

    let state = WasmRunState { interpreter: interp };
    RUN_STATE
        .lock()
        .map_err(|_| JsError::new("lock poisoned"))?
        .insert(handle, state);

    Ok(to_value(&WasmRunOutput {
        successful: result.is_ok(),
        output_lines: Vec::new(),
        output_text: new_output,
        steps,
        terminated,
        paused: false,
        error: result.err().map(|e| convert_run_error(e, "", 0)),
        debug,
        state_handle: Some(handle),
    })?)
}

/// Call a Sub procedure by name within a running project session.
#[wasm_bindgen]
pub fn call_sub(state_handle: u32, name: &str) -> Result<JsValue, JsError> {
    let mut guard = RUN_STATE
        .lock()
        .map_err(|_| JsError::new("lock poisoned"))?;
    let state = guard
        .get_mut(&state_handle)
        .ok_or_else(|| JsError::new("unknown state handle"))?;

    let result = state.interpreter.call_sub(name, Vec::new());
    let new_output = state.interpreter.drain_output();
    let steps = state.interpreter.steps();
    let terminated = state.interpreter.is_terminated();
    let debug = build_debug_state(&state.interpreter);

    match result {
        Ok(Flow::Next | Flow::Return | Flow::BreakLoop) => Ok(to_value(&WasmRunOutput {
            successful: true,
            output_lines: Vec::new(),
            output_text: new_output,
            steps,
            terminated,
            paused: false,
            error: None,
            debug,
            state_handle: None,
        })?),
        Ok(Flow::Terminate) => Ok(to_value(&WasmRunOutput {
            successful: true,
            output_lines: Vec::new(),
            output_text: new_output,
            steps,
            terminated: true,
            paused: false,
            error: None,
            debug,
            state_handle: None,
        })?),
        Err(e) => Ok(to_value(&WasmRunOutput {
            successful: false,
            output_lines: Vec::new(),
            output_text: new_output,
            steps,
            terminated,
            paused: false,
            error: Some(convert_run_error(e, "", 0)),
            debug,
            state_handle: None,
        })?),
    }
}

/// Get all captured output for a session.
#[wasm_bindgen]
pub fn get_output(state_handle: u32) -> Result<Vec<String>, JsError> {
    let mut guard = RUN_STATE
        .lock()
        .map_err(|_| JsError::new("lock poisoned"))?;
    let state = guard
        .get_mut(&state_handle)
        .ok_or_else(|| JsError::new("unknown state handle"))?;
    let new_output = state.interpreter.drain_output();
    Ok(vec![new_output])
}

/// Dispatch a form control event to the interpreter.
///
/// Constructs a procedure name from `{control}_{event}` (e.g.
/// `cmdOK_Click`) and calls it as a sub procedure on the interpreter.
/// This simulates a user interaction with a rendered form control.
#[wasm_bindgen]
pub fn form_event(state_handle: u32, control: String, event: String) -> Result<JsValue, JsError> {
    let mut guard = RUN_STATE
        .lock()
        .map_err(|_| JsError::new("lock poisoned"))?;
    let state = guard
        .get_mut(&state_handle)
        .ok_or_else(|| JsError::new("unknown state handle"))?;

    let proc_name = format!("{}_{}", control, event);
    let result = state.interpreter.call_sub(&proc_name, Vec::new());
    let new_output = state.interpreter.drain_output();
    let steps = state.interpreter.steps();
    let terminated = state.interpreter.is_terminated();
    let debug = build_debug_state(&state.interpreter);

    match result {
        Ok(_) => Ok(to_value(&WasmRunOutput {
            successful: true,
            output_lines: Vec::new(),
            output_text: new_output,
            steps,
            terminated,
            paused: false,
            error: None,
            debug,
            state_handle: None,
        })?),
        Err(e) => Ok(to_value(&WasmRunOutput {
            successful: false,
            output_lines: Vec::new(),
            output_text: new_output,
            steps,
            terminated,
            paused: false,
            error: Some(convert_run_error(e, "", 0)),
            debug,
            state_handle: None,
        })?),
    }
}

/// Dispose a session and free its resources.
#[wasm_bindgen]
pub fn dispose_state(state_handle: u32) -> bool {
    RUN_STATE
        .lock()
        .map_err(|_| JsError::new("lock poisoned"))
        .ok()
        .map(|mut guard| guard.remove(&state_handle).is_some())
        .unwrap_or(false)
}
