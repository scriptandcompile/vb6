# vb6semantic

Semantic analysis and symbol table construction for Visual Basic 6 code.

## Overview

`vb6semantic` provides semantic analysis capabilities for VB6 code parsed by [vb6parse](../vb6parse). It builds symbol tables, performs type checking, resolves names, and validates semantic correctness of VB6 code.

The output of semantic analysis is used by:
- [aspen](../aspen) - for calling out to compile, check, document, and test vb6 code.
- [vb6compile](../vb6compile) - Before lowering to IR and compilation
- [vb6convert](../vb6convert) - For conversion analysis and validation
- [vb6interpret](../vb6interpret) - for runtime scope and validation.

## Features

- **Symbol Tables**: Build comprehensive symbol tables for VB6 projects
- **Scope Management**: Hierarchical scope management (global, class, procedure, block)
- **Name Resolution**: Resolve symbol references with proper scoping rules
- **Type Checking**: Validate type compatibility and assignments
- **Visibility Rules**: Enforce Public/Private/Friend visibility rules
- **Error Reporting**: Detailed semantic error messages with source locations

## Architecture

### Core Components

1. **Symbol Table** (`symbols.rs`)
   - Manages symbols across scopes
   - Handles symbol definitions and lookups
   - Tracks symbol metadata

2. **Scope Manager** (`scope.rs`)
   - Hierarchical scope management
   - Parent-child scope relationships
   - Name resolution with scope walking

3. **Type System** (`types.rs`)
   - Type information representation
   - Type compatibility checking
   - Type promotion rules

4. **Semantic Analyzer** (`analyzer.rs`)
   - Main entry point for analysis
   - Coordinates symbol table building
   - Collects errors and warnings

5. **Name Resolver** (`resolution.rs`)
   - Resolves symbol references
   - Handles qualified names
   - Checks accessibility

### Symbol Kinds

The following symbol kinds are supported:

- **Variable**: Module/local variables
- **Constant**: Constant declarations
- **SubProcedure**: Sub procedures
- **Function**: Functions
- **PropertyGet/Let/Set**: Property accessors
- **Class**: Class definitions
- **Module**: Module files
- **Form**: Form files
- **Control**: Form controls
- **Enum**: Enumeration types
- **EnumMember**: Enumeration values
- **UserType**: User-defined types
- **TypeMember**: Type fields
- **Parameter**: Procedure parameters
- **Label**: GoTo labels

### Scope Hierarchy

```
Global Scope (Module/Form)
  ├─ Class Scope
  │   ├─ Property Scope
  │   │   └─ Block Scope
  │   └─ Procedure Scope
  │       └─ Block Scope
  └─ Procedure Scope
      └─ Block Scope
```

### Type System

Supported VB6 types:

**Primitive Types:**
- Integer, Long, Byte
- Single, Double, Currency
- String, Boolean, Date

**Complex Types:**
- Variant (universal type)
- Object (generic object)
- Class (specific class)
- UserType (custom type)
- Enum (enumeration)

**Special Types:**
- Nothing, Empty, Null
- Sub, Function

**Type Rules:**
- Variant is compatible with all types
- Numeric promotions (Byte → Integer → Long → Single → Double)
- Object-Class relationships

## Integration with vb6parse

vb6parse is responsible for loading sourcefiles, parsing these sourcefiles into the 
corresponding file types (ProjectFile, ModuleFile, ClassFile, FormFile, etc) and then
vb6semantic is responsible for doing semantic analysis on these files.

## Use Cases

### IDE Support
- Code completion (symbol suggestions)
- Go to definition
- Find references
- Rename refactoring
- Type information on hover

### Static Analysis
- Find unused variables
- Detect undefined symbols
- Check type safety
- Validate visibility rules
- Detect dead code

### Code Transformation
- Provide symbol information for converters
- Enable semantic-aware refactoring
- Support automated migration

### Documentation Generation
- Extract symbol information
- Generate API documentation
- Create cross-references

## Implementation Status

### ✅ Completed
- Core type system
- Symbol representation
- Scope management framework
- Basic error types
- Type compatibility checking

### 🚧 In Progress
- Full VB6 project analysis
- Complete name resolution
- All type checking rules

### 📋 Planned
- Control flow analysis
- Dead code detection
- Advanced type inference
- Cross-reference tracking
- Performance optimizations

## Examples

See [docs/EXAMPLES.md](docs/EXAMPLES.md) for detailed examples.

## Testing

```bash
# Run tests
cargo test -p vb6semantic

# Run with output
cargo test -p vb6semantic -- --nocapture

# Test specific module
cargo test -p vb6semantic --lib symbols
```

## Documentation

- [DESIGN.md](docs/DESIGN.md) - Detailed design documentation
- [SYMBOL_TABLES.md](docs/SYMBOL_TABLES.md) - Symbol table design

## Contributing

Contributions welcome! Areas that need work:

- Complete implementation of analysis passes
- Additional type checking rules
- Performance improvements
- More comprehensive tests
- Better error messages

## License

MIT License - see [LICENSE](LICENSE) for details.

## Related Projects

- [vb6parse](../vb6parse) - VB6 parser library
- [aspen](../aspen) - VB6 project tools
