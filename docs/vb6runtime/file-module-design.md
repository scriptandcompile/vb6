# File Module Design

This document outlines the architecture for the `vb6runtime` file module, covering VB6 file system operations with cross-platform support including WASM.

## Overview

The file module implements VB6's file system operations (file I/O, directory management, file attributes, and file locking). The key challenge is supporting both native platforms (Windows/Linux/macOS) and WASM environments (JavaScript playground) through a pluggable backend system.

## Architecture

### FileBackend Trait

The core abstraction is the `FileBackend` trait, which defines all file system operations:

```rust
pub trait FileBackend {
    // Current directory management
    fn get_current_dir(&self, drive: Option<char>) -> Result<PathBuf, IoError>;
    fn set_current_dir(&mut self, path: &Path) -> Result<(), IoError>;
    
    // Drive management
    fn get_drives(&self) -> Vec<char>;
    fn set_current_drive(&mut self, drive: char) -> Result<(), IoError>;
    
    // File attributes
    fn get_attrs(&self, path: &Path) -> Result<u32, IoError>;
    fn set_attrs(&self, path: &Path, attrs: u32) -> Result<(), IoError>;
    
    // Lock/Unlock
    fn lock(&self, handle: FileHandle, range: Option<RecordRange>) -> Result<(), IoError>;
    fn unlock(&self, handle: FileHandle, range: Option<RecordRange>) -> Result<(), IoError>;
    
    // File operations
    fn open(&mut self, path: &Path, mode: OpenMode, lock: LockMode) -> Result<FileHandle, IoError>;
    fn close(&mut self, handle: FileHandle) -> Result<(), IoError>;
    fn read(&self, handle: FileHandle, buf: &mut [u8]) -> Result<usize, IoError>;
    fn write(&mut self, handle: FileHandle, buf: &[u8]) -> Result<usize, IoError>;
    fn seek(&mut self, handle: FileHandle, pos: SeekFrom) -> Result<u64, IoError>;
    
    // File metadata
    fn file_len(&self, path: &Path) -> Result<u64, IoError>;
    fn file_datetime(&self, path: &Path) -> Result<FileDateTime, IoError>;
    fn file_exists(&self, path: &Path) -> bool;
    
    // Directory operations
    fn create_dir(&self, path: &Path) -> Result<(), IoError>;
    fn remove_dir(&self, path: &Path) -> Result<(), IoError>;
    fn list_dir(&self, path: &Path, pattern: &str) -> Result<Vec<DirEntry>, IoError>;
    
    // File operations
    fn copy_file(&self, src: &Path, dst: &Path) -> Result<(), IoError>;
    fn rename_file(&self, src: &Path, dst: &Path) -> Result<(), IoError>;
    fn delete_file(&self, path: &Path) -> Result<(), IoError>;
}
```

### Backend Implementations

#### NativeBackend

For Windows, Linux, and macOS. Delegates to actual OS calls.

```rust
pub struct NativeBackend {
    current_dirs: HashMap<Option<char>, PathBuf>,  // VB6 tracks per-drive
    open_files: HashMap<FileHandle, OpenFile>,
    next_handle: FileHandle,
}

impl FileBackend for NativeBackend {
    fn set_current_dir(&mut self, path: &Path) -> Result<(), IoError> {
        std::env::set_current_dir(path)?;
        // Update VB6's per-drive tracking
        self.current_dirs.insert(None, path.canonicalize()?);
        Ok(())
    }
    
    fn set_attrs(&self, path: &Path, attrs: u32) -> Result<(), IoError> {
        // Platform-specific implementation
        #[cfg(windows)]
        { /* SetFileAttributesW */ }
        #[cfg(unix)]
        { /* chmod, chown, etc. */ }
    }
    
    fn lock(&self, handle: FileHandle, range: Option<RecordRange>) -> Result<(), IoError> {
        // OS-level file locking (flock on Unix, LockFileEx on Windows)
    }
}
```

#### MemoryBackend

For WASM playground. Virtual filesystem in memory.

