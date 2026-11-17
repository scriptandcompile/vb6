use std::cmp::Ordering;
use std::ops::Range;
use std::sync::Arc;

/// Owned VB string storage in UTF-16 code units.
///
/// NUL code units and unpaired surrogates are preserved. This models string
/// contents, not BSTR allocation or locale-sensitive collation.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct VbString {
    units: Arc<[u16]>,
}

impl VbString {
    /// Creates a string from UTF-16 code units without validating surrogates.
    pub fn from_utf16_units(units: impl Into<Vec<u16>>) -> Self {
        Self {
            units: Arc::from(units.into()),
        }
    }

    /// Encodes valid Rust text as UTF-16 code units.
    pub fn from_text(value: &str) -> Self {
        Self {
            units: Arc::from(value.encode_utf16().collect::<Vec<_>>()),
        }
    }

    /// Borrows the stored UTF-16 code units.
    pub fn as_utf16_units(&self) -> &[u16] {
        &self.units
    }

    /// Returns the number of UTF-16 code units.
    pub fn len_utf16(&self) -> usize {
        self.units.len()
    }

    /// Returns whether the string has no code units.
    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    /// Concatenates strings by appending their code units.
    pub fn concat(&self, other: &Self) -> Self {
        let mut units = Vec::with_capacity(self.units.len() + other.units.len());
        units.extend_from_slice(&self.units);
        units.extend_from_slice(&other.units);
        Self::from_utf16_units(units)
    }

    /// Copies a code-unit range, or returns `None` for invalid bounds.
    pub fn slice_utf16(&self, range: Range<usize>) -> Option<Self> {
        if range.start > range.end || range.end > self.units.len() {
            return None;
        }
        Some(Self::from_utf16_units(self.units[range].to_vec()))
    }

    /// Compares the stored code units lexicographically.
    pub fn cmp_binary(&self, other: &Self) -> Ordering {
        self.units.cmp(&other.units)
    }

    /// Converts to Rust text, failing if the units contain invalid surrogates.
    pub fn try_to_string(&self) -> Result<String, std::string::FromUtf16Error> {
        String::from_utf16(&self.units)
    }
}

impl From<&str> for VbString {
    fn from(value: &str) -> Self {
        Self::from_text(value)
    }
}

impl From<String> for VbString {
    fn from(value: String) -> Self {
        Self::from_text(&value)
    }
}

impl Ord for VbString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_binary(other)
    }
}

impl PartialOrd for VbString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::VbString;

    #[test]
    fn retains_embedded_nul_and_unpaired_surrogates() {
        let value = VbString::from_utf16_units(vec![0x61, 0, 0xd800, 0x7a]);
        assert_eq!(value.as_utf16_units(), &[0x61, 0, 0xd800, 0x7a]);
        assert_eq!(value.len_utf16(), 4);
        assert!(value.try_to_string().is_err());
    }

    #[test]
    fn concatenation_and_slicing_use_utf16_code_units() {
        let value = VbString::from("A🦀").concat(&VbString::from("\0B"));
        assert_eq!(value.as_utf16_units(), &[0x41, 0xd83e, 0xdd80, 0, 0x42]);
        assert_eq!(
            value.slice_utf16(1..3).unwrap().as_utf16_units(),
            &[0xd83e, 0xdd80]
        );
        assert!(value.slice_utf16(0..6).is_none());
    }
}
