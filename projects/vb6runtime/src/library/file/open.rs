//! VB6 Open statement syntax:
//! - Open pathname For mode [Access access] [lock] As [#]filenumber [Len=reclength]
//!
//! Enables input/output (I/O) to a file.
//!
//!
//! The Open statement syntax has these parts:
//!
//! | Part       | Description |
//! |------------|-------------|
//! | pathname   | Required. String expression that specifies a file name — may include directory or folder, and drive. |
//! | mode       | Required. Keyword specifying the file mode: Append, Binary, Input, Output, or Random. If unspecified, the file is opened for Random access. |
//! | access     | Optional. Keyword specifying the operations permitted on the open file: Read, Write, or Read Write. |
//! | lock       | Optional. Keyword specifying the operations restricted on the open file by other processes: Shared, Lock Read, Lock Write, and Lock Read Write. |
//! | filenumber | Required. A valid file number in the range 1 to 511, inclusive. Use the FreeFile function to obtain the next available file number. |
//! | reclength  | Optional. Number less than or equal to 32,767 (bytes). For files opened for random access, this value is the record length. For sequential files, this value is the number of characters buffered. |
//!
//! ## Remarks
//!
//! - You must open a file before any I/O operation can be performed on it.
//! - If pathname specifies a file that doesn't exist, it is created when a file is opened for Append, Binary, Output, or Random modes.
//! - If the file is already opened by another process and the specified type of access is not allowed, the Open operation fails and an error occurs.
//! - The Len clause is ignored if mode is Binary.
//! - In Binary, Input, and Random modes, you can open a file using a different file number without first closing the file. In Append and Output modes, you must close a file before opening it with a different file number.
//!
//! ## Examples
//!
//! ```vb
//! ' Open for input
//! Open "TESTFILE" For Input As #1
//!
//! ' Open for output
//! Open "TESTFILE" For Output As #1
//!
//! ' Open for append
//! Open "TESTFILE" For Append As #1
//!
//! ' Open for binary
//! Open "TESTFILE" For Binary As #1
//!
//! ' Open for random with record length
//! Open "TESTFILE" For Random As #1 Len = 512
//!
//! ' Open with access control
//! Open "TESTFILE" For Input Access Read As #1
//!
//! ' Open with locking
//! Open "TESTFILE" For Binary Lock Read Write As #1
//!
//! ' Open with variable
//! Dim fileNum As Integer
//! fileNum = FreeFile
//! Open fileName For Input As fileNum
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/open-statement)

use std::io;
use std::path::Path;
use vb6core::error::err_number;

use crate::error::{VBError, VBResult};
use crate::state::file;
use crate::state::file::{
    backend::{AccessMode, LockMode, OpenMode},
    MAX_FILE_NUMBER, MIN_FILE_NUMBER,
};
use crate::value::VBVariant;

/// Open a file for I/O operations.
///
/// # Arguments
///
/// * `pathname` - The file path to open
/// * `mode` - The file mode (Input, Output, Append, Random, Binary)
/// * `access` - The access mode (Read, Write, ReadWrite)
/// * `lock` - The lock mode (Shared, LockRead, LockWrite, LockReadWrite)
/// * `filenumber` - The file number (1-511)
/// * `record_length` - Record length for Random mode (default 128)
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn open_file(
    pathname: &VBVariant,
    mode: OpenMode,
    access: AccessMode,
    lock: LockMode,
    filenumber: i16,
    record_length: i32,
) -> VBResult<()> {
    // Get the file path
    let path_str = pathname.as_string().map_err(|_| {
        VBError::with_description(
            13, // Type mismatch
            "Type mismatch in Open statement",
        )
    })?;

    // Validate file number range
    if !(MIN_FILE_NUMBER..=MAX_FILE_NUMBER).contains(&filenumber) {
        return Err(VBError::with_description(
            52, // Bad file name or number
            "Bad file name or number",
        ));
    }

    // Check if file number is already in use
    if file::is_file_open(filenumber) {
        return Err(VBError::with_description(
            55, // File already open
            format!("File already open: #{}", filenumber),
        ));
    }

    // Validate record length
    if !(file::MIN_RECORD_NUMBER..=file::MAX_RECORD_NUMBER).contains(&record_length) {
        return Err(VBError::with_description(
            59, // Bad record length
            "Bad record length",
        ));
    }

    // For Input mode, check if file exists
    let path = Path::new(&path_str);
    if mode == OpenMode::Input && !file::file_exists(path) {
        return Err(VBError::with_description(
            53, // File not found
            format!("File not found: {}", path_str),
        ));
    }

    // Open the file
    file::open_file(path, mode, access, lock, record_length, filenumber).map_err(|e| {
        VBError::with_description(
            match e.kind() {
                io::ErrorKind::NotFound => err_number::FILE_NOT_FOUND, // File not found
                io::ErrorKind::PermissionDenied => err_number::PERMISSION_DENIED, // Permission denied
                io::ErrorKind::AlreadyExists => err_number::FILE_ALREADY_OPEN, // File already open
                _ => err_number::DEVICE_IO_ERROR,                              // Device I/O error
            },
            e.to_string(),
        )
    })?;

    Ok(())
}

