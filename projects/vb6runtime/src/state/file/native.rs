//! Native file I/O backend for Windows, Linux, and macOS.
//!
//! Uses actual file system calls through `std::fs::File`.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use super::backend::{AccessMode, FileBackend, LockMode, OpenFile, OpenMode};

/// A locked range: `(start, end)` inclusive, both 1-based.
type LockRange = (i32, i32);

/// Native file I/O backend using the real file system.
pub struct NativeBackend {
    /// Map from path to the actual file handle.
    files: HashMap<String, File>,
    /// Current working directory.
    current_dir: PathBuf,
    /// Current drive letter (tracked for VB6 semantics).
    current_drive: char,
    /// Locked regions per file path. `None` means the entire file is locked.
    locks: HashMap<String, Vec<Option<LockRange>>>,
}

impl NativeBackend {
    /// Create a new native backend.
    pub fn new() -> Self {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        Self {
            files: HashMap::new(),
            current_dir: cwd,
            current_drive: 'C',
            locks: HashMap::new(),
        }
    }
}

impl Default for NativeBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl FileBackend for NativeBackend {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn open(
        &mut self,
        path: &Path,
        mode: OpenMode,
        access: AccessMode,
        lock: LockMode,
        record_length: i32,
    ) -> io::Result<OpenFile> {
        // Create parent directories if they don't exist for Output/Append modes
        if mode == OpenMode::Output || mode == OpenMode::Append {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }
        }

