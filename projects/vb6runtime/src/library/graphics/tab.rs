//! VB6 `Tab` Function
//!
//! The `Tab` function is used in Print statements to position output at a specific column number.
//!
//! ## Syntax
//! ```vb6
//! Tab([column])
//! ```
//!
//! ## Parameters
//! - `column`: Optional. Numeric expression indicating the column number (1-based) at which to position the next character printed. If omitted, moves to the next print zone.
//!
//! ## Returns
//! Returns a special value used only in Print statements to control output position. It does not return a value for assignment or calculation.
//!
//! ## Remarks
//! - `Tab` is only meaningful within Print statements (e.g., `Print #1, Tab(10); "Hello"`).
//! - If `column` is omitted, output moves to the next print zone (every 14 columns by default).
//! - If `column` is less than the current print position, output moves to that column on the next line.
//! - If `column` is greater than the output line width, output starts at column 1 on the next line.
//! - `Tab` cannot be used in assignment or as a function value.
//! - `Tab` is not evaluated as a function in expressions outside Print context.
//! - `Tab` is not the same as the Tab key or character (Chr$(9)).
//! - In Print statements, `Tab` can be combined with `Spc` for advanced formatting.
//!
//! ## Typical Uses
//! 1. Aligning columns in printed output
//! 2. Formatting reports
//! 3. Creating tabular data in files
//! 4. Printing to the Immediate window
//! 5. Outputting to files with Print #
//! 6. Combining with `Spc` for custom spacing
//! 7. Printing headers and data in columns
//! 8. Generating formatted logs
//!
//! ## Basic Examples
//!
//! ### Example 1: Print with Tab
//! ```vb6
//! Print Tab(10); "Hello"
//! ```
//!
//! ### Example 2: Print to file with Tab
//! ```vb6
//! Print #1, Tab(20); "World"
//! ```
//!
//! ### Example 3: Print with omitted column
//! ```vb6
//! Print Tab; "Next zone"
//! ```
//!
//! ### Example 4: Print multiple columns
//! ```vb6
//! Print Tab(5); "A"; Tab(15); "B"; Tab(25); "C"
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: Print table header
//! ```vb6
//! Print Tab(1); "ID"; Tab(10); "Name"; Tab(30); "Score"
//! ```
//!
//! ### Pattern 2: Print data rows
//! ```vb6
//! For i = 1 To 10
//!     Print Tab(1); i; Tab(10); names(i); Tab(30); scores(i)
//! Next i
//! ```
//!
//! ### Pattern 3: Print with Spc
//! ```vb6
//! Print Tab(10); Spc(5); "Data"
//! ```
//!
//! ### Pattern 4: Print to Immediate window
//! ```vb6
//! Debug.Print Tab(15); "Debug info"
//! ```
//!
//! ### Pattern 5: Print to file
//! ```vb6
//! Print #1, Tab(8); "File data"
//! ```
//!
//! ### Pattern 6: Print with omitted column
//! ```vb6
//! Print Tab; "Default zone"
//! ```
//!
//! ### Pattern 7: Print with calculated column
//! ```vb6
//! Print Tab(i * 5); "Value"
//! ```
//!
//! ### Pattern 8: Print with variable
//! ```vb6
//! col = 12
//! Print Tab(col); "Text"
//! ```
//!
//! ### Pattern 9: Print with multiple Tab calls
//! ```vb6
//! Print Tab(5); "A"; Tab(15); "B"; Tab(25); "C"
//! ```
//!
//! ### Pattern 10: Print with Tab and Spc
//! ```vb6
//! Print Tab(10); Spc(3); "Mix"
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Print formatted report
//! ```vb6
//! Print Tab(1); "Header1"; Tab(20); "Header2"
//! For i = 1 To 5
//!     Print Tab(1); data1(i); Tab(20); data2(i)
//! Next i
//! ```
//!
//! ### Example 2: Print to file with dynamic columns
//! ```vb6
//! For i = 1 To 3
//!     Print #1, Tab(i * 10); "Col" & i
//! Next i
//! ```
//!
//! ### Example 3: Print with omitted column in loop
//! ```vb6
//! For i = 1 To 3
//!     Print Tab; "Row" & i
//! Next i
//! ```
//!
//! ### Example 4: Print with Tab and Spc for alignment
//! ```vb6
//! Print Tab(10); Spc(2); "Aligned"
//! ```
//!
//! ## Error Handling
//! - If `column` is less than 1, output starts at column 1 of the next line.
//! - If `column` is omitted, output moves to the next print zone.
//! - If `column` is greater than line width, output starts at column 1 of the next line.
//!
//! ## Performance Notes
//! - No performance impact; only affects output formatting.
//! - Used only in Print statements.
//!
//! ## Best Practices
//! 1. Use only in Print statements.
//! 2. Avoid using as a function in expressions.
//! 3. Use with Spc for custom spacing.
//! 4. Test output on different devices (screen, file).
//! 5. Use variables for dynamic columns.
//! 6. Document column positions for maintainability.
//! 7. Avoid negative or zero columns.
//! 8. Use for tabular data formatting.
//! 9. Combine with loops for tables.
//! 10. Use omitted column for default zones.
//!
//! ## Comparison Table
//!
//! | Function | Purpose | Input | Returns |
//! |----------|---------|-------|---------|
//! | `Tab`    | Print position | column (optional) | Print formatting |
//! | `Spc`    | Print spaces | count | Print formatting |
//! | `Chr$(9)`| Tab character | n/a | String |
//!
//! ## Platform Notes
//! - Available in VB6, VBA, `VBScript`
//! - Consistent across platforms
//! - Only for Print statements
//!
//! ## Limitations
//! - Not a function for assignment or calculation
//! - Only meaningful in Print context
//! - Not the same as the Tab character (Chr$(9))
//! - Cannot be used outside Print/Debug.Print/Print #

