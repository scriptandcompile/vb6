//! # `LenB` Function
//!
//! The `LenB` function returns a `Long` containing the number of bytes used to represent a string in memory.
//! This function operates on byte count rather than character count, which is important when working with
//! ANSI strings, DBCS (Double-Byte Character Set), or when you need to know the actual memory footprint of a string.
//!
//! ## Syntax
//!
//! ```vb
//! LenB(string | varname)
//! ```
//!
//! ## Parameters
//!
//! - `string` - Any valid `String` expression.
//! - `varname` - Any valid variable name. If `varname` contains `Null`, `Null` is returned.
//!
//! ## Return Value
//!
//! Returns a `Long` specifying the number of bytes required to store the string or variable in memory.
//!
//! ## Behavior
//!
//! - For ANSI strings (single-byte character sets), `LenB` returns the same value as `Len`.
//! - For Unicode strings (VB6 default), `LenB` returns twice the value of `Len` because each Unicode character requires 2 bytes.
//! - For DBCS strings, the byte count depends on whether characters are single-byte or double-byte.
//! - If the argument is `Null`, `LenB` returns `Null`.
//! - When used with user-defined types, `LenB` returns the total byte size of the type.
//!
//! ## Difference from Len
//!
//! The `LenB` function returns the byte count, while the `Len` function returns the character count.
//! For single-byte character sets, they are identical. For Unicode (VB6's default string type),
//! `LenB` will return twice the value of `Len`.
//!
//! ## Examples
//!
//! ```vb
//! ' Get byte length of a string
//! Dim size As Long
//! size = LenB("Hello")  ' Returns 10 (5 characters * 2 bytes each in Unicode)
//!
//! ' Compare with character length
//! Dim charLen As Long
//! Dim byteLen As Long
//! charLen = Len("Test")   ' Returns 4
//! byteLen = LenB("Test")  ' Returns 8 (Unicode)
//!
//! ' Check memory size
//! Dim buffer As String
//! buffer = Space$(100)
//! Dim bufferSize As Long
//! bufferSize = LenB(buffer)  ' Returns 200 bytes
//! ```

use crate::{
    error::VBResult,
    value::{VBLong, VBString},
};

/// Returns the number of bytes used to represent the string in memory.
///
/// This runtime models a VB6 string as a sequence of Unicode scalar values in
/// memory, with each character occupying 2 bytes (matching VB6's UCS-2/UTF-16
/// storage for the BMP). `LenB` therefore returns `2 * Len`. As with `Len`,
/// surrogate pairs count as a single character.
///
/// # Examples
///
/// ```
/// use vb6runtime::library::string::lenb;
/// use vb6runtime::value::{VBLong, VBString};
/// assert_eq!(lenb(&VBString::from("Hello")).unwrap(), VBLong::from(10));
/// assert_eq!(lenb(&VBString::from("")).unwrap(), VBLong::from(0));
/// ```
pub fn lenb(input: &VBString) -> VBResult<VBLong> {
    Ok(VBLong::from(input.as_str().chars().count() as i32 * 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_bytes() {
        assert_eq!(lenb(&VBString::from("Hello")).unwrap(), VBLong::from(10));
        assert_eq!(lenb(&VBString::from("")), Ok(VBLong::from(0)));
        assert_eq!(lenb(&VBString::from("A")).unwrap(), VBLong::from(2));
    }

    #[test]
    fn is_twice_character_length() {
        assert_eq!(lenb(&VBString::from("héllo")).unwrap(), VBLong::from(10));
        assert_eq!(lenb(&VBString::from("中")).unwrap(), VBLong::from(2));
    }
}
