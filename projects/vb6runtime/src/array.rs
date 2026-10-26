//! VB6 array support.
//!
//! VB6 arrays are 1-based by default but may declare arbitrary lower and upper
//! bounds (`Dim x(1 To 5)`). [`ArrayValue`] stores elements in a flat buffer
//! with one inclusive [`ArrayDimension`] per rank.

use crate::error::{VBError, VBResult, err_number};
use crate::types::VBType;
use crate::value::VBVariant;

/// Inclusive bounds of a single array dimension.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayDimension {
    /// Lower bound (VB6 defaults to 1, but may be any value).
    pub lower: i32,
    /// Upper bound. For a dynamic (uninitialized) dimension there are no bounds.
    pub upper: i32,
}

impl ArrayDimension {
    /// Create a new inclusive dimension `lower..=upper`.
    pub fn new(lower: i32, upper: i32) -> Self {
        Self { lower, upper }
    }

    /// The number of elements: `upper - lower + 1`, or 0 when `upper < lower`.
    pub fn len(&self) -> usize {
        (self.upper as i64 - self.lower as i64 + 1).max(0) as usize
    }

    /// Whether this dimension contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A VB6 array value.
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayValue {
    element_type: VBType,
    dimensions: Vec<ArrayDimension>,
    data: Vec<VBVariant>,
}

impl ArrayValue {
    /// Create a fixed-size array with the given bounds, filled with the
    /// type's default value.
    pub fn new_fixed(element_type: VBType, dimensions: &[ArrayDimension]) -> VBResult<Self> {
        let mut size: usize = 1;
        for dimension in dimensions {
            size = size
                .checked_mul(dimension.len())
                .ok_or_else(VBError::out_of_memory)?;
        }
        let default = VBVariant::default_for_type(&element_type);
        Ok(Self {
            element_type,
            dimensions: dimensions.to_vec(),
            data: vec![default; size],
        })
    }

    /// Create a dynamic (uninitialized) array. It must be sized with
    /// `Redim`/`ReDim Preserve` before elements can be accessed.
    pub fn new_dynamic(element_type: VBType) -> Self {
        Self {
            element_type,
            dimensions: Vec::new(),
            data: Vec::new(),
        }
    }

    /// Create a one-dimensional array from an existing element buffer
    /// with `1 To data.len()` bounds.
    pub fn from_vec(element_type: VBType, data: Vec<VBVariant>) -> Self {
        let len = data.len() as i32;
        Self {
            element_type,
            dimensions: vec![ArrayDimension::new(1, len)],
            data,
        }
    }

    /// Create a one-dimensional array from an existing element buffer
    /// with the specified lower bound and upper bound derived from `data.len()`.
    pub fn from_vec_with_bounds(element_type: VBType, data: Vec<VBVariant>, lower: i32) -> Self {
        let upper = if data.is_empty() {
            lower - 1
        } else {
            lower + (data.len() as i32 - 1)
        };
        Self {
            element_type,
            dimensions: vec![ArrayDimension::new(lower, upper)],
            data,
        }
    }

    /// The element type of this array.
    pub fn element_type(&self) -> &VBType {
        &self.element_type
    }

    /// The number of dimensions (rank).
    pub fn rank(&self) -> usize {
        self.dimensions.len()
    }

    /// Whether this array has been sized (`ReDim`'d) and is usable.
    pub fn is_initialized(&self) -> bool {
        !self.dimensions.is_empty()
    }

    /// The declared dimension bounds.
    pub fn dimensions(&self) -> &[ArrayDimension] {
        &self.dimensions
    }

    /// The total number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Whether the array holds no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// The lower bound of a 1-based dimension (index 0 is the first dimension).
    pub fn lower_bound(&self, dimension: usize) -> VBResult<i32> {
        self.dimensions
            .get(dimension)
            .map(|d| d.lower)
            .ok_or_else(VBError::subscript_out_of_range)
    }

    /// The upper bound of a 1-based dimension (index 0 is the first dimension).
    pub fn upper_bound(&self, dimension: usize) -> VBResult<i32> {
        self.dimensions
            .get(dimension)
            .map(|d| d.upper)
            .ok_or_else(VBError::subscript_out_of_range)
    }

    /// Borrow an element by indices (one per dimension, in order).
    pub fn get(&self, indices: &[i32]) -> VBResult<&VBVariant> {
        let offset = self.offset(indices)?;
        self.data
            .get(offset)
            .ok_or_else(VBError::subscript_out_of_range)
    }

