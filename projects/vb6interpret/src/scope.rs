//! Variable storage for interpreter scopes.
//!
//! VB6 identifiers are case-insensitive, so names are normalized to lowercase
//! before being stored. A [`Scope`] maps a normalized name to a runtime
//! [`Value`].

use std::collections::HashMap;

use vb6core::types::VBType;
use vb6runtime::Value;

/// A collection of variables (a single procedure's locals, or the module's
/// globals).
#[derive(Debug, Clone, Default)]
pub struct Scope {
    vars: HashMap<String, Value>,
}

/// Normalize an identifier for case-insensitive lookup.
pub(crate) fn normalize(name: &str) -> String {
    name.to_lowercase()
}

impl Scope {
    /// Create an empty scope.
    pub fn new() -> Self {
        Self::default()
    }

    /// Declare a variable with an initial value.
    pub fn declare(&mut self, name: &str, value: Value) {
        self.vars.insert(normalize(name), value);
    }

    /// Declare a variable with the default value for its static type.
    pub fn declare_with_type(&mut self, name: &str, ty: &VBType) {
        let value = Value::default_for_type(ty);
        self.vars.insert(normalize(name), value);
    }

    /// Whether a variable with this name exists.
    pub fn contains(&self, name: &str) -> bool {
        self.vars.contains_key(&normalize(name))
    }

    /// Look up a variable's value.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.vars.get(&normalize(name))
    }

    /// Write a variable's value, if it is already declared.
    ///
    /// Returns `false` when the variable does not exist in this scope.
    pub fn set(&mut self, name: &str, value: Value) -> bool {
        match self.vars.get_mut(&normalize(name)) {
            Some(slot) => {
                *slot = value;
                true
            }
            None => false,
        }
    }

    /// Iterate over `(name, value)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.vars.iter().map(|(key, value)| (key.as_str(), value))
    }

    /// The number of variables in this scope.
    pub fn len(&self) -> usize {
        self.vars.len()
    }

    /// Whether this scope holds no variables.
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }
}