        let file = match mode {
            OpenMode::Input => OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .truncate(false)
                .open(path)?,
            OpenMode::Output => OpenOptions::new()
                .read(false)
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)?,
            OpenMode::Append => OpenOptions::new()
                .read(false)
                .create(true)
                .append(true)
                .open(path)?,
            OpenMode::Random => OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(path)?,
            OpenMode::Binary => OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(path)?,
        };

        // Get initial position for Append mode
        let initial_position = if mode == OpenMode::Append {
            file.metadata()?.len() as i64
        } else {
            0
        };

        let path_str = path.to_string_lossy().to_string();

        // Store the file handle for later read/write/seek operations
        self.files.insert(path_str.clone(), file);

        Ok(OpenFile {
            number: 0, // Will be set by the caller
            path: path_str,
            mode,
            access,
            lock,
            record_length,
            position: initial_position,
        })
    }

    fn close(&mut self, file: &OpenFile) -> io::Result<()> {
        self.files.remove(&file.path);
        Ok(())
    }

    fn read(&mut self, file: &mut OpenFile, buf: &mut [u8]) -> io::Result<usize> {
        let f = self
            .files
            .get_mut(&file.path)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;

        // Seek to current position
        f.seek(SeekFrom::Start(file.position as u64))?;

        // Read data
        let bytes_read = f.read(buf)?;

        // Update position
        file.position += bytes_read as i64;

        Ok(bytes_read)
    }

    fn write(&mut self, file: &mut OpenFile, buf: &[u8]) -> io::Result<usize> {
        let f = self
            .files
            .get_mut(&file.path)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;

        // Seek to current position
        f.seek(SeekFrom::Start(file.position as u64))?;

        // Write data
        let bytes_written = f.write(buf)?;

        // Update position
        file.position += bytes_written as i64;

        Ok(bytes_written)
    }

    fn seek(&mut self, file: &mut OpenFile, position: i64) -> io::Result<i64> {
        let _ = self
            .files
            .get(&file.path)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "File not open"))?;

        // Update the file's position
        file.position = position;

        Ok(file.position)
    }

    fn file_len(&mut self, path: &Path) -> io::Result<i64> {
        let metadata = std::fs::metadata(path)?;
        Ok(metadata.len() as i64)
    }

    fn file_exists(&mut self, path: &Path) -> bool {
        path.exists()
    }

    fn position(&self, file: &OpenFile) -> i64 {
        // VB6 positions are 1-based
        file.position + 1
    }

    fn lof(&mut self, file: &OpenFile) -> io::Result<i64> {
        let path = Path::new(&file.path);
        let metadata = std::fs::metadata(path)?;
        Ok(metadata.len() as i64)
    }

    fn copy_file(&mut self, src: &Path, dst: &Path) -> io::Result<()> {
        std::fs::copy(src, dst)?;
        Ok(())
    }

    fn rename_file(&mut self, old_path: &Path, new_path: &Path) -> io::Result<()> {
        std::fs::rename(old_path, new_path)
    }

    fn remove_file(&mut self, path: &Path) -> io::Result<()> {
        std::fs::remove_file(path)
    }

    fn create_dir(&mut self, path: &Path) -> io::Result<()> {
        std::fs::create_dir(path)
    }

    fn remove_dir(&mut self, path: &Path) -> io::Result<()> {
        std::fs::remove_dir(path)
    }

    fn get_attrs(&mut self, path: &Path) -> io::Result<i16> {
        let metadata = std::fs::metadata(path)?;
        let perms = metadata.permissions();

        let mut attrs: i16 = 0;
        if perms.readonly() {
            attrs |= 1; // VB_READ_ONLY
        }
        if metadata.is_dir() {
            attrs |= 16; // VB_DIRECTORY
        }
        // Hidden/system require platform-specific APIs; skip for now.
        // Archive bit: set for all files on native (conservative default).
        if metadata.is_file() {
            attrs |= 32; // VB_ARCHIVE
        }
        Ok(attrs)
    }

    fn set_attrs(&mut self, path: &Path, attrs: i16) -> io::Result<()> {
        let metadata = std::fs::metadata(path)?;
        let mut perms = metadata.permissions();
        perms.set_readonly((attrs & 1) != 0);
        std::fs::set_permissions(path, perms)
    }

    fn file_datetime(&mut self, path: &Path) -> io::Result<std::time::SystemTime> {
        let metadata = std::fs::metadata(path)?;
        metadata.modified()
    }

    fn current_dir(&mut self) -> io::Result<PathBuf> {
        Ok(self.current_dir.clone())
    }

    fn set_current_dir(&mut self, path: &Path) -> io::Result<()> {
        // Resolve relative paths against the current directory
        let target = if path.is_relative() {
            self.current_dir.join(path)
        } else {
            path.to_path_buf()
        };
        // Verify the directory exists
        if !target.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Path not found: {}", path.display()),
            ));
        }
        self.current_dir = target;
        Ok(())
    }

    fn drives(&self) -> Vec<char> {
        vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']
    }

    fn current_dir_for_drive(&mut self, drive: char) -> io::Result<PathBuf> {
        if drive == self.current_drive {
            Ok(self.current_dir.clone())
        } else {
            // Non-current drives: return root as default
            Ok(PathBuf::from(format!("{drive}:\\")))
        }
    }

    fn set_current_drive(&mut self, drive: char) -> io::Result<()> {
        self.current_drive = drive.to_ascii_uppercase();
        Ok(())
    }

    fn lock_file(
        &mut self,
        path: &Path,
        record_range: Option<(i32, i32)>,
    ) -> io::Result<()> {
        let path_str = path.to_string_lossy().to_string();
        let existing = self.locks.entry(path_str).or_default();

        // Check for conflicts with existing locks
        for existing_lock in existing.iter() {
            match (existing_lock, &record_range) {
                // Entire file is already locked
                (None, _) | (_, None) => {
                    return Err(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        "File already locked",
                    ));
                }
                // Both are record ranges — check overlap
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

    fn unlock_file(
        &mut self,
        path: &Path,
        record_range: Option<(i32, i32)>,
    ) -> io::Result<()> {
        let path_str = path.to_string_lossy().to_string();
        let existing = self.locks.get_mut(&path_str).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "File not locked")
        })?;

        // Find and remove the matching lock
        let pos = existing.iter().position(|lock| {
            match (lock, &record_range) {
                (None, None) => true,
                (Some((a, b)), Some((c, d))) => a == c && b == d,
                _ => false,
            }
        });

        match pos {
            Some(i) => {
                existing.remove(i);
                if existing.is_empty() {
                    self.locks.remove(&path_str);
                }
                Ok(())
            }
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "File not locked",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_and_close_file() {
        let mut backend = NativeBackend::new();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");

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
        let mut backend = NativeBackend::new();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");

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
        backend.close(&file).unwrap();

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
        backend.close(&file).unwrap();
    }

    #[test]
    fn file_exists_check() {
        let mut backend = NativeBackend::new();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");

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
        backend.close(&file).unwrap();

        assert!(backend.file_exists(&path));
    }

    #[test]
    fn file_len_returns_correct_size() {
        let mut backend = NativeBackend::new();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");

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
}
