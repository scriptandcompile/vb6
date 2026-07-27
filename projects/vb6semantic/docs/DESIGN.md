# Design Documentation for vb6semantic

## Overview

The vb6semantic library provides semantic analysis capabilities for VB6 code. It operates on the parsed output from vb6parse and builds symbol tables, performs type checking, and validates semantic correctness.

## Goals

1. **Accurate Symbol Tables**: Build complete symbol tables reflecting VB6's scoping rules
2. **Type Safety**: Validate type compatibility according to VB6 rules
3. **Error Detection**: Find semantic errors that pure syntax checking misses
4. **IDE Support**: Provide information needed for IDE features
5. **Conversion Support**: Supply semantic information for code converters

## Architecture

### Layered Design

```
┌─────────────────────────────────────┐
│     Public API (SemanticAnalyzer)   │
├─────────────────────────────────────┤
│   Name Resolution │ Type Checking   │
├─────────────────────────────────────┤
│         Scope Manager               │
├─────────────────────────────────────┤
│         Symbol Table                │
└─────────────────────────────────────┘
```

### Component Interactions

```
vb6parse → SemanticAnalyzer
              ↓
         ScopeManager ←→ SymbolTable
              ↓
         NameResolver
              ↓
         TypeChecker (uses vb6runtime::VBType)
```

### Integration with Other Libraries

**vb6runtime** - Type system dependency:
- Uses `vb6runtime::VBType` for all type information
- Uses `vb6runtime::Value` for understanding VB6 value semantics
- Ensures type checking matches VB6 runtime behavior

**vb6core** - Not a direct dependency:
- vb6semantic operates on CST from vb6parse
- Does NOT use vb6core's IR (that's for compilation/interpretation)
- However, vb6core uses semantic analysis results before lowering to IR

**Outputs consumed by**:
- `vb6compile` - Uses semantic analysis before compilation
- `vb6convert` - Uses semantic analysis for conversion
- `vb6interpret` - Uses semantic analysis for runtime checks

## Symbol Representation

### Symbol Structure

A symbol contains:
- **Name**: The identifier
- **Kind**: Variable, function, class, etc.
- **Type**: Type information
- **Visibility**: Public, Private, Friend
- **Location**: Source location
- **Scope**: Which scope it belongs to
- **Attributes**: Additional metadata

### Symbol Kinds

#### Declarations
- **Variable**: `Dim x As Integer`
- **Constant**: `Const PI = 3.14159`
- **Parameter**: Function parameters

#### Procedures
- **SubProcedure**: `Sub DoSomething()`
- **Function**: `Function Calculate() As Integer`
- **PropertyGet/Let/Set**: Property accessors

#### Containers
- **Class**: Class definitions
- **Module**: Module files
- **Form**: Form files
- **UserType**: `Type MyType`
- **Enum**: `Enum Colors`

#### Members
- **TypeMember**: Fields in a Type
- **EnumMember**: Values in an Enum
- **Control**: Controls on a form

## Scope Management

### Scope Hierarchy

VB6 has a hierarchical scope structure:

1. **Global Scope**: Project-wide
2. **Module/Form/Class Scope**: File-level
3. **Procedure Scope**: Inside Sub/Function
4. **Block Scope**: With blocks, For loops

### Scoping Rules

#### Name Resolution Order

When looking up a name, search:
1. Current block scope
2. Enclosing procedure scope
3. Module/class scope
4. Global scope

#### Visibility Rules

- **Public**: Accessible everywhere
- **Private**: Only within the same module/class
- **Friend**: Within the same project
- **Global**: Project-wide (only for variables)

### Scope Examples

```vb6
' Module1.bas - Global scope
Public GlobalVar As Integer
Private ModuleVar As Integer

Sub MyProcedure()  ' Procedure scope starts
    Dim LocalVar As Integer
    
    For i = 1 To 10  ' Block scope (implicit loop variable)
        ' Can access: i, LocalVar, ModuleVar, GlobalVar
    Next i
    
    With SomeObject  ' Block scope (With)
        .Property = 5
    End With
End Sub
```

## Type System

`vb6semantic` uses the type system from `vb6runtime` to ensure exact VB6 semantics.

### Type Categories

These mirror `vb6runtime::VBType`:

#### Primitive Types
- **Numeric**: Integer, Long, Byte, Single, Double, Currency
- **Text**: String
- **Logical**: Boolean
- **Temporal**: Date

#### Complex Types
- **Variant**: Can hold any type (uses `vb6runtime::Value`)
- **Object**: Generic object reference
- **Class**: Specific class instance
- **UserType**: Custom structure
- **Enum**: Enumeration type
- **Array**: Arrays of any type (with VB6 semantics)

**Note**: Type definitions come from `vb6runtime::VBType` to ensure semantic analysis uses the same type system as runtime execution and compilation.

### Type Compatibility

#### Assignment Rules

```
Source → Target: Valid?

Integer → Long:     Yes (widening)
Long → Integer:     No  (narrowing, loses data)
String → Variant:   Yes (Variant accepts all)
Variant → String:   Yes (runtime conversion)
Integer → String:   No  (incompatible)
Class → Object:     Yes (subtype)
Object → Class:     No  (needs type check)
```

#### Operation Rules

Numeric operations promote to larger type:
- Byte + Integer → Integer
- Integer + Long → Long
- Long + Double → Double

String concatenation:
- String & anything → String
- Use & for concatenation, + for addition

### Type Inference

