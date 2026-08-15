//! This module defines the Scope and ScopeManager structures for managing
//! lexical scopes and symbol resolution in VB6 code.
//!
//! The Scope struct represents a single scope, which can be of various kinds (global, class, procedure, etc.)
//! and contains symbols defined within that scope. The ScopeManager struct manages the hierarchy of scopes,
//! allowing for lookups and scope transitions during analysis. It provides methods to push and pop scopes,
//! add symbols, and perform lookups with proper handling of scope hierarchy and symbol visibility.
//!
//! # Examples
//!
//! ```rust
//! use vb6semantic::{ ScopeManager, ScopeKind, Symbol, Visibility, SourceLocation, TypeInfo, SymbolKind };
//!
//! use std::collections::HashMap;
//!
//! pub fn main() {
//!     let mut manager = ScopeManager::new();
//!     let global_scope_id = manager.global_scope_id();
//!     let class_scope_id = manager.push_scope(ScopeKind::Class, "MyClass".to_string());
//!     let myVar_symbol = Symbol {
//!         name: "myVar".to_string(),
//!         kind: SymbolKind::Variable,
//!         type_info: TypeInfo::integer(),
//!         visibility: Visibility::Private,
//!         location: SourceLocation {
//!             file: "MyClass.cls".to_string(),
//!             line: 1,
//!             column: 1
//!         },
//!         scope_id: class_scope_id,
//!         attributes: HashMap::new()
//!     };
//!     manager.add_symbol(myVar_symbol).expect("Failed to add symbol 'MyVar' to class scope");
//!     manager.pop_scope().expect("Failed to pop class scope");
//! }
//! ```

use crate::error::Result;
use crate::symbols::{Symbol, Visibility};
use std::collections::HashMap;

/// Represents a lexical scope in VB6 code
#[derive(Debug, Clone)]
pub struct Scope {
    /// Unique identifier for this scope
    pub id: usize,

    /// Kind of scope
    pub kind: ScopeKind,

    /// Parent scope ID (None for global scope)
    pub parent: Option<usize>,

    /// Child scope IDs
    pub children: Vec<usize>,

    /// Symbols defined directly in this scope
    pub symbols: HashMap<String, Symbol>,

    /// Name of the scope (for debugging)
    pub name: String,
}

/// Manages the hierarchy of scopes and symbol resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    /// Global/Module level scope
    Global,

    /// Class scope
    Class,

    /// Function/Sub scope
    Procedure,

    /// Property scope
    Property,

    /// Block scope (With, For, etc.)
    Block,

    /// Type definition scope
    Type,

    /// Enum scope
    Enum,

    /// Scope populated by a resolved project reference (COM library, sub-project,
    /// or third-party supplied symbol set). Symbols in these scopes are visible
    /// project-wide, after module globals, during name resolution.
    Reference,
}

/// Manages the scope hierarchy and symbol resolution
#[derive(Debug, Clone)]
pub struct ScopeManager {
    /// All scopes indexed by ID
    scopes: HashMap<usize, Scope>,

    /// Current active scope
    current_scope: usize,

    /// Global scope ID
    global_scope: usize,

    /// Next scope ID to allocate
    next_scope_id: usize,

    /// Module-level scopes (modules, classes, and forms) in the order they were
    /// created. This is the order names resolve across modules during the
    /// project-wide fallback: for projects analyzed through `analyze_project`
    /// it matches the `.vbp` file entry order.
    module_scopes: Vec<usize>,

    /// Reference-library scopes in the order they were created (the order of the
    /// project's `Reference=` lines). Searched after `module_scopes` so that
    /// project symbols shadow referenced-library symbols.
    reference_scopes: Vec<usize>,
}

impl ScopeManager {
    /// Create a new scope manager with an initial global scope
    pub fn new() -> Self {
        let mut manager = Self {
            scopes: HashMap::new(),
            current_scope: 0,
            global_scope: 0,
            next_scope_id: 1,
            module_scopes: Vec::new(),
            reference_scopes: Vec::new(),
        };

        // Create global scope
        let global = Scope {
            id: 0,
            kind: ScopeKind::Global,
            parent: None,
            children: Vec::new(),
            symbols: HashMap::new(),
            name: "global".to_string(),
        };
        manager.scopes.insert(0, global);

        manager
    }

    /// Create a new scope as a child of the current scope
    pub fn push_scope(&mut self, kind: ScopeKind, name: String) -> usize {
        self.create_scope(kind, name)
    }

    /// Create a new module-level scope (module, class, or form) as a child of the
    /// current scope, recording it in the project-wide module search order.
    pub fn push_module_scope(&mut self, kind: ScopeKind, name: String) -> usize {
        let scope_id = self.create_scope(kind, name);
        self.module_scopes.push(scope_id);
        scope_id
    }

    /// Create a new reference-library scope as a child of the current scope,
    /// recording it in the project-wide reference search order.
    pub fn push_reference_scope(&mut self, name: String) -> usize {
        let scope_id = self.create_scope(ScopeKind::Reference, name);
        self.reference_scopes.push(scope_id);
        scope_id
    }

