//! VB6 runtime values.
//!
//! [`VBVariant`] is the dynamic counterpart of `VBType` and
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
/// `Variant` value. The trait requires `Debug` so that [`VBVariant`] can derive
/// `Debug`, and `clone_box` so that [`VBVariant::clone`] works without requiring
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

impl From<VBString> for VBVariant {
    fn from(value: VBString) -> Self {
        VBVariant::from_string(value.0)
    }
}

impl TryFrom<&VBVariant> for VBString {
    type Error = VBError;

    fn try_from(value: &VBVariant) -> Result<Self, Self::Error> {
        value.as_vbstring()
    }
}

impl TryFrom<VBVariant> for VBString {
    type Error = VBError;

    fn try_from(value: VBVariant) -> Result<Self, Self::Error> {
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

impl From<VBByte> for VBVariant {
    fn from(value: VBByte) -> Self {
        VBVariant::from_byte(value.0)
    }
}

impl TryFrom<&VBVariant> for VBByte {
    type Error = VBError;

    fn try_from(value: &VBVariant) -> Result<Self, Self::Error> {
        value.as_vbbyte()
    }
}

impl TryFrom<VBVariant> for VBByte {
    type Error = VBError;

    fn try_from(value: VBVariant) -> Result<Self, Self::Error> {
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

impl From<VBLong> for VBVariant {
    fn from(value: VBLong) -> Self {
        VBVariant::from_long(value.0)
    }
}

impl TryFrom<&VBVariant> for VBLong {
    type Error = VBError;

    fn try_from(value: &VBVariant) -> Result<Self, Self::Error> {
        value.as_vblong()
    }
}

impl TryFrom<VBVariant> for VBLong {
    type Error = VBError;

    fn try_from(value: VBVariant) -> Result<Self, Self::Error> {
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

impl From<VBInteger> for VBVariant {
    fn from(value: VBInteger) -> Self {
        VBVariant::from_integer(value.0)
    }
}

impl TryFrom<&VBVariant> for VBInteger {
    type Error = VBError;

    fn try_from(value: &VBVariant) -> Result<Self, Self::Error> {
        value.as_vbinteger()
    }
}

impl TryFrom<VBVariant> for VBInteger {
    type Error = VBError;

    fn try_from(value: VBVariant) -> Result<Self, Self::Error> {
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

impl From<VBBoolean> for VBVariant {
    fn from(value: VBBoolean) -> Self {
        VBVariant::from_bool(value.0)
    }
}

impl TryFrom<&VBVariant> for VBBoolean {
    type Error = VBError;

    fn try_from(value: &VBVariant) -> Result<Self, Self::Error> {
        value.as_vbboolean()
    }
}

impl TryFrom<VBVariant> for VBBoolean {
    type Error = VBError;

    fn try_from(value: VBVariant) -> Result<Self, Self::Error> {
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

impl From<VBDate> for VBVariant {
    fn from(value: VBDate) -> Self {
        VBVariant::from_date_serial(value.0)
    }
}

impl TryFrom<&VBVariant> for VBDate {
    type Error = VBError;

    fn try_from(value: &VBVariant) -> Result<Self, Self::Error> {
        value.as_vbdate()
    }
}

impl TryFrom<VBVariant> for VBDate {
    type Error = VBError;

    fn try_from(value: VBVariant) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// A runtime-safe single wrapper that preserves VB6 coercion semantics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VBSingle(f32);

impl VBSingle {
    /// Access the wrapped single value.
    pub fn as_f32(self) -> f32 {
        self.0
    }
}

impl From<f32> for VBSingle {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<VBSingle> for VBVariant {
    fn from(value: VBSingle) -> Self {
        VBVariant::from_single(value.0)
    }
}

impl TryFrom<&VBVariant> for VBSingle {
    type Error = VBError;

    fn try_from(value: &VBVariant) -> Result<Self, Self::Error> {
        value.as_vbsingle()
    }
}

impl TryFrom<VBVariant> for VBSingle {
    type Error = VBError;

    fn try_from(value: VBVariant) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// A runtime-safe double wrapper that preserves VB6 coercion semantics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VBDouble(f64);

impl VBDouble {
    /// Access the wrapped double value.
    pub fn as_f64(self) -> f64 {
        self.0
    }
}

impl From<f64> for VBDouble {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<VBDouble> for VBVariant {
    fn from(value: VBDouble) -> Self {
        VBVariant::from_double(value.0)
    }
}

impl TryFrom<&VBVariant> for VBDouble {
    type Error = VBError;

    fn try_from(value: &VBVariant) -> Result<Self, Self::Error> {
        value.as_vbdouble()
    }
}

impl TryFrom<VBVariant> for VBDouble {
    type Error = VBError;

    fn try_from(value: VBVariant) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// A runtime-safe currency wrapper that preserves VB6 coercion semantics.
///
/// The wrapped value is the scaled representation (raw / [`CURRENCY_SCALE`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VBCurrency(i64);

impl VBCurrency {
    /// Access the wrapped scaled currency value (raw / [`CURRENCY_SCALE`]).
    pub fn as_i64(self) -> i64 {
        self.0
    }
}

impl From<i64> for VBCurrency {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<VBCurrency> for VBVariant {
    fn from(value: VBCurrency) -> Self {
        VBVariant::from_currency_scaled(value.0)
    }
}

impl TryFrom<&VBVariant> for VBCurrency {
    type Error = VBError;

    fn try_from(value: &VBVariant) -> Result<Self, Self::Error> {
        value.as_vbcurrency()
    }
}

impl TryFrom<VBVariant> for VBCurrency {
    type Error = VBError;

    fn try_from(value: VBVariant) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// A VB6 runtime value.
#[derive(Debug)]
pub enum VBVariant {
    /// An uninitialized `Variant` (`Empty`).
    Empty,
    /// A `Null` value (unknown data). Distinct from [`VBVariant::Empty`].
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

impl Clone for VBVariant {
    fn clone(&self) -> Self {
        match self {
            VBVariant::Empty => VBVariant::Empty,
            VBVariant::Null => VBVariant::Null,
            VBVariant::Nothing => VBVariant::Nothing,
            VBVariant::Byte(v) => VBVariant::Byte(*v),
            VBVariant::Integer(v) => VBVariant::Integer(*v),
            VBVariant::Long(v) => VBVariant::Long(*v),
            VBVariant::Single(v) => VBVariant::Single(*v),
            VBVariant::Double(v) => VBVariant::Double(*v),
            VBVariant::Currency(v) => VBVariant::Currency(*v),
            VBVariant::String(v) => VBVariant::String(v.clone()),
            VBVariant::Boolean(v) => VBVariant::Boolean(*v),
            VBVariant::Date(v) => VBVariant::Date(*v),
            VBVariant::Error(e) => VBVariant::Error(e.clone()),
            VBVariant::Object(o) => VBVariant::Object(o.clone_box()),
            VBVariant::Array(a) => VBVariant::Array(a.clone()),
        }
    }
}

impl PartialEq for VBVariant {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VBVariant::Empty, VBVariant::Empty) => true,
            (VBVariant::Null, VBVariant::Null) => true,
            (VBVariant::Nothing, VBVariant::Nothing) => true,
            (VBVariant::Error(a), VBVariant::Error(b)) => a == b,
            (VBVariant::String(a), VBVariant::String(b)) => a == b,
            (VBVariant::Boolean(a), VBVariant::Boolean(b)) => a == b,
            (VBVariant::Date(a), VBVariant::Date(b)) => a == b,
            (VBVariant::Array(a), VBVariant::Array(b)) => a == b,
            (VBVariant::Object(a), VBVariant::Object(b)) => std::ptr::eq(
                a.as_ref() as *const dyn VBObject as *const (),
                b.as_ref() as *const dyn VBObject as *const (),
            ),
            // Numeric cross-type comparison with VB6 coercion.
            (VBVariant::Currency(a), VBVariant::Currency(b)) => a == b,
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

impl VBVariant {
    /// Create an `Empty` value.
    pub fn empty() -> Self {
        VBVariant::Empty
    }

    /// Create a `Null` value.
    pub fn null() -> Self {
        VBVariant::Null
    }

    /// Create a `Nothing` object reference.
    pub fn nothing() -> Self {
        VBVariant::Nothing
    }

    /// Create a byte value.
    pub fn from_byte(v: u8) -> Self {
        VBVariant::Byte(v)
    }

    /// Create an Integer (16-bit) value.
    pub fn from_integer(v: i16) -> Self {
        VBVariant::Integer(v)
    }

    /// Create a Long (32-bit) value.
    pub fn from_long(v: i32) -> Self {
        VBVariant::Long(v)
    }

    /// Create a value from a signed 64-bit integer, choosing Integer or Long
    /// based on VB6 integer literal rules.
    pub fn from_i64(v: i64) -> Self {
        if let Ok(n) = i16::try_from(v) {
            VBVariant::Integer(n)
        } else if let Ok(n) = i32::try_from(v) {
            VBVariant::Long(n)
        } else {
            VBVariant::Double(v as f64)
        }
    }

    /// Create a Single (32-bit float) value.
    pub fn from_single(v: f32) -> Self {
        VBVariant::Single(v)
    }

    /// Create a Double (64-bit float) value.
    pub fn from_double(v: f64) -> Self {
        VBVariant::Double(v)
    }

    /// Create a currency value from its scaled representation (raw / 10_000).
    pub fn from_currency_scaled(raw: i64) -> Self {
        VBVariant::Currency(raw)
    }

    /// Create a currency value from decimal units (e.g. `1.25`).
    pub fn from_currency(units: f64) -> Self {
        VBVariant::Currency(round_half_even(units * CURRENCY_SCALE as f64).unwrap_or(i64::MAX))
    }

    /// Create a string value.
    pub fn from_string(v: impl Into<String>) -> Self {
        VBVariant::String(v.into())
    }

    /// Create a boolean value.
    pub fn from_bool(v: bool) -> Self {
        VBVariant::Boolean(v)
    }

    /// Create a date value from a serial day number (1899-12-30 == 0).
    pub fn from_date_serial(serial: f64) -> Self {
        VBVariant::Date(serial)
    }

    /// Create an error value.
    pub fn from_error(e: VBError) -> Self {
        VBVariant::Error(e)
    }

    /// Create an object reference value.
    pub fn from_object(o: Box<dyn VBObject>) -> Self {
        VBVariant::Object(o)
    }

    /// Create an array value.
    pub fn from_array(a: ArrayValue) -> Self {
        VBVariant::Array(a)
    }

    /// The dynamic (value) type of this value.
    pub fn type_of(&self) -> VBType {
        match self {
            VBVariant::Empty => VBType::Empty,
            VBVariant::Null => VBType::Null,
            VBVariant::Nothing => VBType::Nothing,
            VBVariant::Byte(_) => VBType::Byte,
            VBVariant::Integer(_) => VBType::Integer,
            VBVariant::Long(_) => VBType::Long,
            VBVariant::Single(_) => VBType::Single,
            VBVariant::Double(_) => VBType::Double,
            VBVariant::Currency(_) => VBType::Currency,
            VBVariant::String(_) => VBType::String,
            VBVariant::Boolean(_) => VBType::Boolean,
            VBVariant::Date(_) => VBType::Date,
            VBVariant::Error(_) => VBType::Error,
            VBVariant::Object(_) => VBType::Object,
            VBVariant::Array(a) => VBType::Array(Box::new(a.element_type().clone())),
        }
    }

    /// The VBA `VarType` code for this value.
    pub fn var_type(&self) -> i32 {
        match self {
            VBVariant::Empty => vartype::EMPTY,
            VBVariant::Null => vartype::NULL,
            VBVariant::Nothing => vartype::OBJECT,
            VBVariant::Byte(_) => vartype::BYTE,
            VBVariant::Integer(_) => vartype::INTEGER,
            VBVariant::Long(_) => vartype::LONG,
            VBVariant::Single(_) => vartype::SINGLE,
            VBVariant::Double(_) => vartype::DOUBLE,
            VBVariant::Currency(_) => vartype::CURRENCY,
            VBVariant::String(_) => vartype::STRING,
            VBVariant::Boolean(_) => vartype::BOOLEAN,
            VBVariant::Date(_) => vartype::DATE,
            VBVariant::Error(_) => vartype::ERROR,
            VBVariant::Object(_) => vartype::OBJECT,
            VBVariant::Array(a) => vartype::ARRAY | a.element_type().var_type(),
        }
    }

    /// Whether this value is `Empty` (uninitialized Variant).
    pub fn is_empty(&self) -> bool {
        matches!(self, VBVariant::Empty)
    }

    /// Whether this value is `Null`.
    pub fn is_null(&self) -> bool {
        matches!(self, VBVariant::Null)
    }

    /// Whether this value is `Nothing`.
    pub fn is_nothing(&self) -> bool {
        matches!(self, VBVariant::Nothing)
    }

    /// Whether this value is of an integral type (Byte, Integer, Long, Boolean,
    /// or Empty, which coerces to 0 in arithmetic). Singles, Doubles, Currency,
    /// and Dates are not integral even though they may hold whole numbers.
    pub fn is_integral(&self) -> bool {
        matches!(
            self,
            VBVariant::Byte(_) | VBVariant::Integer(_) | VBVariant::Long(_) | VBVariant::Boolean(_) | VBVariant::Empty
        )
    }

    /// Whether this value is numeric (`IsNumeric` semantics: excludes Boolean
    /// and Date).
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            VBVariant::Byte(_)
                | VBVariant::Integer(_)
                | VBVariant::Long(_)
                | VBVariant::Single(_)
                | VBVariant::Double(_)
                | VBVariant::Currency(_)
        )
    }

    /// Whether this value is an array.
    pub fn is_array(&self) -> bool {
        matches!(self, VBVariant::Array(_))
    }

    /// Whether this value is an error value.
    pub fn is_error(&self) -> bool {
        matches!(self, VBVariant::Error(_))
    }

    /// Whether this value is a string.
    pub fn is_string(&self) -> bool {
        matches!(self, VBVariant::String(_))
    }

    /// Whether this value is a boolean.
    pub fn is_boolean(&self) -> bool {
        matches!(self, VBVariant::Boolean(_))
    }

    /// Whether this value is an object reference.
    pub fn is_object(&self) -> bool {
        matches!(self, VBVariant::Object(_))
    }

    /// Whether this value is a date.
    pub fn is_date(&self) -> bool {
        matches!(self, VBVariant::Date(_))
    }

    /// The default value for a given static type, as VB6 initializes variables.
    pub fn default_for_type(t: &VBType) -> VBVariant {
        match t {
            VBType::Byte => VBVariant::Byte(0),
            VBType::Integer => VBVariant::Integer(0),
            VBType::Long => VBVariant::Long(0),
            VBType::Single => VBVariant::Single(0.0),
            VBType::Double => VBVariant::Double(0.0),
            VBType::Currency => VBVariant::Currency(0),
            VBType::String => VBVariant::String(String::new()),
            VBType::Boolean => VBVariant::Boolean(false),
            VBType::Date => VBVariant::Date(0.0),
            VBType::Variant => VBVariant::Empty,
            VBType::Object | VBType::Class(_) | VBType::Nothing => VBVariant::Nothing,
            VBType::Enum(_) => VBVariant::Long(0),
            VBType::UserType(_) | VBType::Unknown => VBVariant::Empty,
            VBType::Array(inner) => VBVariant::Array(ArrayValue::new_dynamic((**inner).clone())),
            VBType::Empty => VBVariant::Empty,
            VBType::Null => VBVariant::Null,
            VBType::Error => VBVariant::Error(VBError::new(0)),
            VBType::Sub | VBType::Function { .. } => VBVariant::Empty,
        }
    }

    /// Convert to a String following `CStr` semantics.
    ///
    /// `Null` raises error 94; `Empty` becomes the empty string; objects and
    /// arrays raise error 13 (type mismatch).
    pub fn as_string(&self) -> VBResult<String> {
        match self {
            VBVariant::Empty => Ok(String::new()),
            VBVariant::Null => Err(VBError::invalid_use_of_null()),
            VBVariant::Nothing => Ok("Nothing".to_string()),
            VBVariant::Byte(v) => Ok(v.to_string()),
            VBVariant::Integer(v) => Ok(v.to_string()),
            VBVariant::Long(v) => Ok(v.to_string()),
            VBVariant::Single(v) => Ok(v.to_string()),
            VBVariant::Double(v) => Ok(v.to_string()),
            VBVariant::Currency(raw) => Ok(format_currency(*raw)),
            VBVariant::String(s) => Ok(s.clone()),
            VBVariant::Boolean(b) => Ok(if *b { "True" } else { "False" }.to_string()),
            VBVariant::Date(serial) => Ok(date_serial_to_string(*serial)),
            VBVariant::Error(e) => Ok(format!("Error {}", e.number)),
            VBVariant::Object(_) | VBVariant::Array(_) => Err(VBError::type_mismatch()),
        }
    }

    /// Borrow the string contents of a String value.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            VBVariant::String(s) => Some(s),
            _ => None,
        }
    }

    /// Convert to a Boolean following `CBool` semantics.
    pub fn as_bool(&self) -> VBResult<bool> {
        match self {
            VBVariant::Empty => Ok(false),
            VBVariant::Null => Err(VBError::invalid_use_of_null()),
            VBVariant::Nothing | VBVariant::Object(_) | VBVariant::Array(_) => Err(VBError::type_mismatch()),
            VBVariant::Error(e) => Err(e.clone()),
            VBVariant::Boolean(b) => Ok(*b),
            VBVariant::String(s) => {
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
            VBVariant::Empty => Ok(0),
            VBVariant::Null => Err(VBError::invalid_use_of_null()),
            VBVariant::Nothing | VBVariant::Object(_) | VBVariant::Array(_) => Err(VBError::type_mismatch()),
            VBVariant::Error(e) => Err(e.clone()),
            VBVariant::Byte(v) => Ok(*v as i64),
            VBVariant::Integer(v) => Ok(*v as i64),
            VBVariant::Long(v) => Ok(*v as i64),
            VBVariant::Boolean(b) => Ok(if *b { -1 } else { 0 }),
            VBVariant::Currency(raw) => {
                round_half_even(*raw as f64 / CURRENCY_SCALE as f64).ok_or_else(VBError::overflow)
            }
            VBVariant::Single(v) => round_half_even(*v as f64).ok_or_else(VBError::overflow),
            VBVariant::Double(v) => round_half_even(*v).ok_or_else(VBError::overflow),
            VBVariant::Date(v) => round_half_even(*v).ok_or_else(VBError::overflow),
            VBVariant::String(s) => {
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
            VBVariant::Empty => Ok(0.0),
            VBVariant::Null => Err(VBError::invalid_use_of_null()),
            VBVariant::Nothing | VBVariant::Object(_) | VBVariant::Array(_) => Err(VBError::type_mismatch()),
            VBVariant::Error(e) => Err(e.clone()),
            VBVariant::Boolean(b) => Ok(if *b { -1.0 } else { 0.0 }),
            VBVariant::String(s) => parse_vb_number(s).ok_or_else(VBError::type_mismatch),
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
            VBVariant::Empty => Ok(0),
            VBVariant::Null => Err(VBError::invalid_use_of_null()),
            VBVariant::Nothing | VBVariant::Object(_) | VBVariant::Array(_) => Err(VBError::type_mismatch()),
            VBVariant::Error(e) => Err(e.clone()),
            VBVariant::Boolean(b) => Ok(if *b { -CURRENCY_SCALE } else { 0 }),
            VBVariant::Currency(v) => Ok(*v),
            VBVariant::Byte(v) => Ok(*v as i64 * CURRENCY_SCALE),
            VBVariant::Integer(v) => Ok(*v as i64 * CURRENCY_SCALE),
            VBVariant::Long(v) => Ok(*v as i64 * CURRENCY_SCALE),
            VBVariant::Single(v) => {
                round_half_even(*v as f64 * CURRENCY_SCALE as f64).ok_or_else(VBError::overflow)
            }
            VBVariant::Double(v) => {
                round_half_even(*v * CURRENCY_SCALE as f64).ok_or_else(VBError::overflow)
            }
            VBVariant::Date(v) => {
                round_half_even(*v * CURRENCY_SCALE as f64).ok_or_else(VBError::overflow)
            }
            VBVariant::String(s) => {
                let n = parse_vb_number(s).ok_or_else(VBError::type_mismatch)?;
                round_half_even(n * CURRENCY_SCALE as f64).ok_or_else(VBError::overflow)
            }
        }
    }

    /// Convert to a date serial following `CDate` semantics.
    pub fn as_date_serial(&self) -> VBResult<f64> {
        match self {
            VBVariant::Empty => Ok(0.0),
            VBVariant::Null => Err(VBError::invalid_use_of_null()),
            VBVariant::Nothing | VBVariant::Object(_) | VBVariant::Array(_) => Err(VBError::type_mismatch()),
            VBVariant::Error(e) => Err(e.clone()),
            VBVariant::Boolean(b) => Ok(if *b { -1.0 } else { 0.0 }),
            VBVariant::Date(v) => Ok(*v),
            VBVariant::String(s) => {
                if let Some(serial) = parse_vb_date(s) {
                    Ok(serial)
                } else {
                    parse_vb_number(s).ok_or_else(VBError::type_mismatch)
                }
            }
            _ => Ok(self.numeric_f64_coercion().unwrap_or_default()),
        }
    }

    /// Convert to a [`VBString`] following `CStr` semantics.
    pub fn as_vbstring(&self) -> VBResult<VBString> {
        self.as_string().map(VBString::from)
    }

    /// Convert to a [`VBByte`] following `CByte` semantics.
    pub fn as_vbbyte(&self) -> VBResult<VBByte> {
        self.as_byte().map(VBByte::from)
    }

    /// Convert to a [`VBInteger`] following `CInt` semantics.
    pub fn as_vbinteger(&self) -> VBResult<VBInteger> {
        self.as_i16().map(VBInteger::from)
    }

    /// Convert to a [`VBLong`] following `CLng` semantics.
    pub fn as_vblong(&self) -> VBResult<VBLong> {
        self.as_i32().map(VBLong::from)
    }

    /// Convert to a [`VBSingle`] following `CSng` semantics.
    pub fn as_vbsingle(&self) -> VBResult<VBSingle> {
        self.as_f32().map(VBSingle::from)
    }

    /// Convert to a [`VBDouble`] following `CDbl` semantics.
    pub fn as_vbdouble(&self) -> VBResult<VBDouble> {
        self.as_f64().map(VBDouble::from)
    }

    /// Convert to a [`VBCurrency`] following `CCur` semantics.
    pub fn as_vbcurrency(&self) -> VBResult<VBCurrency> {
        self.as_currency_scaled().map(VBCurrency::from)
    }

    /// Convert to a [`VBBoolean`] following `CBool` semantics.
    pub fn as_vbboolean(&self) -> VBResult<VBBoolean> {
        self.as_bool().map(VBBoolean::from)
    }

    /// Convert to a [`VBDate`] following `CDate` semantics.
    pub fn as_vbdate(&self) -> VBResult<VBDate> {
        self.as_date_serial().map(VBDate::from)
    }

    /// Borrow this value as an array, or error 13 on type mismatch.
    pub fn as_array(&self) -> VBResult<&ArrayValue> {
        match self {
            VBVariant::Array(a) => Ok(a),
            _ => Err(VBError::type_mismatch()),
        }
    }

    /// Borrow this value as an object, or error 424 on type mismatch.
    pub fn as_object(&self) -> VBResult<&dyn VBObject> {
        match self {
            VBVariant::Object(o) => Ok(o.as_ref()),
            _ => Err(VBError::object_required()),
        }
    }

    /// The contained error value, if this is an error value.
    pub fn as_error(&self) -> Option<&VBError> {
        match self {
            VBVariant::Error(e) => Some(e),
            _ => None,
        }
    }

    /// Create a fixed-size array with the given element type and bounds.
    pub fn array_fixed(element_type: VBType, dimensions: &[ArrayDimension]) -> VBResult<VBVariant> {
        Ok(VBVariant::Array(ArrayValue::new_fixed(
            element_type,
            dimensions,
        )?))
    }

    /// Create a dynamic array with the given element type.
    pub fn array_dynamic(element_type: VBType) -> VBVariant {
        VBVariant::Array(ArrayValue::new_dynamic(element_type))
    }

    /// Whether this value compares equal to another following VB6 coercion.
    pub fn coerce_equals(&self, other: &VBVariant) -> bool {
        self == other
    }

    /// Exact integral numeric coercion used by equality (Currency stays scaled).
    fn numeric_i64_exact(&self) -> Option<i64> {
        match self {
            VBVariant::Byte(v) => Some(*v as i64),
            VBVariant::Integer(v) => Some(*v as i64),
            VBVariant::Long(v) => Some(*v as i64),
            VBVariant::Boolean(v) => Some(if *v { -1 } else { 0 }),
            VBVariant::Single(v) => i64_from_exact_f64(*v as f64),
            VBVariant::Double(v) => i64_from_exact_f64(*v),
            VBVariant::Date(v) => i64_from_exact_f64(*v),
            VBVariant::String(s) => i64_from_exact_f64(parse_vb_number(s)?),
            _ => None,
        }
    }

    /// Numeric coercion used by equality and boolean conversion.
    fn numeric_f64_coercion(&self) -> Option<f64> {
        match self {
            VBVariant::Empty => Some(0.0),
            VBVariant::Byte(v) => Some(*v as f64),
            VBVariant::Integer(v) => Some(*v as f64),
            VBVariant::Long(v) => Some(*v as f64),
            VBVariant::Single(v) => Some(*v as f64),
            VBVariant::Double(v) => Some(*v),
            VBVariant::Currency(v) => Some(*v as f64 / CURRENCY_SCALE as f64),
            VBVariant::Date(v) => Some(*v),
            VBVariant::Boolean(v) => Some(if *v { -1.0 } else { 0.0 }),
            VBVariant::String(s) => parse_vb_number(s),
            _ => None,
        }
    }
}

impl fmt::Display for VBVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VBVariant::Empty => f.write_str("Empty"),
            VBVariant::Null => f.write_str("Null"),
            VBVariant::Nothing => f.write_str("Nothing"),
            other => match other.as_string() {
                Ok(s) => f.write_str(&s),
                Err(_) => write!(f, "{other:?}"),
            },
        }
    }
}

#[cfg(test)]
mod wrapper_tests {
    use super::{
        VBBoolean, VBByte, VBCurrency, VBDate, VBDouble, VBInteger, VBLong, VBSingle, VBString,
        VBVariant,
    };

    #[test]
    fn vbstring_round_trips_through_variant() {
        let value = VBVariant::from_string("Hello");
        let typed = VBString::try_from(&value).unwrap();
        assert_eq!(typed.as_str(), "Hello");
        assert_eq!(VBVariant::from(typed), value);
    }

    #[test]
    fn vblong_round_trips_through_variant() {
        let value = VBVariant::from_long(42);
        let typed = VBLong::try_from(&value).unwrap();
        assert_eq!(typed.as_i32(), 42);
        assert_eq!(VBVariant::from(typed), value);
    }

    #[test]
    fn vbbyte_round_trips_through_variant() {
        let value = VBVariant::from_byte(7);
        let typed = VBByte::try_from(&value).unwrap();
        assert_eq!(typed.as_u8(), 7);
        assert_eq!(VBVariant::from(typed), value);
    }

    #[test]
    fn vbinteger_round_trips_through_variant() {
        let value = VBVariant::from_integer(7);
        let typed = VBInteger::try_from(&value).unwrap();
        assert_eq!(typed.as_i16(), 7);
        assert_eq!(VBVariant::from(typed), value);
    }

    #[test]
    fn vbboolean_round_trips_through_variant() {
        let value = VBVariant::from_bool(true);
        let typed = VBBoolean::try_from(&value).unwrap();
        assert!(typed.as_bool());
        assert_eq!(VBVariant::from(typed), value);
    }

    #[test]
    fn vbdate_round_trips_through_variant() {
        let value = VBVariant::from_date_serial(42.5);
        let typed = VBDate::try_from(&value).unwrap();
        assert_eq!(typed.as_f64(), 42.5);
        assert_eq!(VBVariant::from(typed), value);
    }

    #[test]
    fn vbsingle_round_trips_through_variant() {
        let value = VBVariant::from_single(1.5);
        let typed = VBSingle::try_from(&value).unwrap();
        assert_eq!(typed.as_f32(), 1.5);
        assert_eq!(VBVariant::from(typed), value);
    }

    #[test]
    fn vbdouble_round_trips_through_variant() {
        let value = VBVariant::from_double(1.5);
        let typed = VBDouble::try_from(&value).unwrap();
        assert_eq!(typed.as_f64(), 1.5);
        assert_eq!(VBVariant::from(typed), value);
    }

    #[test]
    fn vbcurrency_round_trips_through_variant() {
        let value = VBVariant::from_currency_scaled(12345);
        let typed = VBCurrency::try_from(&value).unwrap();
        assert_eq!(typed.as_i64(), 12345);
        assert_eq!(VBVariant::from(typed), value);
    }

    #[test]
    fn as_vb_accessors_match_wrapper_try_from() {
        let value = VBVariant::from_string("42");
        assert_eq!(value.as_vbstring().unwrap(), VBString::from("42"));
        assert_eq!(value.as_vblong().unwrap(), VBLong::from(42));
        assert_eq!(value.as_vbinteger().unwrap(), VBInteger::from(42));
        assert_eq!(value.as_vbbyte().unwrap(), VBByte::from(42));
        assert_eq!(value.as_vbdouble().unwrap(), VBDouble::from(42.0));
        assert_eq!(value.as_vbsingle().unwrap(), VBSingle::from(42.0));
        assert_eq!(
            value.as_vbcurrency().unwrap(),
            VBCurrency::from(420000)
        );
        assert_eq!(
            value.as_vbdate().unwrap(),
            VBDate::from(42.0)
        );
        assert_eq!(
            VBVariant::from_bool(true).as_vbboolean().unwrap(),
            VBBoolean::from(true)
        );
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
        assert_eq!(VBVariant::empty().type_of(), VBType::Empty);
        assert_eq!(VBVariant::null().type_of(), VBType::Null);
        assert_eq!(VBVariant::Long(3).type_of(), VBType::Long);
        assert_eq!(VBVariant::from_string("x").type_of(), VBType::String);
        assert_eq!(
            VBVariant::array_dynamic(VBType::Integer).type_of(),
            VBType::Array(Box::new(VBType::Integer))
        );
    }

    #[test]
    fn var_type_matches_vba() {
        assert_eq!(VBVariant::empty().var_type(), 0);
        assert_eq!(VBVariant::null().var_type(), 1);
        assert_eq!(VBVariant::Integer(5).var_type(), 2);
        assert_eq!(VBVariant::from_string("x").var_type(), 8);
        assert_eq!(VBVariant::Boolean(true).var_type(), 11);
        assert_eq!(VBVariant::from_error(VBError::new(13)).var_type(), 10);
        assert_eq!(VBVariant::array_dynamic(VBType::Double).var_type(), 8192 + 5);
    }

    #[test]
    fn predicates() {
        assert!(VBVariant::empty().is_empty());
        assert!(VBVariant::null().is_null());
        assert!(VBVariant::nothing().is_nothing());
        assert!(VBVariant::Long(1).is_numeric());
        assert!(!VBVariant::Boolean(true).is_numeric());
        assert!(!VBVariant::Date(0.0).is_numeric());
        assert!(!VBVariant::from_string("1").is_numeric());
        assert!(VBVariant::from_error(VBError::new(5)).is_error());
        assert!(VBVariant::array_dynamic(VBType::Long).is_array());
    }

    #[test]
    fn string_conversion() {
        assert_eq!(VBVariant::empty().as_string().unwrap(), "");
        assert_eq!(VBVariant::Long(-42).as_string().unwrap(), "-42");
        assert_eq!(VBVariant::Boolean(true).as_string().unwrap(), "True");
        assert_eq!(
            VBVariant::from_currency_scaled(12500).as_string().unwrap(),
            "1.25"
        );
        assert_eq!(VBVariant::from_currency_scaled(10000).as_string().unwrap(), "1");
        assert_eq!(
            VBVariant::from_currency_scaled(-12500).as_string().unwrap(),
            "-1.25"
        );
        assert_eq!(
            VBVariant::from_currency_scaled(12345).as_string().unwrap(),
            "1.2345"
        );
        assert_eq!(
            VBVariant::from_error(VBError::new(13)).as_string().unwrap(),
            "Error 13"
        );
        assert_eq!(VBVariant::nothing().as_string().unwrap(), "Nothing");
    }

    #[test]
    fn null_raises_error_94_on_conversion() {
        assert_eq!(VBVariant::null().as_string().unwrap_err().number, 94);
        assert_eq!(VBVariant::null().as_i32().unwrap_err().number, 94);
        assert_eq!(VBVariant::null().as_f64().unwrap_err().number, 94);
        assert_eq!(VBVariant::null().as_bool().unwrap_err().number, 94);
    }

    #[test]
    fn numeric_conversion_from_string() {
        assert_eq!(VBVariant::from_string("42").as_i32().unwrap(), 42);
        assert_eq!(VBVariant::from_string("-1.5").as_f64().unwrap(), -1.5);
        assert_eq!(VBVariant::from_string("&H1F").as_i32().unwrap(), 31);
        assert_eq!(VBVariant::from_string("&O10").as_i32().unwrap(), 8);
        assert_eq!(VBVariant::from_string("5%").as_i32().unwrap(), 5);
        assert_eq!(VBVariant::from_string("abc").as_i32().unwrap_err().number, 13);
    }

    #[test]
    fn banker_rounding_for_integer_conversions() {
        assert_eq!(VBVariant::Double(2.5).as_i16().unwrap(), 2);
        assert_eq!(VBVariant::Double(3.5).as_i16().unwrap(), 4);
        assert_eq!(VBVariant::Double(-2.5).as_i16().unwrap(), -2);
        assert_eq!(VBVariant::Double(-3.5).as_i16().unwrap(), -4);
        assert_eq!(VBVariant::from_string("2.5").as_i32().unwrap(), 2);
    }

    #[test]
    fn overflow_detection() {
        assert_eq!(VBVariant::Double(40_000.0).as_i16().unwrap_err().number, 6);
        assert_eq!(VBVariant::Double(3.0e10).as_i32().unwrap_err().number, 6);
        assert_eq!(VBVariant::Integer(-1).as_byte().unwrap_err().number, 6);
        assert_eq!(VBVariant::Long(300).as_byte().unwrap_err().number, 6);
    }

    #[test]
    fn boolean_conversion() {
        assert!(!VBVariant::empty().as_bool().unwrap());
        assert!(!VBVariant::Long(0).as_bool().unwrap());
        assert!(VBVariant::Long(5).as_bool().unwrap());
        assert!(VBVariant::from_string("True").as_bool().unwrap());
        assert!(!VBVariant::from_string("false").as_bool().unwrap());
        assert!(VBVariant::from_string("1").as_bool().unwrap());
    }

    #[test]
    fn currency_conversion() {
        assert_eq!(VBVariant::Double(1.5).as_currency_scaled().unwrap(), 15000);
        assert_eq!(
            VBVariant::from_currency(1.25).as_currency_scaled().unwrap(),
            12500
        );
        assert_eq!(
            VBVariant::from_currency_scaled(10000)
                .as_currency_scaled()
                .unwrap(),
            10000
        );
        assert_eq!(VBVariant::Currency(15000).as_i64().unwrap(), 2);
        assert_eq!(
            VBVariant::from_string("1.5").as_currency_scaled().unwrap(),
            15000
        );
    }

    #[test]
    fn date_serial_round_trip() {
        assert_eq!(VBVariant::Date(0.0).as_string().unwrap(), "12/30/1899");
        assert_eq!(VBVariant::Date(2.0).as_string().unwrap(), "1/1/1900");
        assert_eq!(VBVariant::Date(0.5).as_string().unwrap(), "12/30/1899 12:00:00");
        assert_eq!(
            VBVariant::from_string("1/1/2026").as_date_serial().unwrap(),
            46023.0
        );
        assert_eq!(VBVariant::Date(46023.0).as_string().unwrap(), "1/1/2026");
        assert_eq!(VBVariant::Long(46023).as_date_serial().unwrap(), 46023.0);
    }

    #[test]
    fn numeric_equality_coerces() {
        assert_eq!(VBVariant::Integer(1), VBVariant::Double(1.0));
        assert_eq!(VBVariant::Integer(1), VBVariant::from_string("1"));
        assert_eq!(VBVariant::from_currency_scaled(10000), VBVariant::Double(1.0));
        assert_ne!(VBVariant::Integer(1), VBVariant::Double(1.5));
        assert_ne!(VBVariant::Integer(1), VBVariant::from_string("abc"));
    }

    #[test]
    fn boolean_equality_follows_vb6() {
        // In VB6, True coerces to -1.
        assert_eq!(VBVariant::Boolean(true), VBVariant::Integer(-1));
        assert_ne!(VBVariant::Boolean(true), VBVariant::Integer(1));
        assert_eq!(VBVariant::Boolean(false), VBVariant::Integer(0));
    }

    #[test]
    fn empty_equality_with_zero() {
        assert_eq!(VBVariant::empty(), VBVariant::Long(0));
    }

    #[test]
    fn null_is_not_equal_to_anything_else() {
        assert_ne!(VBVariant::null(), VBVariant::Long(0));
        assert_ne!(VBVariant::null(), VBVariant::empty());
    }

    #[test]
    fn object_equality_is_identity() {
        let obj = Box::new(TestObject("Coll"));
        let a = VBVariant::from_object(obj.clone_box());
        let b = VBVariant::from_object(obj.clone_box());
        assert_ne!(a, b); // distinct references
        let a2 = a.clone();
        assert_ne!(a, a2); // a clone is a new reference, so not equal by identity
    }

    #[test]
    fn object_clone_and_identity_semantics() {
        let obj = Box::new(TestObject("Coll"));
        let a = VBVariant::from_object(obj);
        let b = VBVariant::from_object(Box::new(TestObject("Coll")));
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
        assert_eq!(VBVariant::default_for_type(&VBType::Integer), VBVariant::Integer(0));
        assert_eq!(
            VBVariant::default_for_type(&VBType::String),
            VBVariant::String(String::new())
        );
        assert_eq!(VBVariant::default_for_type(&VBType::Variant), VBVariant::Empty);
        assert_eq!(VBVariant::default_for_type(&VBType::Object), VBVariant::Nothing);
    }

    #[test]
    fn clone_works_across_all_variants() {
        let values = vec![
            VBVariant::empty(),
            VBVariant::null(),
            VBVariant::nothing(),
            VBVariant::Byte(1),
            VBVariant::Integer(1),
            VBVariant::Long(1),
            VBVariant::Single(1.0),
            VBVariant::Double(1.0),
            VBVariant::Currency(1),
            VBVariant::from_string("x"),
            VBVariant::Boolean(true),
            VBVariant::Date(1.0),
            VBVariant::from_error(VBError::new(13)),
            VBVariant::array_dynamic(VBType::Long),
        ];
        for v in values {
            assert_eq!(v.clone(), v);
        }
    }

    #[test]
    fn display_and_as_string() {
        assert_eq!(VBVariant::empty().to_string(), "Empty");
        assert_eq!(VBVariant::Long(7).to_string(), "7");
        assert_eq!(VBVariant::Boolean(false).to_string(), "False");
    }
}
