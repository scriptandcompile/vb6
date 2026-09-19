//! Shared parsing helpers for WASM bridges.
//!
//! Extracted from `run_bridge.rs` so that both the one-shot module-mode
//! bridge (`run_bridge`) and the handle-based form-mode bridge
//! (`exec_bridge`) can reuse the same parsing and project-detection logic.

use wasm_bindgen::prelude::*;

use crate::project::{LoadedClass, LoadedForm, LoadedModule, StartupObject};
use vb6parse::files::ClassFile;
use vb6parse::files::FormFile;
use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;

/// Extract key-value pairs from a JS object (keys: strings, values: Uint8Array)
/// or a raw `Uint8Array` (treated as a single module/form file).
/// Returns `Vec<(filename, bytes)>` — for raw bytes the filename is empty.
pub(super) fn js_map_to_byte_pairs(map: &JsValue) -> Result<Vec<(String, Vec<u8>)>, JsError> {
    if map.is_undefined() || map.is_null() {
        return Ok(Vec::new());
    }

    // If the value is a raw Uint8Array, treat it as a single file.
    if let Ok(bytes) = serde_wasm_bindgen::from_value::<Vec<u8>>(map.clone()) {
        return Ok(vec![(String::new(), bytes)]);
    }

    let keys: js_sys::Array =
        js_sys::Reflect::own_keys(map).map_err(|_| JsError::new("failed to get object keys"))?;
    let len = keys.length();
    let mut result = Vec::with_capacity(len as usize);
    for i in 0..len {
        let key = keys.get(i);
        let key_str = key
            .as_string()
            .ok_or_else(|| JsError::new("map key is not a string"))?;
        let value =
            js_sys::Reflect::get(map, &key).map_err(|_| JsError::new("failed to get map value"))?;
        let bytes: Vec<u8> =
            serde_wasm_bindgen::from_value(value).map_err(|e| JsError::new(&e.to_string()))?;
        result.push((key_str, bytes));
    }
    Ok(result)
}

/// Parse a collection of `(filename, raw_bytes)` pairs into [`LoadedModule`] entries.
pub(super) fn parse_modules(pairs: Vec<(String, Vec<u8>)>) -> Result<Vec<LoadedModule>, JsError> {
    pairs
        .into_iter()
        .map(|(file_name, bytes)| {
            let source = SourceFile::decode_with_replacement(&file_name, &bytes).map_err(|e| {
                JsError::new(&format!("Failed to decode module '{}': {:?}", file_name, e))
            })?;
            let parsed = ModuleFile::parse(&source).unwrap_or_fail();
            Ok(LoadedModule {
                name: parsed.name.clone(),
                file_name,
                parsed,
                raw_bytes: bytes,
            })
        })
        .collect()
}

/// Parse a collection of `(filename, raw_bytes)` pairs into [`LoadedForm`] entries.
pub(super) fn parse_forms(pairs: Vec<(String, Vec<u8>)>) -> Result<Vec<LoadedForm>, JsError> {
    pairs
        .into_iter()
        .map(|(file_name, bytes)| {
            let source = SourceFile::decode_with_replacement(&file_name, &bytes).map_err(|e| {
                JsError::new(&format!("Failed to decode form '{}': {:?}", file_name, e))
            })?;
            let parsed = FormFile::parse(&source).unwrap_or_fail();
            Ok(LoadedForm {
                name: parsed.attributes.name.clone(),
                file_name,
                parsed,
                raw_bytes: bytes,
            })
        })
        .collect()
}

/// Parse a collection of `(filename, raw_bytes)` pairs into [`LoadedClass`] entries.
pub(super) fn parse_classes(pairs: Vec<(String, Vec<u8>)>) -> Result<Vec<LoadedClass>, JsError> {
    pairs
        .into_iter()
        .map(|(file_name, bytes)| {
            let source = SourceFile::decode_with_replacement(&file_name, &bytes).map_err(|e| {
                JsError::new(&format!("Failed to decode class '{}': {:?}", file_name, e))
            })?;
            let parsed = ClassFile::parse(&source).unwrap_or_fail();
            Ok(LoadedClass {
                name: parsed.header.attributes.name.clone(),
                file_name,
                parsed,
                raw_bytes: bytes,
            })
        })
        .collect()
}

/// Detect the startup object from a raw `startup` string and the loaded
/// forms/modules.
pub(super) fn detect_startup(
    startup: &str,
    forms: &[LoadedForm],
    modules: &[LoadedModule],
) -> StartupObject {
    let startup = startup.trim();
    if startup.is_empty() {
        return StartupObject::None;
    }

    let form_names: Vec<&str> = forms.iter().map(|f| f.name.as_str()).collect();
    let module_names: Vec<&str> = modules.iter().map(|m| m.name.as_str()).collect();

    if form_names.iter().any(|&n| n.eq_ignore_ascii_case(startup)) {
        let matched = form_names
            .iter()
            .find(|&n| n.eq_ignore_ascii_case(startup))
            .unwrap();
        return StartupObject::Form {
            form_name: matched.to_string(),
        };
    }

    if let Some(dot_pos) = startup.find('.') {
        let mod_name = &startup[..dot_pos];
        let sub_name = &startup[dot_pos + 1..];
        return StartupObject::SubMain {
            module_name: mod_name.to_string(),
            sub_name: sub_name.to_string(),
        };
    }

    if startup.eq_ignore_ascii_case("sub main") || startup.eq_ignore_ascii_case("main") {
        return StartupObject::SubMain {
            module_name: String::new(),
            sub_name: "Main".to_string(),
        };
    }

    if module_names
        .iter()
        .any(|&n| n.eq_ignore_ascii_case(startup))
    {
        return StartupObject::SubMain {
            module_name: startup.to_string(),
            sub_name: "Main".to_string(),
        };
    }

    StartupObject::None
}
