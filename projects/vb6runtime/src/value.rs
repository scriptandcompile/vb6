//! VB6 runtime values.
//!
//! [`Value`] is the dynamic counterpart of `VBType` and
//! the representation of a VB6 `Variant`: it can hold `Empty`, `Null`, any
//! primitive, an object reference, an error, or an array. Conversions follow
//! VB6 semantics (banker's rounding, `Null` propagation as error 94, overflow
//! as error 6, type mismatch as error 13).

use std::fmt;

use crate::array::{ArrayDimension, ArrayValue};
use crate::error::{VBError, VBResult};
use crate::types::{vartype, VBType};

/// Number of scaled units per currency value (`1.00` == `10_000`).
pub const CURRENCY_SCALE: i64 = 10_000;

/// A runtime object reference.
///
/// Implement this trait for any object that can be held in a VB6 `Object` or
/// `Variant` value. The trait requires `Debug` so that [`Value`] can derive
/// `Debug`, and `clone_box` so that [`Value::clone`] works without requiring
/// the concrete type to be known.
pub trait VBObject: fmt::Debug + Send + Sync {
    /// The VB6 type name of the object (e.g. `"Collection"`).
    fn type_name(&self) -> &str;

    /// Downcast helper for accessing the concrete object.
    fn as_any(&self) -> &dyn std::any::Any;

    /// Clone this object as a boxed trait object.
    fn clone_box(&self) -> Box<dyn VBObject>;
}

/// A runtime-safe string wrapper that preserves VB6 coercion semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VBString(String);

impl VBString {
    /// Access the wrapped string contents.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the wrapper and return the underlying string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for VBString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for VBString {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<VBString> for Value {
    fn from(value: VBString) -> Self {
        Value::from_string(value.0)
    }
}

impl TryFrom<&Value> for VBString {
    type Error = VBError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(Self(value.as_string()?))
    }
}

impl TryFrom<Value> for VBString {
    type Error = VBError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// A runtime-safe byte wrapper that preserves VB6 coercion semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VBByte(u8);

impl VBByte {
    /// Access the wrapped byte value.
    pub fn as_u8(self) -> u8 {
        self.0
    }
}

impl From<u8> for VBByte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<VBByte> for Value {
    fn from(value: VBByte) -> Self {
        Value::from_byte(value.0)
    }
}

impl TryFrom<&Value> for VBByte {
    type Error = VBError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(Self(value.as_byte()?))
    }
}

impl TryFrom<Value> for VBByte {
    type Error = VBError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// A runtime-safe long wrapper that preserves VB6 coercion semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VBLong(i32);

impl VBLong {
    /// Access the wrapped long value.
    pub fn as_i32(self) -> i32 {
        self.0
    }
}

impl From<i32> for VBLong {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<VBLong> for Value {
    fn from(value: VBLong) -> Self {
        Value::from_long(value.0)
    }
}

impl TryFrom<&Value> for VBLong {
    type Error = VBError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(Self(value.as_i32()?))
    }
}

impl TryFrom<Value> for VBLong {
    type Error = VBError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// A runtime-safe integer wrapper that preserves VB6 coercion semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VBInteger(i16);

impl VBInteger {
    /// Access the wrapped integer value.
    pub fn as_i16(self) -> i16 {
        self.0
    }
}

impl From<i16> for VBInteger {
    fn from(value: i16) -> Self {
        Self(value)
    }
}

impl From<VBInteger> for Value {
    fn from(value: VBInteger) -> Self {
        Value::from_integer(value.0)
    }
}

impl TryFrom<&Value> for VBInteger {
    type Error = VBError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(Self(value.as_i16()?))
    }
}

impl TryFrom<Value> for VBInteger {
    type Error = VBError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// A runtime-safe boolean wrapper that preserves VB6 coercion semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VBBoolean(bool);

impl VBBoolean {
    /// Access the wrapped boolean value.
    pub fn as_bool(self) -> bool {
        self.0
    }
}

impl From<bool> for VBBoolean {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<VBBoolean> for Value {
    fn from(value: VBBoolean) -> Self {
        Value::from_bool(value.0)
    }
}

impl TryFrom<&Value> for VBBoolean {
    type Error = VBError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(Self(value.as_bool()?))
    }
}

impl TryFrom<Value> for VBBoolean {
    type Error = VBError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// A runtime-safe date wrapper that preserves VB6 coercion semantics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VBDate(f64);

impl VBDate {
    /// Access the wrapped date serial value.
    pub fn as_f64(self) -> f64 {
        self.0
    }
}

impl From<f64> for VBDate {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<VBDate> for Value {
    fn from(value: VBDate) -> Self {
        Value::from_date_serial(value.0)
    }
}

impl TryFrom<&Value> for VBDate {
    type Error = VBError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(Self(value.as_date_serial()?))
    }
}

impl TryFrom<Value> for VBDate {
    type Error = VBError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// A VB6 runtime value.
#[derive(Debug)]
pub enum Value {
    /// An uninitialized `Variant` (`Empty`).
    Empty,
    /// A `Null` value (unknown data). Distinct from [`Value::Empty`].
    Null,
    /// An object reference set to `Nothing`.
    Nothing,
    /// An 8-bit unsigned integer.
    Byte(u8),
    /// A 16-bit signed integer.
    Integer(i16),
    /// A 32-bit signed integer.
    Long(i32),
    /// A 32-bit floating point number.
    Single(f32),
    /// A 64-bit floating point number.
    Double(f64),
    /// A currency value stored as a scaled `i64` (raw / [`CURRENCY_SCALE`]).
    Currency(i64),
    /// A Unicode string.
    String(String),
    /// A boolean.
    Boolean(bool),
    /// A date stored as a serial day number since 1899-12-30 (OLE automation).
    Date(f64),
    /// An error value (from `CVErr`).
    Error(VBError),
    /// An object reference.
    Object(Box<dyn VBObject>),
    /// An array value.
    Array(ArrayValue),
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Empty => Value::Empty,
            Value::Null => Value::Null,
            Value::Nothing => Value::Nothing,
            Value::Byte(v) => Value::Byte(*v),
            Value::Integer(v) => Value::Integer(*v),
            Value::Long(v) => Value::Long(*v),
            Value::Single(v) => Value::Single(*v),
            Value::Double(v) => Value::Double(*v),
            Value::Currency(v) => Value::Currency(*v),
            Value::String(v) => Value::String(v.clone()),
            Value::Boolean(v) => Value::Boolean(*v),
            Value::Date(v) => Value::Date(*v),
            Value::Error(e) => Value::Error(e.clone()),
            Value::Object(o) => Value::Object(o.clone_box()),
            Value::Array(a) => Value::Array(a.clone()),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Empty, Value::Empty) => true,
            (Value::Null, Value::Null) => true,
            (Value::Nothing, Value::Nothing) => true,
            (Value::Error(a), Value::Error(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Date(a), Value::Date(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => std::ptr::eq(
                a.as_ref() as *const dyn VBObject as *const (),
                b.as_ref() as *const dyn VBObject as *const (),
            ),
            // Numeric cross-type comparison with VB6 coercion.
            (Value::Currency(a), Value::Currency(b)) => a == b,
            _ => match (self.numeric_i64_exact(), other.numeric_i64_exact()) {
                (Some(a), Some(b)) => a == b,
                _ => match (self.numeric_f64_coercion(), other.numeric_f64_coercion()) {
                    (Some(a), Some(b)) => a == b,
                    _ => false,
                },
            },
        }
    }
}

impl Value {
    /// Create an `Empty` value.
    pub fn empty() -> Self {
        Value::Empty
    }

