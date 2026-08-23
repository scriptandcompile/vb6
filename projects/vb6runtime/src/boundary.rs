//! Typed-argument boundary helpers (plan F6/B1).
//!
//! The single conversion point between the untyped `[VBVariant]` argument
//! slices that hosts receive and the typed wrappers (`VBString`, `VBLong`, ...)
//! that library functions declare. Every coercion routes through the
//! `TryFrom<&VBVariant>` impls on the wrappers, which encode CLng/CInt/CByte
//! semantics — banker's rounding, overflow (error 6), type mismatch (error 13),
//! invalid use of Null (error 94) — so behavior cannot drift between call sites,
//! and a `CVErr` value re-raises its inner error instead of being misread.
//!
//! Evaluation order is canonical (plan A9): presence first (error 450), then
//! left-to-right conversion via `?` at the call site, so the leftmost offending
//! argument wins. [`Nullable`] is the explicit representation for parameters of
//! Variant-returning functions that propagate Null instead of raising 94
//! (decision F1).

use vb6core::error::{err_number, VBError, VBResult};

use crate::value::VBVariant;

/// Coerce the argument at `index` to `T`.
///
/// Absent arguments raise error 450 *before* any conversion is attempted;
/// present-but-uncoercible values raise whatever the wrapper's `TryFrom`
/// decides (13/94/6 or the embedded CVErr error).
pub fn arg<'a, T>(args: &'a [VBVariant], index: usize) -> VBResult<T>
where
    T: TryFrom<&'a VBVariant, Error = VBError>,
{
    let value = args
        .get(index)
        .ok_or_else(|| VBError::new(err_number::WRONG_NUMBER_OF_ARGUMENTS))?;
    T::try_from(value)
}

/// Like [`arg`], but maps an absent argument to `None`.
///
/// A present argument still converts eagerly: "missing" and "present but
/// unconvertible" stay distinct errors.
pub fn opt_arg<'a, T>(args: &'a [VBVariant], index: usize) -> VBResult<Option<T>>
where
    T: TryFrom<&'a VBVariant, Error = VBError>,
{
    match args.get(index) {
        None => Ok(None),
        Some(value) => T::try_from(value).map(Some),
    }
}

/// Borrow the raw argument at `index` without coercion.
///
/// For parameters classified genuinely-Variant (predicates, array sources, ...)
/// and for Null-propagating function bodies that inspect the variant directly.
pub fn variant_arg(args: &[VBVariant], index: usize) -> VBResult<&VBVariant> {
    args.get(index)
        .ok_or_else(|| VBError::new(err_number::WRONG_NUMBER_OF_ARGUMENTS))
}

/// Borrow the optional raw argument at `index`; `None` when absent.
pub fn opt_variant_arg(args: &[VBVariant], index: usize) -> Option<&VBVariant> {
    args.get(index)
}

/// A parameter value that may be VB6 `Null` (plan A2/F1).
///
/// Converting a `Null` variant yields [`Nullable::Null`] instead of raising
/// error 94; every other value coerces through `T`. Function bodies propagate
/// the `Null` to their result (typically `VBVariant::Null`) rather than
/// silently treating it as a default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nullable<T> {
    /// A successfully coerced value.
    Value(T),
    /// The argument was VB6 `Null`.
    Null,
}

impl<T> Nullable<T> {
    /// `true` when the parameter was `Null`.
    pub fn is_null(&self) -> bool {
        matches!(self, Nullable::Null)
    }

    /// The coerced value, if not `Null`.
    pub fn as_value(&self) -> Option<&T> {
        match self {
            Nullable::Value(value) => Some(value),
            Nullable::Null => None,
        }
    }
}

impl<'a, T> TryFrom<&'a VBVariant> for Nullable<T>
where
    T: TryFrom<&'a VBVariant, Error = VBError>,
{
    type Error = VBError;

    fn try_from(value: &'a VBVariant) -> Result<Self, Self::Error> {
        if value.is_null() {
            return Ok(Nullable::Null);
        }
        T::try_from(value).map(Nullable::Value)
    }
}

