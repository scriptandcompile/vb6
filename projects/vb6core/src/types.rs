//! VB6 type system.
//!
//! [`VBType`] is the single source of truth for VB6 static types, shared by
//! `vb6semantic` (type checking) and `vb6runtime` (value typing). It covers
//! the primitive types, complex types (classes, user-defined types, enums,
//! arrays), value states (`Empty`, `Null`, `Nothing`, `Error`), procedure
//! types (`Sub`/`Function`), and `Unknown`.
//!
//! [`TypeInfo`] wraps a [`VBType`] with the metadata static analysis needs:
//! `ByRef` references, array dimensions, and class names.

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

impl ArrayBound {
    /// Creates array bounds; `None` represents an unspecified bound.
    pub const fn new(lower: Option<i32>, upper: Option<i32>) -> Self {
        Self { lower, upper }
    }

    /// Whether the upper bound is not fixed yet.
    pub const fn is_dynamic(&self) -> bool {
        self.upper.is_none()
    }

    /// Whether a fixed upper bound is not below its effective lower bound.
    pub fn is_valid(&self) -> bool {
        match self.upper {
            Some(upper) => self.lower.unwrap_or(0) <= upper,
            None => true,
        }
    }
}

/// The static type of a VB6 value, variable, or procedure.
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
    /// Decimal subtype stored in a Variant (not the Currency fixed-point type).
    Decimal,
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

    // Procedure types
    /// A `Sub` procedure (no return value).
    Sub,
    /// A `Function` procedure with a return type.
    Function {
        /// The type the function returns.
        return_type: Box<TypeInfo>,
    },
    /// An explicit procedure signature for values such as future delegates
    /// and callbacks. Legacy `Function` and `Sub` variants remain unchanged.
    Procedure(Box<ProcedureSignature>),

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
            Self::Decimal => "Decimal".to_string(),
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
            Self::Sub => "Sub".to_string(),
            Self::Function { return_type } => format!("Function -> {}", return_type.kind),
            Self::Procedure(signature) => signature.name(),
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
            Self::Decimal => DECIMAL,
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
            Self::Sub | Self::Procedure(_) => VARIANT,
            Self::Function { return_type } => return_type.kind.var_type(),
            Self::Unknown => VARIANT,
        }
    }

    /// Whether this is one of the numeric primitive types.
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::Byte
                | Self::Integer
                | Self::Long
                | Self::Single
                | Self::Double
                | Self::Currency
                | Self::Decimal
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
                | Self::Decimal
                | Self::String
                | Self::Boolean
                | Self::Date
                | Self::Variant
                | Self::Object
        )
    }

    /// Whether this is one of the value states (`Empty`, `Null`, `Nothing`, `Error`).
    pub fn is_value_state(&self) -> bool {
        matches!(self, Self::Empty | Self::Null | Self::Nothing | Self::Error)
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
        // Decimal is a Variant subtype, not a declared widening step between
        // the fixed-size numeric types and Currency.
        if matches!(self, Self::Decimal) || matches!(target, Self::Decimal) {
            return false;
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
            vartype::DECIMAL => Some(Self::Decimal),
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
            Self::Decimal => u8::MAX,
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

/// How a procedure parameter is passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParameterPassingMode {
    /// Value passing.
    ByVal,
    /// Aliased variable storage.
    ByRef,
}

/// Static metadata for one procedure parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcedureParameter {
    /// Parameter name, when known.
    pub name: Option<String>,
    /// Parameter value type.
    pub type_info: TypeInfo,
    /// Passing mode.
    pub passing_mode: ParameterPassingMode,
    /// Whether the parameter may be omitted.
    pub optional: bool,
    /// Whether the parameter collects a variable number of arguments.
    pub param_array: bool,
}

/// Static signature metadata for future function values and callbacks.
///
/// This describes callable types only; it does not implement calls, closures,
/// capture ownership, or VM frames.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcedureSignature {
    /// Ordered parameters.
    pub parameters: Vec<ProcedureParameter>,
    /// `None` for a Sub-like procedure, otherwise its result type.
    pub return_type: Option<Box<TypeInfo>>,
}

