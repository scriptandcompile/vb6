//! Windows-1252 (ANSI) helpers shared by the string functions.
//!
//! VB6 stores strings as Unicode internally but the `Asc`/`Chr` family operates
//! on the system ANSI code page. This runtime models that code page as
//! Windows-1252, the default on Western Windows systems.

use crate::error::{err_number, VBError, VBResult};

/// Encodes the first character of `input` to its Windows-1252 (ANSI) byte.
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `input` is empty
/// or its first character cannot be represented in Windows-1252.
pub fn encode_first_byte(input: &str) -> VBResult<u8> {
    let first_char = input.chars().next().ok_or_else(|| {
        VBError::with_description(err_number::INVALID_PROCEDURE_CALL, "String cannot be empty")
    })?;
    encode_char(first_char)
}

/// Encodes a single `char` to its Windows-1252 (ANSI) byte.
///
/// ASCII characters map to themselves; characters representable in Windows-1252
/// map to their ANSI byte (e.g. `é` -> 233, `€` -> 128).
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `c` cannot be
/// represented in Windows-1252.
pub fn encode_char(c: char) -> VBResult<u8> {
    if c.is_ascii() {
        return Ok(c as u8);
    }

    let mut buf = [0u8; 4];
    let encoded = c.encode_utf8(&mut buf);
    let (ansi, _, had_errors) = encoding_rs::WINDOWS_1252.encode(encoded);
    if had_errors {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Character code out of ANSI range",
        ));
    }

    Ok(ansi[0])
}

/// Decodes a Windows-1252 (ANSI) byte back into a `String`.
///
/// Every byte value 0-255 maps to a single character in Windows-1252.
pub fn decode_byte(byte: u8) -> String {
    let bytes = [byte];
    let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(&bytes);
    decoded.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_first_byte_round_trips_ascii() {
        assert_eq!(encode_first_byte("A").unwrap(), 65);
        assert_eq!(encode_first_byte("a").unwrap(), 97);
    }

    #[test]
    fn encode_first_byte_maps_ansi_extended() {
        assert_eq!(encode_first_byte("€").unwrap(), 128);
        assert_eq!(encode_first_byte("œ").unwrap(), 156);
    }

    #[test]
    fn encode_first_byte_rejects_unrepresentable() {
        assert_eq!(
            encode_first_byte("😀").unwrap_err().number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }

    #[test]
    fn encode_first_byte_rejects_empty() {
        assert_eq!(
            encode_first_byte("").unwrap_err().number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }

    #[test]
    fn decode_byte_maps_ansi_extended() {
        assert_eq!(decode_byte(128), "€");
        assert_eq!(decode_byte(233), "é");
    }
}
