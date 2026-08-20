//! File I/O state management for VB6.
//!
//! VB6 uses file numbers (1-511) to identify open files. This module manages
//! file handle allocation and provides functions for file operations.
//!
//! The file backend can be switched at runtime with [`set_backend`], or you can
//! use [`set_root`] to point the native backend at a custom directory.
//!
//! # File Number Ranges
//!
//! - Range 0 (default): File numbers 1-255
//! - Range 1: File numbers 256-511

pub mod backend;
pub mod memory;
pub mod native;

use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

pub use backend::{AccessMode, FileBackend, LockMode, OpenFile, OpenMode};

/// Minimum valid file number for VB6 file I/O.
pub const MIN_FILE_NUMBER: i16 = 1;
/// Maximum valid file number for VB6 file I/O.
pub const MAX_FILE_NUMBER: i16 = 511;

/// Minimum valid record length for Random mode in VB6 file I/O.
pub const MIN_RECORD_NUMBER: i32 = 0;
/// Maximum valid record length for Random mode in VB6 file I/O.
pub const MAX_RECORD_NUMBER: i32 = 32767;

/// The active file backend.
static BACKEND: OnceLock<Mutex<Box<dyn FileBackend>>> = OnceLock::new();

/// The file root directory for relative paths.
static ROOT: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

thread_local! {
    /// Map from file number to open file.
    static OPEN_FILES: RefCell<HashMap<i16, OpenFile>> = RefCell::new(HashMap::new());
}

/// Get the active backend, initializing with default if needed.
fn backend() -> &'static Mutex<Box<dyn FileBackend>> {
    BACKEND.get_or_init(|| Mutex::new(default_backend()))
}

/// Get the root directory for relative paths.
fn root() -> &'static Mutex<Option<PathBuf>> {
    ROOT.get_or_init(|| Mutex::new(None))
}

/// Create the default backend for the current platform.
fn default_backend() -> Box<dyn FileBackend> {
    if cfg!(target_arch = "wasm32") {
        Box::new(memory::MemoryBackend::new())
    } else {
        Box::new(native::NativeBackend::new())
    }
}

/// Set the active file backend.
///
/// This is the primary way to switch storage backends at runtime.
pub fn set_backend(new_backend: Box<dyn FileBackend>) {
    let mut backend_guard = backend().lock().unwrap_or_else(|e| e.into_inner());
    *backend_guard = new_backend;
}

/// Reset to the default backend for the current platform.
pub fn reset_backend() {
    set_backend(default_backend());
}

/// Set the root directory for relative file paths.
///
/// If not set, the current working directory is used.
pub fn set_root(path: impl Into<PathBuf>) {
    let mut root_guard = root().lock().unwrap_or_else(|e| e.into_inner());
    *root_guard = Some(path.into());
}

/// Get the root directory for relative paths.
///
/// Falls back to the current working directory if not set, or to `/` if the
/// current working directory can't be queried (e.g. on `wasm32-unknown-unknown`,
/// which has no OS-backed working directory).
pub fn get_root() -> PathBuf {
    let root_guard = root().lock().unwrap_or_else(|e| e.into_inner());
    root_guard
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")))
}

/// Resolve a file path, making it absolute if it's relative.
pub fn resolve_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        let root = get_root();
        root.join(path)
    }
}

/// Open a file and associate it with a file number.
pub fn open_file(
    path: &Path,
    mode: OpenMode,
    access: AccessMode,
    lock: LockMode,
    record_length: i32,
    file_number: i16,
) -> io::Result<()> {
    // Check if file number is already in use
    OPEN_FILES.with(|files| {
        if files.borrow().contains_key(&file_number) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("File number {} is already in use", file_number),
            ));
        }
        Ok(())
    })?;

    // Resolve the path
    let resolved_path = resolve_path(path);

    // Open the file through the backend
    let mut file = backend().lock().unwrap_or_else(|e| e.into_inner()).open(
        &resolved_path,
        mode,
        access,
        lock,
        record_length,
    )?;

    // Set the file number
    file.number = file_number;

    // Store the file handle
    OPEN_FILES.with(|files| {
        files.borrow_mut().insert(file_number, file);
    });

    Ok(())
}

/// Close a file by its file number.
pub fn close_file(file_number: i16) -> io::Result<()> {
    OPEN_FILES.with(|files| {
        let mut files = files.borrow_mut();
        if let Some(file) = files.remove(&file_number) {
            backend()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .close(&file)?;
        }
        Ok(())
    })
}

