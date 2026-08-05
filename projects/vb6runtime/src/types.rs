//! VB6 type system.
//!
//! [`VBType`] is the single source of truth for VB6 static types. It mirrors the
//! types used by `vb6semantic` for type checking and is the type each runtime
//! [`Value`](crate::value::Value) reports. Keeping the type system here ensures
//! semantic analysis, code generation, and the interpreter all agree.

use std::fmt;

use serde::{Deserialize, Serialize};

/// VBA `VarType` function return codes.
pub mod vartype {
    /// Uninitialized `Variant` (Empty).
    pub const EMPTY: i32 = 0;
    /// Null value.
    pub const NULL: i32 = 1;
    /// Integer (16-bit).
    pub const INTEGER: i32 = 2;
    /// Long (32-bit).
    pub const LONG: i32 = 3;
    /// Single (32-bit float).
    pub const SINGLE: i32 = 4;
    /// Double (64-bit float).
    pub const DOUBLE: i32 = 5;
    /// Currency (scaled 64-bit).
    pub const CURRENCY: i32 = 6;
    /// Date (serial day number).
    pub const DATE: i32 = 7;
    /// String.
    pub const STRING: i32 = 8;
    /// Object reference.
    pub const OBJECT: i32 = 9;
    /// Error value.
    pub const ERROR: i32 = 10;
    /// Boolean.
    pub const BOOLEAN: i32 = 11;
    /// Variant (any value).
    pub const VARIANT: i32 = 12;
    /// Automation data object.
    pub const DATA_OBJECT: i32 = 13;
    /// Decimal.
    pub const DECIMAL: i32 = 14;
    /// Byte (8-bit unsigned).
    pub const BYTE: i32 = 17;
    /// User-defined type.
    pub const USER_DEFINED_TYPE: i32 = 36;
    /// Bitwise OR'd with the element type for arrays.
    pub const ARRAY: i32 = 8192;
}

/// The static type of a VB6 value or variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VBType {
    // Primitive types
    /// 8-bit unsigned integer.
    Byte,
    /// 16-bit signed integer.
    Integer,
    /// 32-bit signed integer.
    Long,
    /// 32-bit floating point.
    Single,
    /// 64-bit floating point.
    Double,
    /// Scaled 64-bit fixed-point currency.
    Currency,
    /// Unicode string.
    String,
    /// Boolean.
    Boolean,
    /// Date stored as a serial day number.
    Date,

    // Complex types
    /// Variant, capable of holding any value.
    Variant,
    /// Generic object reference.
    Object,
    /// A specific class instance.
    Class(String),
    /// A user-defined type (custom structure).
    UserType(String),
    /// An enumeration.
    Enum(String),
    /// An array of the inner type.
    Array(Box<VBType>),

    // Value states
    /// An object reference set to `Nothing`.
    Nothing,
    /// An uninitialized `Variant` (`Empty`).
    Empty,
    /// A `Null` value (unknown data).
    Null,
    /// An error value (from `CVErr`).
    Error,

    /// An unresolved type, used when analysis cannot determine a type.
    Unknown,
}

impl VBType {
    /// The VB6 type name, e.g. `"Integer"` or `"Long()"`.
    pub fn name(&self) -> String {
        match self {
            Self::Byte => "Byte".to_string(),
            Self::Integer => "Integer".to_string(),
            Self::Long => "Long".to_string(),
            Self::Single => "Single".to_string(),
            Self::Double => "Double".to_string(),
            Self::Currency => "Currency".to_string(),
            Self::String => "String".to_string(),
            Self::Boolean => "Boolean".to_string(),
            Self::Date => "Date".to_string(),
            Self::Variant => "Variant".to_string(),
            Self::Object => "Object".to_string(),
            Self::Class(name) => name.clone(),
            Self::UserType(name) => name.clone(),
            Self::Enum(name) => name.clone(),
            Self::Array(inner) => format!("{}()", inner.name()),
            Self::Nothing => "Nothing".to_string(),
            Self::Empty => "Empty".to_string(),
            Self::Null => "Null".to_string(),
            Self::Error => "Error".to_string(),
            Self::Unknown => "Unknown".to_string(),
        }
    }

    /// The VBA `VarType` code for this type.
    pub fn var_type(&self) -> i32 {
        use vartype::*;
        match self {
            Self::Byte => BYTE,
            Self::Integer => INTEGER,
            Self::Long => LONG,
            Self::Single => SINGLE,
            Self::Double => DOUBLE,
            Self::Currency => CURRENCY,
            Self::String => STRING,
            Self::Boolean => BOOLEAN,
            Self::Date => DATE,
            Self::Variant => VARIANT,
            Self::Object | Self::Class(_) | Self::Nothing => OBJECT,
            Self::UserType(_) => USER_DEFINED_TYPE,
            Self::Enum(_) => LONG,
            Self::Array(inner) => ARRAY | inner.var_type(),
            Self::Empty => EMPTY,
            Self::Null => NULL,
            Self::Error => ERROR,
            Self::Unknown => VARIANT,
        }
    }

