//! # `MidB` Function
//!
//! The `MidB` function returns a `Variant` (`String`) containing a specified number of bytes from a string.
//! This function operates on byte positions rather than character positions, which is important when working
//! with ANSI strings or when you need byte-level control over string manipulation.
//!
//! ## Syntax
//!
//! ```vb
//! MidB(string, start[, length])
//! ```
//!
//! ## Parameters
//!
//! - `string` - Required. `String` expression from which bytes are returned.
//! - `start` - Required. `Long`. The `Byte` position in string at which the part to be taken begins (1-based).
//! - `length` - Optional. `Long`. Number of bytes to return. If omitted or if there are fewer than `length` bytes in the text (including the byte at `start`), all bytes from the start position to the end of the string are returned.
//!
//! ## Return Value
//!
//! Returns a `Variant` (`String`) containing the specified byte sequence from the input string.
//!
//! ## Behavior
//!
//! - If `start` is greater than the number of bytes in `string`, `MidB` returns a zero-length string ("").
//! - If `start` is less than 1, a runtime error occurs.
//! - If `length` is negative, a runtime error occurs.
//! - The first byte in the string is at position 1.
//! - When working with DBCS (Double-Byte Character Set) strings, `MidB` can split multi-byte characters if not used carefully.
//!
//! ## Difference from Mid
//!
//! The `MidB` function operates on byte positions, while the `Mid` function operates on character positions.
//! For single-byte character sets (like ASCII), they behave identically. For multi-byte character sets
//! (like Unicode or DBCS), `MidB` provides byte-level access which can be useful for binary data manipulation
//! or low-level string operations.
//!
//! ## Examples
//!
//! ```vb
//! ' Extract 3 bytes starting at byte position 2
//! Dim result As Variant
//! result = MidB("Hello World", 2, 3)  ' Returns "ell"
//!
//! ' Extract from byte position 7 to the end
//! result = MidB("Hello World", 7)  ' Returns "World"
//!
//! ' Start position beyond string length
//! result = MidB("Hi", 10)  ' Returns ""
//! ```


use crate::{
    error::VBResult,
    value::{VBLong, VBVariant},
};

use super::midb_dollar::midb_dollar;

/// `MidB` is the Variant-returning counterpart of `MidB$`; a `Null` input
/// propagates as `Null`.
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `start` is less
/// than 1 or `length` is negative.
pub fn midb(
    input: &VBVariant,
    start: &VBLong,
    length: Option<&VBLong>,
) -> VBResult<VBVariant> {
    if input.is_null() {
        return Ok(VBVariant::Null);
    }
    midb_dollar(&input.as_vbstring()?, start, length).map(VBVariant::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propagates_null() {
        assert_eq!(
            midb(&VBVariant::Null, &VBLong::from(3), None).unwrap(),
            VBVariant::Null
        );
    }
}