    /// Allocate a new scope as a child of the current scope without recording it
    /// in the module or reference search order.
    fn create_scope(&mut self, kind: ScopeKind, name: String) -> usize {
        let scope_id = self.next_scope_id;
        self.next_scope_id += 1;

        let scope = Scope {
            id: scope_id,
            kind,
            parent: Some(self.current_scope),
            children: Vec::new(),
            symbols: HashMap::new(),
            name,
        };

        // Add as child to current scope
        if let Some(current) = self.scopes.get_mut(&self.current_scope) {
            current.children.push(scope_id);
        }

        self.scopes.insert(scope_id, scope);
        self.current_scope = scope_id;

        scope_id
    }

    /// Pop the current scope, returning to parent
    pub fn pop_scope(&mut self) -> Result<()> {
        let current = self.scopes.get(&self.current_scope).ok_or_else(|| {
            crate::error::SemanticError::InvalidScope {
                message: format!("Current scope {} not found", self.current_scope),
            }
        })?;

        if let Some(parent) = current.parent {
            self.current_scope = parent;
            Ok(())
        } else {
            Err(crate::error::SemanticError::InvalidScope {
                message: "Cannot pop global scope".to_string(),
            })
        }
    }

    /// Get the current scope ID
    pub fn current_scope_id(&self) -> usize {
        self.current_scope
    }

    /// Directly set the current scope.
    ///
    /// Used when resolving references after all declarations are registered,
    /// where walking a CST a second time must restore each procedure's scope
    /// without re-creating it.
    pub fn set_current_scope(&mut self, scope_id: usize) {
        self.current_scope = scope_id;
    }

    /// Get a scope by ID
    pub fn get_scope(&self, scope_id: usize) -> Option<&Scope> {
        self.scopes.get(&scope_id)
    }

    /// Get a mutable reference to a scope
    pub fn get_scope_mut(&mut self, scope_id: usize) -> Option<&mut Scope> {
        self.scopes.get_mut(&scope_id)
    }

    /// Add a symbol to the current scope
    pub fn add_symbol(&mut self, symbol: Symbol) -> Result<()> {
        let scope = self.scopes.get_mut(&self.current_scope).ok_or_else(|| {
            crate::error::SemanticError::InvalidScope {
                message: format!("Current scope {} not found", self.current_scope),
            }
        })?;

        if let Some(existing) = scope.symbols.get(&symbol.name) {
            return Err(crate::error::SemanticError::DuplicateSymbol {
                name: symbol.name.clone(),
                location: symbol.location.clone(),
                previous_location: existing.location.clone(),
            });
        }

        scope.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    /// Lookup a symbol starting from current scope and walking up
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut current = Some(self.current_scope);

        while let Some(scope_id) = current {
            if let Some(scope) = self.scopes.get(&scope_id) {
                if let Some(symbol) = scope.symbols.get(name) {
                    return Some(symbol);
                }
                current = scope.parent;
            } else {
                break;
            }
        }

        // VB6 exposes Public symbols from every module and every referenced
        // library project-wide. If the lexical chain missed, fall back to the
        // module scopes first and then the reference-library scopes. Modules are
        // searched in creation order (the `.vbp` file order for projects), so
        // the module listed first wins an ambiguous name, and reference scopes
        // are searched in `Reference=` line order. Private symbols are only
        // visible within their own module, so the cross-module pass skips them.
        for module_scope_id in &self.module_scopes {
            if let Some(symbol) = self
                .scopes
                .get(module_scope_id)
                .and_then(|scope| scope.symbols.get(name))
                && self.can_access(symbol)
            {
                return Some(symbol);
            }
        }
        for reference_scope_id in &self.reference_scopes {
            if let Some(symbol) = self
                .scopes
                .get(reference_scope_id)
                .and_then(|scope| scope.symbols.get(name))
                && self.can_access(symbol)
            {
                return Some(symbol);
            }
        }
        None
    }

    /// Lookup a symbol in a specific scope (no parent walk)
    pub fn lookup_in_scope(&self, scope_id: usize, name: &str) -> Option<&Symbol> {
        self.scopes.get(&scope_id)?.symbols.get(name)
    }

    /// Check if we can access a symbol from the current scope
    pub fn can_access(&self, symbol: &Symbol) -> bool {
        match symbol.visibility {
            Visibility::Public | Visibility::Global => true,
            Visibility::Friend => {
                // Friend is accessible within the same project
                // TODO: implement project-level checks
                true
            }
            Visibility::Private => {
                // Private is only accessible within the same module/class
                self.is_in_same_module(symbol.scope_id)
            }
        }
    }

    /// Check if the current scope is in the same module as the given scope
    fn is_in_same_module(&self, scope_id: usize) -> bool {
        // Walk up to find the module-level scope (Global or Class)
        let current_module = self.find_module_scope(self.current_scope);
        let other_module = self.find_module_scope(scope_id);

        current_module == other_module
    }