    /// Create a `Null` value.
    pub fn null() -> Self {
        Value::Null
    }

    /// Create a `Nothing` object reference.
    pub fn nothing() -> Self {
        Value::Nothing
    }

    /// Create a byte value.
    pub fn from_byte(v: u8) -> Self {
        Value::Byte(v)
    }

    /// Create an Integer (16-bit) value.
    pub fn from_integer(v: i16) -> Self {
        Value::Integer(v)
    }

    /// Create a Long (32-bit) value.
    pub fn from_long(v: i32) -> Self {
        Value::Long(v)
    }

    /// Create a value from a signed 64-bit integer, choosing Integer or Long
    /// based on VB6 integer literal rules.
    pub fn from_i64(v: i64) -> Self {
        if let Ok(n) = i16::try_from(v) {
            Value::Integer(n)
        } else if let Ok(n) = i32::try_from(v) {
            Value::Long(n)
        } else {
            Value::Double(v as f64)
        }
    }

    /// Create a Single (32-bit float) value.
    pub fn from_single(v: f32) -> Self {
        Value::Single(v)
    }

    /// Create a Double (64-bit float) value.
    pub fn from_double(v: f64) -> Self {
        Value::Double(v)
    }

    /// Create a currency value from its scaled representation (raw / 10_000).
    pub fn from_currency_scaled(raw: i64) -> Self {
        Value::Currency(raw)
    }

    /// Create a currency value from decimal units (e.g. `1.25`).
    pub fn from_currency(units: f64) -> Self {
        Value::Currency(round_half_even(units * CURRENCY_SCALE as f64).unwrap_or(i64::MAX))
    }

    /// Create a string value.
    pub fn from_string(v: impl Into<String>) -> Self {
        Value::String(v.into())
    }

    /// Create a boolean value.
    pub fn from_bool(v: bool) -> Self {
        Value::Boolean(v)
    }

    /// Create a date value from a serial day number (1899-12-30 == 0).
    pub fn from_date_serial(serial: f64) -> Self {
        Value::Date(serial)
    }

    /// Create an error value.
    pub fn from_error(e: VBError) -> Self {
        Value::Error(e)
    }

    /// Create an object reference value.
    pub fn from_object(o: Box<dyn VBObject>) -> Self {
        Value::Object(o)
    }

    /// Create an array value.
    pub fn from_array(a: ArrayValue) -> Self {
        Value::Array(a)
    }

    /// The dynamic (value) type of this value.
    pub fn type_of(&self) -> VBType {
        match self {
            Value::Empty => VBType::Empty,
            Value::Null => VBType::Null,
            Value::Nothing => VBType::Nothing,
            Value::Byte(_) => VBType::Byte,
            Value::Integer(_) => VBType::Integer,
            Value::Long(_) => VBType::Long,
            Value::Single(_) => VBType::Single,
            Value::Double(_) => VBType::Double,
            Value::Currency(_) => VBType::Currency,
            Value::String(_) => VBType::String,
            Value::Boolean(_) => VBType::Boolean,
            Value::Date(_) => VBType::Date,
            Value::Error(_) => VBType::Error,
            Value::Object(_) => VBType::Object,
            Value::Array(a) => VBType::Array(Box::new(a.element_type().clone())),
        }
    }