    /// Mutably borrow an element by indices.
    pub fn get_mut(&mut self, indices: &[i32]) -> VBResult<&mut VBVariant> {
        let offset = self.offset(indices)?;
        self.data
            .get_mut(offset)
            .ok_or_else(VBError::subscript_out_of_range)
    }

    /// Write an element by indices.
    pub fn set(&mut self, indices: &[i32], value: VBVariant) -> VBResult<()> {
        let offset = self.offset(indices)?;
        self.data[offset] = coerce_array_value(&self.element_type, value)?;
        Ok(())
    }

    /// Resize the array, optionally preserving its existing elements.
    ///
    /// With `Preserve`, only the upper bound of the final dimension may change.
    pub fn redim(&mut self, dimensions: &[ArrayDimension], preserve: bool) -> VBResult<()> {
        if !preserve {
            *self = Self::new_fixed(self.element_type.clone(), dimensions)?;
            return Ok(());
        }
        if self.dimensions.is_empty()
            || dimensions.len() != self.dimensions.len()
            || self.dimensions[..self.dimensions.len() - 1]
                != dimensions[..dimensions.len().saturating_sub(1)]
            || self.dimensions.last().map(|d| d.lower) != dimensions.last().map(|d| d.lower)
        {
            return Err(VBError::invalid_procedure_call());
        }

        let mut resized = Self::new_fixed(self.element_type.clone(), dimensions)?;
        let old_last_len = self.dimensions.last().map_or(0, ArrayDimension::len);
        let new_last_len = dimensions.last().map_or(0, ArrayDimension::len);
        let copied_last_len = old_last_len.min(new_last_len);
        let rows = self.dimensions[..self.dimensions.len() - 1]
            .iter()
            .try_fold(1usize, |n, dimension| n.checked_mul(dimension.len()))
            .ok_or_else(VBError::out_of_memory)?;
        for row in 0..rows {
            let old_start = row * old_last_len;
            let new_start = row * new_last_len;
            resized.data[new_start..new_start + copied_last_len]
                .clone_from_slice(&self.data[old_start..old_start + copied_last_len]);
        }
        *self = resized;
        Ok(())
    }

    /// The raw element buffer.
    pub fn as_slice(&self) -> &[VBVariant] {
        &self.data
    }

