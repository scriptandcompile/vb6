//! WebAssembly bindings for the VB6 interpreter.
//!
//! This module exposes a browser-friendly API for running a single VB6 module
//! from source text and capturing its output.

use crate::error::RunError;
use crate::Interpreter;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;
use wasm_bindgen::prelude::*;

/// Structured runtime or parse error information for the browser UI.
#[derive(Serialize, Deserialize)]
pub struct WasmRunError {
    /// Human-readable error message.
    pub message: String,
    /// The VB6 error number, if the failure is a runtime `Err` value.
    pub error_number: Option<i32>,
    /// 1-based source line, when known.
    pub line: Option<usize>,
    /// Executing procedure name, when known.
    pub procedure: Option<String>,
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
    /// Runtime or parse error details.
    pub error: Option<WasmRunError>,
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
                line,
                procedure: None,
            })
        }
    }
}

fn convert_run_error(error: RunError) -> WasmRunError {
    WasmRunError {
        message: error.to_string(),
        error_number: Some(error.error.number),
        line: error.line,
        procedure: error.procedure,
    }
}

fn build_output(interpreter: &Interpreter, error: Option<WasmRunError>) -> WasmRunOutput {
    WasmRunOutput {
        successful: error.is_none(),
        output_lines: interpreter.output().to_vec(),
        output_text: interpreter.output_text(),
        steps: interpreter.steps(),
        terminated: interpreter.is_terminated(),
        error,
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
                error: Some(error),
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