    /// The VBA `VarType` code for this value.
    pub fn var_type(&self) -> i32 {
        match self {
            Value::Empty => vartype::EMPTY,
            Value::Null => vartype::NULL,
            Value::Nothing => vartype::OBJECT,
            Value::Byte(_) => vartype::BYTE,
            Value::Integer(_) => vartype::INTEGER,
            Value::Long(_) => vartype::LONG,
            Value::Single(_) => vartype::SINGLE,
            Value::Double(_) => vartype::DOUBLE,
            Value::Currency(_) => vartype::CURRENCY,
            Value::String(_) => vartype::STRING,
            Value::Boolean(_) => vartype::BOOLEAN,
            Value::Date(_) => vartype::DATE,
            Value::Error(_) => vartype::ERROR,
            Value::Object(_) => vartype::OBJECT,
            Value::Array(a) => vartype::ARRAY | a.element_type().var_type(),
        }
    }

    /// Whether this value is `Empty` (uninitialized Variant).
    pub fn is_empty(&self) -> bool {
        matches!(self, Value::Empty)
    }

    /// Whether this value is `Null`.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Whether this value is `Nothing`.
    pub fn is_nothing(&self) -> bool {
        matches!(self, Value::Nothing)
    }

    /// Whether this value is numeric (`IsNumeric` semantics: excludes Boolean
    /// and Date).
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Value::Byte(_)
                | Value::Integer(_)
                | Value::Long(_)
                | Value::Single(_)
                | Value::Double(_)
                | Value::Currency(_)
        )
    }

    /// Whether this value is an array.
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    /// Whether this value is an error value.
    pub fn is_error(&self) -> bool {
        matches!(self, Value::Error(_))
    }

    /// Whether this value is a string.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Whether this value is a boolean.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Whether this value is an object reference.
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    /// Whether this value is a date.
    pub fn is_date(&self) -> bool {
        matches!(self, Value::Date(_))
    }

    /// The default value for a given static type, as VB6 initializes variables.
    pub fn default_for_type(t: &VBType) -> Value {
        match t {
            VBType::Byte => Value::Byte(0),
            VBType::Integer => Value::Integer(0),
            VBType::Long => Value::Long(0),
            VBType::Single => Value::Single(0.0),
            VBType::Double => Value::Double(0.0),
            VBType::Currency => Value::Currency(0),
            VBType::String => Value::String(String::new()),
            VBType::Boolean => Value::Boolean(false),
            VBType::Date => Value::Date(0.0),
            VBType::Variant => Value::Empty,
            VBType::Object | VBType::Class(_) | VBType::Nothing => Value::Nothing,
            VBType::Enum(_) => Value::Long(0),
            VBType::UserType(_) | VBType::Unknown => Value::Empty,
            VBType::Array(inner) => Value::Array(ArrayValue::new_dynamic((**inner).clone())),
            VBType::Empty => Value::Empty,
            VBType::Null => Value::Null,
            VBType::Error => Value::Error(VBError::new(0)),
            VBType::Sub | VBType::Function { .. } => Value::Empty,
        }
    }

    /// Convert to a String following `CStr` semantics.
    ///
    /// `Null` raises error 94; `Empty` becomes the empty string; objects and
    /// arrays raise error 13 (type mismatch).
    pub fn as_string(&self) -> VBResult<String> {
        match self {
            Value::Empty => Ok(String::new()),
            Value::Null => Err(VBError::invalid_use_of_null()),
            Value::Nothing => Ok("Nothing".to_string()),
            Value::Byte(v) => Ok(v.to_string()),
            Value::Integer(v) => Ok(v.to_string()),
            Value::Long(v) => Ok(v.to_string()),
            Value::Single(v) => Ok(v.to_string()),
            Value::Double(v) => Ok(v.to_string()),
            Value::Currency(raw) => Ok(format_currency(*raw)),
            Value::String(s) => Ok(s.clone()),
            Value::Boolean(b) => Ok(if *b { "True" } else { "False" }.to_string()),
            Value::Date(serial) => Ok(date_serial_to_string(*serial)),
            Value::Error(e) => Ok(format!("Error {}", e.number)),
            Value::Object(_) | Value::Array(_) => Err(VBError::type_mismatch()),
        }
    }

    /// Borrow the string contents of a String value.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Convert to a Boolean following `CBool` semantics.
    pub fn as_bool(&self) -> VBResult<bool> {
        match self {
            Value::Empty => Ok(false),
            Value::Null => Err(VBError::invalid_use_of_null()),
            Value::Nothing | Value::Object(_) | Value::Array(_) => Err(VBError::type_mismatch()),
            Value::Error(e) => Err(e.clone()),
            Value::Boolean(b) => Ok(*b),
            Value::String(s) => {
                let t = s.trim();
                if t.eq_ignore_ascii_case("true") {
                    return Ok(true);
                }
                if t.eq_ignore_ascii_case("false") {
                    return Ok(false);
                }
                parse_vb_number(t)
                    .map(|n| n != 0.0)
                    .ok_or_else(VBError::type_mismatch)
            }
            _ => self
                .numeric_f64_coercion()
                .map(|n| n != 0.0)
                .ok_or_else(VBError::type_mismatch),
        }
    }