    /// Compute the flat offset for a set of indices.
    fn offset(&self, indices: &[i32]) -> VBResult<usize> {
        if !self.is_initialized() {
            return Err(VBError::new(err_number::SUBSCRIPT_OUT_OF_RANGE));
        }
        if indices.len() != self.dimensions.len() {
            return Err(VBError::new(err_number::WRONG_NUMBER_OF_ARGUMENTS));
        }
        let mut offset: usize = 0;
        for (i, &index) in indices.iter().enumerate() {
            let dimension = &self.dimensions[i];
            if index < dimension.lower || index > dimension.upper {
                return Err(VBError::subscript_out_of_range());
            }
            offset = offset * dimension.len() + (index - dimension.lower) as usize;
        }
        Ok(offset)
    }
}

fn coerce_array_value(element_type: &VBType, value: VBVariant) -> VBResult<VBVariant> {
    match element_type {
        VBType::Variant => Ok(value),
        VBType::Byte => value.as_byte().map(VBVariant::Byte),
        VBType::Integer => value.as_i16().map(VBVariant::Integer),
        VBType::Long | VBType::Enum(_) => value.as_i32().map(VBVariant::Long),
        VBType::Single => value.as_f32().map(VBVariant::Single),
        VBType::Double => value.as_f64().map(VBVariant::Double),
        VBType::Currency => value.as_currency_scaled().map(VBVariant::Currency),
        VBType::Decimal => match value {
            value @ VBVariant::Decimal(_) => Ok(value),
            _ => Err(VBError::type_mismatch()),
        },
        VBType::String => value.as_string().map(VBVariant::String),
        VBType::Boolean => value.as_bool().map(VBVariant::Boolean),
        VBType::Date => value.as_date_serial().map(VBVariant::Date),
        VBType::Object | VBType::Class(_) | VBType::Nothing => match value {
            value @ (VBVariant::Nothing | VBVariant::Object(_)) => Ok(value),
            _ => Err(VBError::type_mismatch()),
        },
        _ if &value.type_of() == element_type => Ok(value),
        _ => Err(VBError::type_mismatch()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimension_length_is_inclusive() {
        assert_eq!(ArrayDimension::new(1, 5).len(), 5);
        assert_eq!(ArrayDimension::new(0, 0).len(), 1);
        assert_eq!(ArrayDimension::new(5, 1).len(), 0);
    }

    #[test]
    fn fixed_array_uses_default_values() {
        let arr = ArrayValue::new_fixed(VBType::Integer, &[ArrayDimension::new(1, 3)]).unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr.get(&[2]).unwrap(), &VBVariant::Integer(0));
        assert_eq!(arr.lower_bound(0).unwrap(), 1);
        assert_eq!(arr.upper_bound(0).unwrap(), 3);
    }

    #[test]
    fn arbitrary_bounds_are_supported() {
        let mut arr = ArrayValue::new_fixed(VBType::String, &[ArrayDimension::new(-2, 0)]).unwrap();
        assert_eq!(arr.lower_bound(0).unwrap(), -2);
        assert_eq!(arr.upper_bound(0).unwrap(), 0);
        arr.set(&[-2], VBVariant::String("a".into())).unwrap();
        arr.set(&[0], VBVariant::String("c".into())).unwrap();
        assert_eq!(arr.get(&[-2]).unwrap(), &VBVariant::String("a".into()));
        assert_eq!(arr.get(&[0]).unwrap(), &VBVariant::String("c".into()));
    }

    #[test]
    fn out_of_range_indexes_error() {
        let arr = ArrayValue::new_fixed(VBType::Long, &[ArrayDimension::new(1, 2)]).unwrap();
        assert_eq!(
            arr.get(&[0]).unwrap_err().number,
            err_number::SUBSCRIPT_OUT_OF_RANGE
        );
        assert_eq!(
            arr.get(&[3]).unwrap_err().number,
            err_number::SUBSCRIPT_OUT_OF_RANGE
        );
    }

    #[test]
    fn wrong_rank_errors() {
        let arr = ArrayValue::new_fixed(VBType::Long, &[ArrayDimension::new(1, 2)]).unwrap();
        assert_eq!(
            arr.get(&[1, 2]).unwrap_err().number,
            err_number::WRONG_NUMBER_OF_ARGUMENTS
        );
    }

    #[test]
    fn dynamic_array_requires_sizing() {
        let arr = ArrayValue::new_dynamic(VBType::Integer);
        assert!(!arr.is_initialized());
        assert_eq!(
            arr.get(&[1]).unwrap_err().number,
            err_number::SUBSCRIPT_OUT_OF_RANGE
        );
    }

    #[test]
    fn multidimension_offset_is_row_major() {
        let dims = [ArrayDimension::new(1, 2), ArrayDimension::new(1, 3)];
        let mut arr = ArrayValue::new_fixed(VBType::Long, &dims).unwrap();
        for i in 1..=2 {
            for j in 1..=3 {
                arr.set(&[i, j], VBVariant::Long(i * 10 + j)).unwrap();
            }
        }
        assert_eq!(arr.get(&[2, 3]).unwrap(), &VBVariant::Long(23));
        assert_eq!(arr.len(), 6);
    }

    #[test]
    fn typed_array_rejects_mismatched_values_without_mutating() {
        let mut arr = ArrayValue::new_fixed(VBType::Integer, &[ArrayDimension::new(1, 1)]).unwrap();
        arr.set(&[1], VBVariant::Integer(8)).unwrap();
        assert_eq!(
            arr.set(&[1], VBVariant::String("not a number".into()))
                .unwrap_err()
                .number,
            err_number::TYPE_MISMATCH
        );
        assert_eq!(arr.get(&[1]).unwrap(), &VBVariant::Integer(8));
    }

    #[test]
    fn multidimensional_redim_preserve_retains_elements_and_defaults_growth() {
        let mut arr = ArrayValue::new_fixed(
            VBType::Integer,
            &[ArrayDimension::new(1, 2), ArrayDimension::new(0, 1)],
        )
        .unwrap();
        arr.set(&[1, 0], VBVariant::Integer(10)).unwrap();
        arr.set(&[2, 1], VBVariant::Integer(21)).unwrap();
        arr.redim(
            &[ArrayDimension::new(1, 2), ArrayDimension::new(0, 2)],
            true,
        )
        .unwrap();

        assert_eq!(arr.get(&[1, 0]).unwrap(), &VBVariant::Integer(10));
        assert_eq!(arr.get(&[2, 1]).unwrap(), &VBVariant::Integer(21));
        assert_eq!(arr.get(&[2, 2]).unwrap(), &VBVariant::Integer(0));
        assert_eq!(
            arr.redim(
                &[ArrayDimension::new(1, 3), ArrayDimension::new(0, 2)],
                true,
            )
            .unwrap_err()
            .number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }
}
