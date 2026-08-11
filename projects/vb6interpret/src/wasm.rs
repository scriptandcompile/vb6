//! WebAssembly bindings for the VB6 interpreter.
//!
//! This module exposes a browser-friendly API for running a single VB6 module
//! from source text and capturing its output.

use crate::error::RunError;
use crate::interpreter::{DebugSnapshot, DebugVariable};
use crate::Interpreter;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;
use wasm_bindgen::prelude::*;

/// Structured runtime or parse error information for the browser UI.
#[derive(Clone, Serialize, Deserialize)]
pub struct WasmRunError {
    /// Human-readable error message.
    pub message: String,
    /// The VB6 error number, if the failure is a runtime `Err` value.
    pub error_number: Option<i32>,
    /// Whether this is an internal step pause rather than a runtime error.
    pub is_debug_pause: bool,
    /// 1-based source line, when known.
    pub line: Option<usize>,
    /// Executing procedure name, when known.
    pub procedure: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct WasmVariableInfo {
    pub name: String,
    pub type_name: String,
    pub value: String,
}

/// Current interpreter position and scope state for the browser debug UI.
#[derive(Serialize, Deserialize)]
pub struct WasmDebugState {
    pub current_steps: u64,
    pub current_line: usize,
    pub current_procedure: Option<String>,
    pub stack_depth: usize,
    pub globals: Vec<WasmVariableInfo>,
    pub locals: Vec<WasmVariableInfo>,
}

/// A trace of statement-boundary snapshots for resume-from-current-state
/// stepping in the browser.
#[derive(Serialize, Deserialize)]
pub struct WasmDebugTrace {
    pub successful: bool,
    pub error: Option<WasmRunError>,
    pub snapshots: Vec<WasmRunOutput>,
}

/// Output returned from the interpreter playground.
#[derive(Serialize, Deserialize)]
pub struct WasmRunOutput {
    /// Whether execution finished without parse/runtime failure.
    pub successful: bool,
    /// Completed output lines.
    pub output_lines: Vec<String>,
    /// Entire output as a single string.
    pub output_text: String,
    /// Number of statements executed.
    pub steps: u64,
    /// Whether `End` terminated the program.
    pub terminated: bool,
    /// Whether execution paused before the next statement.
    pub paused: bool,
    /// Runtime or parse error details.
    pub error: Option<WasmRunError>,
    /// Debug-oriented snapshot of the interpreter state.
    pub debug: WasmDebugState,
}

fn variable_to_wasm(variable: &DebugVariable) -> WasmVariableInfo {
    WasmVariableInfo {
        name: variable.name.clone(),
        type_name: variable.type_name.clone(),
        value: variable.value.clone(),
    }
}

fn build_debug_state(interpreter: &Interpreter) -> WasmDebugState {
    WasmDebugState {
        current_steps: interpreter.steps(),
        current_line: interpreter.current_line(),
        current_procedure: interpreter.current_procedure().map(str::to_string),
        stack_depth: interpreter.frames.len(),
        globals: interpreter
            .debug_snapshots()
            .last()
            .map(|snapshot| snapshot.globals.iter().map(variable_to_wasm).collect())
            .unwrap_or_else(|| {
                interpreter
                    .globals()
                    .iter()
                    .map(|(name, value)| WasmVariableInfo {
                        name: name.to_string(),
                        type_name: value.type_of().to_string(),
                        value: format!("{value}"),
                    })
                    .collect()
            }),
        locals: interpreter
            .debug_snapshots()
            .last()
            .map(|snapshot| snapshot.locals.iter().map(variable_to_wasm).collect())
            .unwrap_or_default(),
    }
}

fn build_debug_state_from_snapshot(snapshot: &DebugSnapshot) -> WasmDebugState {
    WasmDebugState {
        current_steps: snapshot.steps,
        current_line: snapshot.current_line,
        current_procedure: snapshot.current_procedure.clone(),
        stack_depth: snapshot.stack_depth,
        globals: snapshot.globals.iter().map(variable_to_wasm).collect(),
        locals: snapshot.locals.iter().map(variable_to_wasm).collect(),
    }
}

fn byte_offset_to_line_column(source: &str, offset: usize) -> (usize, usize) {
    let target = offset.min(source.len());
    let mut line = 1usize;
    let mut line_start = 0usize;

    for (index, ch) in source.char_indices() {
        if index >= target {
            break;
        }

        if ch == '\n' {
            line += 1;
            line_start = index + ch.len_utf8();
        }
    }

    let column = source[line_start..target].chars().count() + 1;
    (line, column)
}

fn parse_module(code: &str) -> Result<ModuleFile, WasmRunError> {
    let source_file = SourceFile::from_string("playground.bas", code);
    match ModuleFile::parse(&source_file).ok_or_errors() {
        Ok(module) => Ok(module),
        Err(errors) => {
            let first = errors.first();
            let message = first
                .map(|error| error.kind.to_string())
                .unwrap_or_else(|| "Failed to parse the input code as a VB6 module.".to_string());
            let line = first.map(|error| {
                byte_offset_to_line_column(error.source_content, error.error_offset as usize).0
            });

            Err(WasmRunError {
                message,
                error_number: None,
                is_debug_pause: false,
                line,
                procedure: None,
            })
        }
    }
}

fn convert_run_error(error: RunError) -> WasmRunError {
    WasmRunError {
        message: error.to_string(),
        error_number: (!error.is_debug_pause()).then_some(error.error.number),
        is_debug_pause: error.is_debug_pause(),
        line: error.line,
        procedure: error.procedure,
    }
}

fn build_output(interpreter: &Interpreter, error: Option<WasmRunError>) -> WasmRunOutput {
    let paused = error.as_ref().is_some_and(|error| error.is_debug_pause);
    WasmRunOutput {
        successful: error.is_none() || paused,
        output_lines: interpreter.output().to_vec(),
        output_text: interpreter.output_text(),
        steps: interpreter.steps(),
        terminated: interpreter.is_terminated(),
        paused,
        error,
        debug: build_debug_state(interpreter),
    }
}

fn build_output_from_snapshot(
    snapshot: &DebugSnapshot,
    paused: bool,
    successful: bool,
    error: Option<WasmRunError>,
) -> WasmRunOutput {
    WasmRunOutput {
        successful,
        output_lines: snapshot.output_lines.clone(),
        output_text: snapshot.output_text.clone(),
        steps: snapshot.steps,
        terminated: snapshot.terminated,
        paused,
        error,
        debug: build_debug_state_from_snapshot(snapshot),
    }
}

fn empty_debug_state() -> WasmDebugState {
    WasmDebugState {
        current_steps: 0,
        current_line: 1,
        current_procedure: None,
        stack_depth: 0,
        globals: Vec::new(),
        locals: Vec::new(),
    }
}

/// Execute a single VB6 module and return captured output plus runtime status.
///
/// The interpreter playground currently supports module input only.
#[wasm_bindgen]
pub fn interpret_vb6_code(code: &str) -> Result<JsValue, JsError> {
    let module = match parse_module(code) {
        Ok(module) => module,
        Err(error) => {
            return Ok(to_value(&WasmRunOutput {
                successful: false,
                output_lines: Vec::new(),
                output_text: String::new(),
                steps: 0,
                terminated: false,
                paused: false,
                error: Some(error),
                debug: empty_debug_state(),
            })?)
        }
    };

    let mut interpreter = Interpreter::new();
    match interpreter.run_module(&module) {
        Ok(()) => Ok(to_value(&build_output(&interpreter, None))?),
        Err(error) => Ok(to_value(&build_output(
            &interpreter,
            Some(convert_run_error(error)),
        ))?),
    }
}

/// Execute a single VB6 module up to `pause_after_steps` statements and return
/// a snapshot suitable for debugger-style stepping.
#[wasm_bindgen]
pub fn debug_vb6_code(code: &str, pause_after_steps: u32) -> Result<JsValue, JsError> {
    let module = match parse_module(code) {
        Ok(module) => module,
        Err(error) => {
            return Ok(to_value(&WasmRunOutput {
                successful: false,
                output_lines: Vec::new(),
                output_text: String::new(),
                steps: 0,
                terminated: false,
                paused: false,
                error: Some(error),
                debug: empty_debug_state(),
            })?)
        }
    };

    let mut interpreter = Interpreter::new();
    interpreter.set_pause_after_steps(Some(u64::from(pause_after_steps)));

    match interpreter.run_module(&module) {
        Ok(()) => Ok(to_value(&build_output(&interpreter, None))?),
        Err(error) => Ok(to_value(&build_output(
            &interpreter,
            Some(convert_run_error(error)),
        ))?),
    }
}

/// Build a full statement-boundary execution trace that the browser can use
/// for true resume-from-current-state stepping.
#[wasm_bindgen]
pub fn build_debug_trace(code: &str) -> Result<JsValue, JsError> {
    let module = match parse_module(code) {
        Ok(module) => module,
        Err(error) => {
            return Ok(to_value(&WasmDebugTrace {
                successful: false,
                error: Some(error),
                snapshots: Vec::new(),
            })?)
        }
    };

    let mut interpreter = Interpreter::new();
    interpreter.set_record_debug_snapshots(true);

    let error = match interpreter.run_module(&module) {
        Ok(()) => None,
        Err(error) => Some(convert_run_error(error)),
    };

    interpreter.capture_final_debug_snapshot();

    let last_index = interpreter.debug_snapshots().len().saturating_sub(1);
    let snapshots = interpreter
        .debug_snapshots()
        .iter()
        .enumerate()
        .map(|(index, snapshot)| {
            let is_last = index == last_index;
            let snapshot_error = if is_last { error.clone() } else { None };
            build_output_from_snapshot(snapshot, !is_last, snapshot_error.is_none(), snapshot_error)
        })
        .collect();

    Ok(to_value(&WasmDebugTrace {
        successful: error.is_none(),
        error,
        snapshots,
    })?)
}