```rust
pub struct MemoryBackend {
    // Virtual directory structure
    root: VirtualDir,
    current_dirs: HashMap<char, VirtualPath>,
    current_drive: char,
    
    // File attributes (per path)
    attributes: HashMap<VirtualPath, u32>,
    
    // Open file handles
    open_files: HashMap<FileHandle, VirtualOpenFile>,
    next_handle: FileHandle,
}

struct VirtualDir {
    name: String,
    children: HashMap<String, VirtualDirEntry>,
}

enum VirtualDirEntry {
    Dir(VirtualDir),
    File(VirtualFile),
}

struct VirtualFile {
    content: Vec<u8>,
    attributes: u32,
    datetime: SystemTime,
}
```

### State Isolation

Each execution context maintains its own backend instance. For WASM, use `RefCell<MemoryBackend>` or `thread_local!`. For native, thread-local state is also recommended (VB6 wasn't multi-threaded, but the runtime might be).

```rust
thread_local! {
    static FILE_BACKEND: RefCell<Option<Box<dyn FileBackend>>> = RefCell::new(None);
}
```

## VB6 Operation Mapping

### Directory Operations

| VB6 Statement | Native Implementation | WASM/Memory Implementation |
|---------------|----------------------|----------------------------|
| `ChDir path` | `std::env::set_current_dir(path)` + update per-drive tracking | Update `current_dirs` map; validate path exists in virtual FS |
| `CurDir[(drive)]` | Query per-drive CWD; fallback to actual CWD | Return from `current_dirs` map |
| `ChDrive drive` | Update "current drive" state; OS doesn't have this concept | Store `current_drive` letter; validate against virtual drives |

**Key Considerations:**
- VB6 tracks current directory *per drive*. Each drive letter remembers its own CWD.
- On non-Windows systems, emulate drive letters as virtual roots (e.g., `C:` → `/`, `D:` → virtual).
- `ChDir` should validate the path exists before changing.
- `CurDir` should not return trailing backslash (except for root directories).

### File Attributes

| VB6 Statement | Native Implementation | WASM/Memory Implementation |
|---------------|----------------------|----------------------------|
| `GetAttr(pathname)` | Platform-specific stat calls | Look up in `attributes` HashMap |
| `SetAttr pathname, attributes` | Platform-specific chmod/chattr | Store in `attributes` HashMap |

**VB6 File Attribute Constants:**
```vb
vbNormal    = 0
vbReadOnly  = 1
vbHidden    = 2
vbSystem    = 4
vbDirectory = 16
vbArchive   = 32
```

**Key Considerations:**
- In WASM, attributes are metadata-only; they don't affect actual file system behavior.
- `vbDirectory` attribute should be automatically set for directories.
- `SetAttr` on non-existent files should raise Error 53 (File not found).

### File Locking

| VB6 Statement | Native Implementation | WASM/Memory Implementation |
|---------------|----------------------|----------------------------|
| `Lock [#]filenumber[, recordrange]` | OS-level locking (flock/LockFileEx) | No-op (single-threaded WASM) |
| `Unlock [#]filenumber[, recordrange]` | OS-level unlock | No-op |

**VB6 Lock Modes (in Open statement):**
```vb
Shared          ' No locking (default)
Lock Read       ' Others can't read
Lock Write      ' Others can't write  
Lock Read Write ' Others can't read or write
```

**Key Considerations:**
- Locking is advisory on most systems; it only affects other processes using VB6-style locking.
- For Random mode files, locks can be record-range based: `Lock #1, 5 To 10`
- In WASM playground, no multi-process access is possible, so locking is purely bookkeeping.
- Native implementation should use non-blocking locks and raise Error 55 (File already exists) on contention.

### File I/O Operations

| VB6 Statement | Native Implementation | WASM/Memory Implementation |
|---------------|----------------------|----------------------------|
| `Open path For Input As #f` | `std::fs::File::open()` with read-only | Read from virtual file content |
| `Open path For Output As #f` | `std::fs::File::create()` | Create/overwrite virtual file |
| `Open path For Append As #f` | Open with append mode | Append to virtual file content |
| `Open path For Random As #f` | Open with read+write | Read/write virtual file with record positioning |
| `Open path For Binary As #f` | Open with read+write | Same as Random but byte-level access |
| `Input #f, variables` | Read line, parse CSV | Same logic, read from virtual content |
| `Line Input #f, variable` | Read one line | Read line from virtual content |
| `Print #f, data` | Write formatted output | Write to virtual content |
| `Write #f, data` | Write with CSV formatting | Write to virtual content |
| `Get #f, [record], variable` | Read record from Random/Binary | Read from virtual content |
| `Put #f, [record], data` | Write record to Random/Binary | Write to virtual content |
| `Close #f` | Flush and close file handle | Close virtual file handle |
| `Reset` | Close all open files | Close all virtual file handles |

## Known Issues and Considerations

### 1. Drive Letter Emulation

On non-Windows systems (Linux, macOS, WASM), VB6 drive letters don't exist natively. The solution:

- `C:` maps to the virtual root (or actual root for native)
- Additional drive letters can be virtual roots pointing to subdirectories
- `ChDrive` on non-Windows: store the letter but operations still use the same filesystem
- `CurDir("D")` on non-Windows: return virtual path or error if not configured

### 2. Path Separators

VB6 uses backslashes (`\`) as path separators. The backend should:
- Accept both `/` and `\` as input (normalize internally)
- Return paths with the platform-appropriate separator
- For WASM, standardize on `/` internally

### 3. UNC Paths

VB6 doesn't properly support UNC paths (`\\server\share`). The backend should:
- Raise an appropriate error for UNC paths on non-Windows
- On Windows, delegate to the OS for UNC support

### 4. File Handle Management

VB6 uses file numbers 1-511 (with `FreeFile` returning the next available). The backend must:
- Track which file numbers are in use
- Raise Error 55 (File already exists) if opening an already-open file number
- `FreeFile` should return the lowest available number

### 5. Random Mode Records

Random mode files have fixed-length records. The backend must:
- Store record length when file is opened
- Calculate byte offset from record number: `offset = (record - 1) * record_length`
- Handle `Get`/`Put` operations with record-level granularity

### 6. Binary Mode vs Random Mode

Binary mode is similar to Random but:
- No record structure; byte-level access
- `Get`/`Put` operate on byte ranges, not records
- `Seek` returns/accepts byte positions

### 7. Error Codes

Common VB6 file error codes to implement:
- **53**: File not found
- **54**: Bad file mode
- **55**: File already exists (or lock contention)
- **57**: Device I/O error
- **58**: File already exists (for naming)
- **59**: Bad record length
- **61**: Disk full
- **62**: Input past end of file
- **67**: Too many files
- **68**: Device unavailable
- **70**: Permission denied
- **71**: Disk not ready

### 8. WASM Limitations

The WASM/MemoryBackend cannot fully replicate:
- Actual file system persistence (data is lost on page reload unless saved)
- Real file permissions
- Concurrent access from multiple processes
- Actual file timestamps (use simulated time)

**Mitigation:** Provide JavaScript API to save/load the virtual filesystem state, allowing persistence across sessions.

### 9. Thread Safety

For native implementations, consider:
- VB6 is single-threaded, but your runtime might use threads
- Each thread should have its own `FileBackend` instance
- Use `thread_local!` for state isolation

### 10. File System Watching

VB6 doesn't have built-in file system watching, but some operations (like `Dir$`) depend on the file system state at the time of call. The backend should:
- `Dir$` should capture a snapshot of matching files at call time
- No need for real-time watching; VB6 semantics are snapshot-based

## Implementation Priority

1. **Phase 1**: File I/O operations (Open, Close, Read, Write, Input, Print, Get, Put)
2. **Phase 2**: Directory operations (ChDir, CurDir, ChDrive, MkDir, RmDir)
3. **Phase 3**: File attributes (GetAttr, SetAttr)
4. **Phase 4**: File locking (Lock, Unlock)
5. **Phase 5**: Polish and edge cases

## Testing Strategy

### Unit Tests
- Test each backend implementation independently
- Test path normalization (backslash vs forward slash)
- Test error conditions (file not found, permission denied, etc.)

### Integration Tests
- Test complete VB6 file operation sequences
- Test Random mode record operations
- Test `Dir$` pattern matching

### WASM Tests
- Verify MemoryBackend works in browser environment
- Test virtual filesystem save/load
- Test that file operations work without actual filesystem access

## References

- [VB6 File I/O Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/file-i-o-keywords)
- [VB6 Lock Statement](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/lock-statement)
- [VB6 FileAttr Function](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/fileattr-function)