    /// Convert to a signed 64-bit integer following `CLng` semantics.
    pub fn as_i64(&self) -> VBResult<i64> {
        match self {
            Value::Empty => Ok(0),
            Value::Null => Err(VBError::invalid_use_of_null()),
            Value::Nothing | Value::Object(_) | Value::Array(_) => Err(VBError::type_mismatch()),
            Value::Error(e) => Err(e.clone()),
            Value::Byte(v) => Ok(*v as i64),
            Value::Integer(v) => Ok(*v as i64),
            Value::Long(v) => Ok(*v as i64),
            Value::Boolean(b) => Ok(if *b { -1 } else { 0 }),
            Value::Currency(raw) => {
                round_half_even(*raw as f64 / CURRENCY_SCALE as f64).ok_or_else(VBError::overflow)
            }
            Value::Single(v) => round_half_even(*v as f64).ok_or_else(VBError::overflow),
            Value::Double(v) => round_half_even(*v).ok_or_else(VBError::overflow),
            Value::Date(v) => round_half_even(*v).ok_or_else(VBError::overflow),
            Value::String(s) => {
                let n = parse_vb_number(s).ok_or_else(VBError::type_mismatch)?;
                round_half_even(n).ok_or_else(VBError::overflow)
            }
        }
    }

    /// Convert to a Long (32-bit) following `CLng` semantics.
    pub fn as_i32(&self) -> VBResult<i32> {
        let v = self.as_i64()?;
        i32::try_from(v).map_err(|_| VBError::overflow())
    }

    /// Convert to an Integer (16-bit) following `CInt` semantics.
    pub fn as_i16(&self) -> VBResult<i16> {
        let v = self.as_i64()?;
        i16::try_from(v).map_err(|_| VBError::overflow())
    }

    /// Convert to a Byte following `CByte` semantics.
    pub fn as_byte(&self) -> VBResult<u8> {
        let v = self.as_i64()?;
        u8::try_from(v).map_err(|_| VBError::overflow())
    }

    /// Convert to a Double following `CDbl` semantics.
    pub fn as_f64(&self) -> VBResult<f64> {
        match self {
            Value::Empty => Ok(0.0),
            Value::Null => Err(VBError::invalid_use_of_null()),
            Value::Nothing | Value::Object(_) | Value::Array(_) => Err(VBError::type_mismatch()),
            Value::Error(e) => Err(e.clone()),
            Value::Boolean(b) => Ok(if *b { -1.0 } else { 0.0 }),
            Value::String(s) => parse_vb_number(s).ok_or_else(VBError::type_mismatch),
            _ => Ok(self.numeric_f64_coercion().unwrap_or_default()),
        }
    }

    /// Convert to a Single (32-bit float) following `CSng` semantics.
    pub fn as_f32(&self) -> VBResult<f32> {
        Ok(self.as_f64()? as f32)
    }

    /// Convert to a scaled currency integer following `CCur` semantics.
    pub fn as_currency_scaled(&self) -> VBResult<i64> {
        match self {
            Value::Empty => Ok(0),
            Value::Null => Err(VBError::invalid_use_of_null()),
            Value::Nothing | Value::Object(_) | Value::Array(_) => Err(VBError::type_mismatch()),
            Value::Error(e) => Err(e.clone()),
            Value::Boolean(b) => Ok(if *b { -CURRENCY_SCALE } else { 0 }),
            Value::Currency(v) => Ok(*v),
            Value::Byte(v) => Ok(*v as i64 * CURRENCY_SCALE),
            Value::Integer(v) => Ok(*v as i64 * CURRENCY_SCALE),
            Value::Long(v) => Ok(*v as i64 * CURRENCY_SCALE),
            Value::Single(v) => {
                round_half_even(*v as f64 * CURRENCY_SCALE as f64).ok_or_else(VBError::overflow)
            }
            Value::Double(v) => {
                round_half_even(*v * CURRENCY_SCALE as f64).ok_or_else(VBError::overflow)
            }
            Value::Date(v) => {
                round_half_even(*v * CURRENCY_SCALE as f64).ok_or_else(VBError::overflow)
            }
            Value::String(s) => {
                let n = parse_vb_number(s).ok_or_else(VBError::type_mismatch)?;
                round_half_even(n * CURRENCY_SCALE as f64).ok_or_else(VBError::overflow)
            }
        }
    }

    /// Convert to a date serial following `CDate` semantics.
    pub fn as_date_serial(&self) -> VBResult<f64> {
        match self {
            Value::Empty => Ok(0.0),
            Value::Null => Err(VBError::invalid_use_of_null()),
            Value::Nothing | Value::Object(_) | Value::Array(_) => Err(VBError::type_mismatch()),
            Value::Error(e) => Err(e.clone()),
            Value::Boolean(b) => Ok(if *b { -1.0 } else { 0.0 }),
            Value::Date(v) => Ok(*v),
            Value::String(s) => {
                if let Some(serial) = parse_vb_date(s) {
                    Ok(serial)
                } else {
                    parse_vb_number(s).ok_or_else(VBError::type_mismatch)
                }
            }
            _ => Ok(self.numeric_f64_coercion().unwrap_or_default()),
        }
    }

