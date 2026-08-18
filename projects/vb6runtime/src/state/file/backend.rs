//! Trait abstracting over different file I/O backends.
//!
//! VB6 file operations (Open, Close, Read, Write, etc.) need a pluggable backend
//! to support both native platforms and WASM environments:
//!
//! - **Windows/Linux/macOS**: [`NativeBackend`](super::native::NativeBackend) uses actual file system calls
//! - **WASM**: [`MemoryBackend`](super::memory::MemoryBackend) stores files in memory
//!
//! The backend handles all low-level file operations while the state module
//! manages file handle allocation and VB6-specific semantics.

use std::io;
use std::path::{Path, PathBuf};

/// File open modes matching VB6's Open statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenMode {
    /// Sequential input (read only).
    Input,
    /// Sequential output (write, truncate if exists).
    Output,
    /// Sequential append (write at end).
    Append,
    /// Random access (read/write with fixed-length records).
    Random,
    /// Binary access (read/write with byte-level access).
    Binary,
}

/// File access permissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    /// Read only.
    Read,
    /// Write only.
    Write,
    /// Read and write.
    ReadWrite,
}

/// File locking mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockMode {
    /// No locking (default).
    Shared,
    /// Others cannot read.
    LockRead,
    /// Others cannot write.
    LockWrite,
    /// Others cannot read or write.
    LockReadWrite,
}

/// Represents an open file handle with its state.
#[derive(Debug, Clone)]
pub struct OpenFile {
    /// The file number (1-511).
    pub number: i16,
    /// The file path.
    pub path: String,
    /// The open mode.
    pub mode: OpenMode,
    /// The access mode.
    pub access: AccessMode,
    /// The lock mode.
    pub lock: LockMode,
    /// Record length (for Random mode).
    pub record_length: i32,
    /// Current position in the file.
    pub position: i64,
}

/// Abstraction over file I/O operations.
///
/// Implementations must be `Send` so the backend can be shared across threads
/// via a `Mutex<Box<dyn FileBackend>>`.
pub trait FileBackend: Send {
    /// Open a file and return its handle.
    fn open(
        &mut self,
        path: &Path,
        mode: OpenMode,
        access: AccessMode,
        lock: LockMode,
        record_length: i32,
    ) -> io::Result<OpenFile>;

    /// Close a file handle.
    fn close(&mut self, file: &OpenFile) -> io::Result<()>;

    /// Read bytes from a file at the current position.
    /// Returns the number of bytes actually read.
    fn read(&mut self, file: &mut OpenFile, buf: &mut [u8]) -> io::Result<usize>;

    /// Write bytes to a file at the current position.
    fn write(&mut self, file: &mut OpenFile, buf: &[u8]) -> io::Result<usize>;

    /// Seek to a position in the file.
    fn seek(&mut self, file: &mut OpenFile, position: i64) -> io::Result<i64>;

    /// Get the length of a file in bytes.
    fn file_len(&mut self, path: &Path) -> io::Result<i64>;

    /// Check if a file exists.
    fn file_exists(&mut self, path: &Path) -> bool;

    /// Get the current position in a file (1-based for VB6).
    fn position(&self, file: &OpenFile) -> i64;

    /// Get the length of a file that is open.
    fn lof(&mut self, file: &OpenFile) -> io::Result<i64>;

    /// Copy a file from src to dst.
    fn copy_file(&mut self, src: &Path, dst: &Path) -> io::Result<()>;

    /// Rename/move a file from old_path to new_path.
    fn rename_file(&mut self, old_path: &Path, new_path: &Path) -> io::Result<()>;

    /// Delete a file.
    fn remove_file(&mut self, path: &Path) -> io::Result<()>;

    /// Create a directory.
    fn create_dir(&mut self, path: &Path) -> io::Result<()>;

    /// Remove a directory (must be empty).
    fn remove_dir(&mut self, path: &Path) -> io::Result<()>;

    /// Get file attributes as VB6-style bit flags.
    ///
    /// Returns a bitfield: bit 0 = readonly, bit 1 = hidden, bit 2 = system,
    /// bit 4 = directory, bit 5 = archive.
    fn get_attrs(&mut self, path: &Path) -> io::Result<i16>;

    /// Set file attributes from VB6-style bit flags.
    ///
    /// Only the readonly attribute is supported cross-platform; hidden/system
    /// are silently ignored on non-Windows.
    fn set_attrs(&mut self, path: &Path, attrs: i16) -> io::Result<()>;

    /// Get the last-modified time of a file as a `SystemTime`.
    fn file_datetime(&mut self, path: &Path) -> io::Result<std::time::SystemTime>;

    // Current directory management

    /// Get the current working directory.
    fn current_dir(&mut self) -> io::Result<PathBuf>;

    /// Set the current working directory.
    fn set_current_dir(&mut self, path: &Path) -> io::Result<()>;

    /// Get a list of available drive letters.
    fn drives(&self) -> Vec<char>;

    /// Get the current directory for a specific drive.
    ///
    /// On non-Windows, only the default drive is meaningful.
    fn current_dir_for_drive(&mut self, drive: char) -> io::Result<PathBuf>;

    /// Set the current drive letter.
    ///
    /// On non-Windows, this only changes the tracked drive; it does not
    /// affect the actual process working directory.
    fn set_current_drive(&mut self, drive: char) -> io::Result<()>;

    // File locking

    /// Lock a region (or entire file) for exclusive access.
    ///
    /// `record_range` is `(start, end)` where `end >= start`. Both are 1-based.
    /// For Binary/Input/Output modes the entire file is locked regardless.
    fn lock_file(
        &mut self,
        path: &Path,
        record_range: Option<(i32, i32)>,
    ) -> io::Result<()>;

    /// Unlock a previously locked region.
    fn unlock_file(
        &mut self,
        path: &Path,
        record_range: Option<(i32, i32)>,
    ) -> io::Result<()>;
}
