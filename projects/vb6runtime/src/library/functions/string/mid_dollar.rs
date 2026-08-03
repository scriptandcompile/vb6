//! # `Mid$` Function
//!
//! The `Mid$` function returns a `String` containing a specified number of characters from a string.
//! The dollar sign suffix (`$`) indicates that this function always returns a `String` type, never a `Variant`.
//!
//! ## Syntax
//!
//! ```vb
//! Mid$(string, start[, length])
//! ```
//!
//! ## Parameters
//!
//! - `string` - Required. `String` expression from which characters are returned.
//! - `start` - Required. `Long`. Character position in `string` at which the part to be taken begins (1-based).
//! - `length` - Optional. `Long`. Number of characters to return. If omitted or if there are fewer than `length` characters in the text (including the character at `start`), all characters from the start position to the end of the string are returned.
//!
//! ## Return Value
//!
//! Returns a `String` containing the specified portion of the input string.
//!
//! ## Behavior
//!
//! - If `start` is greater than the number of characters in `string`, `Mid$` returns a zero-length string ("").
//! - If `start` is less than 1, a runtime error occurs.
//! - If `length` is negative, a runtime error occurs.
//! - The first character in the string is at position 1.
//!
//! ## Difference from Mid
//!
//! The `Mid$` function always returns a `String`, while the `Mid` function (without the dollar sign) can return a `Variant`.
//! In practice, they behave identically in most scenarios, but the dollar sign version may be slightly more efficient
//! as it avoids the overhead of the `Variant` type.
//!
//! ## Examples
//!
//! ```vb
//! ' Extract 3 characters starting at position 2
//! Dim result As String
//! result = Mid$("Hello World", 2, 3)  ' Returns "ell"
//!
//! ' Extract from position 7 to the end
//! result = Mid$("Hello World", 7)  ' Returns "World"
//!
//! ' Start position beyond string length
//! result = Mid$("Hi", 10)  ' Returns ""
//! ```
