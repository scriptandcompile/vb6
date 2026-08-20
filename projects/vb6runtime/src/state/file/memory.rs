//! In-memory file I/O backend for WASM.
//!
//! Stores files entirely in memory with no persistence. Used for:
//!
//! - **WASM**: The JS playground syncs to/from the host
//! - **Tests**: Avoids filesystem side effects

use std::collections::{HashMap, HashSet};
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use super::backend::{AccessMode, FileBackend, LockMode, OpenFile, OpenMode};

/// A locked range: `(start, end)` inclusive, both 1-based.
type LockRange = (i32, i32);

/// The current wall-clock time, falling back to the Unix epoch on platforms
/// without one (e.g. `wasm32-unknown-unknown`, where `SystemTime::now()` panics).
fn current_time() -> SystemTime {
    if cfg!(target_arch = "wasm32") {
        SystemTime::UNIX_EPOCH
    } else {
        SystemTime::now()
    }
}

/// A virtual file stored in memory.
#[derive(Debug, Clone)]
pub struct VirtualFile {
    /// Absolute path of the virtual file.
    path: String,
    /// The file content.
    content: Vec<u8>,
    /// Whether the file exists.
    exists: bool,
    /// VB6-style attribute bitfield.
    attributes: i16,
    /// Last-modified timestamp.
    modified: SystemTime,
    /// The mode the file was last opened with, if any. Kept after `Close` so
    /// the Files tab can still tell binary files from text files.
    last_mode: Option<OpenMode>,
}

impl VirtualFile {
    /// Get the virtual file path.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Check if the file exists.
    pub fn exists(&self) -> bool {
        self.exists
    }

    /// Get the file content.
    pub fn content(&self) -> &[u8] {
        &self.content
    }

    /// Get the file attributes.
    pub fn attributes(&self) -> i16 {
        self.attributes
    }

    /// Get the mode the file was last opened with, if any.
    pub fn last_mode(&self) -> Option<OpenMode> {
        self.last_mode
    }

    /// Get the last-modified timestamp.
    pub fn modified(&self) -> SystemTime {
        self.modified
    }
}

/// In-memory file I/O backend.
///
/// All data is stored in a `HashMap`. This backend has no persistence;
/// for WASM hosts, the JS layer is responsible for syncing files.
pub struct MemoryBackend {
    /// Virtual filesystem: path -> content.
    files: HashMap<String, VirtualFile>,
    /// Set of directory paths.
    directories: HashSet<String>,
    /// Current working directory.
    current_dir: PathBuf,
    /// Current drive letter (tracked for VB6 semantics).
    current_drive: char,
    /// Per-drive current directories (VB6 tracks CWD per drive).
    drive_dirs: HashMap<char, PathBuf>,
    /// Locked regions per file path. `None` means the entire file is locked.
    locks: HashMap<String, Vec<Option<LockRange>>>,
}

impl MemoryBackend {
    /// Create a new empty in-memory backend.
    pub fn new() -> Self {
        let root = PathBuf::from("/");
        let mut drive_dirs = HashMap::new();
        drive_dirs.insert('C', root.clone());
        Self {
            files: HashMap::new(),
            directories: HashSet::new(),
            current_dir: root.clone(),
            current_drive: 'C',
            drive_dirs,
            locks: HashMap::new(),
        }
    }

    /// Create a pre-populated in-memory backend with a file.
    pub fn with_file(path: &str, content: Vec<u8>) -> Self {
        let root = PathBuf::from("/");
        let mut drive_dirs = HashMap::new();
        drive_dirs.insert('C', root.clone());
        let mut files = HashMap::new();
        files.insert(
            path.to_string(),
            VirtualFile {
                path: path.to_string(),
                content,
                exists: true,
                attributes: 0,
                modified: current_time(),
                last_mode: None,
            },
        );
        Self {
            files,
            directories: HashSet::new(),
            current_dir: root,
            current_drive: 'C',
            drive_dirs,
            locks: HashMap::new(),
        }
    }

