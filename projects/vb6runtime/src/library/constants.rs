//! VB6 built-in data constants.
//!
//! These constants are available in every VB6 program without declaration.
//! They are grouped into character constants, string constants, and
//! miscellaneous constants.
//!
//! Reference: VB6 Language Reference → Constants → Data Constants

/// Carriage return character (`Chr$(13)`).
pub const VB_CR: &str = "\r";

/// Line feed character (`Chr$(10)`).
pub const VB_LF: &str = "\n";

/// Carriage return + Line feed (`vbCr & vbLf`).
pub const VB_CRLF: &str = "\r\n";

/// Platform-specific new-line sequence.
///
/// On Windows (the original VB6 target) this equals `vbCrLf`.
/// On Unix hosts it equals `vbLf` to match the OS convention.
#[cfg(windows)]
pub const VB_NEW_LINE: &str = "\r\n";

/// Platform-specific new-line sequence.
///
/// On non-Windows hosts this equals `vbLf` to match the OS convention.
#[cfg(not(windows))]
pub const VB_NEW_LINE: &str = "\n";

/// Null character (`Chr$(0)`).
pub const VB_NULL_CHAR: &str = "\0";

/// Null string — a zero-length string pointer, distinct from `""`.
///
/// In VB6 `vbNullString` is a pointer-sized null, while `""` is a
/// BSTR with length zero.  For the purposes of string operations the
/// two are interchangeable, so this is exposed as an empty string.
pub const VB_NULL_STRING: &str = "";

/// Tab character (`Chr$(9)`).
pub const VB_TAB: &str = "\t";

/// Backspace character (`Chr$(8)`).
pub const VB_BACK: &str = "\x08";

/// Form feed character (`Chr$(12)`).
pub const VB_FORM_FEED: &str = "\x0C";

/// Vertical tab character (`Chr$(11)`).
pub const VB_VERTICAL_TAB: &str = "\x0B";

/// Conversion constant for `StrConv` — convert to Unicode.
pub const VB_UNICODE: i32 = 64;

/// Conversion constant for `StrConv` — convert from Unicode.
pub const VB_FROM_UNICODE: i32 = 128;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cr_is_single_carriage_return() {
        assert_eq!(VB_CR, "\r");
        assert_eq!(VB_CR.len(), 1);
    }

    #[test]
    fn lf_is_single_line_feed() {
        assert_eq!(VB_LF, "\n");
        assert_eq!(VB_LF.len(), 1);
    }

    #[test]
    fn crlf_is_cr_followed_by_lf() {
        assert_eq!(VB_CRLF, "\r\n");
        assert_eq!(VB_CRLF.len(), 2);
    }

    #[test]
    fn new_line_matches_platform() {
        #[cfg(windows)]
        assert_eq!(VB_NEW_LINE, "\r\n");

        #[cfg(not(windows))]
        assert_eq!(VB_NEW_LINE, "\n");
    }

    #[test]
    fn null_char_is_zero_byte() {
        assert_eq!(VB_NULL_CHAR, "\0");
        assert_eq!(VB_NULL_CHAR.len(), 1);
    }

    #[test]
    fn null_string_is_empty() {
        assert_eq!(VB_NULL_STRING, "");
    }

    #[test]
    fn tab_is_ascii_9() {
        assert_eq!(VB_TAB, "\t");
        assert_eq!(VB_TAB.len(), 1);
    }

    #[test]
    fn back_is_ascii_8() {
        assert_eq!(VB_BACK, "\x08");
    }

    #[test]
    fn form_feed_is_ascii_12() {
        assert_eq!(VB_FORM_FEED, "\x0C");
    }

    #[test]
    fn vertical_tab_is_ascii_11() {
        assert_eq!(VB_VERTICAL_TAB, "\x0B");
    }

    #[test]
    fn unicode_constants_are_numeric() {
        assert_eq!(VB_UNICODE, 64);
        assert_eq!(VB_FROM_UNICODE, 128);
    }
}
