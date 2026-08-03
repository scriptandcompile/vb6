//! # Mid Statement
//!
//! Replaces a specified number of characters in a Variant (String) variable with characters from another string.
//!
//! ## Syntax
//!
//! ```vb
//! Mid(stringvar, start[, length]) = string
//! ```
//!
//! - `stringvar`: Required. Name of string variable to modify
//! - `start`: Required. Character position where replacement begins (1-based)
//! - `length`: Optional. Number of characters to replace. If omitted, uses entire length of `string`
//! - `string`: Required. String expression used as replacement
//!
//! ## Remarks
//!
//! - The number of characters replaced is always less than or equal to the number of characters in `stringvar`
//! - If `start` is greater than the length of `stringvar`, `stringvar` is unchanged
//! - If `length` is omitted, all characters from `start` to the end of the string are replaced
//! - `Mid` statement replaces characters in-place; it does not change the length of the original string
//! - If replacement string is longer than `length`, only `length` characters are used
//! - If replacement string is shorter than `length`, only available characters are replaced
//!
//! ## Examples
//!
//! ```vb
//! Dim s As String
//! s = "Hello World"
//! Mid(s, 7, 5) = "VB6!!"     ' s becomes "Hello VB6!!"
//!
//! s = "ABCDEFGH"
//! Mid(s, 3) = "123"          ' s becomes "AB123FGH"
//!
//! s = "Test"
//! Mid(s, 2, 2) = "XX"        ' s becomes "TXXt"
//! ```
//!
//! ## Reference
//!
//! [Mid Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/mid-statement)