    /// Find the module-level scope for a given scope
    fn find_module_scope(&self, mut scope_id: usize) -> usize {
        while let Some(scope) = self.scopes.get(&scope_id) {
            match scope.kind {
                ScopeKind::Global | ScopeKind::Class => return scope_id,
                _ => {
                    if let Some(parent) = scope.parent {
                        scope_id = parent;
                    } else {
                        return scope_id;
                    }
                }
            }
        }
        scope_id
    }

    /// Get the global scope ID
    pub fn global_scope_id(&self) -> usize {
        self.global_scope
    }

    /// Get all scopes of a specific kind
    pub fn get_scopes_by_kind(&self, kind: ScopeKind) -> Vec<&Scope> {
        self.scopes.values().filter(|s| s.kind == kind).collect()
    }

    /// Get all scopes in the manager
    pub fn all_scopes(&self) -> Vec<&Scope> {
        self.scopes.values().collect()
    }
}

impl Default for ScopeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SourceLocation;
    use crate::symbols::SymbolKind;
    use crate::types::{TypeInfo, VBType};

    fn symbol(name: &str, visibility: Visibility, scope_id: usize) -> Symbol {
        Symbol {
            name: name.to_string(),
            kind: SymbolKind::Variable,
            type_info: TypeInfo::new(VBType::Long),
            visibility,
            location: SourceLocation {
                file: "test.bas".to_string(),
                line: 1,
                column: 1,
            },
            scope_id,
            attributes: HashMap::new(),
        }
    }

    #[test]
    fn lookup_skips_private_symbols_from_other_modules() {
        let mut manager = ScopeManager::new();
        let module_a = manager.push_module_scope(ScopeKind::Global, "ModuleA".to_string());
        manager
            .add_symbol(symbol("Secret", Visibility::Private, module_a))
            .unwrap();
        manager
            .add_symbol(symbol("Visible", Visibility::Public, module_a))
            .unwrap();
        manager.pop_scope().unwrap();

        let _module_b = manager.push_module_scope(ScopeKind::Global, "ModuleB".to_string());

        // From within ModuleB, ModuleA's Private symbol is not visible...
        assert!(manager.lookup("Secret").is_none());
        // ...but its Public symbol is.
        assert!(manager.lookup("Visible").is_some());
    }

    #[test]
    fn lookup_finds_private_symbol_in_same_module() {
        let mut manager = ScopeManager::new();
        let module_a = manager.push_module_scope(ScopeKind::Global, "ModuleA".to_string());
        manager
            .add_symbol(symbol("Secret", Visibility::Private, module_a))
            .unwrap();

        assert!(manager.lookup("Secret").is_some());
    }

    #[test]
    fn lookup_searches_module_scopes_in_push_order() {
        let mut manager = ScopeManager::new();
        let first = manager.push_module_scope(ScopeKind::Global, "First".to_string());
        manager
            .add_symbol(symbol("Name", Visibility::Public, first))
            .unwrap();
        manager.pop_scope().unwrap();

        let second = manager.push_module_scope(ScopeKind::Global, "Second".to_string());
        manager
            .add_symbol(symbol("Name", Visibility::Public, second))
            .unwrap();
        manager.pop_scope().unwrap();

        // The scope pushed first wins the ambiguous name.
        let found = manager.lookup("Name").expect("Symbol should resolve");
        assert_eq!(found.scope_id, first);
    }

    #[test]
    fn lookup_prefers_modules_over_references() {
        let mut manager = ScopeManager::new();
        let module = manager.push_module_scope(ScopeKind::Global, "Module".to_string());
        manager
            .add_symbol(symbol("Name", Visibility::Public, module))
            .unwrap();
        manager.pop_scope().unwrap();

        let reference = manager.push_reference_scope("OLE Automation".to_string());
        manager
            .add_symbol(symbol("Name", Visibility::Public, reference))
            .unwrap();
        manager.pop_scope().unwrap();

        let found = manager.lookup("Name").expect("Symbol should resolve");
        assert_eq!(found.scope_id, module);
    }

    #[test]
    fn all_scopes_returns_every_scope() {
        let mut manager = ScopeManager::new();
        let first = manager.push_scope(ScopeKind::Class, "First".to_string());
        let second = manager.push_scope(ScopeKind::Procedure, "Second".to_string());

        let mut ids: Vec<usize> = manager.all_scopes().iter().map(|scope| scope.id).collect();
        ids.sort_unstable();

        assert_eq!(ids, vec![0, first, second]);
    }

    #[test]
    fn lookup_searches_reference_scopes_in_push_order() {
        let mut manager = ScopeManager::new();
        let first = manager.push_reference_scope("first".to_string());
        manager
            .add_symbol(symbol("Name", Visibility::Public, first))
            .unwrap();
        manager.pop_scope().unwrap();

        let second = manager.push_reference_scope("second".to_string());
        manager
            .add_symbol(symbol("Name", Visibility::Public, second))
            .unwrap();
        manager.pop_scope().unwrap();

        let found = manager.lookup("Name").expect("Symbol should resolve");
        assert_eq!(found.scope_id, first);
    }
}
