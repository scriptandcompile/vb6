//! VB6 ChDrive statement syntax:
//! - ChDrive drive
//!
//! Changes the current drive.
//!
//! The `drive` argument is a `String` expression that specifies the drive letter.
//! Only the first character is used; if the string is empty, no action occurs.
//!
//! ## Remarks
//!
//! - On Windows, `ChDrive` changes the current drive. On other platforms,
//!   it updates the tracked current drive letter for VB6 semantics.
//! - `ChDrive` does not affect the current directory.
//!
//! ## Examples
//!
//! ```vb
//! ChDrive "D"     ' Change to D: drive
//! ChDrive "E:"    ' Change to E: drive
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/chdrive-statement)

use crate::error::{VBError, VBResult};
use vb6core::error::err_number;
use crate::state::file;
use crate::value::VBVariant;

/// Change the current drive.
///
/// # Arguments
///
/// * `drive` - A string whose first character is the drive letter to switch to.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn chdrive(drive: VBVariant) -> VBResult<()> {
    let drive_char = match drive {
        VBVariant::String(s) => {
            let s = s.as_str();
            if s.is_empty() {
                return Ok(());
            }
            let first = s.as_bytes()[0] as char;
            if !first.is_ascii_alphabetic() {
                return Err(VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    "Invalid procedure call or argument",
                ));
            }
            first.to_ascii_uppercase()
        }
        VBVariant::Empty => return Ok(()),
        _ => {
            return Err(VBError::with_description(
                err_number::TYPE_MISMATCH,
                "Type mismatch in ChDrive",
            ));
        }
    };

    file::set_current_drive(drive_char).map_err(|e| {
        VBError::with_description(
            match e.kind() {
                std::io::ErrorKind::NotFound => err_number::DEVICE_UNAVAILABLE,
                _ => err_number::DEVICE_IO_ERROR,
            },
            e.to_string(),
        )
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use vb6core::error::err_number;
    use super::*;
    use crate::state::file::{self};

    #[test]
    fn chdrive_changes_drive() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        chdrive(VBVariant::from_string("D")).unwrap();

        let result = file::current_dir_for_drive('D');
        assert!(result.is_ok());
    }

    #[test]
    fn chdrive_rejects_non_string() {
        let _guard = crate::state::test_support::lock_test();

        let result = chdrive(VBVariant::Long(42));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn chdrive_empty_string_is_noop() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        chdrive(VBVariant::from_string("")).unwrap();
    }
}
