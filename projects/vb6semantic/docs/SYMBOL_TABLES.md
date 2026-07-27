# Symbol Table Design

## Overview

The symbol table is the core data structure for semantic analysis. It stores information about all symbols (variables, functions, classes, etc.) and provides efficient lookup capabilities.

## Requirements

1. **Fast Lookup**: O(1) symbol lookup by name
2. **Scope Support**: Handle hierarchical scopes
3. **Visibility**: Enforce visibility rules
4. **Source Tracking**: Track where symbols are defined
5. **Metadata**: Store additional symbol information
6. **Incremental Updates**: Support adding symbols dynamically

## Data Structures

### Symbol

Each symbol contains:

```rust
pub struct Symbol {
    pub name: String,              // Symbol name
    pub kind: SymbolKind,          // What kind of symbol
    pub type_info: TypeInfo,       // Type information
    pub visibility: Visibility,    // Public/Private/Friend
    pub location: SourceLocation,  // Where defined
    pub scope_id: usize,           // Containing scope
    pub attributes: HashMap<String, String>,  // Metadata
}
```

### Scope

Each scope contains:

```rust
pub struct Scope {
    pub id: usize,                       // Unique scope ID
    pub kind: ScopeKind,                 // Type of scope
    pub parent: Option<usize>,           // Parent scope
    pub children: Vec<usize>,            // Child scopes
    pub symbols: HashMap<String, Symbol>, // Symbols in this scope
    pub name: String,                    // Scope name (debugging)
}
```

### Symbol Table

The symbol table ties it all together:

```rust
pub struct SymbolTable {
    symbols: HashMap<usize, HashMap<String, Symbol>>,
    next_scope_id: usize,
}
```

## Scope Hierarchy

### VB6 Scope Levels

```
Project (implicit global scope)
  ├─ Module A (file scope)
  │   ├─ Function F1 (procedure scope)
  │   │   └─ For loop (block scope)
  │   └─ Sub S1 (procedure scope)
  │       └─ With block (block scope)
  ├─ Class B (file scope)
  │   ├─ Property Get P (procedure scope)
  │   └─ Function F2 (procedure scope)
  └─ Form C (file scope)
      ├─ Control (member)
      └─ Event Handler (procedure scope)
```

### Scope IDs

Each scope gets a unique ID:
- ID 0: Reserved for global scope
- ID 1+: Allocated sequentially

### Parent-Child Relationships

- Each scope (except global) has a parent
- Scopes can have multiple children
- Forms a tree structure

## Symbol Lookup

### Simple Lookup

To find a symbol by name:

1. Look in current scope
2. If not found, look in parent scope
3. Repeat until found or reach global scope
4. If still not found, symbol is undefined

```rust
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
    
    None
}
```

### Qualified Lookup

For qualified names like `Module.Function`:

1. Look up `Module` in current scope chain
2. Switch to `Module`'s scope
3. Look up `Function` in that scope
4. Don't walk up from there

### Visibility-Aware Lookup

Check visibility during lookup:

```rust
pub fn lookup_accessible(&self, name: &str) -> Option<&Symbol> {
    if let Some(symbol) = self.lookup(name) {
        if self.can_access(symbol) {
            return Some(symbol);
        }
    }
    None
}
```

## Symbol Addition

### Adding a Symbol

1. Check for duplicate in current scope
2. If duplicate, return error with both locations
3. If unique, add to current scope's symbol map

```rust
pub fn add_symbol(&mut self, symbol: Symbol) -> Result<()> {
    let scope = self.get_current_scope_mut()?;
    
    if let Some(existing) = scope.symbols.get(&symbol.name) {
        return Err(SemanticError::DuplicateSymbol {
            name: symbol.name,
            location: symbol.location,
            previous_location: existing.location,
        });
    }
    
    scope.symbols.insert(symbol.name.clone(), symbol);
    Ok(())
}
```

### Symbol Overloading

VB6 doesn't support function overloading, but does support:
- Property Get/Let/Set with same name
- Events with same name as procedures

These are handled by making the symbol kind part of the lookup key.

## Special Cases

### Module-Level Variables

```vb6
' Module1.bas
Public GlobalVar As Integer    ' Accessible everywhere
Private ModuleVar As Integer   ' Only in Module1
```

Stored in the module's scope with appropriate visibility.

### Class Members

```vb6
' Person.cls
Private m_name As String

Public Property Get Name() As String
    Name = m_name
End Property
```

Stored in the class scope. The property creates three potential symbols (Get, Let, Set).

### Procedure Parameters

```vb6
Function Calculate(x As Integer, y As Integer) As Integer
```