    /// Whether this is one of the numeric primitive types.
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::Byte | Self::Integer | Self::Long | Self::Single | Self::Double | Self::Currency
        )
    }

    /// Whether this is an integral (non-floating) numeric type.
    pub fn is_integral(&self) -> bool {
        matches!(self, Self::Byte | Self::Integer | Self::Long)
    }

    /// Whether this is a primitive type (no name or element attached).
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Self::Byte
                | Self::Integer
                | Self::Long
                | Self::Single
                | Self::Double
                | Self::Currency
                | Self::String
                | Self::Boolean
                | Self::Date
                | Self::Variant
                | Self::Object
        )
    }

    /// Whether this is one of the value states (`Empty`, `Null`, `Nothing`, `Error`).
    pub fn is_value_state(&self) -> bool {
        matches!(
            self,
            Self::Empty | Self::Null | Self::Nothing | Self::Error
        )
    }

    /// Whether `self` is assignable to `target` following VB6 widening rules.
    ///
    /// Numeric types widen toward `Double`; `Variant` accepts anything; a
    /// specific `Class` widens to generic `Object`.
    pub fn can_assign_to(&self, target: &VBType) -> bool {
        if target == &Self::Variant {
            return true;
        }
        if self == target {
            return true;
        }
        // Numeric widening.
        if self.is_numeric() && target.is_numeric() {
            return self.numeric_rank() <= target.numeric_rank();
        }
        // Specific class widens to generic object.
        if matches!(target, Self::Object) && matches!(self, Self::Class(_)) {
            return true;
        }
        // Value states only assign to Variant (already handled) or themselves.
        if self.is_value_state() || target.is_value_state() {
            return false;
        }
        false
    }

    /// Map from a `VarType` code to the matching type.
    ///
    /// Array codes (bitwise OR of `vbArray` and the element type) are decoded to
    /// [`VBType::Array`]. Returns `None` for codes that have no static type
    /// (e.g. `vbDataObject`).
    pub fn from_var_type(code: i32) -> Option<VBType> {
        use vartype::*;
        if code & ARRAY != 0 {
            let element = Self::from_var_type(code & !ARRAY)?;
            return Some(Self::Array(Box::new(element)));
        }
        match code {
            vartype::EMPTY => Some(Self::Empty),
            vartype::NULL => Some(Self::Null),
            vartype::INTEGER => Some(Self::Integer),
            vartype::LONG => Some(Self::Long),
            vartype::SINGLE => Some(Self::Single),
            vartype::DOUBLE => Some(Self::Double),
            vartype::CURRENCY => Some(Self::Currency),
            vartype::DATE => Some(Self::Date),
            vartype::STRING => Some(Self::String),
            vartype::OBJECT => Some(Self::Object),
            vartype::ERROR => Some(Self::Error),
            vartype::BOOLEAN => Some(Self::Boolean),
            vartype::VARIANT => Some(Self::Variant),
            vartype::DECIMAL => Some(Self::Currency),
            vartype::BYTE => Some(Self::Byte),
            vartype::USER_DEFINED_TYPE => Some(Self::UserType(String::new())),
            _ => None,
        }
    }

    /// Order used for numeric promotion: Byte < Integer < Long < Single < Double < Currency.
    fn numeric_rank(&self) -> u8 {
        match self {
            Self::Byte => 0,
            Self::Integer => 1,
            Self::Long => 2,
            Self::Single => 3,
            Self::Double => 4,
            Self::Currency => 5,
            _ => u8::MAX,
        }
    }
}

impl fmt::Display for VBType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name())
    }
}

impl From<&VBType> for VBType {
    fn from(value: &VBType) -> Self {
        value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_match_vb6() {
        assert_eq!(VBType::Integer.name(), "Integer");
        assert_eq!(VBType::Array(Box::new(VBType::String)).name(), "String()");
        assert_eq!(VBType::Class("Foo".to_string()).name(), "Foo");
    }

    #[test]
    fn var_type_codes_match_vba() {
        assert_eq!(VBType::Integer.var_type(), 2);
        assert_eq!(VBType::Long.var_type(), 3);
        assert_eq!(VBType::String.var_type(), 8);
        assert_eq!(VBType::Boolean.var_type(), 11);
        assert_eq!(VBType::Byte.var_type(), 17);
        assert_eq!(VBType::Empty.var_type(), 0);
        assert_eq!(VBType::Null.var_type(), 1);
        assert_eq!(VBType::Class("x".into()).var_type(), 9);
        assert_eq!(
            VBType::Array(Box::new(VBType::Double)).var_type(),
            8192 + 5
        );
    }

    #[test]
    fn numeric_predicates() {
        assert!(VBType::Long.is_numeric());
        assert!(VBType::Integer.is_integral());
        assert!(!VBType::String.is_numeric());
        assert!(!VBType::Boolean.is_numeric());
        assert!(!VBType::Date.is_numeric());
    }

    #[test]
    fn widening_rules() {
        assert!(VBType::Integer.can_assign_to(&VBType::Long));
        assert!(VBType::Byte.can_assign_to(&VBType::Double));
        assert!(!VBType::Long.can_assign_to(&VBType::Integer));
        assert!(VBType::String.can_assign_to(&VBType::Variant));
        assert!(VBType::Class("A".into()).can_assign_to(&VBType::Object));
        assert!(!VBType::Integer.can_assign_to(&VBType::String));
    }

    #[test]
    fn from_var_type_decodes_arrays() {
        assert_eq!(VBType::from_var_type(2), Some(VBType::Integer));
        assert_eq!(
            VBType::from_var_type(8194),
            Some(VBType::Array(Box::new(VBType::Integer)))
        );
        assert_eq!(VBType::from_var_type(8197), Some(VBType::Array(Box::new(VBType::Double))));
        assert_eq!(VBType::from_var_type(13), None);
    }
}