/// Close all open files.
pub fn close_all_files() -> io::Result<()> {
    OPEN_FILES.with(|files| {
        let mut files = files.borrow_mut();
        let numbers: Vec<i16> = files.keys().copied().collect();
        for number in numbers {
            if let Some(file) = files.remove(&number) {
                backend()
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .close(&file)?;
            }
        }
        Ok(())
    })
}

/// Read bytes from an open file.
pub fn read_file(file_number: i16, buf: &mut [u8]) -> io::Result<usize> {
    OPEN_FILES.with(|files| {
        let mut files = files.borrow_mut();
        let file = files
            .get_mut(&file_number)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;

        backend()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .read(file, buf)
    })
}

/// Write bytes to an open file.
pub fn write_file(file_number: i16, buf: &[u8]) -> io::Result<usize> {
    OPEN_FILES.with(|files| {
        let mut files = files.borrow_mut();
        let file = files
            .get_mut(&file_number)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;

        backend()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .write(file, buf)
    })
}

/// Seek to a position in an open file (1-based for VB6).
pub fn seek_file(file_number: i16, position: i64) -> io::Result<i64> {
    OPEN_FILES.with(|files| {
        let mut files = files.borrow_mut();
        let file = files
            .get_mut(&file_number)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;

        // Convert 1-based VB6 position to 0-based
        let pos = position - 1;
        if pos < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid file position",
            ));
        }

        backend()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .seek(file, pos)
    })
}

/// Get the current position in an open file (1-based for VB6).
pub fn position_file(file_number: i16) -> io::Result<i64> {
    OPEN_FILES.with(|files| {
        let files = files.borrow();
        let file = files
            .get(&file_number)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;

        // VB6 positions are 1-based
        Ok(file.position + 1)
    })
}

/// Get the length of an open file.
pub fn lof_file(file_number: i16) -> io::Result<i64> {
    OPEN_FILES.with(|files| {
        let files = files.borrow();
        let file = files
            .get(&file_number)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;

        backend()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .lof(file)
    })
}

/// Get the length of a file by path.
pub fn file_len(path: &Path) -> io::Result<i64> {
    let resolved_path = resolve_path(path);
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .file_len(&resolved_path)
}

/// Check if a file exists.
pub fn file_exists(path: &Path) -> bool {
    let resolved_path = resolve_path(path);
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .file_exists(&resolved_path)
}

/// Check if a file number is in use.
pub fn is_file_open(file_number: i16) -> bool {
    OPEN_FILES.with(|files| files.borrow().contains_key(&file_number))
}

/// Get the next available file number in the specified range.
pub fn free_file(range: i16) -> i16 {
    let (min, max) = if range == 1 { (256, 511) } else { (1, 255) };

    OPEN_FILES.with(|files| {
        let files = files.borrow();
        for num in min..=max {
            if !files.contains_key(&num) {
                return num;
            }
        }
        // No free file numbers available
        0
    })
}

/// Get the open file handle for a file number.
pub fn get_file(file_number: i16) -> Option<OpenFile> {
    OPEN_FILES.with(|files| files.borrow().get(&file_number).cloned())
}

/// Get all open files as a vector of (file_number, OpenFile) pairs.
pub fn get_open_files() -> Vec<(i16, OpenFile)> {
    OPEN_FILES.with(|files| {
        files
            .borrow()
            .iter()
            .map(|(&num, file)| (num, file.clone()))
            .collect()
    })
}

/// Read the entire content of an open file as a vector of bytes.
pub fn read_file_to_vec(file_number: i16) -> io::Result<Vec<u8>> {
    OPEN_FILES.with(|files| {
        let files = files.borrow();
        let file = files
            .get(&file_number)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;
        let path = Path::new(&file.path);
        let resolved = resolve_path(path);

        // Read the file content through the backend
        let mut backend = backend().lock().unwrap_or_else(|e| e.into_inner());

        // Get the file length
        let len = backend.file_len(&resolved)?;

        // Create a buffer and read the content
        let mut buf = vec![0u8; len as usize];
        let mut file_clone = file.clone();
        file_clone.position = 0;
        let bytes_read = backend.read(&mut file_clone, &mut buf)?;
        buf.truncate(bytes_read);
        Ok(buf)
    })
}

/// List all files in the memory backend with their attributes and content.
/// Returns (path, attributes, content) for each file.
pub fn list_memory_files() -> io::Result<Vec<memory::VirtualFile>> {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .as_any()
        .downcast_ref::<memory::MemoryBackend>()
        .map(|memory| memory.files().values().cloned().collect())
        .ok_or_else(|| io::Error::other("Backend is not a memory backend"))
}