use crate::error::{VBError, VBResult};
use crate::state::file;
use crate::value::{VBLong, VBVariant};

/// Implementation of the `Tab` function.
///
/// Moves to an absolute column position within a Print statement. When the
/// current print position is already at or past the requested column, the
/// function starts a new line first.
///
/// VB6 behavior:
/// - `Tab(n)` moves to column *n* (1-based)
/// - If `column` is less than 1, output starts at column 1
/// - If current position >= `column`, a newline is emitted first
/// - Returns the appropriate spacing to reach the target column
pub fn tab(file_number: i16, column: &VBLong) -> VBResult<VBVariant> {
    let target = column.as_i32();
    if target < 1 {
        return Err(VBError::invalid_procedure_call());
    }

    let current = file::get_print_column(file_number);
    let target = target as usize;

    let spaces = if current >= target {
        // Need to start a new line first; reset puts us at column 1
        file::reset_print_column(file_number);
        target - 1 // move from column 1 to target
    } else {
        target - current
    };

    file::advance_print_column(file_number, spaces);
    Ok(VBVariant::from_string(" ".repeat(spaces)))
}

/// Implementation of the `Tab` function with omitted column.
///
/// When the column argument is omitted, Tab moves to the next print zone.
/// Print zones are every 14 columns by default (zone width).
pub fn tab_next_zone(file_number: i16) -> VBResult<VBVariant> {
    let current = file::get_print_column(file_number);
    let zone_width = file::zone_width();
    let next_zone = ((current / zone_width) + 1) * zone_width;
    let spaces = next_zone - current;

    file::advance_print_column(file_number, spaces);
    Ok(VBVariant::from_string(" ".repeat(spaces)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::err_number;
    use crate::state::file::{self, AccessMode, LockMode, OpenMode};

    fn setup_test_file(file_number: i16) -> tempfile::TempDir {
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
            file_number,
        )
        .unwrap();
        dir
    }

    #[test]
    fn tab_at_start() {
        let _dir = setup_test_file(1);
        let result = tab(1, &VBLong::from(10)).unwrap();
        assert_eq!(result, VBVariant::from_string("         "));
        assert_eq!(file::get_print_column(1), 10);
    }

    #[test]
    fn tab_after_content() {
        let _dir = setup_test_file(1);
        file::advance_print_column(1, 5); // simulate "Hello" -> now at column 6
        let result = tab(1, &VBLong::from(15)).unwrap();
        // From column 6 to column 15 = 9 spaces
        assert_eq!(result, VBVariant::from_string("         "));
        assert_eq!(file::get_print_column(1), 15);
    }

    #[test]
    fn tab_wraps_to_new_line() {
        let _dir = setup_test_file(1);
        file::advance_print_column(1, 20); // already past column 10 -> now at column 21
        let result = tab(1, &VBLong::from(10)).unwrap();
        // Should reset to column 1 and move to column 10 = 9 spaces
        assert_eq!(result, VBVariant::from_string("         "));
        assert_eq!(file::get_print_column(1), 10);
    }

    #[test]
    fn tab_rejects_zero() {
        let _dir = setup_test_file(1);
        assert_eq!(
            tab(1, &VBLong::from(0)).unwrap_err().number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }

    #[test]
    fn tab_rejects_negative() {
        let _dir = setup_test_file(1);
        assert_eq!(
            tab(1, &VBLong::from(-5)).unwrap_err().number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }

    #[test]
    fn tab_next_zone_from_start() {
        let _dir = setup_test_file(1);
        let result = tab_next_zone(1).unwrap();
        // From column 1, next zone is 14, so 13 spaces
        assert_eq!(result, VBVariant::from_string(" ".repeat(13)));
        assert_eq!(file::get_print_column(1), 14);
    }

    #[test]
    fn tab_next_zone_from_middle() {
        let _dir = setup_test_file(1);
        file::advance_print_column(1, 5);
        let result = tab_next_zone(1).unwrap();
        // From column 6, next zone is 14, so 8 spaces
        assert_eq!(result, VBVariant::from_string(" ".repeat(8)));
        assert_eq!(file::get_print_column(1), 14);
    }

    #[test]
    fn conversion_error() {
        let non_numeric = VBVariant::from_string("abc");
        let result = VBLong::try_from(&non_numeric);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn tab_separate_files() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();
        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let path1 = std::path::PathBuf::from("test1.txt");
        let path2 = std::path::PathBuf::from("test2.txt");
        file::open_file(
            &path1,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();
        file::open_file(
            &path2,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            2,
        )
        .unwrap();

        file::advance_print_column(1, 5);
        file::advance_print_column(2, 10);
        let _ = tab(1, &VBLong::from(20)).unwrap();
        let _ = tab(2, &VBLong::from(30)).unwrap();
        assert_eq!(file::get_print_column(1), 20);
        assert_eq!(file::get_print_column(2), 30);
    }
}
