//! Type information and type checking for VB6 symbols.
//!
//! This module defines the `TypeInfo` struct which represents the type of a symbol
//! in VB6 code, including primitive types, object types, user-defined types, and
//! function types. It also defines the `TypeKind` enum for categorizing types and
//! the `TypeChecker` struct which provides methods for checking type compatibility
//! and assignment rules according to VB6 semantics.
//!
//! The `TypeInfo` struct includes methods for determining if one type can be assigned
//! to another and if two types are compatible for operations.
//!
//! The `TypeChecker` struct can be extended in the future to include more complex
//! type relationships and conversion rules.
//!
//! # Examples
//!
//! ```rust
//! use vb6semantic::TypeInfo;
//! use vb6semantic::TypeKind;
//!
//! let int_type = TypeInfo::integer();
//! let long_type = TypeInfo::long();
//! let string_type = TypeInfo::string();
//!
//! assert!(int_type.can_assign_to(&long_type));
//! assert!(!string_type.can_assign_to(&int_type));
//! ```

use std::fmt::Display;

use crate::error::{Result, SourceLocation};
use serde::{Deserialize, Serialize};

/// Type information for VB6 symbols
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeInfo {
    /// The kind of type
    pub kind: TypeKind,

    /// Whether this is a reference type (ByRef)
    pub is_reference: bool,

    /// Whether this is an array
    pub is_array: bool,

    /// Array dimensions (if is_array is true)
    pub array_dimensions: Option<Vec<ArrayBound>>,

    /// For Object and Class types, the class name
    pub class_name: Option<String>,
}

/// Represents the kind of type in VB6
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeKind {
    // Primitive types
    /// Integer type
    Integer,
    /// Long type
    Long,
    /// Single type
    Single,
    /// Double type
    Double,
    /// Currency type
    Currency,
    /// String type
    String,
    /// Boolean type
    Boolean,
    /// Byte type
    Byte,
    /// Date type
    Date,

    // Complex types
    /// Variant type that can hold any value
    Variant,
    /// Object type that can hold any object reference
    Object,
    /// Class type with a specific class name
    Class(String),
    /// User-defined type with a specific name
    UserType(String),
    /// Enum type with a specific name
    Enum(String),

    // Special types
    /// Represents the absence of a value (used for functions that return nothing)
    Nothing,
    /// Represents an uninitialized variable
    Empty,
    /// Represents a null value (used for object references)
    Null,

    // Function types
    /// Represents a Sub procedure (no return value)
    Sub,
    /// Represents a Function with a return type
    Function {
        /// The return type of the function
        return_type: Box<TypeInfo>,
    },

    // Unknown/unresolved
    /// Represents an unknown or unresolved type (used for error cases)
    Unknown,
}

/// Represents the bounds of an array dimension
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayBound {
    /// Lower bound of the array dimension (optional, defaults to 0)
    pub lower: Option<i32>,
    /// Upper bound of the array dimension (optional, defaults to -1 for dynamic arrays)
    pub upper: Option<i32>,
}

impl Display for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = match &self.kind {
            TypeKind::Integer => "Integer",
            TypeKind::Long => "Long",
            TypeKind::Single => "Single",
            TypeKind::Double => "Double",
            TypeKind::Currency => "Currency",
            TypeKind::String => "String",
            TypeKind::Boolean => "Boolean",
            TypeKind::Byte => "Byte",
            TypeKind::Date => "Date",
            TypeKind::Variant => "Variant",
            TypeKind::Object => "Object",
            TypeKind::Class(name) => return write!(f, "{}", name),
            TypeKind::UserType(name) => return write!(f, "{}", name),
            TypeKind::Enum(name) => return write!(f, "{}", name),
            TypeKind::Nothing => "Nothing",
            TypeKind::Empty => "Empty",
            TypeKind::Null => "Null",
            TypeKind::Sub => "Sub",
            TypeKind::Function { return_type } => return write!(f, "Function -> {}", return_type),
            TypeKind::Unknown => "Unknown",
        };

        if self.is_array {
            write!(f, "{}()", base)
        } else {
            write!(f, "{}", base)
        }
    }
}

impl TypeInfo {
    /// Create a new TypeInfo with the given kind
    pub fn new(kind: TypeKind) -> Self {
        Self {
            kind,
            is_reference: false,
            is_array: false,
            array_dimensions: None,
            class_name: None,
        }
    }

    /// Helper constructor for integer type
    pub fn integer() -> Self {
        Self::new(TypeKind::Integer)
    }

    /// Helper constructor for long type
    pub fn long() -> Self {
        Self::new(TypeKind::Long)
    }