    /// Borrow this value as an array, or error 13 on type mismatch.
    pub fn as_array(&self) -> VBResult<&ArrayValue> {
        match self {
            Value::Array(a) => Ok(a),
            _ => Err(VBError::type_mismatch()),
        }
    }

    /// Borrow this value as an object, or error 424 on type mismatch.
    pub fn as_object(&self) -> VBResult<&dyn VBObject> {
        match self {
            Value::Object(o) => Ok(o.as_ref()),
            _ => Err(VBError::object_required()),
        }
    }

    /// The contained error value, if this is an error value.
    pub fn as_error(&self) -> Option<&VBError> {
        match self {
            Value::Error(e) => Some(e),
            _ => None,
        }
    }

    /// Create a fixed-size array with the given element type and bounds.
    pub fn array_fixed(element_type: VBType, dimensions: &[ArrayDimension]) -> VBResult<Value> {
        Ok(Value::Array(ArrayValue::new_fixed(
            element_type,
            dimensions,
        )?))
    }

    /// Create a dynamic array with the given element type.
    pub fn array_dynamic(element_type: VBType) -> Value {
        Value::Array(ArrayValue::new_dynamic(element_type))
    }

    /// Whether this value compares equal to another following VB6 coercion.
    pub fn coerce_equals(&self, other: &Value) -> bool {
        self == other
    }

    /// Exact integral numeric coercion used by equality (Currency stays scaled).
    fn numeric_i64_exact(&self) -> Option<i64> {
        match self {
            Value::Byte(v) => Some(*v as i64),
            Value::Integer(v) => Some(*v as i64),
            Value::Long(v) => Some(*v as i64),
            Value::Boolean(v) => Some(if *v { -1 } else { 0 }),
            Value::Single(v) => i64_from_exact_f64(*v as f64),
            Value::Double(v) => i64_from_exact_f64(*v),
            Value::Date(v) => i64_from_exact_f64(*v),
            Value::String(s) => i64_from_exact_f64(parse_vb_number(s)?),
            _ => None,
        }
    }

    /// Numeric coercion used by equality and boolean conversion.
    fn numeric_f64_coercion(&self) -> Option<f64> {
        match self {
            Value::Empty => Some(0.0),
            Value::Byte(v) => Some(*v as f64),
            Value::Integer(v) => Some(*v as f64),
            Value::Long(v) => Some(*v as f64),
            Value::Single(v) => Some(*v as f64),
            Value::Double(v) => Some(*v),
            Value::Currency(v) => Some(*v as f64 / CURRENCY_SCALE as f64),
            Value::Date(v) => Some(*v),
            Value::Boolean(v) => Some(if *v { -1.0 } else { 0.0 }),
            Value::String(s) => parse_vb_number(s),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Empty => f.write_str("Empty"),
            Value::Null => f.write_str("Null"),
            Value::Nothing => f.write_str("Nothing"),
            other => match other.as_string() {
                Ok(s) => f.write_str(&s),
                Err(_) => write!(f, "{other:?}"),
            },
        }
    }
}

#[cfg(test)]
mod wrapper_tests {
    use super::{VBBoolean, VBByte, VBDate, VBInteger, VBLong, VBString, Value};

    #[test]
    fn vbstring_round_trips_through_variant() {
        let value = Value::from_string("Hello");
        let typed = VBString::try_from(&value).unwrap();
        assert_eq!(typed.as_str(), "Hello");
        assert_eq!(Value::from(typed), value);
    }

    #[test]
    fn vblong_round_trips_through_variant() {
        let value = Value::from_long(42);
        let typed = VBLong::try_from(&value).unwrap();
        assert_eq!(typed.as_i32(), 42);
        assert_eq!(Value::from(typed), value);
    }

    #[test]
    fn vbbyte_round_trips_through_variant() {
        let value = Value::from_byte(7);
        let typed = VBByte::try_from(&value).unwrap();
        assert_eq!(typed.as_u8(), 7);
        assert_eq!(Value::from(typed), value);
    }

    #[test]
    fn vbinteger_round_trips_through_variant() {
        let value = Value::from_integer(7);
        let typed = VBInteger::try_from(&value).unwrap();
        assert_eq!(typed.as_i16(), 7);
        assert_eq!(Value::from(typed), value);
    }

    #[test]
    fn vbboolean_round_trips_through_variant() {
        let value = Value::from_bool(true);
        let typed = VBBoolean::try_from(&value).unwrap();
        assert!(typed.as_bool());
        assert_eq!(Value::from(typed), value);
    }

