//! # `CurDir` Function
//!
//! Returns a `String` representing the current path for the specified drive or the default drive.
//!
//! ## Syntax
//!
//! ```vb
//! CurDir[(drive)]
//! ```
//!
//! ## Parameters
//!
//! - **`drive`**: Optional. `String` expression that specifies an existing drive. If no drive is
//!   specified or if drive is a zero-length string (""), `CurDir` returns the path for the
//!   current drive. The drive parameter can be just the drive letter (e.g., "C") or include
//!   a colon (e.g., "C:").
//!
//! ## Return Value
//!
//! Returns a `String` containing the current directory path for the specified drive. The returned
//! path does not include a trailing backslash unless the current directory is the root directory.
//!
//! ## Examples
//!
//! ```vb
//! Dim currentDir As String
//! currentDir = CurDir()         ' Current drive
//!
//! Dim cDrive As String
//! cDrive = CurDir("C")          ' Drive C
//!
//! Dim dDrive As String
//! dDrive = CurDir("D:")         ' Drive D
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/curdir-function)

use crate::error::{VBError, VBResult};
use vb6core::error::err_number;
use crate::state::file;
use crate::value::VBVariant;

/// Return the current directory as a `Variant`.
///
/// # Arguments
///
/// * `drive` - Optional drive letter. If empty or omitted, uses the current drive.
///
/// # Returns
///
/// Returns `Ok(VBVariant::String(...))` with the current directory path,
/// or `Err(VBError)` on failure.
pub fn curdir(drive: VBVariant) -> VBResult<VBVariant> {
    let dir = resolve_curdir(drive)?;
    Ok(VBVariant::from_string(&dir))
}

/// Shared implementation for `CurDir` and `CurDir$`.
pub(super) fn resolve_curdir(drive: VBVariant) -> VBResult<String> {
    let drive_char: Option<char> = match drive {
        VBVariant::String(s) => {
            let s = s.as_str().trim().to_string();
            if s.is_empty() {
                None
            } else {
                // Extract first character as drive letter
                let first = s.as_bytes()[0] as char;
                if !first.is_ascii_alphabetic() {
                    return Err(VBError::with_description(
                        err_number::INVALID_PROCEDURE_CALL,
                        "Invalid procedure call or argument",
                    ));
                }
                Some(first.to_ascii_uppercase())
            }
        }
        VBVariant::Empty => None,
        _ => {
            return Err(VBError::with_description(
                err_number::TYPE_MISMATCH,
                "Type mismatch in CurDir",
            ));
        }
    };

    let dir = if let Some(d) = drive_char {
        file::current_dir_for_drive(d).map_err(|e| {
            VBError::with_description(
                match e.kind() {
                    std::io::ErrorKind::NotFound => err_number::DEVICE_UNAVAILABLE,
                    _ => err_number::DEVICE_IO_ERROR,
                },
                e.to_string(),
            )
        })?
    } else {
        file::current_dir().map_err(|e| {
            VBError::with_description(err_number::DEVICE_IO_ERROR, e.to_string())
        })?
    };

    // Convert to string, using platform separator
    Ok(dir.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use vb6core::error::err_number;
    use super::*;
    use crate::state::file::{self};

    #[test]
    fn curdir_returns_current_directory() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());
        file::set_current_dir(dir.path()).unwrap();

        let result = curdir(VBVariant::Empty).unwrap();
        match result {
            VBVariant::String(s) => {
                let s_str = s.as_str();
                assert!(s_str.ends_with(dir.path().file_name().unwrap().to_str().unwrap()));
            }
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn curdir_rejects_non_string() {
        let _guard = crate::state::test_support::lock_test();

        let result = curdir(VBVariant::Long(42));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);
    }
}