    /// Helper constructor for single type
    pub fn string() -> Self {
        Self::new(TypeKind::String)
    }

    /// Helper constructor for boolean type
    pub fn boolean() -> Self {
        Self::new(TypeKind::Boolean)
    }

    /// Helper constructor for variant type
    pub fn variant() -> Self {
        Self::new(TypeKind::Variant)
    }

    /// Helper constructor for object type
    pub fn object() -> Self {
        Self::new(TypeKind::Object)
    }

    /// Helper constructor for unknown type
    pub fn unknown() -> Self {
        Self::new(TypeKind::Unknown)
    }

    /// Check if this type is compatible with another type
    pub fn is_compatible_with(&self, other: &TypeInfo) -> bool {
        // Variant is compatible with everything
        if matches!(self.kind, TypeKind::Variant) || matches!(other.kind, TypeKind::Variant) {
            return true;
        }

        // Same types are compatible
        if self.kind == other.kind {
            return true;
        }

        // Numeric types have some compatibility
        matches!(
            (&self.kind, &other.kind),
            (TypeKind::Integer, TypeKind::Long)
                | (TypeKind::Long, TypeKind::Integer)
                | (TypeKind::Single, TypeKind::Double)
                | (TypeKind::Double, TypeKind::Single)
                | (TypeKind::Object, TypeKind::Class(_))
                | (TypeKind::Class(_), TypeKind::Object)
        )
    }

    /// Check if this type can be assigned to another type
    pub fn can_assign_to(&self, other: &TypeInfo) -> bool {
        // Variant can be assigned to anything
        if matches!(other.kind, TypeKind::Variant) {
            return true;
        }

        // Same types can be assigned
        if self.kind == other.kind {
            return true;
        }

        // Smaller numeric types can be assigned to larger
        matches!(
            (&self.kind, &other.kind),
            (TypeKind::Integer, TypeKind::Long)
                | (TypeKind::Integer, TypeKind::Double)
                | (TypeKind::Long, TypeKind::Double)
                | (TypeKind::Single, TypeKind::Double)
        )
    }
}

/// Type checker for VB6 code
pub struct TypeChecker {
    // TODO: store type relationships, conversion rules, etc.
}

impl TypeChecker {
    /// Create a new type checker instance
    pub fn new() -> Self {
        Self {}
    }

    /// Check if an assignment is valid
    pub fn check_assignment(
        &self,
        target_type: &TypeInfo,
        source_type: &TypeInfo,
        _location: &SourceLocation,
    ) -> Result<()> {
        if source_type.can_assign_to(target_type) {
            Ok(())
        } else {
            Err(crate::error::SemanticError::TypeMismatch {
                expected: target_type.to_string(),
                found: source_type.to_string(),
                location: _location.clone(),
            })
        }
    }

    /// Check if two types are compatible for an operation
    pub fn check_operation(
        &self,
        left_type: &TypeInfo,
        right_type: &TypeInfo,
        _operation: &str,
        _location: &SourceLocation,
    ) -> Result<TypeInfo> {
        // Variant propagates
        if matches!(left_type.kind, TypeKind::Variant)
            || matches!(right_type.kind, TypeKind::Variant)
        {
            return Ok(TypeInfo::variant());
        }

        // Numeric operations
        if self.is_numeric(&left_type.kind) && self.is_numeric(&right_type.kind) {
            return Ok(self.promote_numeric_types(left_type, right_type));
        }

        // String concatenation
        if matches!(left_type.kind, TypeKind::String) || matches!(right_type.kind, TypeKind::String)
        {
            return Ok(TypeInfo::string());
        }

        // Default to variant for unknown operations
        Ok(TypeInfo::variant())
    }

    fn is_numeric(&self, kind: &TypeKind) -> bool {
        matches!(
            kind,
            TypeKind::Integer
                | TypeKind::Long
                | TypeKind::Single
                | TypeKind::Double
                | TypeKind::Currency
                | TypeKind::Byte
        )
    }

    fn promote_numeric_types(&self, left: &TypeInfo, right: &TypeInfo) -> TypeInfo {
        // Promotion rules: Byte < Integer < Long < Single < Double < Currency
        use TypeKind::*;

        let promoted = match (&left.kind, &right.kind) {
            (Double, _) | (_, Double) => Double,
            (Currency, _) | (_, Currency) => Currency,
            (Single, _) | (_, Single) => Single,
            (Long, _) | (_, Long) => Long,
            (Integer, _) | (_, Integer) => Integer,
            (Byte, Byte) => Byte,
            _ => Variant,
        };

        TypeInfo::new(promoted)
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