/// Write `content` directly into the memory backend at `path`, creating or
/// replacing the file without going through `Open`/`Close` (e.g. to restore a
/// saved snapshot). Fails if the active backend is not a memory backend.
pub fn write_memory_file(path: &str, content: &[u8]) -> io::Result<()> {
    let resolved_path = resolve_path(Path::new(path));
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .as_any_mut()
        .downcast_mut::<memory::MemoryBackend>()
        .map(|memory| memory.insert_file(&resolved_path.to_string_lossy(), content.to_vec()))
        .ok_or_else(|| io::Error::other("Backend is not a memory backend"))
}

/// Get the open file handle for a file number (mutable reference).
pub fn with_file_mut<T>(file_number: i16, f: impl FnOnce(&mut OpenFile) -> T) -> io::Result<T> {
    OPEN_FILES.with(|files| {
        let mut files = files.borrow_mut();
        let file = files
            .get_mut(&file_number)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;
        Ok(f(file))
    })
}

/// Reset all file state (for testing).
pub fn reset() {
    let _ = close_all_files();
    reset_backend();
}

/// Set the root directory and reset all file state (for testing).
pub fn reset_with_root(path: impl Into<PathBuf>) {
    let _ = close_all_files();
    set_root(path);
}

/// Copy a file from src to dst through the backend.
pub fn copy_file(src: &Path, dst: &Path) -> io::Result<()> {
    let resolved_src = resolve_path(src);
    let resolved_dst = resolve_path(dst);
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .copy_file(&resolved_src, &resolved_dst)
}

/// Rename/move a file through the backend.
pub fn rename_file(old_path: &Path, new_path: &Path) -> io::Result<()> {
    let resolved_old = resolve_path(old_path);
    let resolved_new = resolve_path(new_path);
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .rename_file(&resolved_old, &resolved_new)
}

/// Delete a file through the backend.
pub fn remove_file(path: &Path) -> io::Result<()> {
    let resolved_path = resolve_path(path);
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove_file(&resolved_path)
}

/// Create a directory through the backend.
pub fn create_dir(path: &Path) -> io::Result<()> {
    let resolved_path = resolve_path(path);
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .create_dir(&resolved_path)
}

/// Remove a directory through the backend.
pub fn remove_dir(path: &Path) -> io::Result<()> {
    let resolved_path = resolve_path(path);
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove_dir(&resolved_path)
}

/// Get file attributes as VB6-style bit flags through the backend.
pub fn get_attrs(path: &Path) -> io::Result<i16> {
    let resolved_path = resolve_path(path);
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get_attrs(&resolved_path)
}

/// Set file attributes from VB6-style bit flags through the backend.
pub fn set_attrs(path: &Path, attrs: i16) -> io::Result<()> {
    let resolved_path = resolve_path(path);
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .set_attrs(&resolved_path, attrs)
}

/// Get the last-modified time of a file through the backend.
pub fn file_datetime(path: &Path) -> io::Result<std::time::SystemTime> {
    let resolved_path = resolve_path(path);
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .file_datetime(&resolved_path)
}

/// Get the current working directory through the backend.
pub fn current_dir() -> io::Result<PathBuf> {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .current_dir()
}

/// Set the current working directory through the backend.
pub fn set_current_dir(path: &Path) -> io::Result<()> {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .set_current_dir(path)
}

/// Get a list of available drive letters through the backend.
pub fn drives() -> Vec<char> {
    backend().lock().unwrap_or_else(|e| e.into_inner()).drives()
}

/// Get the current directory for a specific drive through the backend.
pub fn current_dir_for_drive(drive: char) -> io::Result<PathBuf> {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .current_dir_for_drive(drive)
}

/// Set the current drive through the backend.
pub fn set_current_drive(drive: char) -> io::Result<()> {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .set_current_drive(drive)
}

/// Lock a region of an open file for exclusive access.
///
/// `file_number` is the VB6 file number. `record_range` is an optional
/// `(start, end)` pair (1-based, inclusive). When `None`, the entire file
/// is locked.
pub fn lock_file(file_number: i16, record_range: Option<(i32, i32)>) -> io::Result<()> {
    OPEN_FILES.with(|files| {
        let files = files.borrow();
        let file = files
            .get(&file_number)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;
        let path = Path::new(&file.path);
        let resolved = resolve_path(path);
        backend()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .lock_file(&resolved, record_range)
    })
}

/// Unlock a region of an open file.
pub fn unlock_file(file_number: i16, record_range: Option<(i32, i32)>) -> io::Result<()> {
    OPEN_FILES.with(|files| {
        let files = files.borrow();
        let file = files
            .get(&file_number)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;
        let path = Path::new(&file.path);
        let resolved = resolve_path(path);
        backend()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .unlock_file(&resolved, record_range)
    })
}

