//! # `CurDir$` Function
//!
//! Returns a `String` representing the current path for the specified drive or the default drive.
//! The dollar sign suffix (`$`) explicitly indicates that this function returns a `String` type
//! (not a `Variant`).
//!
//! ## Syntax
//!
//! ```vb
//! CurDir$[(drive)]
//! ```
//!
//! ## Parameters
//!
//! - **`drive`**: Optional. `String` expression that specifies an existing drive. If no drive is
//!   specified or if drive is a zero-length string (""), `CurDir$` returns the path for the
//!   current drive.
//!
//! ## Return Value
//!
//! Returns a `String` containing the current directory path for the specified drive.
//!
//! ## Examples
//!
//! ```vb
//! Dim currentDir As String
//! currentDir = CurDir$()
//!
//! Dim cDrive As String
//! cDrive = CurDir$("C")
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/curdir-function)

use crate::error::VBResult;
use crate::value::VBVariant;

use super::curdir::resolve_curdir;

/// Return the current directory as a `String`.
///
/// # Arguments
///
/// * `drive` - Optional drive letter. If empty or omitted, uses the current drive.
///
/// # Returns
///
/// Returns `Ok(VBVariant::String(...))` with the current directory path,
/// or `Err(VBError)` on failure.
pub fn curdir_dollar(drive: VBVariant) -> VBResult<VBVariant> {
    let dir = resolve_curdir(drive)?;
    Ok(VBVariant::from_string(&dir))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::file::{self};

    #[test]
    fn curdir_dollar_returns_current_directory() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());
        file::set_current_dir(dir.path()).unwrap();

        let result = curdir_dollar(VBVariant::Empty).unwrap();
        match result {
            VBVariant::String(s) => {
                let s_str = s.as_str();
                assert!(s_str.ends_with(dir.path().file_name().unwrap().to_str().unwrap()));
            }
            _ => panic!("Expected String variant"),
        }
    }
}