Parameters are symbols in the procedure's scope.

### Implicit Declarations

VB6 without `Option Explicit` allows undeclared variables:

```vb6
Sub Test()
    x = 5  ' x is implicitly Variant
End Sub
```

These are added to the symbol table as Variant when first encountered (with a warning).

### With Blocks

```vb6
With MyObject
    .Property1 = 5
    .Property2 = 10
End With
```

Create a block scope that tracks the With target (for IntelliSense).

### Enum Values

```vb6
Enum Colors
    Red = 0
    Green = 1
    Blue = 2
End Enum
```

Each enum value is a symbol in the enum's scope, and also visible in parent scope.

## Efficiency Considerations

### HashMap Performance

Using `HashMap<String, Symbol>` provides:
- O(1) average lookup
- O(1) insertion
- Efficient memory usage

### Scope Walking

Walking up the scope chain is O(depth):
- Typical depth: 2-4 levels
- Maximum depth: ~10 levels
- Acceptable performance

### Memory Usage

Per symbol overhead:
- Symbol struct: ~200 bytes
- HashMap entry: ~50 bytes
- Total per symbol: ~250 bytes

For a 10,000 line project (estimated 1,000 symbols):
- Memory usage: ~250 KB
- Very acceptable

### Incremental Updates

When code changes:
1. Identify affected scope
2. Clear symbols in that scope
3. Re-analyze that scope
4. Update parent pointers as needed

## Serialization

Symbol tables can be serialized for:
- Caching analysis results
- Inter-process communication
- Storage in databases

Using `serde`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Symbol { /* ... */ }
```

## Query API

### Common Queries

```rust
// Get all variables in current scope
pub fn get_variables_in_scope(&self, scope_id: usize) -> Vec<&Symbol> {
    self.get_symbols_by_kind_in_scope(scope_id, SymbolKind::Variable)
}

// Get all functions in entire project
pub fn get_all_functions(&self) -> Vec<&Symbol> {
    self.get_symbols_by_kind(SymbolKind::Function)
}

// Get symbol by kind and name
pub fn lookup_by_kind(&self, name: &str, kind: SymbolKind) -> Option<&Symbol> {
    self.lookup(name)
        .filter(|s| s.kind == kind)
}

// Get all symbols in a module
pub fn get_module_symbols(&self, module_name: &str) -> Vec<&Symbol> {
    // Find module scope, return all symbols
}
```

### IDE Support Queries

```rust
// Get symbols for code completion
pub fn get_completions(&self, prefix: &str) -> Vec<&Symbol> {
    // Return accessible symbols starting with prefix
}

// Get symbol at location (for hover info)
pub fn get_symbol_at(&self, file: &Path, line: usize, col: usize) -> Option<&Symbol> {
    // Find symbol at that location
}

// Get all references to a symbol
pub fn find_references(&self, symbol: &Symbol) -> Vec<SourceLocation> {
    // Return all uses of this symbol
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_addition() {
        let mut table = SymbolTable::new();
        let scope_id = table.create_scope();
        
        let symbol = Symbol { /* ... */ };
        assert!(table.add_symbol(scope_id, symbol).is_ok());
    }

    #[test]
    fn test_duplicate_detection() {
        let mut table = SymbolTable::new();
        let scope_id = table.create_scope();
        
        let sym1 = Symbol { name: "x", /* ... */ };
        let sym2 = Symbol { name: "x", /* ... */ };
        
        table.add_symbol(scope_id, sym1).unwrap();
        assert!(table.add_symbol(scope_id, sym2).is_err());
    }

    #[test]
    fn test_scope_walking() {
        let mut manager = ScopeManager::new();
        
        // Create nested scopes
        let global = manager.global_scope_id();
        manager.push_scope(ScopeKind::Procedure, "Test".into());
        
        // Add symbol to inner scope
        let symbol = Symbol { /* ... */ };
        manager.add_symbol(symbol).unwrap();
        
        // Should be able to look it up
        assert!(manager.lookup("symbol_name").is_some());
    }
}
```

## Future Enhancements

1. **Symbol Indexing**: Add B-tree index for range queries
2. **Weak References**: Use weak references to avoid cycles
3. **Concurrent Access**: Add RwLock for multi-threaded access
4. **Persistence**: Save/load from disk
5. **Versioning**: Track symbol table versions for incremental updates
6. **Statistics**: Collect usage statistics for optimization

## References

- Compiler Design textbooks (Dragon Book, Tiger Book)
- LLVM symbol table design
- Roslyn (C# compiler) symbol model
- LSP (Language Server Protocol) specifications