impl<T> From<Nullable<T>> for VBVariant
where
    T: Into<VBVariant>,
{
    fn from(value: Nullable<T>) -> Self {
        match value {
            Nullable::Value(inner) => inner.into(),
            Nullable::Null => VBVariant::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::{
        VBBoolean, VBByte, VBCurrency, VBDate, VBDouble, VBInteger, VBLong, VBSingle,
    };
    use vb6core::error::err_number;

    #[test]
    fn arg_coerces_through_wrapper_semantics() {
        let args = [VBVariant::from_string("2.5")];
        // Banker's rounding via the CLng path, not truncation.
        assert_eq!(arg::<VBLong>(&args, 0).unwrap().as_i32(), 2);
    }

    #[test]
    fn missing_argument_is_450_before_conversion() {
        let args = [VBVariant::from_string("nope")];
        assert_eq!(
            arg::<VBLong>(&args, 3).unwrap_err().number,
            err_number::WRONG_NUMBER_OF_ARGUMENTS
        );
    }

    #[test]
    fn null_raises_94_for_typed_parameters() {
        let args = [VBVariant::Null];
        assert_eq!(
            arg::<VBSingle>(&args, 0).unwrap_err().number,
            err_number::INVALID_USE_OF_NULL
        );
    }

    #[test]
    fn cverr_re_raises_inner_error() {
        let args = [VBVariant::from_error(VBError::new(31337))];
        assert_eq!(arg::<VBBoolean>(&args, 0).unwrap_err().number, 31337);
    }

    #[test]
    fn opt_arg_distinguishes_absent_from_unconvertible() {
        let absent: [VBVariant; 0] = [];
        assert_eq!(opt_arg::<VBInteger>(&absent, 0).unwrap(), None);

        let bad = [VBVariant::from_string("x")];
        assert_eq!(
            opt_arg::<VBInteger>(&bad, 0).unwrap_err().number,
            err_number::TYPE_MISMATCH
        );

        let good = [VBVariant::from_long(-7)];
        assert_eq!(
            opt_arg::<VBInteger>(&good, 0).unwrap().map(|v| v.as_i16()),
            Some(-7)
        );
    }

    #[test]
    fn variant_args_check_presence_only() {
        let args = [VBVariant::Null];
        assert!(variant_arg(&args, 0).unwrap().is_null());
        assert!(opt_variant_arg(&args, 5).is_none());
        assert_eq!(
            variant_arg(&args, 5).unwrap_err().number,
            err_number::WRONG_NUMBER_OF_ARGUMENTS
        );
    }

    #[test]
    fn nullable_propagates_null_without_raising() {
        let null = [VBVariant::Null];
        let converted: Nullable<VBDate> = arg(&null, 0).unwrap();
        assert!(converted.is_null());
        assert_eq!(VBVariant::from(converted), VBVariant::Null);
    }

    #[test]
    fn nullable_coerces_non_null_values() {
        let args = [VBVariant::from_long(4)];
        let converted: Nullable<VBByte> = arg(&args, 0).unwrap();
        assert!(!converted.is_null());
        assert_eq!(converted.as_value().map(|v| v.as_u8()), Some(4));
        assert_eq!(VBVariant::from(converted), VBVariant::from_byte(4));
    }

    #[test]
    fn nullable_still_rejects_type_mismatch() {
        let args = [VBVariant::from_string("abc")];
        let converted: Result<Nullable<VBCurrency>, _> = arg(&args, 0);
        assert_eq!(converted.unwrap_err().number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn nullable_empty_becomes_default_value() {
        // Empty coerces like the underlying wrapper (0/""/false), it is not Null.
        let empty = [VBVariant::Empty];
        let converted: Nullable<VBDouble> = arg(&empty, 0).unwrap();
        assert_eq!(converted.as_value().map(|v| v.as_f64()), Some(0.0));
    }
}
