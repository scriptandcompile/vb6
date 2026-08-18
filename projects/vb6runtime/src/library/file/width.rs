//! # Width Statement
//!
//! Assigns an output line width to a file opened using the Open statement.
//!
//! ## Syntax
//!
//! ```vb
//! Width #filenumber, width
//! ```
//!
//! ## Parts
//!
//! - **filenumber**: Required. Any valid file number.
//! - **width**: Required. Numeric expression in the range 0–255, inclusive, that indicates how
//!   many characters appear on a line before a new line is started. If width equals 0, there is
//!   no limit to the length of a line. The default value for width is 0.
//!
//! ## Remarks
//!
//! - **Output Formatting**: The Width # statement is used with the Print # or Write # statements
//!   to control output formatting to files.
//! - **Line Length Control**: For files opened for sequential output, if the width of a line of
//!   output exceeds the value specified for width, a new line is automatically started.
//! - **No Effect on Input**: The Width # statement has no effect on files opened for input or
//!   binary access.
//! - **Zero Width**: Setting width to 0 means there is no line length limit, allowing continuous
//!   output without automatic line breaks.
//! - **Maximum Width**: The maximum width value is 255 characters.
//!
//! ## Examples
//!
//! ### Basic Width Setting
//!
//! ```vb
//! Open "output.txt" For Output As #1
//! Width #1, 80
//! Print #1, "This output will wrap at 80 characters"
//! Close #1
//! ```
//!
//! ### Set Unlimited Width
//!
//! ```vb
//! Open "data.csv" For Output As #2
//! Width #2, 0  ' No line length limit
//! Print #2, LongDataString
//! Close #2
//! ```
//!
//! ### Width with Multiple Files
//!
//! ```vb
//! Open "narrow.txt" For Output As #1
//! Open "wide.txt" For Output As #2
//! Width #1, 40
//! Width #2, 120
//! ```
//!
//! ### Dynamic Width Setting
//!
//! ```vb
//! Dim lineWidth As Integer
//! lineWidth = 80
//! Open "report.txt" For Output As #1
//! Width #1, lineWidth
//! ```
//!
//! ### Width for Formatted Output
//!
//! ```vb
//! Open "report.txt" For Output As #1
//! Width #1, 80
//! Print #1, Tab(10); "Header"
//! Print #1, Tab(10); String$(50, "-")
//! Close #1
//! ```
//!
//! ## Common Patterns
//!
//! ### Report Generation with Fixed Width
//!
//! ```vb
//! Sub GenerateReport()
//!     Open "report.txt" For Output As #1
//!     Width #1, 80
//!     
//!     Print #1, "Annual Sales Report"
//!     Print #1, String$(80, "=")
//!     ' ... report content ...
//!     
//!     Close #1
//! End Sub
//! ```
//!
//! ### CSV Export (No Width Limit)
//!
//! ```vb
//! Sub ExportCSV()
//!     Open "export.csv" For Output As #1
//!     Width #1, 0  ' Allow unlimited line length
//!     
//!     For i = 1 To RecordCount
//!         Print #1, BuildCSVLine(i)
//!     Next i
//!     
//!     Close #1
//! End Sub
//! ```
//!
//! ### Console-Style Output
//!
//! ```vb
//! Open "console.log" For Output As #1
//! Width #1, 80  ' Standard console width
//! Print #1, "System Log - "; Now()
//! Close #1
//! ```

use crate::error::{VBError, VBResult};
use crate::state::file;

/// Set the line-output width for an open file.
///
/// # Arguments
///
/// * `file_number` - The file number.
/// * `width` - The line width (0 means no limit).
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn width_statement(file_number: i16, _width: i16) -> VBResult<()> {
    // Check file number is valid
    if !(file::MIN_FILE_NUMBER..=file::MAX_FILE_NUMBER).contains(&file_number) {
        return Err(VBError::with_description(
            52, // Bad file name or number
            format!("Bad file name or number: {}", file_number),
        ));
    }

    // Check file is open
    if !file::is_file_open(file_number) {
        return Err(VBError::with_description(
            52, // Bad file name or number
            format!("File not open: #{}", file_number),
        ));
    }

    // Width is stored but not currently used by the runtime
    // This is a placeholder for future implementation
    // TODO: Implement actual width handling for the file output.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::file::{self, AccessMode, LockMode, OpenMode};
    use vb6core::error::err_number;

    #[test]
    fn width_succeeds_for_open_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let path = std::path::PathBuf::from("test.txt");
        file::open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        let result = width_statement(1, 80);
        assert!(result.is_ok());

        let _ = file::close_all_files();
    }

    #[test]
    fn width_rejects_invalid_file_number() {
        let _guard = crate::state::test_support::lock_test();

        let result = width_statement(0, 80);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let result = width_statement(512, 80);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let _ = file::close_all_files();
    }

    #[test]
    fn width_rejects_closed_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let result = width_statement(1, 80);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let _ = file::close_all_files();
    }
}
