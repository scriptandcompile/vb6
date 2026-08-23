//! Bindings exposing and mutating the shared runtime state snapshot —
//! environment variables, application settings, the mock clock, and the
//! in-memory file system — for the browser IDE's environment tabs.

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use vb6runtime::state::clock as clock_state;
use vb6runtime::state::environment as env_state;
use vb6runtime::state::file as file_state;
use vb6runtime::state::settings as settings_state;
use wasm_bindgen::prelude::*;

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
