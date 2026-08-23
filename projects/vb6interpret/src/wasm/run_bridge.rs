//! Browser entry points for running, stepping, and tracing a module, plus
//! the debug-state builders that translate interpreter snapshots into the
//! wire structs declared in [`super`].

use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use super::{WasmDebugState, WasmDebugTrace, WasmRunError, WasmRunOutput, WasmVariableInfo};
use crate::error::RunError;
use crate::interpreter::{DebugSnapshot, DebugVariable};
use crate::Interpreter;
use vb6runtime::state::clock as clock_state;
use vb6runtime::state::file as file_state;

fn variable_to_wasm(variable: &DebugVariable) -> WasmVariableInfo {
    WasmVariableInfo {
        name: variable.name.clone(),
        type_name: variable.type_name.clone(),
        value: variable.value.clone(),
    }
}

pub(super) fn build_debug_state(interpreter: &Interpreter) -> WasmDebugState {
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
        cursor: None,
    }
}

pub(super) fn build_debug_state_from_snapshot(
    snapshot: &DebugSnapshot,
    code: &str,
    delta: u32,
) -> WasmDebugState {
    WasmDebugState {
        current_steps: snapshot.steps,
        current_line: snapshot.current_line,
        current_procedure: snapshot.current_procedure.clone(),
        stack_depth: snapshot.stack_depth,
        globals: snapshot.globals.iter().map(variable_to_wasm).collect(),
        locals: snapshot.locals.iter().map(variable_to_wasm).collect(),
        cursor: cursor_to_wasm(snapshot, code, delta),
    }
}

/// Convert a snapshot's cursor byte range (relative to the module body, i.e.
/// the CST with the `Attribute` header stripped) into 1-based line/column
/// coordinates in the original source.
fn cursor_to_wasm(snapshot: &DebugSnapshot, code: &str, delta: u32) -> Option<[u32; 4]> {
    let (start, end) = snapshot.cursor_range?;
    let (start_line, start_column) =
        byte_offset_to_line_column(code, start as usize + delta as usize);
    let (end_line, end_column) = byte_offset_to_line_column(code, end as usize + delta as usize);
    Some([
        start_line as u32,
        start_column as u32,
        end_line as u32,
        end_column as u32,
    ])
}

pub(super) fn byte_offset_to_line_column(source: &str, offset: usize) -> (usize, usize) {
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

/// Execute a single VB6 module and return captured output plus runtime status.
///
/// The interpreter playground currently supports module input only.
#[wasm_bindgen]
pub fn interpret_vb6_code(code: &str) -> Result<JsValue, JsError> {
    let module = match super::parse_module(code) {
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
    // In wasm, use the memory backend for file and clock operations.
    interpreter.set_file_backend(Box::new(file_state::memory::MemoryBackend::new()));
    interpreter.set_clock_backend(Box::new(clock_state::memory::MemoryBackend::new(
        jiff::Timestamp::now(),
    )));
    match interpreter.run_module(&module) {
        Ok(()) => Ok(to_value(&super::build_output(&interpreter, None))?),
        Err(error) => Ok(to_value(&super::build_output(
            &interpreter,
            Some(super::convert_run_error(error, code, module.line_offset)),
        ))?),
    }
}

/// Execute a single VB6 module up to `pause_after_steps` statements and return
/// a snapshot suitable for debugger-style stepping.
#[wasm_bindgen]
pub fn debug_vb6_code(code: &str, pause_after_steps: u32) -> Result<JsValue, JsError> {
    let module = match super::parse_module(code) {
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
    // In wasm, use the memory backend for file and clock operations.
    interpreter.set_file_backend(Box::new(file_state::memory::MemoryBackend::new()));
    interpreter.set_clock_backend(Box::new(clock_state::memory::MemoryBackend::new(
        jiff::Timestamp::now(),
    )));
    interpreter.set_pause_after_steps(Some(u64::from(pause_after_steps)));

    match interpreter.run_module(&module) {
        Ok(()) => Ok(to_value(&super::build_output(&interpreter, None))?),
        Err(error) => Ok(to_value(&super::build_output(
            &interpreter,
            Some(super::convert_run_error(error, code, module.line_offset)),
        ))?),
    }
}

/// Build a full statement-boundary execution trace that the browser can use
/// for true resume-from-current-state stepping.
#[wasm_bindgen]
pub fn build_debug_trace(code: &str) -> Result<JsValue, JsError> {
    let module = match super::parse_module(code) {
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
    // In wasm, use the memory backend for file and clock operations.
    interpreter.set_file_backend(Box::new(file_state::memory::MemoryBackend::new()));
    interpreter.set_clock_backend(Box::new(clock_state::memory::MemoryBackend::new(
        jiff::Timestamp::now(),
    )));
    interpreter.set_record_debug_snapshots(true);

    let error = match interpreter.run_module(&module) {
        Ok(()) => None,
        Err(error) => Some(super::convert_run_error(error, code, module.line_offset)),
    };

    interpreter.capture_final_debug_snapshot();

    // Cursor byte offsets from the interpreter are relative to the module
    // body (the CST has the `Attribute` header stripped); find where that
    // body starts inside the original source so cursors can be converted to
    // line/column coordinates.
    let filtered_text = module.cst.text();
    let delta = code
        .find(&filtered_text)
        .map(|offset| offset as u32)
        .unwrap_or(0);

    let last_index = interpreter.debug_snapshots().len().saturating_sub(1);
    let snapshots = interpreter
        .debug_snapshots()
        .iter()
        .enumerate()
        .map(|(index, snapshot)| {
            let is_last = index == last_index;
            let snapshot_error = if is_last { error.clone() } else { None };
            super::build_output_from_snapshot(
                snapshot,
                !is_last,
                snapshot_error.is_none(),
                snapshot_error,
                code,
                delta,
            )
        })
        .collect();

    Ok(to_value(&WasmDebugTrace {
        successful: error.is_none(),
        error,
        snapshots,
    })?)
}