impl ProcedureSignature {
    /// Human-readable type description for diagnostics.
    pub fn name(&self) -> String {
        let parameters = self
            .parameters
            .iter()
            .map(|parameter| {
                let mode = match parameter.passing_mode {
                    ParameterPassingMode::ByVal => "ByVal",
                    ParameterPassingMode::ByRef => "ByRef",
                };
                let optional = if parameter.optional { "Optional " } else { "" };
                let name = parameter.name.as_deref().unwrap_or("_");
                let param_array = if parameter.param_array {
                    "ParamArray "
                } else {
                    ""
                };
                format!(
                    "{optional}{param_array}{mode} {name} As {}",
                    parameter.type_info
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        match &self.return_type {
            Some(return_type) => format!("Function ({parameters}) As {return_type}"),
            None => format!("Sub ({parameters})"),
        }
    }
}

/// Represents the bounds of an array dimension.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArrayBound {
    /// Lower bound of the array dimension (optional, defaults to 0).
    pub lower: Option<i32>,
    /// Upper bound of the array dimension (optional, defaults to -1 for dynamic arrays).
    pub upper: Option<i32>,
}

/// Type information for a VB6 symbol.
///
/// Wraps a [`VBType`] with the metadata static analysis needs: whether the
/// symbol is passed `ByRef`, whether it is an array (plus its dimensions),
/// and the resolved class name for `Object`/`Class` kinds.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeInfo {
    /// The kind of type.
    pub kind: VBType,

    /// Whether this is a reference type (ByRef).
    pub is_reference: bool,

    /// Whether this is an array.
    pub is_array: bool,

    /// Array dimensions (if is_array is true).
    pub array_dimensions: Option<Vec<ArrayBound>>,

    /// For Object and Class types, the class name.
    pub class_name: Option<String>,
}

impl TypeInfo {
    /// Create a new `TypeInfo` with the given kind.
    pub fn new(kind: VBType) -> Self {
        Self {
            kind,
            is_reference: false,
            is_array: false,
            array_dimensions: None,
            class_name: None,
        }
    }

    /// Helper constructor for integer type.
    pub fn integer() -> Self {
        Self::new(VBType::Integer)
    }

    /// Helper constructor for long type.
    pub fn long() -> Self {
        Self::new(VBType::Long)
    }

    /// Helper constructor for string type.
    pub fn string() -> Self {
        Self::new(VBType::String)
    }

    /// Helper constructor for boolean type.
    pub fn boolean() -> Self {
        Self::new(VBType::Boolean)
    }

    /// Helper constructor for variant type.
    pub fn variant() -> Self {
        Self::new(VBType::Variant)
    }

    /// Helper constructor for object type.
    pub fn object() -> Self {
        Self::new(VBType::Object)
    }

    /// Helper constructor for unknown type.
    pub fn unknown() -> Self {
        Self::new(VBType::Unknown)
    }

    /// Mark this symbol as passed by reference.
    pub fn with_by_ref(mut self, by_ref: bool) -> Self {
        self.is_reference = by_ref;
        self
    }

    /// Whether the symbol is passed by reference.
    pub const fn is_by_ref(&self) -> bool {
        self.is_reference
    }

    /// Attach declared array dimensions without changing the element type.
    pub fn with_array_dimensions(mut self, dimensions: Vec<ArrayBound>) -> Self {
        self.is_array = true;
        self.array_dimensions = Some(dimensions);
        self
    }

    /// The declared array rank, if dimensions are known.
    pub fn array_rank(&self) -> Option<usize> {
        self.array_dimensions.as_ref().map(Vec::len)
    }

    /// Check if this type is compatible with another type.
    pub fn is_compatible_with(&self, other: &TypeInfo) -> bool {
        // Variant is compatible with everything
        if matches!(self.kind, VBType::Variant) || matches!(other.kind, VBType::Variant) {
            return true;
        }

        // Same types are compatible
        if self.kind == other.kind {
            return true;
        }

        // Numeric types have some compatibility
        matches!(
            (&self.kind, &other.kind),
            (VBType::Integer, VBType::Long)
                | (VBType::Long, VBType::Integer)
                | (VBType::Single, VBType::Double)
                | (VBType::Double, VBType::Single)
                | (VBType::Object, VBType::Class(_))
                | (VBType::Class(_), VBType::Object)
        )
    }