/// Convert a VB6 mode string to an OpenMode.
pub fn parse_mode(mode: &str) -> VBResult<OpenMode> {
    match mode.to_uppercase().as_str() {
        "INPUT" => Ok(OpenMode::Input),
        "OUTPUT" => Ok(OpenMode::Output),
        "APPEND" => Ok(OpenMode::Append),
        "RANDOM" => Ok(OpenMode::Random),
        "BINARY" => Ok(OpenMode::Binary),
        "" => Ok(OpenMode::Random), // Default mode
        _ => Err(VBError::with_description(
            54, // Bad file mode
            format!("Bad file mode: {}", mode),
        )),
    }
}

/// Convert a VB6 access string to an AccessMode.
pub fn parse_access(access: &str) -> VBResult<AccessMode> {
    match access.to_uppercase().as_str() {
        "READ" => Ok(AccessMode::Read),
        "WRITE" => Ok(AccessMode::Write),
        "READ WRITE" => Ok(AccessMode::ReadWrite),
        "READWRITE" => Ok(AccessMode::ReadWrite),
        "" => Ok(AccessMode::ReadWrite), // Default access
        _ => Err(VBError::with_description(
            54, // Bad file mode
            format!("Bad access mode: {}", access),
        )),
    }
}

/// Convert a VB6 lock string to a LockMode.
pub fn parse_lock(lock: &str) -> VBResult<LockMode> {
    match lock.to_uppercase().as_str() {
        "SHARED" => Ok(LockMode::Shared),
        "LOCK READ" => Ok(LockMode::LockRead),
        "LOCKWRITE" => Ok(LockMode::LockWrite),
        "LOCK WRITE" => Ok(LockMode::LockWrite),
        "LOCK READ WRITE" => Ok(LockMode::LockReadWrite),
        "LOCKREADWRITE" => Ok(LockMode::LockReadWrite),
        "" => Ok(LockMode::Shared), // Default lock
        _ => Err(VBError::with_description(
            54, // Bad file mode
            format!("Bad lock mode: {}", lock),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::file;
    use vb6core::error::err_number;

    #[test]
    fn open_and_close_output_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        crate::state::file::set_root(dir.path());

        let path = VBVariant::from_string("test.txt");
        open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            1,
            0,
        )
        .unwrap();

        assert!(file::is_file_open(1));

        file::close_file(1).unwrap();
        assert!(!file::is_file_open(1));

        let _ = file::close_all_files();
    }

    #[test]
    fn open_rejects_invalid_file_number() {
        let _guard = crate::state::test_support::lock_test();

        let path = VBVariant::from_string("test.txt");
        let result = open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0, // Invalid
            0,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let result = open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            512, // Invalid
            0,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let _ = file::close_all_files();
    }

    #[test]
    fn open_rejects_file_already_open() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        crate::state::file::set_root(dir.path());

        let path = VBVariant::from_string("test.txt");
        open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            1,
            0,
        )
        .unwrap();

        let result = open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            1, // Already in use
            0,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::FILE_ALREADY_OPEN);

        let _ = file::close_all_files();
    }

    #[test]
    fn open_rejects_invalid_record_length() {
        let _guard = crate::state::test_support::lock_test();

        let path = VBVariant::from_string("test.txt");
        let result = open_file(
            &path,
            OpenMode::Random,
            AccessMode::ReadWrite,
            LockMode::Shared,
            1,
            -1, // Invalid
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::BAD_RECORD_LENGTH);

        let result = open_file(
            &path,
            OpenMode::Random,
            AccessMode::ReadWrite,
            LockMode::Shared,
            1,
            32768, // Invalid
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::BAD_RECORD_LENGTH);

        let _ = file::close_all_files();
    }

    #[test]
    fn open_input_rejects_nonexistent_file() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        crate::state::file::set_root(dir.path());

        let path = VBVariant::from_string("nonexistent.txt");
        let result = open_file(
            &path,
            OpenMode::Input,
            AccessMode::Read,
            LockMode::Shared,
            1,
            0,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::FILE_NOT_FOUND);

        let _ = file::close_all_files();
    }

    #[test]
    fn parse_mode_works() {
        assert_eq!(parse_mode("INPUT").unwrap(), OpenMode::Input);
        assert_eq!(parse_mode("OUTPUT").unwrap(), OpenMode::Output);
        assert_eq!(parse_mode("APPEND").unwrap(), OpenMode::Append);
        assert_eq!(parse_mode("RANDOM").unwrap(), OpenMode::Random);
        assert_eq!(parse_mode("BINARY").unwrap(), OpenMode::Binary);
        assert_eq!(parse_mode("").unwrap(), OpenMode::Random);
        assert!(parse_mode("INVALID").is_err());
    }

    #[test]
    fn parse_access_works() {
        assert_eq!(parse_access("READ").unwrap(), AccessMode::Read);
        assert_eq!(parse_access("WRITE").unwrap(), AccessMode::Write);
        assert_eq!(parse_access("READ WRITE").unwrap(), AccessMode::ReadWrite);
        assert_eq!(parse_access("").unwrap(), AccessMode::ReadWrite);
        assert!(parse_access("INVALID").is_err());
    }

    #[test]
    fn parse_lock_works() {
        assert_eq!(parse_lock("SHARED").unwrap(), LockMode::Shared);
        assert_eq!(parse_lock("LOCK READ").unwrap(), LockMode::LockRead);
        assert_eq!(parse_lock("LOCK WRITE").unwrap(), LockMode::LockWrite);
        assert_eq!(
            parse_lock("LOCK READ WRITE").unwrap(),
            LockMode::LockReadWrite
        );
        assert_eq!(parse_lock("").unwrap(), LockMode::Shared);
        assert!(parse_lock("INVALID").is_err());
    }
}
