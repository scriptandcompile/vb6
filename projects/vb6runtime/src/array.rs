//! VB6 array support.
//!
//! VB6 arrays are 1-based by default but may declare arbitrary lower and upper
//! bounds (`Dim x(1 To 5)`). [`ArrayValue`] stores elements in a flat buffer
//! with one inclusive [`ArrayDimension`] per rank.

use crate::error::{err_number, VBError, VBResult};
use crate::types::VBType;
use crate::value::Value;

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
    data: Vec<Value>,
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
        let default = Value::default_for_type(&element_type);
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
    pub fn from_vec(element_type: VBType, data: Vec<Value>) -> Self {
        let len = data.len() as i32;
        Self {
            element_type,
            dimensions: vec![ArrayDimension::new(1, len)],
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
    pub fn get(&self, indices: &[i32]) -> VBResult<&Value> {
        let offset = self.offset(indices)?;
        self.data
            .get(offset)
            .ok_or_else(VBError::subscript_out_of_range)
    }

    /// Mutably borrow an element by indices.
    pub fn get_mut(&mut self, indices: &[i32]) -> VBResult<&mut Value> {
        let offset = self.offset(indices)?;
        self.data
            .get_mut(offset)
            .ok_or_else(VBError::subscript_out_of_range)
    }

    /// Write an element by indices.
    pub fn set(&mut self, indices: &[i32], value: Value) -> VBResult<()> {
        let offset = self.offset(indices)?;
        self.data[offset] = value;
        Ok(())
    }

    /// The raw element buffer.
    pub fn as_slice(&self) -> &[Value] {
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
        assert_eq!(arr.get(&[2]).unwrap(), &Value::Integer(0));
        assert_eq!(arr.lower_bound(0).unwrap(), 1);
        assert_eq!(arr.upper_bound(0).unwrap(), 3);
    }

    #[test]
    fn arbitrary_bounds_are_supported() {
        let mut arr = ArrayValue::new_fixed(VBType::String, &[ArrayDimension::new(-2, 0)]).unwrap();
        assert_eq!(arr.lower_bound(0).unwrap(), -2);
        assert_eq!(arr.upper_bound(0).unwrap(), 0);
        arr.set(&[-2], Value::String("a".into())).unwrap();
        arr.set(&[0], Value::String("c".into())).unwrap();
        assert_eq!(arr.get(&[-2]).unwrap(), &Value::String("a".into()));
        assert_eq!(arr.get(&[0]).unwrap(), &Value::String("c".into()));
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
        let mut arr = ArrayValue::new_fixed(VBType::Integer, &dims).unwrap();
        for i in 1..=2 {
            for j in 1..=3 {
                arr.set(&[i, j], Value::Long((i * 10 + j) as i32)).unwrap();
            }
        }
        assert_eq!(arr.get(&[2, 3]).unwrap(), &Value::Long(23));
        assert_eq!(arr.len(), 6);
    }
}
