//! # `MidB` Statement
//!
//! Replaces a specified number of bytes in a Variant (String) variable with bytes from another string.
//!
//! ## Syntax
//!
//! ```vb
//! MidB(stringvar, start[, length]) = string
//! ```
//!
//! - `stringvar`: Required. Name of string variable to modify
//! - `start`: Required. Byte position where replacement begins (1-based)
//! - `length`: Optional. Number of bytes to replace. If omitted, uses entire length of `string`
//! - `string`: Required. String expression used as replacement
//!
//! ## Remarks
//!
//! - `MidB` is used with byte data contained in a string
//! - Works with byte positions rather than character positions (important for double-byte character sets)
//! - The number of bytes replaced is always less than or equal to the number of bytes in `stringvar`
//! - If `start` is greater than the number of bytes in `stringvar`, `stringvar` is unchanged
//! - If `length` is omitted, all bytes from `start` to the end of the string are replaced
//! - `MidB` statement replaces bytes in-place; it does not change the byte length of the original string
//! - If replacement string is longer than `length`, only `length` bytes are used
//! - If replacement string is shorter than `length`, only available bytes are replaced
//! - Primarily used when working with double-byte character sets (DBCS) like Japanese, Chinese, or Korean
//!
//! ## Examples
//!
//! ```vb
//! Dim s As String
//! s = "ABCDEFGH"
//! MidB(s, 3, 2) = "12"       ' Replaces 2 bytes starting at byte 3
//!
//! ' For DBCS strings:
//! Dim dbcsStr As String
//! dbcsStr = "日本語"          ' Japanese characters
//! MidB(dbcsStr, 1, 2) = "XX" ' Replaces first 2 bytes
//! ```
//!
//! ## Reference
//!
//! [MidB Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/midb-statement)
