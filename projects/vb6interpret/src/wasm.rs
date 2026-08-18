//! WebAssembly bindings for the VB6 interpreter.
//!
//! This module exposes a browser-friendly API for running a single VB6 module
//! from source text and capturing its output.

use crate::error::{render_error_report, render_report_at_line, RunError};
use crate::interpreter::{DebugSnapshot, DebugVariable};
use crate::Interpreter;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;
use vb6runtime::state::clock as clock_state;
use vb6runtime::state::environment as env_state;
use vb6runtime::state::file as file_state;
use vb6runtime::state::settings as settings_state;
use wasm_bindgen::prelude::*;

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
        cursor: None,
    }
}

fn build_debug_state_from_snapshot(
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
    // In wasm, use the memory backend for file and clock operations.
    interpreter.set_file_backend(Box::new(file_state::memory::MemoryBackend::new()));
    interpreter.set_clock_backend(Box::new(clock_state::memory::MemoryBackend::new(
        jiff::Timestamp::now(),
    )));
    match interpreter.run_module(&module) {
        Ok(()) => Ok(to_value(&build_output(&interpreter, None))?),
        Err(error) => Ok(to_value(&build_output(
            &interpreter,
            Some(convert_run_error(error, code, module.line_offset)),
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
    // In wasm, use the memory backend for file and clock operations.
    interpreter.set_file_backend(Box::new(file_state::memory::MemoryBackend::new()));
    interpreter.set_clock_backend(Box::new(clock_state::memory::MemoryBackend::new(
        jiff::Timestamp::now(),
    )));
    interpreter.set_pause_after_steps(Some(u64::from(pause_after_steps)));

    match interpreter.run_module(&module) {
        Ok(()) => Ok(to_value(&build_output(&interpreter, None))?),
        Err(error) => Ok(to_value(&build_output(
            &interpreter,
            Some(convert_run_error(error, code, module.line_offset)),
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
    // In wasm, use the memory backend for file and clock operations.
    interpreter.set_file_backend(Box::new(file_state::memory::MemoryBackend::new()));
    interpreter.set_clock_backend(Box::new(clock_state::memory::MemoryBackend::new(
        jiff::Timestamp::now(),
    )));
    interpreter.set_record_debug_snapshots(true);

    let error = match interpreter.run_module(&module) {
        Ok(()) => None,
        Err(error) => Some(convert_run_error(error, code, module.line_offset)),
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
            build_output_from_snapshot(
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

/// A single setting's full registry-style path and value.
#[derive(Clone, Serialize, Deserialize)]
pub struct WasmSetting {
    pub appname: String,
    pub section: String,
    pub key: String,
    pub value: String,
}

/// A single `NAME`/value pair of the environment snapshot.
#[derive(Clone, Serialize, Deserialize)]
pub struct WasmEnvEntry {
    pub name: String,
    pub value: String,
}

/// The mock clock's current date and time, for the Clock section of the
/// Environment tab.
#[derive(Clone, Serialize, Deserialize)]
pub struct WasmClockState {
    /// Current mock date (`YYYY-MM-DD`).
    pub date: String,
    /// Current mock time (`HH:MM:SS`).
    pub time: String,
}

/// File information for the Files tab.
#[derive(Clone, Serialize, Deserialize)]
pub struct WasmFile {
    /// The file path.
    pub path: String,
    /// The file number (1-511) if open, 0 otherwise.
    pub number: i16,
    /// The open mode (Input, Output, Append, Random, Binary).
    pub mode: String,
    /// VB6-style attribute bitfield.
    pub attributes: i16,
    /// File content as a string (lossy UTF-8) for text files.
    pub content_text: Option<String>,
    /// File content as base64-encoded bytes for binary files.
    pub content_base64: Option<String>,
}

/// Complete file state for the Files tab.
#[derive(Clone, Serialize, Deserialize)]
pub struct WasmFileState {
    /// Current working directory.
    pub current_dir: String,
    /// Current drive letter.
    pub current_drive: char,
    /// List of currently open file numbers.
    pub open_file_numbers: Vec<i16>,
    /// All files in the memory backend.
    pub files: Vec<WasmFile>,
}

/// Every environment variable currently in the snapshot, for display and
/// persisting back to `localStorage`.
#[wasm_bindgen]
pub fn dump_env() -> Result<JsValue, JsError> {
    let entries: Vec<WasmEnvEntry> = env_state::entries()
        .into_iter()
        .map(|(name, value)| WasmEnvEntry { name, value })
        .collect();
    Ok(to_value(&entries)?)
}

/// Set (or replace) the value of environment variable `name` in the snapshot.
///
/// The webassembly host has no process environment, so the snapshot starts
/// empty and is seeded from `localStorage` before a run; `Environ$` reads
/// whatever is installed here.
#[wasm_bindgen]
pub fn set_env(name: &str, value: &str) {
    env_state::set_env(name, value);
}

/// Remove environment variable `name` from the snapshot, if present.
#[wasm_bindgen]
pub fn remove_env(name: &str) {
    env_state::remove_env(name);
}

/// Install or overwrite the setting `(appname, section, key)` with `value`.
///
/// The webassembly host has no filesystem, so `localStorage` takes the role
/// of the settings store root: the host calls [`install_setting`] once per
/// persisted entry before running a module, and persists [`dump_settings`]
/// afterwards. `GetSetting` reads whatever is installed.
#[wasm_bindgen]
pub fn install_setting(appname: &str, section: &str, key: &str, value: &str) {
    let _ = settings_state::set(appname, section, key, value);
}

/// Remove the setting `(appname, section, key)`, if present.
#[wasm_bindgen]
pub fn remove_setting(appname: &str, section: &str, key: &str) {
    let _ = settings_state::remove_key(appname, section, key);
}

/// Every setting currently in the store, for persisting back to `localStorage`.
#[wasm_bindgen]
pub fn dump_settings() -> Result<JsValue, JsError> {
    let settings: Vec<WasmSetting> = settings_state::entries()
        .into_iter()
        .map(|(appname, section, key, value)| WasmSetting {
            appname,
            section,
            key,
            value,
        })
        .collect();
    Ok(to_value(&settings)?)
}

/// The current mock clock date and time, for the Clock section of the
/// Environment tab. Displayed in the system's local time zone, matching
/// what VB6's `Now`/`Date`/`Time` functions would report.
///
/// `time` is 24-hour (`HH:MM:SS`) so it round-trips with [`set_clock`] and
/// native `<input type="time">` elements; the browser formats it for
/// display (e.g. 12-hour with AM/PM).
#[wasm_bindgen]
pub fn dump_clock() -> Result<JsValue, JsError> {
    let zoned = jiff::Zoned::new(clock_state::get(), jiff::tz::TimeZone::system());
    let date = zoned.date();
    let time = zoned.time();
    Ok(to_value(&WasmClockState {
        date: format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day()),
        time: format!(
            "{:02}:{:02}:{:02}",
            time.hour(),
            time.minute(),
            time.second()
        ),
    })?)
}

/// Set the in-memory clock to `date` (`YYYY-MM-DD`) and `time` (`HH:MM:SS`)
/// in the system's local time zone.
///
/// This rewrites the memory clock backend directly; it never touches (and is
/// never echoed back to) the real system clock.
#[wasm_bindgen]
pub fn set_clock(date: &str, time: &str) -> Result<(), JsError> {
    let date: jiff::civil::Date = date
        .parse()
        .map_err(|e| JsError::new(&format!("invalid date: {e}")))?;
    let time: jiff::civil::Time = time
        .parse()
        .map_err(|e| JsError::new(&format!("invalid time: {e}")))?;
    let zoned = date
        .at(time.hour(), time.minute(), time.second(), 0)
        .to_zoned(jiff::tz::TimeZone::system())
        .map_err(|e| JsError::new(&format!("invalid date/time: {e}")))?;
    clock_state::system_set(zoned.timestamp())
        .map_err(|e| JsError::new(&format!("failed to set clock: {e}")))?;
    clock_state::reset();
    Ok(())
}

/// Convert a file mode to a human-readable string.
fn open_mode_to_string(mode: file_state::OpenMode) -> String {
    match mode {
        file_state::OpenMode::Input => "Input".to_string(),
        file_state::OpenMode::Output => "Output".to_string(),
        file_state::OpenMode::Append => "Append".to_string(),
        file_state::OpenMode::Random => "Random".to_string(),
        file_state::OpenMode::Binary => "Binary".to_string(),
    }
}

/// Close every open file and wipe the in-memory file backend, restoring it to
/// an empty filesystem.
#[wasm_bindgen]
pub fn clear_files() {
    file_state::reset();
}

/// Create or replace the file at `path` with raw `content`, bypassing
/// `Open`/`Close`. Used to restore a snapshot saved from the Files tab.
#[wasm_bindgen]
pub fn install_file(path: &str, content: &[u8]) -> Result<(), JsError> {
    file_state::write_memory_file(path, content).map_err(|e| JsError::new(&e.to_string()))
}

/// Snapshot of the memory file backend for the Files tab.
#[wasm_bindgen]
pub fn dump_files() -> Result<JsValue, JsError> {
    use base64::Engine;

    let current_dir = file_state::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "/".to_string());

    let current_drive = file_state::current_dir()
        .map(|p| {
            p.to_string_lossy()
                .chars()
                .next()
                .filter(|c| c.is_ascii_alphabetic())
                .map(|c| c.to_ascii_uppercase())
                .unwrap_or('C')
        })
        .unwrap_or('C');

    let open_files: Vec<i16> = file_state::get_open_files()
        .into_iter()
        .map(|(num, _)| num)
        .collect();

    let mut files: Vec<WasmFile> = file_state::get_open_files()
        .into_iter()
        .map(|(num, open_file)| {
            let content = file_state::read_file_to_vec(num).ok();
            let (content_text, content_base64) = match content {
                Some(bytes) => {
                    let is_text = open_file.mode == file_state::OpenMode::Input
                        || open_file.mode == file_state::OpenMode::Output
                        || open_file.mode == file_state::OpenMode::Append;
                    if is_text {
                        let text = String::from_utf8_lossy(&bytes).to_string();
                        (Some(text), None)
                    } else {
                        let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                        (None, Some(b64))
                    }
                }
                None => (None, None),
            };

            WasmFile {
                path: open_file.path.clone(),
                number: num,
                mode: open_mode_to_string(open_file.mode),
                attributes: 0,
                content_text,
                content_base64,
            }
        })
        .collect();

    // Also include files that exist in the memory backend but aren't currently open.
    if let Ok(memory_files) = file_state::list_memory_files() {
        let open_paths: std::collections::HashSet<String> =
            files.iter().map(|f| f.path.clone()).collect();

        for file in memory_files {
            if !open_paths.contains(file.path()) {
                // Prefer the mode the file was last opened with (persists across
                // Close); fall back to a UTF-8 guess for files never opened via
                // Open (e.g. installed/uploaded directly).
                let is_text = match file.last_mode() {
                    Some(file_state::OpenMode::Input)
                    | Some(file_state::OpenMode::Output)
                    | Some(file_state::OpenMode::Append) => true,
                    Some(file_state::OpenMode::Random) | Some(file_state::OpenMode::Binary) => {
                        false
                    }
                    None => std::str::from_utf8(file.content()).is_ok(),
                };

                let (content_text, content_base64) = if file.exists() {
                    if is_text {
                        (
                            Some(String::from_utf8_lossy(file.content()).to_string()),
                            None,
                        )
                    } else {
                        let b64 = base64::engine::general_purpose::STANDARD.encode(file.content());
                        (None, Some(b64))
                    }
                } else {
                    (None, None)
                };

                files.push(WasmFile {
                    path: file.path().to_string(),
                    number: 0,
                    mode: "Closed".to_string(),
                    attributes: file.attributes(),
                    content_text,
                    content_base64,
                });
            }
        }
    }

    Ok(to_value(&WasmFileState {
        current_dir,
        current_drive,
        open_file_numbers: open_files,
        files,
    })?)
}
