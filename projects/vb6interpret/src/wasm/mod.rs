//! WebAssembly bindings for the VB6 interpreter.
//!
//! This module exposes a browser-friendly API for running a single VB6 module
//! from source text and capturing its output, as well as multi-file VBP-style
//! projects via [`WasmProject`].
//!
//! Submodules hold the browser entry points and debug-state builders
//! ([`run_bridge`]), the runtime-state tab bindings ([`state_bridge`]), and
//! the form-rendering bridge ([`form_bridge`]); this file keeps the wire
//! structs and the output converters they share.

mod exec_bridge;
mod form_bridge;
mod project_parser;
mod run_bridge;
mod state_bridge;

use run_bridge::{build_debug_state, build_debug_state_from_snapshot, byte_offset_to_line_column};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::Interpreter;
use crate::error::{RunError, render_error_report, render_report_at_line};
use crate::interpreter::DebugSnapshot;
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
    /// Zero-based parameter index that failed in a builtin call, when the
    /// error originates from one.
    pub param_index: Option<usize>,
    /// Parameter name from the builtin declaration, when the error
    /// originates from a builtin call.
    pub param_name: Option<String>,
}

/// Information about a single variable in the browser debug UI.
#[derive(Serialize, Deserialize)]
pub struct WasmVariableInfo {
    /// The variable name.
    pub name: String,
    /// The variable type as a string.
    pub type_name: String,
    /// The variable value as a string.
    pub value: String,
}

/// Current interpreter position and scope state for the browser debug UI.
#[derive(Serialize, Deserialize)]
pub struct WasmDebugState {
    /// Number of statements executed so far.
    pub current_steps: u64,
    /// 1-based current source line.
    pub current_line: usize,
    /// Executing procedure name, when known.
    pub current_procedure: Option<String>,
    /// Current call stack depth.
    pub stack_depth: usize,
    /// Global variables in the current scope.
    pub globals: Vec<WasmVariableInfo>,
    /// Local variables in the current scope.
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
    /// Whether the execution was successful.
    pub successful: bool,
    /// Runtime or parse error details, if any.
    pub error: Option<WasmRunError>,
    /// Sequence of interpreter snapshots for each statement executed.
    pub snapshots: Vec<WasmRunOutput>,
}

/// A VB6 project loaded from JS-provided byte maps for multi-file VBP-style
/// projects in the browser.
///
/// The JS layer constructs a `WasmProject` by populating the `forms`,
/// `modules`, and `classes` fields with [`JsMap`] instances whose keys are
/// file names and whose values are `Uint8Array` (raw file bytes).  The
/// `startup` field holds the startup object name (form name or module name).
///
/// # Examples
///
/// Constructed on the JS side from a file picker or a build tool:
///
/// ```js
/// const project = new WasmProject();
/// project.forms.set("Form1.frm", form1Bytes);
/// project.modules.set("Module1.bas", module1Bytes);
/// project.startup = "Sub Main";
/// ```
#[wasm_bindgen]
pub struct WasmProject {
    /// Map of file name → raw bytes for form (`.frm`) files.
    #[wasm_bindgen(skip)]
    pub forms: JsValue,
    /// Map of file name → raw bytes for module (`.bas`) files.
    #[wasm_bindgen(skip)]
    pub modules: JsValue,
    /// Map of file name → raw bytes for class (`.cls`) files.
    #[wasm_bindgen(skip)]
    pub classes: JsValue,
    /// The startup object name (form name or module name).
    #[wasm_bindgen(getter_with_clone)]
    pub startup: String,
}

#[wasm_bindgen]
impl WasmProject {
    /// Create a new, empty `WasmProject`.
    ///
    /// Callers on the JS side should populate `forms`, `modules`, and
    /// `classes` with [`JsMap`] instances and set `startup` before passing
    /// the project to a WASM function.
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmProject {
        WasmProject {
            forms: JsValue::UNDEFINED,
            modules: JsValue::UNDEFINED,
            classes: JsValue::UNDEFINED,
            startup: String::new(),
        }
    }
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
                param_index: None,
                param_name: None,
            })
        }
    }
}

fn convert_run_error(error: RunError, code: &str, line_offset: usize) -> WasmRunError {
    let pretty_report = render_error_report("playground.bas", code, &error, line_offset);
    let param_index = error.builtin_call.as_ref().map(|c| c.param_index);
    let param_name = error.builtin_call.as_ref().map(|c| c.param_name.clone());
    WasmRunError {
        message: error.to_string(),
        pretty_report,
        error_number: (!error.is_debug_pause()).then_some(error.error.number),
        is_debug_pause: error.is_debug_pause(),
        line: error.line,
        procedure: error.procedure,
        param_index,
        param_name,
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