// ── Print column tracking (for Tab/Spc) ──────────────────────────────────

/// Get the current 1-based print column for `file_number`.
///
/// Returns 1 (start of line) if the file has no recorded position.
pub fn get_print_column(file_number: i16) -> usize {
    OPEN_FILES.with(|files| {
        files
            .borrow()
            .get(&file_number)
            .map(|f| f.print_column)
            .unwrap_or(1)
    })
}

/// Set the current 1-based print column for `file_number`.
pub fn set_print_column(file_number: i16, column: usize) {
    OPEN_FILES.with(|files| {
        if let Some(file) = files.borrow_mut().get_mut(&file_number) {
            file.print_column = column;
        }
    });
}

/// Advance the print column by `count` characters.
///
/// Called after emitting output so subsequent Tab/Spc calls see the correct position.
pub fn advance_print_column(file_number: i16, count: usize) {
    OPEN_FILES.with(|files| {
        if let Some(file) = files.borrow_mut().get_mut(&file_number) {
            file.print_column += count;
        }
    });
}

/// Reset the print column to 1 (start of line) for `file_number`.
///
/// Called after a newline is written.
pub fn reset_print_column(file_number: i16) {
    set_print_column(file_number, 1);
}

/// Default print zone width (columns between tab stops) — VB6 default is 14.
pub const DEFAULT_ZONE_WIDTH: usize = 14;

/// Return the default zone width (14 columns).
pub fn zone_width() -> usize {
    DEFAULT_ZONE_WIDTH
}

/// Get a list of files in a directory matching a pattern and attributes through the backend.
pub fn file_dir(path: &Path, pattern: &str, attributes: i16) -> io::Result<Vec<String>> {
    let resolved_path = resolve_path(path);
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .file_dir(&resolved_path, pattern, attributes)
}

/// Match a filename against a pattern (supports * and ? wildcards).
///
/// This is a utility function that works the same way for both the native
/// and memory backends. The pattern matching logic:
/// - If pattern contains no wildcards (* or ?), does a case-insensitive substring match
/// - If pattern is "*", matches everything
/// - Otherwise, does case-insensitive substring match
pub fn matches_wildcard(file_name: &str, pattern: &str) -> bool {
    // If pattern doesn't contain wildcards, check substring match (case-insensitive)
    if !pattern.contains('*') && !pattern.contains('?') {
        return file_name.to_lowercase().contains(&pattern.to_lowercase());
    }

    // Simple wildcard: * matches everything, ? matches single character
    // For pattern "*", match all
    if pattern == "*" {
        return true;
    }

    // Very simple: check if pattern is a substring (case-insensitive)
    file_name.to_lowercase().contains(&pattern.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn free_file_returns_lowest_available() {
        let _guard = crate::state::test_support::lock_test();
        let _ = close_all_files();

        assert_eq!(free_file(0), 1);

        // Open file 1
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        assert_eq!(free_file(0), 2);

        // Open file 2
        let path = dir.path().join("test2.txt");
        open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            2,
        )
        .unwrap();

        assert_eq!(free_file(0), 3);

        // Close file 1
        close_file(1).unwrap();
        assert_eq!(free_file(0), 1);

        let _ = close_all_files();
    }

    #[test]
    fn free_file_high_range() {
        let _guard = crate::state::test_support::lock_test();
        let _ = close_all_files();

        assert_eq!(free_file(1), 256);

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            256,
        )
        .unwrap();

        assert_eq!(free_file(1), 257);

        let _ = close_all_files();
    }

    #[test]
    fn open_and_close_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = close_all_files();

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");

        open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();
        assert!(is_file_open(1));

        close_file(1).unwrap();
        assert!(!is_file_open(1));

        let _ = close_all_files();
    }

    #[test]
    fn write_and_read_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = close_all_files();

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");

        // Write
        open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();
        write_file(1, b"Hello, World!").unwrap();

        // Read
        open_file(
            &path,
            OpenMode::Input,
            AccessMode::Read,
            LockMode::Shared,
            0,
            2,
        )
        .unwrap();
        let mut buf = [0u8; 13];
        let bytes_read = read_file(2, &mut buf).unwrap();
        assert_eq!(bytes_read, 13);
        assert_eq!(&buf, b"Hello, World!");

        let _ = close_all_files();
    }

    #[test]
    fn file_exists_check() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");

        assert!(!file_exists(&path));

        open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();
        close_file(1).unwrap();

        assert!(file_exists(&path));

        let _ = close_all_files();
    }
}