    /// Check if this type can be assigned to another type.
    pub fn can_assign_to(&self, other: &TypeInfo) -> bool {
        self.kind.can_assign_to(&other.kind)
    }
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base = match &self.kind {
            VBType::Integer => "Integer",
            VBType::Long => "Long",
            VBType::Single => "Single",
            VBType::Double => "Double",
            VBType::Currency => "Currency",
            VBType::Decimal => "Decimal",
            VBType::String => "String",
            VBType::Boolean => "Boolean",
            VBType::Byte => "Byte",
            VBType::Date => "Date",
            VBType::Variant => "Variant",
            VBType::Object => "Object",
            VBType::Class(name) => return write!(f, "{}", name),
            VBType::UserType(name) => return write!(f, "{}", name),
            VBType::Enum(name) => return write!(f, "{}", name),
            VBType::Nothing => "Nothing",
            VBType::Empty => "Empty",
            VBType::Null => "Null",
            VBType::Error => "Error",
            VBType::Sub => "Sub",
            VBType::Function { return_type } => return write!(f, "Function -> {}", return_type),
            VBType::Procedure(signature) => return write!(f, "{}", signature.name()),
            VBType::Unknown => "Unknown",
            VBType::Array(inner) => return write!(f, "{}()", inner),
        };

        if self.is_array {
            write!(f, "{}()", base)
        } else {
            write!(f, "{}", base)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_match_vb6() {
        assert_eq!(VBType::Integer.name(), "Integer");
        assert_eq!(VBType::Decimal.name(), "Decimal");
        assert_eq!(TypeInfo::new(VBType::Decimal).to_string(), "Decimal");
        assert_eq!(VBType::Array(Box::new(VBType::String)).name(), "String()");
        assert_eq!(VBType::Class("Foo".to_string()).name(), "Foo");
        assert_eq!(VBType::Sub.name(), "Sub");
        assert_eq!(
            VBType::Function {
                return_type: Box::new(TypeInfo::long()),
            }
            .name(),
            "Function -> Long"
        );
    }

    #[test]
    fn var_type_codes_match_vba() {
        assert_eq!(VBType::Integer.var_type(), 2);
        assert_eq!(VBType::Long.var_type(), 3);
        assert_eq!(VBType::String.var_type(), 8);
        assert_eq!(VBType::Boolean.var_type(), 11);
        assert_eq!(VBType::Byte.var_type(), 17);
        assert_eq!(VBType::Currency.var_type(), vartype::CURRENCY);
        assert_eq!(VBType::Decimal.var_type(), vartype::DECIMAL);
        assert_eq!(VBType::Empty.var_type(), 0);
        assert_eq!(VBType::Null.var_type(), 1);
        assert_eq!(VBType::Class("x".into()).var_type(), 9);
        assert_eq!(VBType::Array(Box::new(VBType::Double)).var_type(), 8192 + 5);
        assert_eq!(
            VBType::Function {
                return_type: Box::new(TypeInfo::long()),
            }
            .var_type(),
            3
        );
    }

    #[test]
    fn numeric_predicates() {
        assert!(VBType::Long.is_numeric());
        assert!(VBType::Integer.is_integral());
        assert!(!VBType::String.is_numeric());
        assert!(!VBType::Boolean.is_numeric());
        assert!(!VBType::Date.is_numeric());
        assert!(VBType::Decimal.is_numeric());
        assert!(VBType::Decimal.is_primitive());
        assert!(!VBType::Decimal.is_integral());
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
        assert_eq!(
            VBType::from_var_type(8197),
            Some(VBType::Array(Box::new(VBType::Double)))
        );
        assert_eq!(VBType::from_var_type(13), None);
        assert_eq!(
            VBType::from_var_type(vartype::DECIMAL),
            Some(VBType::Decimal)
        );
        assert_eq!(
            VBType::from_var_type(vartype::ARRAY | vartype::DECIMAL),
            Some(VBType::Array(Box::new(VBType::Decimal)))
        );
    }

    #[test]
    fn decimal_keeps_its_identity_and_does_not_widen_to_currency() {
        assert_ne!(VBType::Decimal, VBType::Currency);
        assert_eq!(VBType::Decimal.name(), "Decimal");
        assert_eq!(VBType::Decimal.var_type(), vartype::DECIMAL);
        assert_eq!(TypeInfo::new(VBType::Decimal).to_string(), "Decimal");
        assert!(VBType::Decimal.can_assign_to(&VBType::Decimal));
        assert!(VBType::Decimal.can_assign_to(&VBType::Variant));
        assert!(!VBType::Decimal.can_assign_to(&VBType::Currency));
        assert!(!VBType::Currency.can_assign_to(&VBType::Decimal));
        assert!(!TypeInfo::new(VBType::Decimal).can_assign_to(&TypeInfo::new(VBType::Currency)));
    }

    #[test]
    fn type_info_display_shows_arrays() {
        let mut info = TypeInfo::new(VBType::Integer);
        assert_eq!(info.to_string(), "Integer");
        info.is_array = true;
        assert_eq!(info.to_string(), "Integer()");
        assert_eq!(TypeInfo::string().to_string(), "String");
    }

    #[test]
    fn type_info_assignment_rules() {
        assert!(TypeInfo::integer().can_assign_to(&TypeInfo::long()));
        assert!(!TypeInfo::string().can_assign_to(&TypeInfo::integer()));
        assert!(TypeInfo::integer().is_compatible_with(&TypeInfo::long()));
    }

    #[test]
    fn type_info_assignment_uses_the_vbtype_widening_rules() {
        let widening = [
            (VBType::Byte, VBType::Integer),
            (VBType::Byte, VBType::Long),
            (VBType::Integer, VBType::Long),
            (VBType::Long, VBType::Single),
            (VBType::Single, VBType::Double),
            (VBType::Double, VBType::Currency),
            (VBType::Class("Widget".into()), VBType::Object),
            (VBType::Enum("Color".into()), VBType::Enum("Color".into())),
            (VBType::Integer, VBType::Variant),
        ];

        for (source, target) in widening {
            assert!(
                TypeInfo::new(source.clone()).can_assign_to(&TypeInfo::new(target.clone())),
                "{source} should be assignable to {target}"
            );
            assert!(source.can_assign_to(&target));
        }
    }

    #[test]
    fn type_info_assignment_rejects_narrowing_and_distinct_named_types() {
        let rejected = [
            (VBType::Long, VBType::Integer),
            (VBType::Double, VBType::Single),
            (VBType::Currency, VBType::Double),
            (
                VBType::Class("Widget".into()),
                VBType::Class("Gadget".into()),
            ),
            (VBType::Enum("Color".into()), VBType::Enum("Shape".into())),
            (VBType::Integer, VBType::String),
            (VBType::Null, VBType::Nothing),
        ];

        for (source, target) in rejected {
            assert!(
                !TypeInfo::new(source.clone()).can_assign_to(&TypeInfo::new(target.clone())),
                "{source} should not be assignable to {target}"
            );
            assert!(!source.can_assign_to(&target));
        }
    }

    #[test]
    fn procedure_signatures_retain_typed_parameters_and_passing_modes() {
        let signature = ProcedureSignature {
            parameters: vec![
                ProcedureParameter {
                    name: Some("value".into()),
                    type_info: TypeInfo::long(),
                    passing_mode: ParameterPassingMode::ByVal,
                    optional: false,
                    param_array: false,
                },
                ProcedureParameter {
                    name: Some("items".into()),
                    type_info: TypeInfo::variant(),
                    passing_mode: ParameterPassingMode::ByRef,
                    optional: true,
                    param_array: true,
                },
            ],
            return_type: Some(Box::new(TypeInfo::boolean())),
        };
        let procedure = VBType::Procedure(Box::new(signature));
        assert_eq!(
            procedure.name(),
            "Function (ByVal value As Long, Optional ParamArray ByRef items As Variant) As Boolean"
        );
        assert_eq!(procedure.var_type(), vartype::VARIANT);
        assert_eq!(procedure, procedure.clone());
    }

    #[test]
    fn array_and_byref_metadata_preserve_rank_and_bounds() {
        let info = TypeInfo::long()
            .with_by_ref(true)
            .with_array_dimensions(vec![
                ArrayBound::new(Some(0), Some(9)),
                ArrayBound::new(None, Some(3)),
            ]);
        assert!(info.is_by_ref());
        assert_eq!(info.array_rank(), Some(2));
        assert_eq!(info.array_dimensions.as_ref().unwrap()[0].lower, Some(0));
        assert!(!ArrayBound::new(Some(5), Some(4)).is_valid());
        assert!(ArrayBound::new(None, None).is_dynamic());
        assert!(ArrayBound::new(None, Some(2)).is_valid());
    }
}