    #[test]
    fn vbdate_round_trips_through_variant() {
        let value = Value::from_date_serial(42.5);
        let typed = VBDate::try_from(&value).unwrap();
        assert_eq!(typed.as_f64(), 42.5);
        assert_eq!(Value::from(typed), value);
    }
}

/// Round half to even (banker's rounding), as VB6 `CInt`/`CLng`/`CCur` use.
fn round_half_even(x: f64) -> Option<i64> {
    if !x.is_finite() {
        return None;
    }
    let fl = x.floor();
    let diff = x - fl;
    let rounded = if diff < 0.5 {
        fl
    } else if diff > 0.5 {
        fl + 1.0
    } else if (fl as i64) % 2 == 0 {
        fl
    } else {
        fl + 1.0
    };
    if rounded < i64::MIN as f64 || rounded > i64::MAX as f64 {
        return None;
    }
    Some(rounded as i64)
}

/// Convert an f64 to i64 only when it is integral and in range.
fn i64_from_exact_f64(v: f64) -> Option<i64> {
    if !v.is_finite() || v.fract() != 0.0 {
        return None;
    }
    if v < i64::MIN as f64 || v > i64::MAX as f64 {
        return None;
    }
    Some(v as i64)
}

/// Format a scaled currency value (e.g. `12500` -> `"1.25"`, `10000` -> `"1"`).
fn format_currency(raw: i64) -> String {
    let sign = if raw < 0 { "-" } else { "" };
    let abs = raw.unsigned_abs();
    let whole = abs / CURRENCY_SCALE as u64;
    let frac = abs % CURRENCY_SCALE as u64;
    if frac == 0 {
        format!("{sign}{whole}")
    } else {
        let frac_s = format!("{frac:04}");
        let frac_s = frac_s.trim_end_matches('0');
        format!("{sign}{whole}.{frac_s}")
    }
}

/// Convert a date serial to a `M/D/YYYY` string (with time when non-midnight).
fn date_serial_to_string(serial: f64) -> String {
    let Some(dt) = date_serial_to_datetime(serial) else {
        return format!("{serial}");
    };
    let date = format!("{}/{}/{}", dt.month(), dt.day(), dt.year());
    if dt.hour() == 0 && dt.minute() == 0 && dt.second() == 0 {
        date
    } else {
        format!(
            "{date} {}:{}:{}",
            pad2(dt.hour() as i64),
            pad2(dt.minute() as i64),
            pad2(dt.second() as i64)
        )
    }
}

fn pad2(v: i64) -> String {
    if v < 10 {
        format!("0{v}")
    } else {
        v.to_string()
    }
}

/// Convert an OLE automation date serial to a civil datetime.
pub(crate) fn date_serial_to_datetime(serial: f64) -> Option<jiff::civil::DateTime> {
    use jiff::civil::{Date, DateTime};
    use jiff::Span;

    if !serial.is_finite() {
        return None;
    }
    let epoch = Date::new(1899, 12, 30).ok()?;
    let days = serial.floor();
    let date = if days >= 0.0 {
        epoch.checked_add(Span::new().days(days as i64)).ok()?
    } else {
        epoch.checked_sub(Span::new().days((-days) as i64)).ok()?
    };
    let frac = serial - days;
    let seconds = (frac * 86_400.0).round() as i64;
    let secs = (seconds % 60) as i8;
    let mins = ((seconds / 60) % 60) as i8;
    let hours = (seconds / 3600) as i8;
    DateTime::new(date.year(), date.month(), date.day(), hours, mins, secs, 0).ok()
}

/// Parse a VB6 numeric string, honoring `&H`/`&O` prefixes and type suffixes.
fn parse_vb_number(raw: &str) -> Option<f64> {
    let s = raw.trim();
    if s.is_empty() {
        return None;
    }
    let upper = s.to_ascii_uppercase();
    if let Some(rest) = upper
        .strip_prefix("&H")
        .or_else(|| upper.strip_prefix("&O"))
    {
        let radix = if upper.starts_with("&H") { 16 } else { 8 };
        let digits = rest.trim().trim_end_matches('%').trim_end_matches('&');
        let v = i64::from_str_radix(digits, radix).ok()?;
        return Some(v as f64);
    }
    let s = match s.chars().last() {
        Some('%') | Some('&') | Some('!') | Some('#') | Some('@') | Some('$') => &s[..s.len() - 1],
        _ => s,
    };
    if s.is_empty() {
        return None;
    }
    s.parse::<f64>().ok()
}

/// Parse a VB6 date string into a serial day number.
fn parse_vb_date(raw: &str) -> Option<f64> {
    use jiff::civil::Date;
    use jiff::{SpanRelativeTo, Unit};

    let t = raw.trim();
    let (date_part, time_part) = match t.split_once([' ', 'T']) {
        Some((d, tm)) => (d, Some(tm)),
        None => (t, None),
    };
    let date = parse_date_part(date_part)?;
    let base = Date::new(1899, 12, 30).ok()?;
    let delta = date
        .since(base)
        .ok()?
        .total((Unit::Day, SpanRelativeTo::days_are_24_hours()))
        .ok()?;
    let serial = match time_part {
        Some(tm) => delta + parse_time_part(tm)?,
        None => delta,
    };
    Some(serial)
}

/// Parse the date portion (`M/D/YYYY`, `YYYY-M-D`, `YYYYMMDD`).
fn parse_date_part(s: &str) -> Option<jiff::civil::Date> {
    let s = s.trim();
    if s.len() == 8 && s.chars().all(|c| c.is_ascii_digit()) {
        let year: i16 = s[0..4].parse().ok()?;
        let month: i8 = s[4..6].parse().ok()?;
        let day: i8 = s[6..8].parse().ok()?;
        return jiff::civil::Date::new(year, month, day).ok();
    }
    let parts: Vec<&str> = s.split(['/', '-']).collect();
    if parts.len() != 3 {
        return None;
    }
    let (y, m, d) = if parts[0].len() == 4 {
        (
            parts[0].parse().ok()?,
            parts[1].parse().ok()?,
            parts[2].parse().ok()?,
        )
    } else {
        (
            parts[2].parse().ok()?,
            parts[0].parse().ok()?,
            parts[1].parse().ok()?,
        )
    };
    jiff::civil::Date::new(y, m, d).ok()
}

/// Parse the time portion (`H:MM[:SS][ AM/PM]`) as a fraction of a day.
fn parse_time_part(s: &str) -> Option<f64> {
    let mut t = s.trim();
    let mut pm = false;
    let lower = t.to_ascii_lowercase();
    if lower.ends_with("am") {
        t = t[..t.len() - 2].trim();
    } else if lower.ends_with("pm") {
        pm = true;
        t = t[..t.len() - 2].trim();
    }
    let parts: Vec<&str> = t.split(':').collect();
    if parts.is_empty() {
        return None;
    }
    let hours: u32 = parts[0].trim().parse().ok()?;
    let minutes: u32 = parts
        .get(1)
        .map(|p| p.trim().parse().ok())
        .unwrap_or(Some(0))?;
    let seconds: f64 = parts
        .get(2)
        .map(|p| p.trim().parse().ok())
        .unwrap_or(Some(0.0))?;
    let h = if pm { hours % 12 + 12 } else { hours % 12 };
    Some((h as f64 * 3600.0 + minutes as f64 * 60.0 + seconds) / 86_400.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::VBType;

    #[derive(Debug)]
    struct TestObject(&'static str);

    impl VBObject for TestObject {
        fn type_name(&self) -> &str {
            self.0
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn clone_box(&self) -> Box<dyn VBObject> {
            Box::new(TestObject(self.0))
        }
    }

    #[test]
    fn type_of_reflects_value_kind() {
        assert_eq!(Value::empty().type_of(), VBType::Empty);
        assert_eq!(Value::null().type_of(), VBType::Null);
        assert_eq!(Value::Long(3).type_of(), VBType::Long);
        assert_eq!(Value::from_string("x").type_of(), VBType::String);
        assert_eq!(
            Value::array_dynamic(VBType::Integer).type_of(),
            VBType::Array(Box::new(VBType::Integer))
        );
    }

    #[test]
    fn var_type_matches_vba() {
        assert_eq!(Value::empty().var_type(), 0);
        assert_eq!(Value::null().var_type(), 1);
        assert_eq!(Value::Integer(5).var_type(), 2);
        assert_eq!(Value::from_string("x").var_type(), 8);
        assert_eq!(Value::Boolean(true).var_type(), 11);
        assert_eq!(Value::from_error(VBError::new(13)).var_type(), 10);
        assert_eq!(Value::array_dynamic(VBType::Double).var_type(), 8192 + 5);
    }

    #[test]
    fn predicates() {
        assert!(Value::empty().is_empty());
        assert!(Value::null().is_null());
        assert!(Value::nothing().is_nothing());
        assert!(Value::Long(1).is_numeric());
        assert!(!Value::Boolean(true).is_numeric());
        assert!(!Value::Date(0.0).is_numeric());
        assert!(!Value::from_string("1").is_numeric());
        assert!(Value::from_error(VBError::new(5)).is_error());
        assert!(Value::array_dynamic(VBType::Long).is_array());
    }

    #[test]
    fn string_conversion() {
        assert_eq!(Value::empty().as_string().unwrap(), "");
        assert_eq!(Value::Long(-42).as_string().unwrap(), "-42");
        assert_eq!(Value::Boolean(true).as_string().unwrap(), "True");
        assert_eq!(
            Value::from_currency_scaled(12500).as_string().unwrap(),
            "1.25"
        );
        assert_eq!(Value::from_currency_scaled(10000).as_string().unwrap(), "1");
        assert_eq!(
            Value::from_currency_scaled(-12500).as_string().unwrap(),
            "-1.25"
        );
        assert_eq!(
            Value::from_currency_scaled(12345).as_string().unwrap(),
            "1.2345"
        );
        assert_eq!(
            Value::from_error(VBError::new(13)).as_string().unwrap(),
            "Error 13"
        );
        assert_eq!(Value::nothing().as_string().unwrap(), "Nothing");
    }

    #[test]
    fn null_raises_error_94_on_conversion() {
        assert_eq!(Value::null().as_string().unwrap_err().number, 94);
        assert_eq!(Value::null().as_i32().unwrap_err().number, 94);
        assert_eq!(Value::null().as_f64().unwrap_err().number, 94);
        assert_eq!(Value::null().as_bool().unwrap_err().number, 94);
    }

    #[test]
    fn numeric_conversion_from_string() {
        assert_eq!(Value::from_string("42").as_i32().unwrap(), 42);
        assert_eq!(Value::from_string("-1.5").as_f64().unwrap(), -1.5);
        assert_eq!(Value::from_string("&H1F").as_i32().unwrap(), 31);
        assert_eq!(Value::from_string("&O10").as_i32().unwrap(), 8);
        assert_eq!(Value::from_string("5%").as_i32().unwrap(), 5);
        assert_eq!(Value::from_string("abc").as_i32().unwrap_err().number, 13);
    }

    #[test]
    fn banker_rounding_for_integer_conversions() {
        assert_eq!(Value::Double(2.5).as_i16().unwrap(), 2);
        assert_eq!(Value::Double(3.5).as_i16().unwrap(), 4);
        assert_eq!(Value::Double(-2.5).as_i16().unwrap(), -2);
        assert_eq!(Value::Double(-3.5).as_i16().unwrap(), -4);
        assert_eq!(Value::from_string("2.5").as_i32().unwrap(), 2);
    }

    #[test]
    fn overflow_detection() {
        assert_eq!(Value::Double(40_000.0).as_i16().unwrap_err().number, 6);
        assert_eq!(Value::Double(3.0e10).as_i32().unwrap_err().number, 6);
        assert_eq!(Value::Integer(-1).as_byte().unwrap_err().number, 6);
        assert_eq!(Value::Long(300).as_byte().unwrap_err().number, 6);
    }

    #[test]
    fn boolean_conversion() {
        assert!(!Value::empty().as_bool().unwrap());
        assert!(!Value::Long(0).as_bool().unwrap());
        assert!(Value::Long(5).as_bool().unwrap());
        assert!(Value::from_string("True").as_bool().unwrap());
        assert!(!Value::from_string("false").as_bool().unwrap());
        assert!(Value::from_string("1").as_bool().unwrap());
    }

    #[test]
    fn currency_conversion() {
        assert_eq!(Value::Double(1.5).as_currency_scaled().unwrap(), 15000);
        assert_eq!(
            Value::from_currency(1.25).as_currency_scaled().unwrap(),
            12500
        );
        assert_eq!(
            Value::from_currency_scaled(10000)
                .as_currency_scaled()
                .unwrap(),
            10000
        );
        assert_eq!(Value::Currency(15000).as_i64().unwrap(), 2);
        assert_eq!(
            Value::from_string("1.5").as_currency_scaled().unwrap(),
            15000
        );
    }

    #[test]
    fn date_serial_round_trip() {
        assert_eq!(Value::Date(0.0).as_string().unwrap(), "12/30/1899");
        assert_eq!(Value::Date(2.0).as_string().unwrap(), "1/1/1900");
        assert_eq!(Value::Date(0.5).as_string().unwrap(), "12/30/1899 12:00:00");
        assert_eq!(
            Value::from_string("1/1/2026").as_date_serial().unwrap(),
            46023.0
        );
        assert_eq!(Value::Date(46023.0).as_string().unwrap(), "1/1/2026");
        assert_eq!(Value::Long(46023).as_date_serial().unwrap(), 46023.0);
    }

    #[test]
    fn numeric_equality_coerces() {
        assert_eq!(Value::Integer(1), Value::Double(1.0));
        assert_eq!(Value::Integer(1), Value::from_string("1"));
        assert_eq!(Value::from_currency_scaled(10000), Value::Double(1.0));
        assert_ne!(Value::Integer(1), Value::Double(1.5));
        assert_ne!(Value::Integer(1), Value::from_string("abc"));
    }

    #[test]
    fn boolean_equality_follows_vb6() {
        // In VB6, True coerces to -1.
        assert_eq!(Value::Boolean(true), Value::Integer(-1));
        assert_ne!(Value::Boolean(true), Value::Integer(1));
        assert_eq!(Value::Boolean(false), Value::Integer(0));
    }

    #[test]
    fn empty_equality_with_zero() {
        assert_eq!(Value::empty(), Value::Long(0));
    }

    #[test]
    fn null_is_not_equal_to_anything_else() {
        assert_ne!(Value::null(), Value::Long(0));
        assert_ne!(Value::null(), Value::empty());
    }

    #[test]
    fn object_equality_is_identity() {
        let obj = Box::new(TestObject("Coll"));
        let a = Value::from_object(obj.clone_box());
        let b = Value::from_object(obj.clone_box());
        assert_ne!(a, b); // distinct references
        let a2 = a.clone();
        assert_ne!(a, a2); // a clone is a new reference, so not equal by identity
    }

    #[test]
    fn object_clone_and_identity_semantics() {
        let obj = Box::new(TestObject("Coll"));
        let a = Value::from_object(obj);
        let b = Value::from_object(Box::new(TestObject("Coll")));
        assert_ne!(a, b);
        assert_eq!(a.as_object().unwrap().type_name(), "Coll");
        assert_eq!(
            a.as_object()
                .unwrap()
                .as_any()
                .downcast_ref::<TestObject>()
                .unwrap()
                .0,
            "Coll"
        );
    }

    #[test]
    fn default_values() {
        assert_eq!(Value::default_for_type(&VBType::Integer), Value::Integer(0));
        assert_eq!(
            Value::default_for_type(&VBType::String),
            Value::String(String::new())
        );
        assert_eq!(Value::default_for_type(&VBType::Variant), Value::Empty);
        assert_eq!(Value::default_for_type(&VBType::Object), Value::Nothing);
    }

    #[test]
    fn clone_works_across_all_variants() {
        let values = vec![
            Value::empty(),
            Value::null(),
            Value::nothing(),
            Value::Byte(1),
            Value::Integer(1),
            Value::Long(1),
            Value::Single(1.0),
            Value::Double(1.0),
            Value::Currency(1),
            Value::from_string("x"),
            Value::Boolean(true),
            Value::Date(1.0),
            Value::from_error(VBError::new(13)),
            Value::array_dynamic(VBType::Long),
        ];
        for v in values {
            assert_eq!(v.clone(), v);
        }
    }

    #[test]
    fn display_and_as_string() {
        assert_eq!(Value::empty().to_string(), "Empty");
        assert_eq!(Value::Long(7).to_string(), "7");
        assert_eq!(Value::Boolean(false).to_string(), "False");
    }
}
