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
use super::{build_debug_state, convert_run_error, WasmDebugState, WasmRunOutput};
use crate::Interpreter;
use crate::interpreter::Flow;
use crate::project::LoadedProject;
use vb6runtime::state::clock as clock_state;
use vb6runtime::state::file as file_state;

/// Persistent interpreter session for form-based execution.
struct WasmRunState {
    interpreter: Interpreter,
    output_lines: Vec<String>,
}

static RUN_STATE: LazyLock<Mutex<HashMap<u32, WasmRunState>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static NEXT_STATE_HANDLE: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

fn empty_debug_state() -> WasmDebugState {
    WasmDebugState {
        current_steps: 0,
        current_line: 1,
        current_procedure: None,
        stack_depth: 0,
        globals: Vec::new(),
        locals: Vec::new(),
        cursor: None,
    }
}

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

    let mut h = NEXT_STATE_HANDLE.lock().map_err(|_| JsError::new("lock poisoned"))?;
    let handle = *h;
    *h += 1;

    let mut interp = Interpreter::new();
    interp.set_file_backend(Box::new(file_state::memory::MemoryBackend::new()));
    interp.set_clock_backend(Box::new(clock_state::memory::MemoryBackend::new(
        jiff::Timestamp::now(),
    )));

    let result = interp.run_project(&project);
    let output_text = interp.output_text();
    let steps = interp.steps();
    let terminated = interp.is_terminated();
    let output_lines = interp.output().to_vec();

    let state = WasmRunState {
        interpreter: interp,
        output_lines: output_lines.clone(),
    };
    RUN_STATE.lock().map_err(|_| JsError::new("lock poisoned"))?.insert(handle, state);

    Ok(to_value(&WasmRunOutput {
        successful: result.is_ok(),
        output_lines,
        output_text,
        steps,
        terminated,
        paused: false,
        error: result.err().map(|e| convert_run_error(e, "", 0)),
        debug: empty_debug_state(),
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

    match state.interpreter.call_sub(name, Vec::new()) {
        Ok(Flow::Next | Flow::Return | Flow::BreakLoop) => Ok(to_value(&WasmRunOutput {
            successful: true,
            output_lines: state.interpreter.output().to_vec(),
            output_text: state.interpreter.output_text(),
            steps: state.interpreter.steps(),
            terminated: state.interpreter.is_terminated(),
            paused: false,
            error: None,
            debug: build_debug_state(&state.interpreter),
        })?),
        Ok(Flow::Terminate) => Ok(to_value(&WasmRunOutput {
            successful: true,
            output_lines: state.interpreter.output().to_vec(),
            output_text: state.interpreter.output_text(),
            steps: state.interpreter.steps(),
            terminated: true,
            paused: false,
            error: None,
            debug: build_debug_state(&state.interpreter),
        })?),
        Err(e) => Ok(to_value(&WasmRunOutput {
            successful: false,
            output_lines: state.interpreter.output().to_vec(),
            output_text: state.interpreter.output_text(),
            steps: state.interpreter.steps(),
            terminated: state.interpreter.is_terminated(),
            paused: false,
            error: Some(convert_run_error(e, "", 0)),
            debug: build_debug_state(&state.interpreter),
        })?),
    }
}

/// Get all captured output for a session.
#[wasm_bindgen]
pub fn get_output(state_handle: u32) -> Result<Vec<String>, JsError> {
    let guard = RUN_STATE
        .lock()
        .map_err(|_| JsError::new("lock poisoned"))?;
    let state = guard
        .get(&state_handle)
        .ok_or_else(|| JsError::new("unknown state handle"))?;
    Ok(state.output_lines.clone())
}

/// Dispose a session and free its resources.
#[wasm_bindgen]
pub fn dispose_state(state_handle: u32) -> bool {
    RUN_STATE.lock().map_err(|_| JsError::new("lock poisoned"))
        .ok()
        .map(|mut guard| guard.remove(&state_handle).is_some())
        .unwrap_or(false)
}
