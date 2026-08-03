//! VB6 Line Input statement syntax:
//! - Line Input #filenumber, varname
//!
//! Reads a single line from an open sequential file and assigns it to a String variable.
//!
//! The Line Input # statement syntax has these parts:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | filenumber    | Required. Any valid file number. |
//! | varname       | Required. Valid String or Variant variable name. |
//!
//! ## Remarks
//!
//! - Data read with Line Input # is usually written to a file with Print #.
//! - The Line Input # statement reads from a file one character at a time until it encounters
//!   a carriage return (Chr(13)) or carriage return–linefeed (Chr(13) + Chr(10)) sequence.
//! - Carriage return–linefeed sequences are skipped rather than appended to the character string.
//! - Line Input # is useful for reading text files that have been created in a text editor or
//!   with the Print # statement.
//! - Unlike Input #, Line Input # doesn't parse the data as it's read – you get the entire line as-is.
//! - If end of file is reached before reading a complete line, an error occurs.
//!
//! ## Examples
//!
//! ```vb
//! Line Input #1, textLine
//! Line Input #fileNum, dataBuffer
//! Line Input #1, myString
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/line-input-statement)
