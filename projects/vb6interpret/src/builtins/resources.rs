//! VB6 resource function registry.
//!
//! One [`Builtin`](super::Builtin) entry per resource function, each wrapping
//! the `vb6runtime::library::resources` implementation. All three read from the
//! single `.res` file bound via `vb6runtime::state::resources`.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::resources as resourcefn;
use vb6runtime::VBVariant;

/// Register the resource functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("loadresdata", 2, 2, |args| {
        resourcefn::loadresdata::loadresdata(&args[0], &args[1])
    }));
    registry.insert(builtin!("loadrespicture", 2, 2, |args| {
        resourcefn::loadrespicture::loadrespicture(&args[0], &args[1])
    }));
    registry.insert(builtin!("loadresstring", 1, 1, |args| {
        resourcefn::loadresstring::loadresstring(&args[0])
    }));
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use crate::builtins::call_builtin;
    use vb6runtime::state::{file, resources};
    use vb6runtime::VBVariant;

    /// Serializes these tests against each other: the file backend and the
    /// linked resource file are process-global, so parallel tests would
    /// otherwise overwrite each other's fixture.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    /// Builds a `.res` record with an ordinal type and name.
    fn record(res_type: u16, name: u16, data: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(data.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&32u32.to_le_bytes());
        bytes.extend_from_slice(&0xFFFFu16.to_le_bytes());
        bytes.extend_from_slice(&res_type.to_le_bytes());
        bytes.extend_from_slice(&0xFFFFu16.to_le_bytes());
        bytes.extend_from_slice(&name.to_le_bytes());
        bytes.extend_from_slice(&[0u8; 16]);
        bytes.extend_from_slice(data);
        bytes.resize(bytes.len().next_multiple_of(4), 0);
        bytes
    }

    /// A string bundle holding `text` in slot 0.
    fn string_bundle(text: &str) -> Vec<u8> {
        let mut bytes = Vec::new();
        let units: Vec<u16> = text.encode_utf16().collect();
        bytes.extend_from_slice(&(units.len() as u16).to_le_bytes());
        for unit in units {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        // Remaining 15 slots are empty.
        for _ in 1..16 {
            bytes.extend_from_slice(&0u16.to_le_bytes());
        }
        bytes
    }

    /// Links a `.res` image through the memory backend and runs `f`.
    fn with_linked_res<T>(image: &[u8], f: impl FnOnce() -> T) -> T {
        let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _ = file::close_all_files();
        file::set_backend(Box::new(file::memory::MemoryBackend::new()));
        file::set_root("/");
        file::write_memory_file("/app.res", image).unwrap();
        resources::clear();
        resources::set_file("/app.res");

        let result = f();

        resources::clear();
        let _ = file::close_all_files();
        file::reset_backend();
        result
    }

    #[test]
    fn loadresstring_dispatches_by_name() {
        let mut image = record(0, 0, &[]);
        image.extend(record(6, 1, &string_bundle("hello")));

        with_linked_res(&image, || {
            // Case-insensitive, as VB6 identifiers are.
            let value = call_builtin("LoadResString", &[VBVariant::from_integer(0)]).unwrap();
            assert_eq!(value.as_str(), Some("hello"));
        });
    }

    #[test]
    fn loadresdata_dispatches_by_name() {
        let mut image = record(0, 0, &[]);
        image.extend(record(10, 101, b"bytes"));

        with_linked_res(&image, || {
            let value = call_builtin(
                "LoadResData",
                &[VBVariant::from_integer(101), VBVariant::from_integer(10)],
            )
            .unwrap();
            assert_eq!(value.as_array().unwrap().len(), 5);
        });
    }

    #[test]
    fn loadrespicture_dispatches_by_name() {
        let mut header = vec![0u8; 40];
        header[0..4].copy_from_slice(&40u32.to_le_bytes());
        header[4..8].copy_from_slice(&24i32.to_le_bytes());
        header[8..12].copy_from_slice(&12i32.to_le_bytes());
        let mut image = record(0, 0, &[]);
        image.extend(record(2, 1, &header));

        with_linked_res(&image, || {
            let value = call_builtin(
                "LoadResPicture",
                &[VBVariant::from_integer(1), VBVariant::from_integer(0)],
            )
            .unwrap();
            assert_eq!(value.as_object().unwrap().type_name(), "StdPicture");
        });
    }

    #[test]
    fn wrong_argument_count_is_rejected() {
        // LoadResString takes exactly one argument.
        let error = call_builtin("LoadResString", &[]).unwrap_err();
        assert_eq!(error.number, 450);

        let error = call_builtin(
            "LoadResString",
            &[VBVariant::from_integer(1), VBVariant::from_integer(2)],
        )
        .unwrap_err();
        assert_eq!(error.number, 450);
    }
}
