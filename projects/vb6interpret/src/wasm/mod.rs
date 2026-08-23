//! WebAssembly bindings for the VB6 interpreter.
//!
//! This module exposes a browser-friendly API for running a single VB6 module
//! from source text and capturing its output.
//!
//! Submodules hold the browser entry points and debug-state builders
//! ([`run_bridge`]) and the runtime-state tab bindings ([`state_bridge`]);
//! this file keeps the wire structs and the output converters they share.

mod run_bridge;
mod state_bridge;

use run_bridge::{build_debug_state, build_debug_state_from_snapshot, byte_offset_to_line_column};

use serde::{Deserialize, Serialize};

use crate::error::{render_error_report, render_report_at_line, RunError};
use crate::interpreter::DebugSnapshot;
use crate::Interpreter;
use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;

/// Structured runtime or parse error information for the browser UI.
#[derive(Clone, Serialize, Deserialize)]
pub struct WasmRunError {
    /// Human-readable error message.
    pub message: String,
    /// Ariadne-rendered pretty report pointing at the offending source line,
    /// when the source location is known.
    pub pretty_report: Option<String>,
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
    /// 1-based `[start_line, start_column, end_line, end_column]` of the
    /// sub-line element currently being executed (e.g. a loop's counter,
    /// step, or `Next`), when the snapshot targets one. `None` means the
    /// whole `current_line` is highlighted.
    pub cursor: Option<[u32; 4]>,
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
            let pretty_report = first.zip(line).and_then(|(error, line)| {
                render_report_at_line("playground.bas", code, line, &error.kind.to_string())
            });

            Err(WasmRunError {
                message,
                pretty_report,
                error_number: None,
                is_debug_pause: false,
                line,
                procedure: None,
            })
        }
    }
}

fn convert_run_error(error: RunError, code: &str, line_offset: usize) -> WasmRunError {
    let pretty_report = render_error_report("playground.bas", code, &error, line_offset);
    WasmRunError {
        message: error.to_string(),
        pretty_report,
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
    code: &str,
    delta: u32,
) -> WasmRunOutput {
    WasmRunOutput {
        successful,
        output_lines: snapshot.output_lines.clone(),
        output_text: snapshot.output_text.clone(),
        steps: snapshot.steps,
        terminated: snapshot.terminated,
        paused,
        error,
        debug: build_debug_state_from_snapshot(snapshot, code, delta),
    }
}
