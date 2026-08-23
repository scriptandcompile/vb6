//! Pure VB6 literal parsing.
//!
//! Turns raw literal token text into [`VBVariant`] values without touching
//! the CST or an [`Interpreter`](crate::interpreter::Interpreter).

use vb6parse::parsers::SyntaxKind;
use vb6runtime::VBVariant;

/// Parse a literal token's text into a runtime value.
///
/// Handles the raw literal token kinds (`IntegerLiteral`, `StringLiteral`, ...)
/// and suffix characters (`%` Integer, `&` Long, `!` Single, `#` Double, `@`
/// Currency).
pub(crate) fn literal_value(text: &str, kind: SyntaxKind) -> Option<VBVariant> {
    let raw = text.trim();

    match kind {
        SyntaxKind::StringLiteral => {
            let inner = raw
                .strip_prefix('"')
                .and_then(|rest| rest.strip_suffix('"'))?;
            let unescaped = inner.replace("\"\"", "\"");
            Some(VBVariant::from_string(unescaped))
        }
        SyntaxKind::DateLiteral => {
            let inner = raw
                .strip_prefix('#')
                .and_then(|rest| rest.strip_suffix('#'))?;
            VBVariant::from_string(inner)
                .as_date_serial()
                .ok()
                .map(VBVariant::Date)
        }
        SyntaxKind::TrueKeyword => Some(VBVariant::Boolean(true)),
        SyntaxKind::FalseKeyword => Some(VBVariant::Boolean(false)),
        SyntaxKind::IntegerLiteral => parse_integer(raw),
        SyntaxKind::LongLiteral => parse_long(raw),
        SyntaxKind::SingleLiteral => {
            let text = strip_suffix(raw);
            text.parse::<f32>().ok().map(VBVariant::from_single)
        }
        SyntaxKind::DoubleLiteral => {
            let text = strip_suffix(raw);
            text.parse::<f64>().ok().map(VBVariant::from_double)
        }
        SyntaxKind::CurrencyLiteral => {
            let text = strip_suffix(raw);
            text.parse::<f64>().ok().map(VBVariant::from_currency)
        }
        SyntaxKind::DecimalLiteral => {
            let text = strip_suffix(raw);
            text.parse::<f64>().ok().map(VBVariant::from_double)
        }
        _ => None,
    }
}

/// Strip a trailing VB6 type-suffix character.
fn strip_suffix(raw: &str) -> &str {
    match raw.chars().last() {
        Some('%') | Some('&') | Some('!') | Some('#') | Some('@') => &raw[..raw.len() - 1],
        _ => raw,
    }
}

/// Parse an integer literal into Integer (i16) or Long (i32) semantics.
fn parse_integer(raw: &str) -> Option<VBVariant> {
    let text = strip_suffix(raw);
    let upper = text.to_ascii_uppercase();
    if let Some(digits) = upper.strip_prefix("&H") {
        return radix_value(digits, 16);
    }
    if let Some(digits) = upper.strip_prefix("&O") {
        return radix_value(digits, 8);
    }
    let value = text.parse::<i64>().ok()?;
    Some(VBVariant::from_i64(value))
}

/// Parse a `LongLiteral` (always a Long).
fn parse_long(raw: &str) -> Option<VBVariant> {
    let text = strip_suffix(raw);
    let upper = text.to_ascii_uppercase();
    if let Some(digits) = upper.strip_prefix("&H") {
        return radix_value(digits, 16);
    }
    if let Some(digits) = upper.strip_prefix("&O") {
        return radix_value(digits, 8);
    }
    text.parse::<i32>().ok().map(VBVariant::Long)
}

/// Parse a radix-prefixed literal, honoring VB6's wrap of 32-bit values.
///
/// Values above `i32::MAX` but within `u32::MAX` wrap to their two's
/// complement Long (so `&HFFFFFFFF` is -1). Values beyond 32 bits widen to
/// Double, and digits that overflow even `u32`/`u64` produce no value.
fn radix_value(digits: &str, radix: u32) -> Option<VBVariant> {
    let digits = digits.trim().trim_end_matches('%').trim_end_matches('&');
    if digits.is_empty() {
        return None;
    }
    let v = i64::from_str_radix(digits, radix).ok()?;
    // `&HFFFFFFFF` wraps to -1 Long in VB6.
    if v > i32::MAX as i64 && v <= u32::MAX as i64 {
        return Some(VBVariant::Long(v as i32));
    }
    Some(VBVariant::from_i64(v))
}

#[cfg(test)]
mod tests {
    use vb6parse::parsers::SyntaxKind;
    use vb6runtime::VBVariant;

    use super::*;