    /// Get a reference to the virtual filesystem.
    pub fn files(&self) -> &HashMap<String, VirtualFile> {
        &self.files
    }

    /// Get a mutable reference to the virtual filesystem.
    pub fn files_mut(&mut self) -> &mut HashMap<String, VirtualFile> {
        &mut self.files
    }

    /// Create or replace the file at `path` with `content`, bypassing
    /// `Open`/`Close` semantics (e.g. for restoring a saved snapshot).
    pub fn insert_file(&mut self, path: &str, content: Vec<u8>) {
        self.files.insert(
            path.to_string(),
            VirtualFile {
                path: path.to_string(),
                content,
                exists: true,
                attributes: 0,
                modified: current_time(),
                last_mode: None,
            },
        );
    }
}

impl Default for MemoryBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl FileBackend for MemoryBackend {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn open(
        &mut self,
        path: &std::path::Path,
        mode: OpenMode,
        access: AccessMode,
        lock: LockMode,
        record_length: i32,
    ) -> io::Result<OpenFile> {
        let path_str = path.to_string_lossy().to_string();

        // Check if file exists
        let file_exists = self.files.contains_key(&path_str);

        // For Input mode, file must exist
        if mode == OpenMode::Input && !file_exists {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File not found: {}", path_str),
            ));
        }

        // Create or clear file based on mode
        match mode {
            OpenMode::Output => {
                // Truncate existing file
                self.files.insert(
                    path_str.clone(),
                    VirtualFile {
                        path: path_str.clone(),
                        content: Vec::new(),
                        exists: true,
                        attributes: 0,
                        modified: current_time(),
                        last_mode: Some(mode),
                    },
                );
            }
            OpenMode::Append => {
                // Create if doesn't exist, keep content if it does
                if !file_exists {
                    self.files.insert(
                        path_str.clone(),
                        VirtualFile {
                            path: path_str.clone(),
                            content: Vec::new(),
                            exists: true,
                            attributes: 0,
                            modified: current_time(),
                            last_mode: Some(mode),
                        },
                    );
                }
            }
            OpenMode::Random | OpenMode::Binary => {
                // Create if doesn't exist
                if !file_exists {
                    self.files.insert(
                        path_str.clone(),
                        VirtualFile {
                            path: path_str.clone(),
                            content: Vec::new(),
                            exists: true,
                            attributes: 0,
                            modified: current_time(),
                            last_mode: Some(mode),
                        },
                    );
                }
            }
            OpenMode::Input => {
                // File must exist, already checked above
            }
        }

        // Remember the mode used, even for pre-existing files being reopened,
        // so the Files tab can distinguish binary content from text after Close.
        if let Some(virtual_file) = self.files.get_mut(&path_str) {
            virtual_file.last_mode = Some(mode);
        }

        // Calculate initial position for Append mode
        let initial_position = if mode == OpenMode::Append {
            self.files
                .get(&path_str)
                .map(|f| f.content.len() as i64)
                .unwrap_or(0)
        } else {
            0
        };

        Ok(OpenFile {
            number: 0, // Will be set by the caller
            path: path_str,
            mode,
            access,
            lock,
            record_length,
            position: initial_position,
            width: 0,        // Default: no line length limit
            print_column: 1, // Start of line
        })
    }

    fn close(&mut self, _file: &OpenFile) -> io::Result<()> {
        // Nothing to do for memory backend
        Ok(())
    }

    fn read(&mut self, file: &mut OpenFile, buf: &mut [u8]) -> io::Result<usize> {
        let virtual_file = self
            .files
            .get(&file.path)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;

        let pos = file.position as usize;
        if pos >= virtual_file.content.len() {
            return Ok(0); // EOF
        }

        let available = virtual_file.content.len() - pos;
        let to_read = buf.len().min(available);

        buf[..to_read].copy_from_slice(&virtual_file.content[pos..pos + to_read]);
        file.position += to_read as i64;

        Ok(to_read)
    }

    fn write(&mut self, file: &mut OpenFile, buf: &[u8]) -> io::Result<usize> {
        let virtual_file = self
            .files
            .get_mut(&file.path)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;

        let pos = file.position as usize;

        // Ensure buffer is large enough
        if pos + buf.len() > virtual_file.content.len() {
            virtual_file.content.resize(pos + buf.len(), 0);
        }

        virtual_file.content[pos..pos + buf.len()].copy_from_slice(buf);
        file.position += buf.len() as i64;

        Ok(buf.len())
    }

    fn seek(&mut self, file: &mut OpenFile, position: i64) -> io::Result<i64> {
        // Update the file's position
        file.position = position;
        Ok(file.position)
    }

    fn file_len(&mut self, path: &std::path::Path) -> io::Result<i64> {
        let path_str = path.to_string_lossy().to_string();
        self.files
            .get(&path_str)
            .filter(|f| f.exists)
            .map(|f| f.content.len() as i64)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }

    fn file_exists(&mut self, path: &std::path::Path) -> bool {
        let path_str = path.to_string_lossy().to_string();
        self.files.get(&path_str).map(|f| f.exists).unwrap_or(false)
    }

    fn file_dir(
        &mut self,
        path: &std::path::Path,
        pattern: &str,
        attributes: i16,
    ) -> io::Result<Vec<String>> {
        let path_str = path.to_string_lossy().to_string();
        let mut results = Vec::new();

        // Check if path is a directory in memory
        if self.directories.contains(&path_str) || path_str == "/" {
            // List all files and directories
            for (file_path, virtual_file) in &self.files {
                if virtual_file.exists {
                    let file_name = Path::new(file_path)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    // Skip "." and ".."
                    if file_name == "." || file_name == ".." {
                        continue;
                    }

                    // Check pattern match using utility function
                    let matches_pattern = crate::state::file::matches_wildcard(&file_name, pattern);

                    // Get attributes
                    let mut file_attrs: i16 = 0;
                    if self.directories.contains(file_path) {
                        file_attrs |= 16; // vbDirectory
                    } else {
                        file_attrs |= 32; // vbArchive (default for files)
                    }

                    // Match attributes if specified
                    let attr_match = if attributes == 0 || attributes == file_attrs {
                        true
                    } else {
                        (attributes & file_attrs) == attributes
                    };

                    if matches_pattern && attr_match {
                        results.push(file_name);
                    }
                }
            }
        }

        Ok(results)
    }

    fn position(&self, file: &OpenFile) -> i64 {
        // VB6 positions are 1-based
        file.position + 1
    }

    fn lof(&mut self, file: &OpenFile) -> io::Result<i64> {
        self.files
            .get(&file.path)
            .filter(|f| f.exists)
            .map(|f| f.content.len() as i64)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))
    }

    fn copy_file(&mut self, src: &std::path::Path, dst: &std::path::Path) -> io::Result<()> {
        let src_str = src.to_string_lossy().to_string();
        let dst_str = dst.to_string_lossy().to_string();

        let src_file = self
            .files
            .get(&src_str)
            .filter(|f| f.exists)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))?;

        self.files.insert(dst_str, src_file);
        Ok(())
    }

    fn rename_file(
        &mut self,
        old_path: &std::path::Path,
        new_path: &std::path::Path,
    ) -> io::Result<()> {
        let old_str = old_path.to_string_lossy().to_string();
        let new_str = new_path.to_string_lossy().to_string();

        let file = self
            .files
            .remove(&old_str)
            .filter(|f| f.exists)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))?;

        self.files.insert(new_str, file);
        Ok(())
    }

    fn remove_file(&mut self, path: &std::path::Path) -> io::Result<()> {
        let path_str = path.to_string_lossy().to_string();
        match self.files.remove(&path_str) {
            Some(f) if f.exists => Ok(()),
            _ => Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
        }
    }

    fn create_dir(&mut self, path: &std::path::Path) -> io::Result<()> {
        let path_str = path.to_string_lossy().to_string();
        if self.directories.contains(&path_str) || self.files.contains_key(&path_str) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Directory already exists",
            ));
        }
        self.directories.insert(path_str);
        Ok(())
    }

    fn remove_dir(&mut self, path: &std::path::Path) -> io::Result<()> {
        let path_str = path.to_string_lossy().to_string();
        if !self.directories.remove(&path_str) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Directory not found",
            ));
        }
        Ok(())
    }

    fn get_attrs(&mut self, path: &std::path::Path) -> io::Result<i16> {
        let path_str = path.to_string_lossy().to_string();

        // Check if it's a directory
        if self.directories.contains(&path_str) {
            return Ok(16); // VB_DIRECTORY
        }

        // Check if it's a file
        self.files
            .get(&path_str)
            .filter(|f| f.exists)
            .map(|f| f.attributes)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }

    fn set_attrs(&mut self, path: &std::path::Path, attrs: i16) -> io::Result<()> {
        let path_str = path.to_string_lossy().to_string();
        match self.files.get_mut(&path_str) {
            Some(f) if f.exists => {
                f.attributes = attrs;
                Ok(())
            }
            _ => Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
        }
    }

    fn file_datetime(&mut self, path: &std::path::Path) -> io::Result<SystemTime> {
        let path_str = path.to_string_lossy().to_string();
        self.files
            .get(&path_str)
            .filter(|f| f.exists)
            .map(|f| f.modified)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }

    fn current_dir(&mut self) -> io::Result<PathBuf> {
        Ok(self.current_dir.clone())
    }

    fn set_current_dir(&mut self, path: &Path) -> io::Result<()> {
        let path_str = path.to_string_lossy().to_string();
        // Validate the path exists as a directory (root always exists)
        if path_str != "/" && !self.directories.contains(&path_str) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Path not found: {}", path_str),
            ));
        }
        self.current_dir = path.to_path_buf();
        self.drive_dirs
            .insert(self.current_drive, path.to_path_buf());
        Ok(())
    }

    fn drives(&self) -> Vec<char> {
        vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']
    }

    fn current_dir_for_drive(&mut self, drive: char) -> io::Result<PathBuf> {
        self.drive_dirs
            .get(&drive)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Drive not available"))
    }

    fn set_current_drive(&mut self, drive: char) -> io::Result<()> {
        self.current_drive = drive.to_ascii_uppercase();
        // Ensure the drive has a directory entry
        self.drive_dirs
            .entry(self.current_drive)
            .or_insert_with(|| PathBuf::from("/"));
        Ok(())
    }

    fn lock_file(&mut self, path: &Path, record_range: Option<(i32, i32)>) -> io::Result<()> {
        let path_str = path.to_string_lossy().to_string();
        let existing = self.locks.entry(path_str).or_default();

        for existing_lock in existing.iter() {
            match (existing_lock, &record_range) {
                (None, _) | (_, None) => {
                    return Err(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        "File already locked",
                    ));
                }
                (Some((e_start, e_end)), Some((n_start, n_end))) => {
                    if n_start <= e_end && n_end >= e_start {
                        return Err(io::Error::new(
                            io::ErrorKind::PermissionDenied,
                            "File already locked",
                        ));
                    }
                }
            }
        }

        existing.push(record_range);
        Ok(())
    }

    fn unlock_file(&mut self, path: &Path, record_range: Option<(i32, i32)>) -> io::Result<()> {
        let path_str = path.to_string_lossy().to_string();
        let existing = self
            .locks
            .get_mut(&path_str)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not locked"))?;

        let pos = existing
            .iter()
            .position(|lock| match (lock, &record_range) {
                (None, None) => true,
                (Some((a, b)), Some((c, d))) => a == c && b == d,
                _ => false,
            });

        match pos {
            Some(i) => {
                existing.remove(i);
                if existing.is_empty() {
                    self.locks.remove(&path_str);
                }
                Ok(())
            }
            None => Err(io::Error::new(io::ErrorKind::NotFound, "File not locked")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn open_and_close_file() {
        let mut backend = MemoryBackend::new();
        let path = PathBuf::from("/test.txt");

        let mut file = backend
            .open(
                &path,
                OpenMode::Output,
                AccessMode::Write,
                LockMode::Shared,
                0,
            )
            .unwrap();
        file.number = 1;
        backend.close(&file).unwrap();
    }

    #[test]
    fn write_and_read_file() {
        let mut backend = MemoryBackend::new();
        let path = PathBuf::from("/test.txt");

        // Write
        let mut file = backend
            .open(
                &path,
                OpenMode::Output,
                AccessMode::Write,
                LockMode::Shared,
                0,
            )
            .unwrap();
        file.number = 1;
        backend.write(&mut file, b"Hello, World!").unwrap();

        // Read
        let mut file = backend
            .open(
                &path,
                OpenMode::Input,
                AccessMode::Read,
                LockMode::Shared,
                0,
            )
            .unwrap();
        file.number = 1;
        let mut buf = [0u8; 13];
        let bytes_read = backend.read(&mut file, &mut buf).unwrap();
        assert_eq!(bytes_read, 13);
        assert_eq!(&buf, b"Hello, World!");
    }

    #[test]
    fn file_exists_check() {
        let mut backend = MemoryBackend::new();
        let path = PathBuf::from("/test.txt");

        assert!(!backend.file_exists(&path));

        let file = backend
            .open(
                &path,
                OpenMode::Output,
                AccessMode::Write,
                LockMode::Shared,
                0,
            )
            .unwrap();
        drop(file);

        assert!(backend.file_exists(&path));
    }

    #[test]
    fn file_len_returns_correct_size() {
        let mut backend = MemoryBackend::new();
        let path = PathBuf::from("/test.txt");

        let mut file = backend
            .open(
                &path,
                OpenMode::Output,
                AccessMode::Write,
                LockMode::Shared,
                0,
            )
            .unwrap();
        file.number = 1;
        backend.write(&mut file, b"12345").unwrap();

        let len = backend.file_len(&path).unwrap();
        assert_eq!(len, 5);
    }

    #[test]
    fn append_mode_preserves_content() {
        let mut backend = MemoryBackend::new();
        let path = PathBuf::from("/test.txt");

        // Write initial content
        let mut file = backend
            .open(
                &path,
                OpenMode::Output,
                AccessMode::Write,
                LockMode::Shared,
                0,
            )
            .unwrap();
        file.number = 1;
        backend.write(&mut file, b"Hello").unwrap();

        // Append more content
        let mut file = backend
            .open(
                &path,
                OpenMode::Append,
                AccessMode::Write,
                LockMode::Shared,
                0,
            )
            .unwrap();
        file.number = 1;
        backend.write(&mut file, b", World!").unwrap();

        // Verify
        let mut file = backend
            .open(
                &path,
                OpenMode::Input,
                AccessMode::Read,
                LockMode::Shared,
                0,
            )
            .unwrap();
        file.number = 1;
        let mut buf = [0u8; 13];
        backend.read(&mut file, &mut buf).unwrap();
        assert_eq!(&buf, b"Hello, World!");
    }

    #[test]
    fn output_mode_truncates() {
        let mut backend = MemoryBackend::new();
        let path = PathBuf::from("/test.txt");

        // Write initial content
        let mut file = backend
            .open(
                &path,
                OpenMode::Output,
                AccessMode::Write,
                LockMode::Shared,
                0,
            )
            .unwrap();
        file.number = 1;
        backend.write(&mut file, b"Hello").unwrap();

        // Open for output again (should truncate)
        let mut file = backend
            .open(
                &path,
                OpenMode::Output,
                AccessMode::Write,
                LockMode::Shared,
                0,
            )
            .unwrap();
        file.number = 1;
        backend.write(&mut file, b"Hi").unwrap();

        // Verify
        let mut file = backend
            .open(
                &path,
                OpenMode::Input,
                AccessMode::Read,
                LockMode::Shared,
                0,
            )
            .unwrap();
        file.number = 1;
        let mut buf = [0u8; 2];
        backend.read(&mut file, &mut buf).unwrap();
        assert_eq!(&buf, b"Hi");
    }
}
