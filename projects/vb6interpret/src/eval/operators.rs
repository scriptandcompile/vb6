//! Pure operator semantics for binary expressions.
//!
//! These functions implement VB6 arithmetic, logical/bitwise, and ordered
//! comparison over [`VBVariant`] values without touching the CST or an
//! [`Interpreter`](crate::interpreter::Interpreter). Errors are returned as
//! bare [`VBError`]s; callers attach source position.

use vb6core::error::{VBError, VBResult};
use vb6runtime::VBVariant;

/// Logical/bitwise operators shared by `And`, `Or`, `Xor`, `Eqv`, `Imp`.
#[derive(Clone, Copy, Debug)]
pub(crate) enum LogicalOperator {
    And,
    Or,
    Xor,
    Eqv,
    Imp,
}

/// Arithmetic operators shared by `+`, `-`, `*`, `/`, `\`, `Mod`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    IntegerDivide,
    Modulus,
    Exponent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Ordering {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

/// `+` operator: numeric addition, or string concatenation when both
/// operands are strings.
pub(crate) fn add(lhs: VBVariant, rhs: VBVariant) -> VBResult<VBVariant> {
    match (&lhs, &rhs) {
        (VBVariant::String(_), VBVariant::String(_)) => {
            let left = lhs.as_string()?;
            let right = rhs.as_string()?;
            Ok(VBVariant::from_string(format!("{left}{right}")))
        }
        (VBVariant::String(_), _) | (_, VBVariant::String(_)) => Err(VBError::type_mismatch()),
        _ => arith(lhs, rhs, ArithmeticOperator::Add),
    }
}

/// Generic arithmetic dispatch on an arithmetic operator.
pub(crate) fn arith(lhs: VBVariant, rhs: VBVariant, op: ArithmeticOperator) -> VBResult<VBVariant> {
    // Integer arithmetic when both operands are integral types. (This
    // must not fire for Singles/Doubles/Currency, which hold fractional
    // values that `as_i64` would silently round.)
    if op != ArithmeticOperator::Divide
        && op != ArithmeticOperator::Exponent
        && lhs.is_integral()
        && rhs.is_integral()
    {
        let li = lhs.as_i64().ok();
        let ri = rhs.as_i64().ok();
        if let (Some(left), Some(right)) = (li, ri) {
            let result = match op {
                ArithmeticOperator::Add => left.checked_add(right),
                ArithmeticOperator::Subtract => left.checked_sub(right),
                ArithmeticOperator::Multiply => left.checked_mul(right),
                ArithmeticOperator::IntegerDivide => {
                    if right == 0 {
                        return Err(VBError::division_by_zero());
                    }
                    Some(left.div_euclid(right))
                }
                ArithmeticOperator::Modulus => {
                    if right == 0 {
                        return Err(VBError::division_by_zero());
                    }
                    Some(left.rem_euclid(right))
                }
                _ => None,
            };
            if let Some(value) = result {
                return Ok(VBVariant::from_i64(value));
            }
        }
    }

    let left = lhs.as_f64()?;
    let right = rhs.as_f64()?;
    let result = match op {
        ArithmeticOperator::Add => left + right,
        ArithmeticOperator::Subtract => left - right,
        ArithmeticOperator::Multiply => left * right,
        ArithmeticOperator::Divide => {
            if right == 0.0 {
                return Err(VBError::division_by_zero());
            }
            left / right
        }
        ArithmeticOperator::IntegerDivide => {
            if right == 0.0 {
                return Err(VBError::division_by_zero());
            }
            (left / right).floor()
        }
        ArithmeticOperator::Modulus => {
            if right == 0.0 {
                return Err(VBError::division_by_zero());
            }
            left % right
        }
        ArithmeticOperator::Exponent => left.powf(right),
        // Add other operators here if needed.
    };
    Ok(VBVariant::from_double(result))
}

/// Logical/bitwise operators. Booleans combine logically and yield a
/// Boolean; any other operands combine bitwise over their integral value
/// (booleans coerce to -1/0), like VB6.
pub(crate) fn bitwise(lhs: VBVariant, rhs: VBVariant, op: LogicalOperator) -> VBResult<VBVariant> {
    if let (VBVariant::Boolean(left), VBVariant::Boolean(right)) = (&lhs, &rhs) {
        let result = match op {
            LogicalOperator::And => *left && *right,
            LogicalOperator::Or => *left || *right,
            LogicalOperator::Xor => *left != *right,
            LogicalOperator::Eqv => *left == *right,
            LogicalOperator::Imp => !*left || *right,
        };
        return Ok(VBVariant::Boolean(result));
    }

    let left = lhs.as_i64()?;
    let right = rhs.as_i64()?;
    let result = match op {
        LogicalOperator::And => left & right,
        LogicalOperator::Or => left | right,
        LogicalOperator::Xor => left ^ right,
        LogicalOperator::Eqv => !(left ^ right),
        LogicalOperator::Imp => !left | right,
    };

    Ok(VBVariant::from_i64(result))
}

/// Ordered comparison with VB6 coercion.
pub(crate) fn compare_ord(lhs: VBVariant, rhs: VBVariant, ord: Ordering) -> VBResult<VBVariant> {
    let ordering = match (&lhs, &rhs) {
        (VBVariant::String(left), VBVariant::String(right)) => compare_strings(left, right),
        _ => match (lhs.as_f64(), rhs.as_f64()) {
            (Ok(left), Ok(right)) => left.partial_cmp(&right),
            _ => {
                return Err(VBError::type_mismatch());
            }
        },
    };
    let result = matches!(
        (ordering, ord),
        (Some(std::cmp::Ordering::Less), Ordering::Less)
            | (Some(std::cmp::Ordering::Less), Ordering::LessOrEqual)
            | (Some(std::cmp::Ordering::Equal), Ordering::LessOrEqual)
            | (Some(std::cmp::Ordering::Greater), Ordering::Greater)
            | (Some(std::cmp::Ordering::Greater), Ordering::GreaterOrEqual)
            | (Some(std::cmp::Ordering::Equal), Ordering::GreaterOrEqual)
    );
    Ok(VBVariant::Boolean(result))
}

/// Case-insensitive string comparison (VB6 default `Option Compare`).
fn compare_strings(left: &str, right: &str) -> Option<std::cmp::Ordering> {
    let left_lower = left.to_lowercase();
    let right_lower = right.to_lowercase();
    Some(left_lower.cmp(&right_lower))
}

#[cfg(test)]
mod tests {
    use vb6core::error::err_number;

    use super::*;

    #[test]
    fn add_concatenates_two_strings() {
        let result = add(VBVariant::from_string("a"), VBVariant::from_string("b")).unwrap();
        assert_eq!(result.as_string().unwrap(), "ab");
    }

    #[test]
    fn add_with_any_string_operand_is_type_mismatch() {
        let err = add(VBVariant::from_string("a"), VBVariant::from_i64(1)).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn add_propagates_null_propagation_from_the_numeric_path() {
        // `Null` is not a string operand, so `add` falls through to `arith`,
        // where the numeric conversion raises "Invalid use of Null" (94).
        let err = add(VBVariant::from_i64(1), VBVariant::Null).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn add_on_integers_stays_integral() {
        let result = add(VBVariant::from_i64(2), VBVariant::from_i64(3)).unwrap();
        assert_eq!(result, VBVariant::from_i64(5));
    }

    #[test]
    fn integer_overflow_falls_back_to_double() {
        let max = VBVariant::from_i64(i64::MAX);
        let one = VBVariant::from_i64(1);
        let result = arith(max, one, ArithmeticOperator::Add).unwrap();
        assert!(matches!(result, VBVariant::Double(_)));
    }

    #[test]
    fn integer_division_uses_euclid_semantics() {
        let result = arith(
            VBVariant::from_i64(-7),
            VBVariant::from_i64(2),
            ArithmeticOperator::IntegerDivide,
        )
        .unwrap();
        assert_eq!(result, VBVariant::from_i64(-4));
        let result = arith(
            VBVariant::from_i64(-7),
            VBVariant::from_i64(2),
            ArithmeticOperator::Modulus,
        )
        .unwrap();
        assert_eq!(result, VBVariant::from_i64(1));
    }

    #[test]
    fn division_by_zero_is_error_11() {
        let zero = VBVariant::from_i64(0);
        let one = VBVariant::from_i64(1);
        for op in [
            ArithmeticOperator::IntegerDivide,
            ArithmeticOperator::Modulus,
            ArithmeticOperator::Divide,
        ] {
            let err = arith(one.clone(), zero.clone(), op).unwrap_err();
            assert_eq!(err.number, err_number::DIVISION_BY_ZERO);
        }
    }

    #[test]
    fn fractional_operands_take_the_float_path() {
        let result = arith(
            VBVariant::from_single(7.5),
            VBVariant::from_i64(2),
            ArithmeticOperator::IntegerDivide,
        )
        .unwrap();
        assert!(matches!(result, VBVariant::Double(v) if (v - 3.0).abs() < f64::EPSILON));
    }

    #[test]
    fn exponent_always_produces_a_double() {
        let result = arith(
            VBVariant::from_i64(2),
            VBVariant::from_i64(10),
            ArithmeticOperator::Exponent,
        )
        .unwrap();
        assert!(matches!(result, VBVariant::Double(v) if v == 1024.0));
    }

    #[test]
    fn boolean_operands_combine_logically() {
        let t = VBVariant::Boolean(true);
        let f = VBVariant::Boolean(false);
        assert_eq!(
            bitwise(t.clone(), f.clone(), LogicalOperator::Imp).unwrap(),
            VBVariant::Boolean(false)
        );
        assert_eq!(
            bitwise(t.clone(), t.clone(), LogicalOperator::Eqv).unwrap(),
            VBVariant::Boolean(true)
        );
        assert_eq!(
            bitwise(f, t, LogicalOperator::Or).unwrap(),
            VBVariant::Boolean(true)
        );
    }

    #[test]
    fn non_boolean_operands_combine_bitwise() {
        let six = VBVariant::from_i64(6);
        let three = VBVariant::from_i64(3);
        assert_eq!(
            bitwise(six.clone(), three.clone(), LogicalOperator::And).unwrap(),
            VBVariant::from_i64(2)
        );
        assert_eq!(
            bitwise(six, three, LogicalOperator::Xor).unwrap(),
            VBVariant::from_i64(5)
        );
    }

    #[test]
    fn string_comparison_is_case_insensitive() {
        let a = VBVariant::from_string("a");
        let b = VBVariant::from_string("B");
        assert_eq!(
            compare_ord(a, b, Ordering::Less).unwrap(),
            VBVariant::Boolean(true)
        );
    }

    #[test]
    fn numeric_comparison_uses_magnitude() {
        assert_eq!(
            compare_ord(
                VBVariant::from_i64(5),
                VBVariant::from_i64(5),
                Ordering::GreaterOrEqual
            )
            .unwrap(),
            VBVariant::Boolean(true)
        );
        assert_eq!(
            compare_ord(
                VBVariant::from_i64(4),
                VBVariant::from_i64(5),
                Ordering::GreaterOrEqual
            )
            .unwrap(),
            VBVariant::Boolean(false)
        );
    }

    #[test]
    fn mixed_comparison_of_number_and_text_is_type_mismatch() {
        let err = compare_ord(
            VBVariant::from_i64(1),
            VBVariant::from_string("x"),
            Ordering::Less,
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }
}