Some VB6 features require type inference:

```vb6
Dim x  ' Type is Variant (default)
x = 5  ' Now holds Integer

For i = 1 To 10  ' i is implicitly Variant
```

## Analysis Passes

### Pass 1: Symbol Declaration

Collect all declarations:
1. Scan module-level declarations
2. Scan class members
3. Scan procedure signatures
4. Build initial symbol table

### Pass 2: Type Resolution

Resolve type references:
1. Resolve user-defined types
2. Resolve class references
3. Build type dependency graph
4. Check for circular references

### Pass 3: Name Resolution

Resolve all symbol references:
1. Resolve variable references
2. Resolve function calls
3. Check accessibility
4. Validate qualified names

### Pass 4: Type Checking

Validate types:
1. Check assignments
2. Check operations
3. Check function calls
4. Check array access

### Pass 5: Semantic Validation

Additional checks:
1. Unreachable code
2. Unused variables
3. Duplicate labels
4. Invalid GoTo targets

## Special Cases

### Late Binding

```vb6
Dim obj As Object
Set obj = CreateObject("Excel.Application")
obj.Visible = True  ' Late binding - no compile-time check
```

**Handling**: Track as Object type, perform minimal checking

### Variant Type

Variant can hold any type and changes at runtime:

```vb6
Dim v As Variant
v = 5        ' Now Integer
v = "Hello"  ' Now String
```

**Handling**: Accept all operations, track as Variant

### Arrays

Arrays can be:
- Fixed size: `Dim arr(10) As Integer`
- Dynamic: `Dim arr() As Integer` + `ReDim arr(10)`
- Multi-dimensional: `Dim arr(5, 10) As Integer`

**Handling**: Track array flag and dimensions in TypeInfo

### Optional Parameters

```vb6
Sub DoSomething(Required As Integer, Optional Opt As Integer = 10)
```

**Handling**: Store default values in symbol metadata

### ParamArray

```vb6
Sub DoSomething(ParamArray args() As Variant)
```

**Handling**: Mark as variable argument list

### Property Procedures

Properties have three forms:
- `Property Get`: Read accessor
- `Property Let`: Write accessor (for values)
- `Property Set`: Write accessor (for objects)

**Handling**: Store all three as separate symbols, link them

### Events

```vb6
Event StatusChanged(NewStatus As String)
```

**Handling**: Store as special symbol kind, track event handlers

### Implements

```vb6
Implements IInterface
```

**Handling**: Track interface relationships, validate implementation

## Error Handling

### Error Categories

1. **Undefined Symbol**: Reference to undeclared name
2. **Duplicate Symbol**: Multiple declarations of same name
3. **Type Mismatch**: Incompatible types
4. **Invalid Scope**: Scope errors
5. **Accessibility**: Visibility violations
6. **Circular Dependency**: Type/module cycles

### Error Reporting

Errors include:
- Error message
- Source location (file, line, column)
- Related locations (e.g., previous definition)
- Suggested fixes (when possible)

## Performance Considerations

### Symbol Table Structure

Use HashMap for O(1) lookup:
```rust
HashMap<ScopeId, HashMap<Name, Symbol>>
```

### Lazy Analysis

Analyze on demand:
- Build symbol table eagerly
- Perform type checking lazily
- Cache analysis results

### Incremental Analysis

Support incremental updates:
- Track dependencies between files
- Re-analyze only affected files
- Maintain valid symbol table

## Integration Points

### With vb6parse

Input: Parsed structures from vb6parse
- ProjectFile
- ModuleFile
- ClassFile
- FormFile

Walk the parsed AST and build symbols.

### With vb6runtime

Uses type system:
- `vb6runtime::VBType` for all type information
- `vb6runtime::Value` for understanding value semantics
- Type compatibility rules match vb6runtime behavior

### With vb6compile

Provides semantic analysis before compilation:
- Symbol tables for IR generation
- Type information for optimization
- Semantic validation before lowering to vb6core IR

### With vb6convert

Provide symbol information for conversion:
- Symbol lookup during conversion
- Type information for mapping (via vb6runtime types)
- Scope information for code generation

### With vb6interpret

Provides semantic information for interpretation:
- Symbol lookup during execution
- Type checking for runtime operations

### With IDEs

Support IDE features:
- Code completion (symbol suggestions)
- Go to definition (symbol locations)
- Hover information (types, docs)
- Find references (symbol usage)

## Future Enhancements

### Control Flow Analysis

Track control flow:
- Reachability
- Definite assignment
- Initialization checking

### Data Flow Analysis

Track data flow:
- Use-def chains
- Def-use chains
- Constant propagation

### Advanced Type Inference

Infer more precise types:
- Track Variant contents when possible
- Infer array dimensions
- Infer object types

### Cross-Project Analysis

Analyze multiple projects:
- Track project references
- Validate cross-project dependencies
- Handle COM references

### Performance Profiling

Add performance tracking:
- Analysis time per file
- Memory usage
- Bottleneck identification

## Testing Strategy

### Unit Tests

Test each component:
- Symbol table operations
- Scope management
- Type checking rules
- Name resolution

### Integration Tests

Test with real VB6 code:
- Parse and analyze complete projects
- Validate against known-good results
- Test edge cases

### Regression Tests

Maintain test suite:
- Prevent regressions
- Test fixes
- Document expected behavior

## References

- VB6 Language Specification
- Compiler Design textbooks
- Static analysis literature
- IDE implementation guides
