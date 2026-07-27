# VB6 Semantic Analysis & Symbol Table Design

## Executive Summary

This document proposes a semantic analysis module for vb6parse that builds upon the existing CST to provide symbol resolution, type checking, and scope analysis. The design extends the seven-layer architecture with an eighth "Semantic Layer" while maintaining the library's philosophy of offline analysis, partial success patterns, and comprehensive error reporting.

## Table of Contents

1. [Background & Motivation](#background--motivation)
2. [Architecture Overview](#architecture-overview)
3. [Core Components](#core-components)
4. [Symbol Table Design](#symbol-table-design)
5. [Semantic Analysis Pipeline](#semantic-analysis-pipeline)
6. [VB6-Specific Considerations](#vb6-specific-considerations)
7. [Integration with Existing Architecture](#integration-with-existing-architecture)
8. [Error Handling & Reporting](#error-handling--reporting)
9. [Performance Considerations](#performance-considerations)
10. [Testing Strategy](#testing-strategy)
11. [Technical Costs & Benefits](#technical-costs--benefits)
12. [Alternative Crate Choices](#alternative-crate-choices)
13. [Implementation Roadmap](#implementation-roadmap)
14. [Future Extensions](#future-extensions)

---

## Background & Motivation

### Current State

VB6Parse currently provides:
- Full lexing and tokenization
- CST (Concrete Syntax Tree) construction preserving all tokens
- File format parsers for `.vbp`, `.cls`, `.bas`, `.frm`, `.frx` files
- Language-level objects (Controls, Colors, Properties)
- Comprehensive VB6 library function/statement definitions

### What's Missing

- **Symbol resolution**: Cannot answer "what does this identifier refer to?"
- **Type inference**: No understanding of variable/expression types
- **Scope analysis**: Cannot validate if symbols are accessible in context
- **Cross-file references**: Cannot resolve module imports or form controls
- **Semantic validation**: Cannot detect type mismatches, undeclared variables, etc.

### Use Cases Enabled

1. **Static analysis tools**: Detect unused variables, type errors, missing declarations
2. **Code navigation**: Go-to-definition, find-references, rename refactoring
3. **Documentation generation**: Generate API docs with resolved types
4. **Migration tools**: Convert VB6 to modern languages with type information
5. **Linting & code quality**: Enforce naming conventions, detect anti-patterns
6. **IDE features**: Autocomplete, hover information, signature help

---

## Architecture Overview

### Extended Pipeline (Eight Layers)

```
Bytes → SourceFile → SourceStream → TokenStream → CST → Object Layer → Semantic Layer → Query API
       (Windows-1252) (Characters)   (Tokens)    (Tree) (Structured)   (Symbols)      (Analysis)
```

**New Layer 7: Semantic Layer**
- Builds symbol tables from CST + Object Layer
- Resolves references and performs type checking
- Provides query interface for semantic information

**New Layer 8: Query API**
- High-level interface for semantic queries
- Find-references, go-to-definition, type-at-position
- Diagnostic generation for semantic errors

### Module Structure

```
src/
  semantic/
    mod.rs                    # Public API and layer integration
    symbol_table.rs           # Core symbol table implementation
    scope.rs                  # Scope hierarchy and resolution
    types.rs                  # VB6 type system representation
    resolver.rs               # Symbol resolution logic
    analyzer.rs               # Semantic analysis passes
    diagnostics.rs            # Semantic error definitions
    query.rs                  # Query API for IDE features
    
    builders/                 # Symbol table construction
      project_builder.rs      # Build symbols from ProjectFile
      module_builder.rs       # Build symbols from ModuleFile
      class_builder.rs        # Build symbols from ClassFile
      form_builder.rs         # Build symbols from FormFile
    
    visitors/                 # CST traversal for analysis
      declaration_visitor.rs  # Find declarations in CST
      reference_visitor.rs    # Find references in CST
      type_visitor.rs         # Extract type information
```

---

## Core Components

### 1. Symbol Table (`symbol_table.rs`)

The core data structure organizing all symbols in a VB6 project.

```rust
/// Root symbol table for a VB6 project
pub struct SymbolTable {
    /// All scopes indexed by unique ID
    scopes: Arena<Scope>,
    
    /// Root project scope
    project_scope: ScopeId,
    
    /// Map from file path to root scope of that file
    file_scopes: HashMap<PathBuf, ScopeId>,
    
    /// All symbols indexed by unique ID
    symbols: Arena<Symbol>,
    
    /// Fast lookup: normalized name -> symbol IDs in each scope
    /// Uses case-insensitive comparison for VB6
    name_index: HashMap<ScopeId, HashMap<SmolStr, Vec<SymbolId>>>,
    
    /// References: symbol ID -> locations where used
    references: HashMap<SymbolId, Vec<SourceLocation>>,
    
    /// Reverse map: location -> symbol at that location
    location_index: IntervalTree<SourceLocation, SymbolId>,
}

impl SymbolTable {
    /// Create empty symbol table for a project
    pub fn new() -> Self;
    
    /// Add a scope to the table
    pub fn add_scope(&mut self, parent: Option<ScopeId>, kind: ScopeKind) -> ScopeId;
    
    /// Add a symbol to a scope
    pub fn add_symbol(&mut self, scope: ScopeId, symbol: Symbol) -> SymbolId;
    
    /// Resolve a name in a given scope (follows scope chain)
    pub fn resolve(&self, name: &str, scope: ScopeId) -> Option<SymbolId>;
    
    /// Find all references to a symbol
    pub fn references(&self, symbol: SymbolId) -> &[SourceLocation];
    
    /// Find symbol at a specific location
    pub fn symbol_at(&self, location: SourceLocation) -> Option<SymbolId>;
    
    /// Get symbol information
    pub fn symbol(&self, id: SymbolId) -> &Symbol;
    
    /// Get scope information
    pub fn scope(&self, id: ScopeId) -> &Scope;
}
```

**Design Rationale:**
- **Arena allocation**: Uses typed-arena or generational-arena for stable IDs without lifetimes
- **Hierarchical scopes**: Mirrors VB6's project → module → procedure scope structure
- **Case-insensitive index**: Uses normalized keys (lowercase) for VB6 semantics
- **Interval tree**: Efficient location-to-symbol queries for hover/definition lookup
- **Separated concerns**: Symbol storage, indexing, and reference tracking are distinct

### 2. Symbol Representation (`symbol_table.rs`)

```rust
/// Unique identifier for a symbol
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SymbolId(u32);

/// A named entity in VB6 code
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Symbol identifier (as written in source)
    pub name: SmolStr,
    
    /// What kind of symbol (variable, function, type, etc.)
    pub kind: SymbolKind,
    
    /// Resolved type of this symbol
    pub ty: Option<TypeId>,
    
    /// Where the symbol is declared
    pub declaration: SourceLocation,
    
    /// Visibility/accessibility
    pub visibility: Visibility,
    
    /// Modifiers (Static, Const, WithEvents, etc.)
    pub modifiers: SymbolModifiers,
    
    /// Link to containing scope
    pub scope: ScopeId,
    
    /// For functions/subs: parameter symbols
    pub parameters: Vec<SymbolId>,
    
    /// For UDTs/Enums: member symbols
    pub members: Vec<SymbolId>,
    
    /// Documentation comment if present
    pub doc_comment: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    // Variables
    Variable,           // Dim, ReDim
    Constant,           // Const
    
    // Procedures
    Function,           // Function
    Sub,                // Sub
    Property(PropertyKind), // Property Get/Let/Set
    
    // Types
    Class,              // Class module
    Form,               // Form
    UserControl,        // User control (.ctl)
    UserDefinedType,    // Type...End Type
    Enum,               // Enum...End Enum
    
    // Members
    EnumMember,         // Member of Enum
    TypeMember,         // Member of UDT
    Event,              // Event declaration
    
    // Special
    Module,             // Standard module (.bas)
    Project,            // Project root
    Parameter,          // Function/Sub parameter
    Label,              // Line label for GoTo
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PropertyKind {
    Get,
    Let,
    Set,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Visibility {
    Public,      // Public
    Private,     // Private
    Friend,      // Friend (visible in project, not to clients)
    Global,      // Global (BAS module only)
}

bitflags::bitflags! {
    pub struct SymbolModifiers: u16 {
        const STATIC = 1 << 0;      // Static (preserves value)
        const CONST = 1 << 1;       // Const (immutable)
        const WITH_EVENTS = 1 << 2; // WithEvents (can handle events)
        const OPTIONAL = 1 << 3;    // Optional (parameter)
        const BYVAL = 1 << 4;       // ByVal (parameter)
        const BYREF = 1 << 5;       // ByRef (parameter, default)
        const PARAM_ARRAY = 1 << 6; // ParamArray (varargs)
    }
}

/// Source location for symbol declarations/references
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    /// File path (relative to project root)
    pub file: PathBuf,
    
    /// Byte offset range in file
    pub range: Range<usize>,
    
    /// Line:column start (1-based, for display)
    pub start: Position,
    
    /// Line:column end (1-based, for display)
    pub end: Position,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}
```

### 3. Scope Hierarchy (`scope.rs`)

```rust
/// Unique identifier for a scope
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ScopeId(u32);

/// A lexical scope in VB6 code
#[derive(Debug, Clone)]
pub struct Scope {
    /// What kind of scope
    pub kind: ScopeKind,
    
    /// Parent scope (None for project root)
    pub parent: Option<ScopeId>,
    
    /// Child scopes (e.g., procedures in a module)
    pub children: Vec<ScopeId>,
    
    /// Symbols directly declared in this scope
    pub symbols: Vec<SymbolId>,
    
    /// Associated file/module name
    pub name: Option<SmolStr>,
    
    /// Source location of scope (if applicable)
    pub location: Option<SourceLocation>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScopeKind {
    /// Root project scope
    Project,
    
    /// Standard module (.bas) - module-level scope
    Module,
    
    /// Class module (.cls) - class-level scope
    Class,
    
    /// Form (.frm) - form-level scope
    Form,
    
    /// Function/Sub/Property - procedure-level scope
    Procedure,
    
    /// Type...End Type - UDT scope
    UserDefinedType,
    
    /// Enum...End Enum - enum scope
    Enum,
    
    /// With...End With - auxiliary scope (doesn't introduce symbols)
    With,
    
    /// For...Next, Do...Loop - block scope (for loop variables)
    Block,
}

impl Scope {
    /// Check if this scope can access symbols from another scope
    pub fn can_access(&self, other: &Scope, symbol_visibility: Visibility) -> bool;
    
    /// Get the chain of scopes from this to root
    pub fn scope_chain<'a>(&self, table: &'a SymbolTable) -> Vec<&'a Scope>;
}
```

**VB6 Scope Rules:**
1. **Module-level scope**: Variables/procedures declared at module/class/form level
2. **Procedure-level scope**: Parameters and local variables in Function/Sub
3. **Block scope**: Limited to For loop variables (`For i = 1 To 10`)
4. **Implicit hierarchy**: Procedure → Module → Project
5. **Cross-module access**: Public symbols in other modules accessible via qualified names
6. **Form controls**: Form controls create module-level symbols in that form

### 4. Type System (`types.rs`)

```rust
/// Unique identifier for a type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TypeId(u32);

/// VB6 type system representation
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // Primitive types
    Byte,
    Boolean,
    Integer,
    Long,
    Single,
    Double,
    Currency,
    Decimal,
    Date,
    String(StringKind),
    Variant,
    Object(Option<SmolStr>), // Object or specific class
    
    // Compound types
    Array(Box<Type>, ArrayDimensions),
    UserDefinedType(SymbolId),
    Enum(SymbolId),
    
    // Special
    Any,        // For APIs and late binding
    Null,       // Special Variant value
    Empty,      // Uninitialized Variant
    Nothing,    // Null object reference
    Error,      // Type error placeholder
    Unknown,    // Cannot infer type
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StringKind {
    /// Variable-length string
    Variable,
    
    /// Fixed-length string (String * 10)
    Fixed(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayDimensions {
    /// Dynamic array (can be ReDim'd)
    Dynamic,
    
    /// Fixed dimensions: Array(1 To 10, 1 To 20)
    Fixed(Vec<ArrayBound>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayBound {
    pub lower: i32,
    pub upper: i32,
}

/// Type arena for managing type instances
pub struct TypeArena {
    types: Arena<Type>,
    
    /// Interning for common types
    primitives: HashMap<Type, TypeId>,
}

impl TypeArena {
    /// Intern a type (reuse existing if identical)
    pub fn intern(&mut self, ty: Type) -> TypeId;
    
    /// Get type by ID
    pub fn get(&self, id: TypeId) -> &Type;
    
    /// Check if type A is assignable to type B
    pub fn is_assignable(&self, from: TypeId, to: TypeId) -> bool;
    
    /// Get common type for implicit conversions
    pub fn common_type(&self, a: TypeId, b: TypeId) -> Option<TypeId>;
}
```

**VB6 Type System Characteristics:**
- **Variant is universal**: Can hold any value (with runtime type)
- **Implicit conversions**: Numeric types convert freely, strings convert with CStr/Val
- **Object hierarchy**: Minimal - only Object and specific classes, no inheritance
- **DefType directives**: `DefInt A-Z` sets default type for undeclared variables
- **Type suffixes**: `%` = Integer, `&` = Long, `!` = Single, `#` = Double, `@` = Currency, `$` = String
- **As New**: Creates objects on first use (lazy initialization)
- **Optional parameters**: Can be `Variant` with `IsMissing()` check

---

## Semantic Analysis Pipeline

### Analysis Phases

```rust
/// Main entry point for semantic analysis
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    type_arena: TypeArena,
    diagnostics: Vec<SemanticDiagnostic>,
}

impl SemanticAnalyzer {
    /// Create analyzer for a project
    pub fn new() -> Self;
    
    /// Phase 1: Build symbol table from project structure
    pub fn build_symbols(&mut self, project: &ProjectFile) -> AnalysisResult;
    
    /// Phase 2: Resolve symbol references
    pub fn resolve_references(&mut self) -> AnalysisResult;
    
    /// Phase 3: Infer and check types
    pub fn check_types(&mut self) -> AnalysisResult;
    
    /// Phase 4: Validate semantic rules
    pub fn validate(&mut self) -> AnalysisResult;
    
    /// Run all phases in sequence
    pub fn analyze(&mut self, project: &ProjectFile) -> AnalysisResult;
    
    /// Get resulting symbol table
    pub fn symbol_table(&self) -> &SymbolTable;
    
    /// Get all diagnostics
    pub fn diagnostics(&self) -> &[SemanticDiagnostic];
}

/// Result of analysis (mirrors ParseResult pattern)
pub struct AnalysisResult {
    /// Analysis succeeded (possibly with warnings)
    success: bool,
    
    /// Diagnostic messages (errors and warnings)
    diagnostics: Vec<SemanticDiagnostic>,
}

impl AnalysisResult {
    pub fn success(&self) -> bool;
    pub fn diagnostics(&self) -> &[SemanticDiagnostic];
    pub fn into_diagnostics(self) -> Vec<SemanticDiagnostic>;
    pub fn has_errors(&self) -> bool;
}
```

### Phase 1: Symbol Collection

**Goal**: Build complete symbol table from CST without resolving references

**Process**:
1. Create project root scope
2. For each file in project:
   - Create module/class/form scope
   - Walk CST to find declarations (Dim, Function, Sub, Type, Enum, Const, etc.)
   - Create Symbol entries with partial information
   - Add to appropriate scope
3. For forms, collect control symbols from form header
4. Build scope hierarchy

**Output**: Symbol table with all declarations, unresolved types

### Phase 2: Reference Resolution

**Goal**: Link identifier uses to their declarations

**Process**:
1. For each file, walk CST to find identifier references
2. For each reference:
   - Determine containing scope
   - Resolve name in scope chain
   - Record reference in symbol table
   - Report error if unresolved (unless Option Explicit is off)
3. Handle qualified names (`Module.Function`, `Form.Control`)
4. Handle implicit references (properties without dot notation in forms)

**Output**: Updated symbol table with reference links, undefined reference diagnostics

### Phase 3: Type Inference & Checking

**Goal**: Determine types and validate type compatibility

**Process**:
1. **Declared types**: Apply explicit type annotations (As Integer, As String, etc.)
2. **DefType defaults**: Apply DefInt/DefStr/etc. for untyped variables
3. **Type inference**: Infer types from:
   - Literal assignments (`x = 10` → Integer)
   - Function return types
   - Built-in function signatures (from library definitions)
   - Control properties (from control definitions)
4. **Type checking**:
   - Assignment compatibility
   - Function argument types
   - Operator type rules
   - Property access validity
5. **Variant handling**: Track potential runtime types where needed

**Output**: Symbols with resolved types, type error diagnostics

### Phase 4: Semantic Validation

**Goal**: Check VB6-specific semantic rules

**Validations**:
- Option Explicit enforcement (undeclared variables)
- Variable must be declared before use (in scope order)
- Function/Sub call argument count/types match
- Property access on appropriate types
- Event handlers match event signatures
- Const values must be compile-time constants
- Static only valid in procedures
- WithEvents only for object variables with events
- ParamArray must be last parameter, ByVal, Variant()
- Optional parameters must follow required ones
- Form load order dependencies
- Circular reference detection

**Output**: Comprehensive semantic diagnostics

---

## VB6-Specific Considerations

### 1. Case Sensitivity

VB6 is completely case-insensitive for identifiers:

```rust
/// Case-insensitive string wrapper for symbol names
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VbIdentifier {
    /// Original casing (as written in source)
    original: SmolStr,
    
    /// Normalized (lowercase) for comparison
    normalized: SmolStr,
}

impl VbIdentifier {
    pub fn new(s: &str) -> Self {
        Self {
            original: SmolStr::new(s),
            normalized: SmolStr::new(&s.to_lowercase()),
        }
    }
    
    pub fn matches(&self, other: &str) -> bool {
        self.normalized.eq_ignore_ascii_case(other)
    }
}
```

**Use lowercase for all HashMap keys in symbol table.**

### 2. Option Explicit Handling

VB6 has two modes:
- **Option Explicit**: All variables must be declared before use
- **No Option Explicit**: Undeclared identifiers become implicit Variant declarations

```rust
pub enum DeclarationMode {
    /// Option Explicit: require declarations
    Explicit,
    
    /// Implicit: undeclared variables are Variant
    Implicit,
}
```

**Strategy**: Track per-module. In implicit mode, first reference to unknown identifier creates implicit Variant symbol (with warning diagnostic).

### 3. Default Member Properties

VB6 objects often have default properties accessed without explicit property name:

```vb
Text1 = "Hello"  ' Actually: Text1.Text = "Hello"
```

**Strategy**: Store default property in control definitions, resolve implicit property access during type checking.

### 4. Name Shadowing & Qualified Access

VB6 allows name shadowing but with quirks:

```vb
' Module1.bas
Public x As Integer

Sub Test()
    Dim x As String  ' Shadows module-level x
    x = "local"      ' Refers to local x
    Module1.x = 10   ' Explicit qualified access
End Sub
```

**Strategy**: Resolve unqualified names using innermost scope first. For qualified names, skip directly to specified scope.

### 5. Form Control Special Scoping

Controls on forms are accessible as module-level members:

```vb
Private Sub Command1_Click()
    Text1.Text = "Hello"  ' Text1 is like a module-level variable
End Sub
```

**Strategy**: When building symbols for FormFile, add control symbols to form scope with Public visibility.

### 6. Event Handler Name Convention

Event handlers follow naming convention: `ObjectName_EventName`

```vb
Private Sub Command1_Click()  ' Handler for Command1 control's Click event
```

**Strategy**: Validate event handler signatures match event definitions. Store event name mapping in form control symbols.

### 7. Late Binding (As Object)

`Dim x As Object` allows calling any method/property at runtime:

```vb
Dim x As Object
Set x = CreateObject("Excel.Application")
x.Visible = True  ' No compile-time validation possible
```

**Strategy**: Don't report errors for member access on `Object` type. Emit warnings if desired.

### 8. Circular Dependencies

VB6 allows circular module dependencies in many cases:

```vb
' Module1.bas
Public Sub A()
    Module2.B
End Sub

' Module2.bas
Public Sub B()
    Module1.A
End Sub
```

**Strategy**: Build all symbols first (Phase 1) before resolving references (Phase 2). Detect and report actual circular definitions (e.g., Const referencing each other).

---

## Integration with Existing Architecture

### Extending ParseResult Pattern

Semantic analysis follows the same "partial success" philosophy:

```rust
/// Analyze a project file with semantic analysis
pub fn analyze_project(project_file: ProjectFile) -> SemanticResult {
    let mut analyzer = SemanticAnalyzer::new();
    
    // Build symbols even if some files fail to parse
    let build_result = analyzer.build_symbols(&project_file);
    
    // Continue with analysis phases
    analyzer.resolve_references();
    analyzer.check_types();
    analyzer.validate();
    
    SemanticResult {
        symbol_table: analyzer.symbol_table,
        type_arena: analyzer.type_arena,
        diagnostics: analyzer.diagnostics,
    }
}

pub struct SemanticResult {
    /// Symbol table (always present, even if incomplete)
    symbol_table: SymbolTable,
    
    /// Type arena
    type_arena: TypeArena,
    
    /// All diagnostics (errors and warnings)
    diagnostics: Vec<SemanticDiagnostic>,
}

impl SemanticResult {
    pub fn symbol_table(&self) -> &SymbolTable;
    pub fn diagnostics(&self) -> &[SemanticDiagnostic];
    pub fn has_errors(&self) -> bool;
}
```

**Philosophy**: Always produce a symbol table, even if incomplete. Collect diagnostics rather than failing fast.

### CST Traversal Integration

Reuse existing CST navigation:

```rust
use crate::parsers::cst::{CstNode, SyntaxKind};

/// Visitor pattern for extracting symbols from CST
pub trait CstVisitor {
    fn visit_node(&mut self, node: &CstNode);
}

pub struct DeclarationVisitor<'a> {
    symbol_table: &'a mut SymbolTable,
    current_scope: ScopeId,
    diagnostics: &'a mut Vec<SemanticDiagnostic>,
}

impl CstVisitor for DeclarationVisitor<'_> {
    fn visit_node(&mut self, node: &CstNode) {
        match node.kind() {
            SyntaxKind::FunctionStmt => self.visit_function(node),
            SyntaxKind::SubStmt => self.visit_sub(node),
            SyntaxKind::DimStmt => self.visit_dim(node),
            SyntaxKind::ConstStmt => self.visit_const(node),
            SyntaxKind::TypeStmt => self.visit_type(node),
            SyntaxKind::EnumStmt => self.visit_enum(node),
            _ => {
                // Recurse to children
                for child in node.children() {
                    self.visit_node(child);
                }
            }
        }
    }
}

impl DeclarationVisitor<'_> {
    fn visit_function(&mut self, node: &CstNode) {
        // Extract function name, return type, parameters, visibility
        // Create Symbol and add to symbol_table
        // Create child scope for function body
        // Recurse into body
    }
    
    // Similar for other declaration types...
}
```

### File-Level Integration

Each file type needs a builder:

```rust
// In src/semantic/builders/module_builder.rs
pub fn build_module_symbols(
    module: &ModuleFile,
    symbol_table: &mut SymbolTable,
    project_scope: ScopeId,
) -> ScopeId {
    // Create module scope
    let module_scope = symbol_table.add_scope(Some(project_scope), ScopeKind::Module);
    
    // Parse module header for attributes
    // Walk CST to find declarations
    let mut visitor = DeclarationVisitor {
        symbol_table,
        current_scope: module_scope,
        diagnostics: &mut Vec::new(),
    };
    
    // Visit CST nodes
    // ...
    
    module_scope
}
```

### Library Function Integration

Leverage existing VB6 library definitions:

```rust
// Preload built-in functions into a global scope
pub fn create_builtin_symbols(symbol_table: &mut SymbolTable) -> ScopeId {
    let builtin_scope = symbol_table.add_scope(None, ScopeKind::Builtin);
    
    // Add symbols for all 160+ built-in functions
    // Use metadata from src/syntax/library/functions/
    
    // Example: MsgBox function
    let msgbox_sym = Symbol {
        name: SmolStr::new("MsgBox"),
        kind: SymbolKind::Function,
        ty: Some(type_arena.intern(Type::Integer)), // Returns Integer
        visibility: Visibility::Global,
        parameters: vec![/* prompt, buttons, title, ... */],
        // ...
    };
    symbol_table.add_symbol(builtin_scope, msgbox_sym);
    
    builtin_scope
}
```

**Integration Point**: Project scope has builtin scope as implicit parent.

---

## Error Handling & Reporting

### Diagnostic System

```rust
#[derive(Debug, Clone)]
pub struct SemanticDiagnostic {
    /// Error code (e.g., "E0001", "W0042")
    pub code: DiagnosticCode,
    
    /// Severity level
    pub severity: Severity,
    
    /// Human-readable message
    pub message: String,
    
    /// Primary location of issue
    pub location: SourceLocation,
    
    /// Additional related locations (e.g., original declaration)
    pub related: Vec<(SourceLocation, String)>,
    
    /// Suggested fixes (for IDEs)
    pub fixes: Vec<DiagnosticFix>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Severity {
    Error,      // Prevents compilation
    Warning,    // Suspicious but valid
    Information, // FYI
    Hint,       // Style/convention suggestion
}

#[derive(Debug, Clone)]
pub struct DiagnosticFix {
    /// Description of fix
    pub message: String,
    
    /// Text edits to apply
    pub edits: Vec<TextEdit>,
}

#[derive(Debug, Clone)]
pub struct TextEdit {
    pub range: Range<usize>,
    pub new_text: String,
}

pub struct DiagnosticCode(&'static str);

// Predefined diagnostic codes
impl DiagnosticCode {
    pub const UNDECLARED_VARIABLE: Self = Self("E0001");
    pub const TYPE_MISMATCH: Self = Self("E0002");
    pub const UNDEFINED_MEMBER: Self = Self("E0003");
    pub const WRONG_ARGUMENT_COUNT: Self = Self("E0004");
    pub const DUPLICATE_DECLARATION: Self = Self("E0005");
    pub const CIRCULAR_REFERENCE: Self = Self("E0006");
    
    pub const UNUSED_VARIABLE: Self = Self("W0001");
    pub const IMPLICIT_VARIANT: Self = Self("W0002");
    pub const SUSPICIOUS_COMPARISON: Self = Self("W0003");
    
    // ... more codes
}
```

**Example Diagnostic Output:**

```
error[E0001]: Undeclared variable 'x'
  --> src/Module1.bas:10:5
   |
10 |     x = 10
   |     ^ not declared in this scope
   |
help: declare the variable first
   |
8  |     Dim x As Integer
   |     ^^^^^^^^^^^^^^^^
```

### Error Categories

1. **Resolution Errors**: Undeclared symbols, ambiguous references
2. **Type Errors**: Type mismatches, invalid operations
3. **Semantic Errors**: Violating VB6 language rules
4. **Warnings**: Code smells, unused symbols, implicit conversions
5. **Information**: Deprecated features, style suggestions

---

## Performance Considerations

### Design Choices for Performance

1. **Arena Allocation**
   - Use `typed-arena` or `generational-arena` for symbols/scopes
   - Avoids allocation fragmentation
   - Stable IDs without lifetimes
   - Fast batch deallocation

2. **Interning**
   - Intern type representations (reuse primitives)
   - Use `SmolStr` for symbol names (inline small strings, arc large)
   - Use `PathBuf` for file paths (only allocate once per file)

3. **Indexing Strategy**
   - HashMap per scope (not global flat namespace)
   - Case-insensitive keys (lowercase normalization once)
   - Interval tree for location-based queries
   - Lazy reference collection (build on first query)

4. **Incremental Analysis** (Future)
   - Track file dependencies
   - Only re-analyze changed files
   - Cache symbol tables per file
   - Invalidate dependent files on change

5. **Parallel Analysis** (Future)
   - Phase 1 (symbol collection) parallelizable per file
   - Phase 2/3 need coordination but can batch
   - Use `rayon` for parallel iteration

### Memory Footprint

**Estimated per-file cost:**
- Small file (100 lines): ~10 KB
- Medium file (1000 lines): ~100 KB
- Large file (5000 lines): ~500 KB

**Project scaling:**
- 100 files: ~10 MB
- 1000 files: ~100 MB
- 10000 files: ~1 GB (large projects rare in VB6)

**Optimization strategies:**
- Strip doc comments in production mode
- Use indices instead of storing full locations everywhere
- Lazy load symbol details on demand

### Benchmark Targets

- **Lexing**: 1M lines/second (baseline from current impl)
- **Symbol collection**: 500K lines/second (2x slower than lexing)
- **Full analysis**: 100K lines/second (10x slower than lexing)
- **Query latency**: <1ms for go-to-definition, <10ms for find-references

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_symbol_resolution_simple() {
        let source = r#"
            Sub Test()
                Dim x As Integer
                x = 10
            End Sub
        "#;
        
        let module = ModuleFile::parse(source);
        let mut analyzer = SemanticAnalyzer::new();
        // ... build and resolve
        
        // Assert x is resolved correctly
        let sym = analyzer.symbol_table().resolve("x", procedure_scope).unwrap();
        assert_eq!(sym.kind, SymbolKind::Variable);
    }
    
    #[test]
    fn test_undeclared_variable_error() {
        let source = r#"
            Sub Test()
                x = 10
            End Sub
        "#;
        
        let result = analyze_module(source);
        assert!(result.has_errors());
        assert!(result.diagnostics().iter()
            .any(|d| d.code == DiagnosticCode::UNDECLARED_VARIABLE));
    }
    
    // ... hundreds more unit tests
}
```

### Integration Tests

Use real VB6 projects from `tests/data/` submodules:

```rust
#[test]
fn test_audiostation_project_analysis() {
    let project_path = "tests/data/audiostation/AudioStation.vbp";
    let project_file = ProjectFile::load(project_path).unwrap();
    
    let result = analyze_project(project_file);
    
    // Should successfully build symbol table
    assert!(result.symbol_table().scope_count() > 100);
    
    // May have warnings but no errors
    assert!(!result.has_errors());
    
    // Snapshot diagnostics
    insta::assert_yaml_snapshot!(result.diagnostics());
}
```

### Snapshot Tests

Use `insta` for symbol table snapshots:

```rust
#[test]
fn test_module_symbols_snapshot() {
    let source = include_str!("../data/Module1.bas");
    let result = analyze_module(source);
    
    insta::assert_yaml_snapshot!("module1_symbols", result.symbol_table());
}
```

### Property-Based Testing

Use `proptest` to generate random but valid VB6 code:

```rust
proptest! {
    #[test]
    fn test_symbol_roundtrip(decl in declaration_generator()) {
        // Parse declaration
        // Build symbol
        // Query symbol
        // Should always find what was declared
        let result = analyze_code(&decl);
        prop_assert!(!result.has_errors());
    }
}
```

### Fuzzing Integration

Extend existing fuzz targets:

```rust
// fuzz/fuzz_targets/semantic_analysis.rs
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let source_file = SourceFile::from_string(s);
        if let Some(module) = ModuleFile::parse(&source_file).result() {
            let mut analyzer = SemanticAnalyzer::new();
            // Should never panic
            let _ = analyzer.analyze_module(&module);
        }
    }
});
```

### Test Coverage Goals

- **Unit tests**: >90% line coverage for semantic module
- **Integration tests**: All test projects analyzed without panics
- **Fuzz tests**: 1M+ executions without crashes
- **Snapshot tests**: All major language features covered

---

## Technical Costs & Benefits

### Benefits

1. **Enables Advanced Tooling**
   - IDE features (autocomplete, navigation, refactoring)
   - Static analysis and linting
   - Documentation generation
   - Code migration tools

2. **Better Error Reporting**
   - Early detection of semantic errors
   - More precise error messages
   - Suggested fixes

3. **Foundation for Code Generation**
   - Type-aware code generation
   - Accurate cross-references
   - Dependency tracking

4. **Complementary to CST**
   - CST preserves syntax, semantic layer adds meaning
   - Both can coexist without redundancy

5. **VB6 Ecosystem Value**
   - Few tools exist for VB6 semantic analysis
   - Large codebases still in production
   - Migration to modern languages challenging

### Costs

1. **Implementation Complexity**
   - 5,000+ lines of new code estimated
   - Complex VB6 scoping rules
   - Edge cases in type system
   - **Timeline**: 3-6 months for initial implementation

2. **Maintenance Burden**
   - More code to maintain
   - Additional test coverage needed
   - Documentation overhead

3. **Memory Overhead**
   - Symbol tables can be large (100MB+ for big projects)
   - Type arena requires additional memory
   - Location indices add overhead

4. **Performance Impact**
   - Semantic analysis is 10x slower than parsing
   - Large projects may take seconds to analyze
   - Incremental analysis needed for responsiveness

5. **API Stability**
   - Semantic layer is higher-level, more likely to change
   - May need breaking changes during iteration
   - Consider putting behind feature flag initially

6. **Dependency Additions**
   - Arena allocator crate
   - Interval tree implementation
   - Potentially parallel analysis (rayon)

### Risk Mitigation

1. **Feature Flag**: Initially hide behind `semantic-analysis` feature
2. **Incremental Rollout**: Release symbol table first, then analysis passes
3. **Extensive Testing**: Leverage existing test projects, add 1000+ tests
4. **Performance Budget**: Set and monitor benchmark thresholds
5. **User Opt-In**: Make semantic analysis optional for users

---

## Alternative Crate Choices

### 1. Arena Allocation

**Option A: `typed-arena` (Recommended)**
- **Pros**: Simple, fast, type-safe, minimal dependencies
- **Cons**: No deletion, must allocate in batches
- **Use**: Perfect for symbol tables (rarely delete individual symbols)

**Option B: `generational-arena`**
- **Pros**: Stable indices, supports deletion, free list reuse
- **Cons**: Slightly more overhead, generation checking adds cost
- **Use**: Better if symbols need individual deletion

**Option C: Manual `Vec<T>` with indices**
- **Pros**: Zero dependencies, full control
- **Cons**: Manual index management, no safety guarantees
- **Use**: Avoid - too error-prone

**Recommendation**: Start with `typed-arena` for simplicity. Switch to `generational-arena` if deletion becomes important.

### 2. String Interning

**Option A: `smol_str` (Recommended)**
- **Pros**: Inline small strings, Arc for large, zero-copy from String
- **Cons**: Not a true interner (doesn't deduplicate)
- **Use**: Symbol names (most are short identifiers)

**Option B: `string-interner`**
- **Pros**: True interning with deduplication, integer keys
- **Cons**: Extra indirection, lifetime management
- **Use**: If memory is critical and many duplicate names

**Option C: `Arc<str>`**
- **Pros**: Built-in, cheap clones, no dependencies
- **Cons**: Heap allocation for all strings
- **Use**: Fallback if smol_str is insufficient

**Recommendation**: Use `smol_str` for symbol names, `Arc<str>` for longer strings (doc comments, file paths).

### 3. Interval Tree (Location Indexing)

**Option A: `intervaltree` crate**
- **Pros**: Maintained, general-purpose, good performance
- **Cons**: Not heavily optimized for our use case
- **Use**: Location-to-symbol queries

**Option B: Custom augmented tree**
- **Pros**: Tailored to our needs, potentially faster
- **Cons**: Significant implementation effort
- **Use**: Only if profiling shows bottleneck

**Option C: Simple sorted Vec with binary search**
- **Pros**: Simple, cache-friendly, good for small scopes
- **Cons**: O(n) insertion, O(log n) query
- **Use**: Per-scope indexing if few symbols per scope

**Recommendation**: Start with `intervaltree` crate. Optimize later if needed.

### 4. Parallel Processing

**Option A: `rayon` (Recommended)**
- **Pros**: Easy parallel iterators, work-stealing, mature
- **Cons**: Adds 500KB to binary size
- **Use**: Parallel file analysis in Phase 1

**Option B: Manual threading with `std::thread`**
- **Pros**: No dependencies, full control
- **Cons**: Manual thread management, no work-stealing
- **Use**: Avoid - rayon is better

**Option C: `crossbeam`**
- **Pros**: Scoped threads, good for fork-join
- **Cons**: More low-level than rayon
- **Use**: Only if rayon doesn't fit

**Recommendation**: Add `rayon` behind feature flag `parallel-analysis`. Default to sequential.

### 5. Case-Insensitive Hashing

**Option A: Manual `to_lowercase()` + `HashMap<String, V>`**
- **Pros**: Simple, no dependencies, predictable
- **Cons**: Extra allocations for keys
- **Use**: Name resolution in symbol table

**Option B: `unicase` crate**
- **Pros**: Zero-cost case-insensitive wrapper
- **Cons**: Another dependency
- **Use**: If performance critical

**Recommendation**: Manual `to_lowercase()` is fine. VB6 identifiers are ASCII-only.

### 6. Diagnostics

**Option A: Custom diagnostic types (Recommended)**
- **Pros**: Tailored to VB6, matches existing ParseResult pattern
- **Cons**: No LSP interop built-in
- **Use**: Core library

**Option B: `codespan-reporting`**
- **Pros**: Beautiful terminal output, annotation support
- **Cons**: Another API layer, potential mismatch
- **Use**: For CLI tools built on vb6parse

**Option C: LSP types (`lsp-types` crate)**
- **Pros**: Direct LSP compatibility
- **Cons**: Couples library to LSP protocol
- **Use**: In separate vb6parse-lsp wrapper crate

**Recommendation**: Custom types in core, provide conversion functions for LSP/codespan.

---

## Implementation Roadmap

### Phase 1: Foundation (4-6 weeks)

**Goals**: Core data structures and basic symbol collection

**Deliverables**:
1. `symbol_table.rs` with Symbol, SymbolTable, SymbolId
2. `scope.rs` with Scope, ScopeKind, ScopeId
3. `types.rs` with Type, TypeId, TypeArena
4. `builders/module_builder.rs` for basic module analysis
5. Unit tests for data structures
6. API documentation

**Success Criteria**:
- Can parse a simple module and extract Function/Sub/Variable symbols
- Symbol table stores and retrieves symbols correctly
- Tests achieve >80% coverage

### Phase 2: Symbol Resolution (3-4 weeks)

**Goals**: Link references to declarations

**Deliverables**:
1. `resolver.rs` with name resolution logic
2. `visitors/reference_visitor.rs` for finding identifier uses
3. Qualified name resolution (Module.Function)
4. Cross-file reference handling
5. Integration tests with multi-file projects

**Success Criteria**:
- Can resolve all references in test projects
- Detects undeclared variables
- Handles Option Explicit correctly

### Phase 3: Type System (4-5 weeks)

**Goals**: Type inference and checking

**Deliverables**:
1. Complete `types.rs` with all VB6 types
2. Type inference from literals, assignments, function returns
3. Built-in function signatures from library definitions
4. Type checking for assignments, operators, function calls
5. DefType directive handling

**Success Criteria**:
- Infers types correctly in 90% of cases
- Detects type mismatches
- Handles Variant conversions

### Phase 4: Advanced Features (4-6 weeks)

**Goals**: Forms, controls, events, and validation

**Deliverables**:
1. `builders/form_builder.rs` for form analysis
2. Control symbol handling
3. Event handler validation
4. Complete semantic validation pass
5. Diagnostic improvements (related locations, fixes)

**Success Criteria**:
- Analyzes forms with controls correctly
- Validates event handler signatures
- Comprehensive diagnostics for all error types

### Phase 5: Query API (2-3 weeks)

**Goals**: High-level query interface for tools

**Deliverables**:
1. `query.rs` with query API
2. Go-to-definition, find-references implementations
3. Hover information (type-at-position)
4. Rename refactoring validation
5. Example CLI tool using query API

**Success Criteria**:
- Queries execute in <10ms on medium projects
- API ergonomic for external use
- Documentation with examples

### Phase 6: Optimization & Polish (3-4 weeks)

**Goals**: Performance, documentation, stability

**Deliverables**:
1. Performance benchmarks
2. Memory profiling and optimization
3. Comprehensive documentation
4. Integration with existing examples
5. Blog post / announcement

**Success Criteria**:
- Meets performance benchmarks (100K lines/sec)
- Memory usage <1GB for large projects
- All public APIs documented
- Ready for 0.6.0 release

**Total Timeline**: 20-28 weeks (5-7 months)

---

## Future Extensions

### 1. Incremental Analysis

Cache symbol tables per file, only re-analyze changed files and dependents.

**Benefits**: Real-time responsiveness for IDEs
**Cost**: Complexity in dependency tracking, cache invalidation

### 2. Control Flow Analysis

Track reachability, detect dead code, validate initialization before use.

**Benefits**: More sophisticated static analysis
**Cost**: Requires CFG construction, dataflow analysis

### 3. Constant Folding & Evaluation

Evaluate compile-time constant expressions.

**Benefits**: Better error messages, dead code elimination
**Cost**: Need expression evaluator

### 4. Module Inference

Infer module structure from files even without .vbp project file.

**Benefits**: Analyze standalone files
**Cost**: Heuristics for file relationships

### 5. LSP Server

Full Language Server Protocol implementation for IDE integration.

**Benefits**: First-class VB6 IDE support
**Cost**: Separate project, protocol complexity, threading model

### 6. Cross-Language Support

Analyze VB6 + COM components together.

**Benefits**: Full understanding of VB6 projects using external components
**Cost**: Requires COM type library parsing

### 7. Deprecated API Detection

Flag usage of deprecated/unsafe VB6 features.

**Benefits**: Modernization assistance
**Cost**: Curated list of deprecated APIs

### 8. Metrics & Complexity Analysis

Cyclomatic complexity, coupling/cohesion metrics.

**Benefits**: Code quality insights
**Cost**: Additional analysis passes

---

## Summary & Recommendations

### Recommended Approach

1. **Start with Feature Flag**: Implement behind `semantic-analysis` feature to avoid breaking changes
2. **Incremental Release**: Ship symbol table (v0.6), then analysis (v0.7), then query API (v0.8)
3. **Focus on Correctness**: Comprehensive testing before optimization
4. **Performance Budget**: Set limits, measure continuously
5. **User Feedback**: Beta release to gather use cases and requirements

### Decision Points

| Decision | Recommendation | Rationale |
|----------|---------------|-----------|
| Arena allocator | `typed-arena` | Simple, fast, fits use case |
| String type | `smol_str` | Inline optimization for small identifiers |
| Interval tree | `intervaltree` crate | Good enough, can optimize later |
| Parallelism | `rayon` optional | Easy wins, feature-gated |
| Case handling | Manual lowercase | Simple, VB6 is ASCII-only |
| Diagnostics | Custom types | Matches project patterns |

### Go/No-Go Criteria

**Reasons to proceed:**
- High demand for VB6 analysis tools
- Fills gap in ecosystem
- Logical extension of vb6parse
- Enables valuable use cases (migration, linting, IDE features)

**Reasons to pause:**
- Limited resources for 5-7 month project
- Maintenance burden concern
- Uncertainty about user demand
- Should focus on other priorities (AST, more file formats)

### Immediate Next Steps

1. **Validate demand**: Survey users, gauge interest
2. **Prototype**: Build minimal symbol table (2-3 weeks) to validate approach
3. **Benchmark**: Test performance on large real projects
4. **Decide**: Go/no-go based on prototype results

---

## Appendix A: Example Usage

### Basic Symbol Lookup

```rust
use vb6parse::semantic::{SemanticAnalyzer, SymbolTable};
use vb6parse::files::ProjectFile;

fn main() {
    let project = ProjectFile::load("MyProject.vbp").unwrap();
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&project);
    
    if result.has_errors() {
        for diagnostic in result.diagnostics() {
            println!("{}", diagnostic);
        }
    }
    
    let symbol_table = result.symbol_table();
    
    // Find a symbol by name
    if let Some(sym_id) = symbol_table.resolve("MyFunction", project_scope) {
        let symbol = symbol_table.symbol(sym_id);
        println!("Found: {} at {:?}", symbol.name, symbol.declaration);
    }
}
```

### Go-to-Definition

```rust
use vb6parse::semantic::query::SymbolQuery;

fn goto_definition(file: &str, line: u32, column: u32) -> Option<SourceLocation> {
    let symbol_table = /* ... */;
    let query = SymbolQuery::new(&symbol_table);
    
    let location = SourceLocation { file, line, column };
    query.definition_at(location)
}
```

### Find References

```rust
fn find_references(symbol_name: &str, scope: ScopeId) -> Vec<SourceLocation> {
    let symbol_table = /* ... */;
    let query = SymbolQuery::new(&symbol_table);
    
    if let Some(symbol_id) = symbol_table.resolve(symbol_name, scope) {
        query.find_references(symbol_id)
    } else {
        Vec::new()
    }
}
```

### Type Checking

```rust
fn check_assignment_types(lhs_type: TypeId, rhs_type: TypeId) -> bool {
    let type_arena = /* ... */;
    type_arena.is_assignable(rhs_type, lhs_type)
}
```

---

## Appendix B: VB6 Scoping Rules Reference

### Module-Level Scope

- **Public**: Visible to entire project and external users
- **Private**: Visible only within this module
- **Friend**: Visible within project, not to external users (classes only)
- **Global**: Visible everywhere (BAS modules only, deprecated)

### Procedure-Level Scope

- **Dim**: Local to procedure
- **Static**: Local but preserves value between calls
- Parameters are procedure-local

### Block Scope

- **For loop variables**: Only `For i = 1 To 10` introduces block scope
- No other block-level scoping (no If/While local scope)

### Name Resolution Order

1. Procedure-level symbols (parameters, locals, static)
2. Module-level symbols (module variables, functions)
3. Referenced modules via Imports/Requires
4. Built-in functions/constants
5. Implicit Variant (if no Option Explicit)

### Qualified Names

- `Module.Function`: Access public symbol in specific module
- `Form.Control`: Access control on form
- `Object.Property`: Member access (runtime resolved if Object type)

---

## Appendix C: Related Projects & Prior Art

### Existing VB6 Tools

- **Visual Studio 6.0 IDE**: Provides IntelliSense, limited refactoring
- **MZ-Tools**: Commercial add-in with navigation, analysis
- **VB Watch**: Runtime analysis, no static analysis

### Open Source Projects

- **VB6 Antlr Grammar**: Grammar only, no implementation
- **VB6ToCS**: VB6 to C# converter, limited symbol resolution
- **TwinBasic**: Modern VB6 compiler, closed-source analysis

### Inspiration from Other Languages

- **rust-analyzer**: Symbol table design, incremental analysis
- **TypeScript Compiler**: Type inference, gradual typing
- **Roslyn**: Multi-pass compilation, comprehensive diagnostics

### Academic Research

- Limited research on VB6 specifically
- General compiler design principles apply
- Type inference algorithms (Hindley-Milner adapted for VB6's implicit system)

---

**Document Version**: 1.0  
**Date**: February 2026  
**Author**: GitHub Copilot (Claude Sonnet 4.5)  
**Status**: Design Proposal - Not Implemented
