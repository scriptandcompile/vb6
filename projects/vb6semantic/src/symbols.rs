//! Defines the `Symbol` struct and `SymbolTable` for managing symbols in the VB6 code
//! This module provides the core data structures for representing symbols
//! (variables, functions, classes, etc.) and a symbol table for managing them within
//! different scopes. The `Symbol` struct includes information about the symbol's name,
//! kind, type, visibility, location, and scope. The `SymbolTable` struct provides
//! methods to create scopes, add symbols, and perform lookups.
//!
//! # Examples
//!
//! ```rust
//! use vb6semantic::{Symbol, Visibility, SymbolKind, SourceLocation, TypeInfo};
//!
//! use std::collections::HashMap;
//!
//! let symbol = Symbol {
//!     name: "myVariable".to_string(),
//!     kind: SymbolKind::Variable,
//!     type_info: TypeInfo::integer(),
//!     visibility: Visibility::Private,
//!     location: SourceLocation {
//!         file: "Module1.bas".to_string(),
//!         line: 10,
//!         column: 5,
//!     },
//!     scope_id: 1,
//!     attributes: HashMap::new(),
//! };
//! println!("Defined symbol: {:?}", symbol);
//! ```

use crate::error::{Result, SourceLocation};
use crate::types::TypeInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a symbol in the VB6 code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    /// Name of the symbol
    pub name: String,

    /// Kind of symbol (variable, function, class, etc.)
    pub kind: SymbolKind,

    /// Type information
    pub type_info: TypeInfo,

    /// Visibility (Public, Private, Friend)
    pub visibility: Visibility,

    /// Location where defined
    pub location: SourceLocation,

    /// Scope ID where this symbol is defined
    pub scope_id: usize,

    /// Additional attributes
    pub attributes: HashMap<String, String>,
}

/// Represents the kind of symbol (variable, function, class, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolKind {
    /// Variable declaration
    Variable,

    /// Constant declaration
    Constant,

    /// Sub procedure
    SubProcedure,

    /// Function
    Function,

    /// Property Get
    PropertyGet,

    /// Property Let
    PropertyLet,

    /// Property Set
    PropertySet,

    /// Class
    Class,

    /// Module
    Module,

    /// Form
    Form,

    /// Control on a form
    Control,

    /// Enum
    Enum,

    /// Enum member
    EnumMember,

    /// Type (User-defined type)
    UserType,

    /// Type member
    TypeMember,

    /// Parameter
    Parameter,

    /// Label (for GoTo)
    Label,
}

/// Represents the symbol table for managing symbols in scopes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default, Deserialize)]
pub enum Visibility {
    /// Public symbol, accessible from anywhere
    #[default]
    Public,
    /// Private symbol, accessible only within the defining module/class
    Private,
    /// Friend symbol, accessible within the same project
    Friend,
    /// Global symbol, accessible from anywhere (legacy VB6 global scope)
    Global,
}

/// Symbol table for managing symbols in scopes
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// Symbols indexed by name and scope
    symbols: HashMap<usize, HashMap<String, Symbol>>,

    /// Next scope ID
    next_scope_id: usize,
}

impl SymbolTable {
    /// Create a new symbol table
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            next_scope_id: 0,
        }
    }

    /// Create a new scope and return its ID
    pub fn create_scope(&mut self) -> usize {
        let scope_id = self.next_scope_id;
        self.next_scope_id += 1;
        self.symbols.insert(scope_id, HashMap::new());
        scope_id
    }

    /// Add a symbol to a scope
    pub fn add_symbol(&mut self, scope_id: usize, symbol: Symbol) -> Result<()> {
        let scope = self.symbols.get_mut(&scope_id).ok_or_else(|| {
            crate::error::SemanticError::InvalidScope {
                message: format!("Scope {} does not exist", scope_id),
            }
        })?;

        if let Some(existing) = scope.get(&symbol.name) {
            return Err(crate::error::SemanticError::DuplicateSymbol {
                name: symbol.name.clone(),
                location: symbol.location.clone(),
                previous_location: existing.location.clone(),
            });
        }

        scope.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    /// Look up a symbol in a specific scope
    pub fn lookup_in_scope(&self, scope_id: usize, name: &str) -> Option<&Symbol> {
        self.symbols.get(&scope_id)?.get(name)
    }

    /// Get all symbols in a scope
    pub fn get_scope_symbols(&self, scope_id: usize) -> Option<&HashMap<String, Symbol>> {
        self.symbols.get(&scope_id)
    }

    /// Get all symbols of a specific kind across all scopes
    pub fn get_symbols_by_kind(&self, kind: SymbolKind) -> Vec<&Symbol> {
        self.symbols
            .values()
            .flat_map(|scope| scope.values())
            .filter(|s| s.kind == kind)
            .collect()
    }

    /// Check if a symbol exists in any scope
    pub fn symbol_exists(&self, name: &str) -> bool {
        self.symbols.values().any(|scope| scope.contains_key(name))
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
