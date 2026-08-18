//! VB6 FileCopy statement syntax:
//! - FileCopy source, destination
//!
//! Copies a file.
//!
//! The FileCopy statement syntax has these named arguments:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | source        | Required. String expression that specifies a file name. May include directory or folder, and drive. |
//! | destination   | Required. String expression that specifies a file name. May include directory or folder, and drive. |
//!
//! ## Remarks
//!
//! - If you try to use the FileCopy statement on a currently open file, an error occurs.
//! - FileCopy can copy files between directories/folders and between drives.
//! - Both source and destination can include path information (drive and directory/folder).
//! - If destination specifies a directory/folder that doesn't exist, FileCopy creates it.
//!
//! ## Examples
//!
//! ```vb
//! FileCopy "C:\SOURCE.TXT", "C:\DEST.TXT"
//! FileCopy oldFile, newFile
//! FileCopy App.Path & "\data.dat", "C:\Backup\data.dat"
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/filecopy-statement)

use crate::error::{VBError, VBResult};
use crate::state::file;
use crate::value::VBVariant;
use vb6core::error::err_number;

/// Copy a file from one location to another.
///
/// # Arguments
///
/// * `source` - The source file path.
/// * `destination` - The destination file path.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn file_copy(source: VBVariant, destination: VBVariant) -> VBResult<()> {
    // Get source path
    let source_str = match source {
        VBVariant::String(s) => s.as_str().to_string(),
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in FileCopy",
            ));
        }
    };

    // Get destination path
    let dest_str = match destination {
        VBVariant::String(s) => s.as_str().to_string(),
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in FileCopy",
            ));
        }
    };

    // Copy the file through the backend
    file::copy_file(
        std::path::Path::new(&source_str),
        std::path::Path::new(&dest_str),
    )
    .map_err(|e| {
        VBError::with_description(
            match e.kind() {
                std::io::ErrorKind::NotFound => err_number::FILE_NOT_FOUND, // File not found
                std::io::ErrorKind::PermissionDenied => err_number::PERMISSION_DENIED, // Permission denied
                _ => err_number::DEVICE_IO_ERROR, // Device I/O error
            },
            e.to_string(),
        )
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::file::{self};
    use vb6core::error::err_number;

    #[test]
    fn file_copy_copies_file() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create source file
        std::fs::write(dir.path().join("source.txt"), "Hello").unwrap();

        // Copy it
        file_copy(
            VBVariant::from_string("source.txt"),
            VBVariant::from_string("dest.txt"),
        )
        .unwrap();

        // Verify
        assert!(dir.path().join("dest.txt").exists());
        let content = std::fs::read_to_string(dir.path().join("dest.txt")).unwrap();
        assert_eq!(content, "Hello");
    }

    #[test]
    fn file_copy_overwrites_existing() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create source and destination files
        std::fs::write(dir.path().join("source.txt"), "Hello").unwrap();
        std::fs::write(dir.path().join("dest.txt"), "Old").unwrap();

        // Copy
        file_copy(
            VBVariant::from_string("source.txt"),
            VBVariant::from_string("dest.txt"),
        )
        .unwrap();

        // Verify
        let content = std::fs::read_to_string(dir.path().join("dest.txt")).unwrap();
        assert_eq!(content, "Hello");
    }

    #[test]
    fn file_copy_rejects_nonexistent_source() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let result = file_copy(
            VBVariant::from_string("nonexistent.txt"),
            VBVariant::from_string("dest.txt"),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::FILE_NOT_FOUND);
    }

    #[test]
    fn file_copy_rejects_non_string() {
        let _guard = crate::state::test_support::lock_test();

        let result = file_copy(VBVariant::Long(42), VBVariant::from_string("dest.txt"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);

        let result = file_copy(VBVariant::from_string("source.txt"), VBVariant::Long(42));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);
    }
}
