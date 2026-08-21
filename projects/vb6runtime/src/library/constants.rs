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
// Message-box constants (`VbMsgBoxStyle` / `VbMsgBoxResult`)
// ---------------------------------------------------------------------------

/// `MsgBox` style — display OK button only (`vbOKOnly`).
pub const VB_OK_ONLY: i32 = 0;

/// `MsgBox` style — display OK and Cancel buttons (`vbOKCancel`).
pub const VB_OK_CANCEL: i32 = 1;

/// `MsgBox` style — display Abort, Retry, and Ignore buttons
/// (`vbAbortRetryIgnore`).
pub const VB_ABORT_RETRY_IGNORE: i32 = 2;

/// `MsgBox` style — display Yes, No, and Cancel buttons (`vbYesNoCancel`).
pub const VB_YES_NO_CANCEL: i32 = 3;

/// `MsgBox` style — display Yes and No buttons (`vbYesNo`).
pub const VB_YES_NO: i32 = 4;

/// `MsgBox` style — display Retry and Cancel buttons (`vbRetryCancel`).
pub const VB_RETRY_CANCEL: i32 = 5;

/// `MsgBox` style — display Critical Message icon (`vbCritical`).
pub const VB_CRITICAL: i32 = 16;

/// `MsgBox` style — display Warning Query icon (`vbQuestion`).
pub const VB_QUESTION: i32 = 32;

/// `MsgBox` style — display Warning Message icon (`vbExclamation`).
pub const VB_EXCLAMATION: i32 = 48;

/// `MsgBox` style — display Information Message icon (`vbInformation`).
pub const VB_INFORMATION: i32 = 64;

/// `MsgBox` style — first button is the default (`vbDefaultButton1`).
pub const VB_DEFAULT_BUTTON_1: i32 = 0;

/// `MsgBox` style — second button is the default (`vbDefaultButton2`).
pub const VB_DEFAULT_BUTTON_2: i32 = 256;

/// `MsgBox` style — third button is the default (`vbDefaultButton3`).
pub const VB_DEFAULT_BUTTON_3: i32 = 512;

/// `MsgBox` style — fourth button is the default (`vbDefaultButton4`).
pub const VB_DEFAULT_BUTTON_4: i32 = 768;

/// `MsgBox` style — application-modal dialog (`vbApplicationModal`).
pub const VB_APPLICATION_MODAL: i32 = 0;

/// `MsgBox` style — system-modal dialog (`vbSystemModal`).
pub const VB_SYSTEM_MODAL: i32 = 4096;

/// `MsgBox` style — add a Help button (`vbMsgBoxHelpButton`).
pub const VB_MSG_BOX_HELP_BUTTON: i32 = 16384;

/// `MsgBox` style — make the message box window foreground
/// (`vbMsgBoxSetForeground`).
pub const VB_MSG_BOX_SET_FOREGROUND: i32 = 65536;

/// `MsgBox` style — right-align the text (`vbMsgBoxRight`).
pub const VB_MSG_BOX_RIGHT: i32 = 524288;

/// `MsgBox` style — right-to-left reading (`vbMsgBoxRtlReading`).
pub const VB_MSG_BOX_RTL_READING: i32 = 1048576;

/// `MsgBox` result — OK button was clicked (`vbOK`).
pub const VB_OK: i32 = 1;

/// `MsgBox` result — Cancel button was clicked (`vbCancel`).
pub const VB_CANCEL: i32 = 2;

/// `MsgBox` result — Abort button was clicked (`vbAbort`).
pub const VB_ABORT: i32 = 3;

/// `MsgBox` result — Retry button was clicked (`vbRetry`).
pub const VB_RETRY: i32 = 4;

/// `MsgBox` result — Ignore button was clicked (`vbIgnore`).
pub const VB_IGNORE: i32 = 5;

/// `MsgBox` result — Yes button was clicked (`vbYes`).
pub const VB_YES: i32 = 6;

/// `MsgBox` result — No button was clicked (`vbNo`).
pub const VB_NO: i32 = 7;

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

    #[test]
    fn msgbox_style_constants_match_vb6() {
        assert_eq!(VB_OK_ONLY, 0);
        assert_eq!(VB_OK_CANCEL, 1);
        assert_eq!(VB_ABORT_RETRY_IGNORE, 2);
        assert_eq!(VB_YES_NO_CANCEL, 3);
        assert_eq!(VB_YES_NO, 4);
        assert_eq!(VB_RETRY_CANCEL, 5);
        assert_eq!(VB_CRITICAL, 16);
        assert_eq!(VB_QUESTION, 32);
        assert_eq!(VB_EXCLAMATION, 48);
        assert_eq!(VB_INFORMATION, 64);
        assert_eq!(VB_DEFAULT_BUTTON_2, 256);
        assert_eq!(VB_DEFAULT_BUTTON_3, 512);
        assert_eq!(VB_DEFAULT_BUTTON_4, 768);
        assert_eq!(VB_SYSTEM_MODAL, 4096);
        assert_eq!(VB_MSG_BOX_HELP_BUTTON, 16384);
        assert_eq!(VB_MSG_BOX_SET_FOREGROUND, 65536);
        assert_eq!(VB_MSG_BOX_RIGHT, 524288);
        assert_eq!(VB_MSG_BOX_RTL_READING, 1048576);
    }

    #[test]
    fn msgbox_result_constants_match_vb6() {
        assert_eq!(VB_OK, 1);
        assert_eq!(VB_CANCEL, 2);
        assert_eq!(VB_ABORT, 3);
        assert_eq!(VB_RETRY, 4);
        assert_eq!(VB_IGNORE, 5);
        assert_eq!(VB_YES, 6);
        assert_eq!(VB_NO, 7);
    }
}
