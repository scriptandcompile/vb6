//! The resource file linked to the running VB6 project.
//!
//! A VB6 project links exactly one `.res` file at compile time, and its
//! contents are embedded in the produced EXE/DLL. `LoadResData`,
//! `LoadResPicture`, and `LoadResString` all read from that single file, and
//! the program never names it — the resource ID alone identifies a resource.
//!
//! This module holds the equivalent process-global binding. A host sets the
//! path once before the program runs (mirroring the compile-time link), and the
//! `LoadRes*` functions read through it.
//!
//! The parsed file is cached on first use, since the embedded resources of a
//! running EXE cannot change underneath it. `LoadRes*` still returns a fresh
//! copy of the resource *data* on every call, as VB6 does.

use std::sync::{Mutex, OnceLock};

use crate::error::{err_number, VBError, VBResult};
use crate::library::resources::resfile::ResFile;

/// The linked resource file: its path, and the parsed file once loaded.
///
/// `None` means no resource file is linked, which is the state of a project
/// that never added one.
static RESOURCE_FILE: OnceLock<Mutex<Option<Linked>>> = OnceLock::new();

/// The linked `.res` file and its parse cache.
#[derive(Debug)]
struct Linked {
    /// Path the file is linked from.
    path: String,
    /// The parsed file, populated on first access.
    parsed: Option<ResFile>,
}

/// Access the process-global binding, initializing it as unlinked.
fn state() -> &'static Mutex<Option<Linked>> {
    RESOURCE_FILE.get_or_init(|| Mutex::new(None))
}

/// Link the `.res` file at `path` to the running program.
///
/// Mirrors adding a resource file to a VB6 project. The file is not read until
/// a `LoadRes*` call needs it, so linking a missing file fails at first use
/// rather than here.
pub fn set_file(path: impl Into<String>) {
    let mut guard = state().lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(Linked {
        path: path.into(),
        parsed: None,
    });
}

/// The path of the linked resource file, or `None` if none is linked.
pub fn file_path() -> Option<String> {
    let guard = state().lock().unwrap_or_else(|e| e.into_inner());
    guard.as_ref().map(|linked| linked.path.clone())
}

/// Unlink the resource file and drop any cached parse.
pub fn clear() {
    let mut guard = state().lock().unwrap_or_else(|e| e.into_inner());
    *guard = None;
}

/// Drop the cached parse but keep the file linked, forcing the next access to
/// re-read through the file backend.
pub fn invalidate_cache() {
    let mut guard = state().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(linked) = guard.as_mut() {
        linked.parsed = None;
    }
}

/// Run `f` against the linked resource file, loading and caching it if needed.
///
/// # Errors
///
/// - Error 326 (`Resource with identifier not found`) if no resource file is
///   linked. A program asking for a resource when the project has none is
///   indistinguishable, from the program's side, from asking for one that
///   isn't there.
/// - Whatever [`ResFile::load`] reports if the file is missing or malformed.
pub fn with_file<T>(f: impl FnOnce(&ResFile) -> VBResult<T>) -> VBResult<T> {
    let mut guard = state().lock().unwrap_or_else(|e| e.into_inner());
    let linked = guard
        .as_mut()
        .ok_or_else(|| VBError::new(err_number::RESOURCE_NOT_FOUND))?;

    if linked.parsed.is_none() {
        linked.parsed = Some(ResFile::load(&linked.path)?);
    }

    // Just populated above, so this cannot be None.
    let parsed = linked
        .parsed
        .as_ref()
        .expect("resource file was just loaded");
    f(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::file;

    /// A minimal well-formed `.res` file: the null record plus one `RT_RCDATA`
    /// record named 101.
    fn sample_res() -> Vec<u8> {
        let mut bytes = res_record(0, 0, &[]);
        bytes.extend(res_record(10, 101, b"payload"));
        bytes
    }

    /// Builds a `.res` record with an ordinal type and name.
    fn res_record(res_type: u16, name: u16, data: &[u8]) -> Vec<u8> {
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

    /// Installs a memory backend holding `sample_res()` at `/app.res`.
    fn with_linked_res<T>(f: impl FnOnce() -> T) -> T {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();
        file::set_backend(Box::new(file::memory::MemoryBackend::new()));
        file::set_root("/");
        file::write_memory_file("/app.res", &sample_res()).unwrap();
        clear();
        set_file("/app.res");

        let result = f();

        clear();
        let _ = file::close_all_files();
        file::reset_backend();
        result
    }

    #[test]
    fn unlinked_file_reports_resource_not_found() {
        let _guard = crate::state::test_support::lock_test();
        clear();
        let error = with_file(|_| Ok(())).unwrap_err();
        assert_eq!(error.number, err_number::RESOURCE_NOT_FOUND);
    }

    #[test]
    fn set_file_records_the_path() {
        let _guard = crate::state::test_support::lock_test();
        clear();
        set_file("/some/app.res");
        assert_eq!(file_path().as_deref(), Some("/some/app.res"));
        clear();
        assert_eq!(file_path(), None);
    }

    #[test]
    fn with_file_parses_the_linked_file() {
        with_linked_res(|| {
            let count = with_file(|res| Ok(res.entry_count())).unwrap();
            assert_eq!(count, 1);
        });
    }

    #[test]
    fn parse_is_cached_across_calls() {
        with_linked_res(|| {
            with_file(|res| Ok(res.entry_count())).unwrap();

            // Replace the file on disk. The cached parse must be returned,
            // so the entry count stays at the original value.
            file::write_memory_file("/app.res", &res_record(0, 0, &[])).unwrap();
            let cached = with_file(|res| Ok(res.entry_count())).unwrap();
            assert_eq!(cached, 1, "cached parse should be reused");

            // After invalidating, the new contents are read.
            invalidate_cache();
            let fresh = with_file(|res| Ok(res.entry_count())).unwrap();
            assert_eq!(fresh, 0);
        });
    }

    #[test]
    fn missing_linked_file_reports_file_not_found() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();
        file::set_backend(Box::new(file::memory::MemoryBackend::new()));
        file::set_root("/");
        clear();
        set_file("/absent.res");

        let error = with_file(|_| Ok(())).unwrap_err();
        assert_eq!(error.number, err_number::FILE_NOT_FOUND);

        clear();
        let _ = file::close_all_files();
        file::reset_backend();
    }
}