    #[test]
    fn string_literal_unescapes_doubled_quotes() {
        assert_eq!(
            literal_value("\"a\"\"b\"", SyntaxKind::StringLiteral).unwrap(),
            VBVariant::from_string("a\"b")
        );
    }

    #[test]
    fn string_literal_requires_quotes_on_both_ends() {
        assert!(literal_value("unquoted", SyntaxKind::StringLiteral).is_none());
    }

    #[test]
    fn integer_literal_parses_and_keeps_small_values_integral() {
        assert_eq!(
            literal_value("-5", SyntaxKind::IntegerLiteral).unwrap(),
            VBVariant::from_i64(-5)
        );
    }

    #[test]
    fn integer_literal_honors_hex_and_octal_prefixes_case_insensitively() {
        assert_eq!(
            literal_value("&hff", SyntaxKind::IntegerLiteral).unwrap(),
            VBVariant::from_i64(255)
        );
        assert_eq!(
            literal_value("&O17", SyntaxKind::LongLiteral).unwrap(),
            VBVariant::Long(15)
        );
    }

    #[test]
    fn type_suffixes_are_stripped_before_parsing() {
        assert_eq!(
            literal_value("42%", SyntaxKind::IntegerLiteral).unwrap(),
            VBVariant::from_i64(42)
        );
        assert!(matches!(
            literal_value("3.14#", SyntaxKind::DoubleLiteral).unwrap(),
            VBVariant::Double(_)
        ));
        assert!(matches!(
            literal_value("1.5!", SyntaxKind::SingleLiteral).unwrap(),
            VBVariant::Single(_)
        ));
        assert!(matches!(
            literal_value("12.34@", SyntaxKind::CurrencyLiteral).unwrap(),
            VBVariant::Currency(_)
        ));
    }

    #[test]
    fn radix_literals_wrap_at_32_bits_like_vb6() {
        // VB6 interprets 32-bit hex/octal literals as two's-complement Longs.
        assert_eq!(
            literal_value("&HFFFFFFFF", SyntaxKind::IntegerLiteral).unwrap(),
            VBVariant::Long(-1)
        );
        assert_eq!(
            literal_value("&HFFFFFFFF&", SyntaxKind::LongLiteral).unwrap(),
            VBVariant::Long(-1)
        );
        assert_eq!(
            literal_value("&O37777777777", SyntaxKind::IntegerLiteral).unwrap(),
            VBVariant::Long(-1)
        );
        assert_eq!(
            literal_value("&H80000000", SyntaxKind::IntegerLiteral).unwrap(),
            VBVariant::Long(i32::MIN)
        );
        assert_eq!(
            literal_value("&H7FFFFFFF", SyntaxKind::IntegerLiteral).unwrap(),
            VBVariant::Long(i32::MAX)
        );
    }

    #[test]
    fn radix_literals_wider_than_32_bits_fall_back_to_double() {
        // Values beyond 32 bits are outside the wrap range and widen to
        // Double via `from_i64`, as before.
        let value = literal_value("&H100000000", SyntaxKind::IntegerLiteral).unwrap();
        assert!(matches!(value, VBVariant::Double(v) if v == 4294967296.0));
    }

    #[test]
    fn trailing_suffix_is_tolerated_on_radix_literals() {
        assert_eq!(
            literal_value("&H7F%", SyntaxKind::IntegerLiteral).unwrap(),
            VBVariant::from_i64(127)
        );
        assert_eq!(
            literal_value("&O17&", SyntaxKind::LongLiteral).unwrap(),
            VBVariant::Long(15)
        );
    }

    #[test]
    fn boolean_keywords_map_to_boolean_variants() {
        assert_eq!(
            literal_value("True", SyntaxKind::TrueKeyword).unwrap(),
            VBVariant::Boolean(true)
        );
        assert_eq!(
            literal_value("False", SyntaxKind::FalseKeyword).unwrap(),
            VBVariant::Boolean(false)
        );
    }

    #[test]
    fn date_literal_parses_into_a_date_variant() {
        let value = literal_value("#1/1/2020#", SyntaxKind::DateLiteral).unwrap();
        assert!(matches!(value, VBVariant::Date(_)));
        // Malformed dates yield no value at all.
        assert!(literal_value("#not-a-date#", SyntaxKind::DateLiteral).is_none());
    }

    #[test]
    fn unknown_kinds_yield_none() {
        assert!(literal_value("x", SyntaxKind::Identifier).is_none());
    }

    #[test]
    fn surrounding_whitespace_is_trimmed() {
        assert_eq!(
            literal_value(" 7 ", SyntaxKind::IntegerLiteral).unwrap(),
            VBVariant::from_i64(7)
        );
    }
}
