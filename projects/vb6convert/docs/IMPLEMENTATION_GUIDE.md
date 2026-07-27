# Implementation Guide for vb6convert

This guide provides step-by-step instructions for implementing a new conversion backend for vb6convert.

## Quick Start Checklist

To implement a new conversion target, you need to:

- [ ] Add feature flag to `Cargo.toml`
- [ ] Create module directory structure
- [ ] Implement `ProjectConverter` trait
- [ ] Implement specialized converter traits (as needed)
- [ ] Register converter in registry
- [ ] Add tests
- [ ] Add documentation
- [ ] Update CLI to recognize new target

## Step 1: Add Feature Flag

Edit `vb6convert/Cargo.toml` and add your feature:

```toml
[features]
# ... existing features ...
my-target = ["my-target-deps"]  # Add your feature

[dependencies]
# Add target-specific dependencies (optional)
my-target-lib = { version = "1.0", optional = true }
```

## Step 2: Create Module Structure

Create a new directory under `src/` for your converter:

```
src/
  my_target/
    mod.rs          # Module entry point
    converter.rs    # Main ProjectConverter implementation
    expressions.rs  # Expression/statement conversion
    types.rs        # Type mapping
    modules.rs      # Module conversion
    classes.rs      # Class conversion
    forms.rs        # Form conversion (if UI is supported)
    controls.rs     # Control conversion (if UI is supported)
```

## Step 3: Implement ProjectConverter

In `converter.rs`, implement the main `ProjectConverter` trait:

```rust
use crate::traits::*;
use crate::types::*;
use crate::error::Result;
use vb6parse::language::Project;

pub struct MyTargetConverter {
    // Add configuration or state as needed
}

impl MyTargetConverter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ProjectConverter for MyTargetConverter {
    fn name(&self) -> &str {
        "my-target"
    }

    fn description(&self) -> &str {
        "Converts VB6 projects to My Target Language"
    }

    fn convert_project(&self, project: &Project, config: &ConversionConfig) 
        -> Result<ConversionResult> 
    {
        // Implementation goes here
        todo!("Implement project conversion")
    }

    fn supports_feature(&self, feature: VB6Feature) -> bool {
        match feature {
            VB6Feature::OptionExplicit => true,
            VB6Feature::Classes => true,
            // ... list supported features
            _ => false,
        }
    }

    fn required_dependencies(&self) -> Vec<Dependency> {
        vec![
            Dependency {
                name: "my-runtime".to_string(),
                version: Some("1.0.0".to_string()),
                description: "Runtime library for converted code".to_string(),
            },
        ]
    }
}
```

## Step 4: Implement Specialized Converters

### Module Converter

```rust
impl ModuleConverter for MyTargetConverter {
    fn convert_module(&self, module: &Module, config: &ConversionConfig) 
        -> Result<String> 
    {
        let mut output = String::new();
        
        // Add file header
        output.push_str("// Converted from VB6\n\n");
        
        // Convert module-level variables
        // ...
        
        // Convert procedures
        // ...
        
        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "mytarget"
    }
}
```

### Class Converter

```rust
impl ClassConverter for MyTargetConverter {
    fn convert_class(&self, class: &Class, config: &ConversionConfig) 
        -> Result<String> 
    {
        let mut output = String::new();
        
        // Convert class declaration
        // ...
        
        // Convert properties
        // ...
        
        // Convert methods
        // ...
        
        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "mytarget"
    }
}
```

### Form Converter (for UI targets)

```rust
impl FormConverter for MyTargetConverter {
    fn convert_form(&self, form: &Form, config: &ConversionConfig) 
        -> Result<FormOutput> 
    {
        let layout = self.convert_layout(form)?;
        let code_behind = self.convert_code_behind(form, config)?;
        
        Ok(FormOutput {
            layout: Some(ConvertedFile {
                filename: format!("{}.ui", form.name),
                content: layout,
                file_type: FileType::SourceCode,
            }),
            code_behind: ConvertedFile {
                filename: format!("{}.mytarget", form.name),
                content: code_behind,
                file_type: FileType::SourceCode,
            },
            styling: None,
            assets: vec![],
        })
    }

    fn convert_layout(&self, form: &Form) -> Result<String> {
        // Convert form layout to target UI format
        todo!()
    }

    fn convert_code_behind(&self, form: &Form, config: &ConversionConfig) 
        -> Result<String> 
    {
        // Convert form event handlers and methods
        todo!()
    }
}
```

## Step 5: Implement Helper Converters

### Expression Converter

```rust
impl ExpressionConverter for MyTargetConverter {
    fn convert_expression(&self, expr: &str, context: &ConversionContext) 
        -> Result<String> 
    {
        // Parse and convert VB6 expression to target syntax
        // Handle operators, function calls, literals, etc.
        todo!()
    }

    fn convert_statement(&self, stmt: &str, context: &ConversionContext) 
        -> Result<String> 
    {
        // Convert VB6 statement to target syntax
        // Handle If/Then, For/Next, While, etc.
        todo!()
    }
}
```

### Type Converter

```rust
impl TypeConverter for MyTargetConverter {
    fn convert_type(&self, vb6_type: &VB6Type) -> Result<String> {
        let target_type = match vb6_type {
            VB6Type::Integer => "i32",
            VB6Type::Long => "i64",
            VB6Type::String => "String",
            VB6Type::Boolean => "bool",
            VB6Type::Variant => "Variant",  // May need custom type
            VB6Type::Object => "Box<dyn Any>",
            VB6Type::Custom(name) => name.as_str(),
            // ... other mappings
            _ => return Err(ConversionError::UnsupportedFeature(
                format!("Type conversion for {:?}", vb6_type)
            )),
        };
        Ok(target_type.to_string())
    }

    fn is_lossless_conversion(&self, vb6_type: &VB6Type) -> bool {
        match vb6_type {
            VB6Type::Integer | VB6Type::Long | VB6Type::String | VB6Type::Boolean => true,
            VB6Type::Variant => false,  // Information about actual type is lost
            _ => false,
        }
    }
}
```

## Step 6: Register Converter

Update `src/lib.rs` to conditionally include your module:

```rust
#[cfg(feature = "my-target")]
pub mod my_target;
```

Update `src/converters.rs` to register your converter:

```rust
impl Default for ConverterRegistry {
    fn default() -> Self {
        let mut registry = Self::new();

        // ... existing registrations ...

        #[cfg(feature = "my-target")]
        {
            registry.register(Arc::new(crate::my_target::MyTargetConverter::new()));
        }

        registry
    }
}
```

## Step 7: Add Tests

Create `tests/my_target_conversion.rs`:

```rust
#[cfg(feature = "my-target")]
mod my_target_tests {
    use vb6convert::*;
    
    #[test]
    fn test_simple_module_conversion() {
        // Test converting a simple module
    }
    
    #[test]
    fn test_class_conversion() {
        // Test converting a class
    }
    
    #[test]
    fn test_form_conversion() {
        // Test converting a form
    }
}
```

## Step 8: Document Your Converter

Create `docs/targets/my-target.md`:

```markdown
# My Target Converter

## Overview
Description of the target language/framework and conversion approach.

## Supported Features
- List of supported VB6 features
- Known limitations

## Type Mappings
| VB6 Type | Target Type | Notes |
|----------|-------------|-------|
| Integer  | i32         | -     |
| ...      | ...         | ...   |

## Example
Show example VB6 code and converted output.
```

## Common Patterns

### Handling Unsupported Features

```rust
fn convert_something(&self, ...) -> Result<String> {
    if uses_unsupported_feature {
        return Err(ConversionError::UnsupportedFeature(
            "Feature X is not supported by this converter".to_string()
        ));
    }
    // ... conversion logic
}
```

### Adding Warnings

```rust
let mut result = ConversionResult {
    generated_files: vec![],
    warnings: vec![],
    stats: ConversionStats::default(),
};

result.warnings.push(ConversionWarning {
    message: "Using fallback conversion for Variant type".to_string(),
    location: Some(SourceLocation { /* ... */ }),
    severity: WarningSeverity::Warning,
});
```

### Managing Scope

```rust
let mut context = ConversionContext {
    current_file: "Module1.bas".to_string(),
    scope: ScopeInfo {
        variables: vec!["x".to_string(), "y".to_string()],
        functions: vec!["DoSomething".to_string()],
        parent: None,
    },
    imports: vec![],
};
```

## Testing Your Converter

```bash
# Build with your feature enabled
cargo build --features my-target

# Run tests
cargo test --features my-target

# Test the CLI
cargo run --features my-target -- convert test.vbp --target my-target --output ./out
```

## Best Practices

1. **Start Small**: Begin with basic modules before tackling forms and UI
2. **Incremental Implementation**: Implement feature by feature, not all at once
3. **Test Early**: Write tests as you implement features
4. **Document Limitations**: Be clear about what is and isn't supported
5. **Use Warnings**: When perfect conversion isn't possible, convert to something reasonable and warn
6. **Preserve Intent**: Even if syntax changes, preserve the semantic meaning
7. **Comment Generated Code**: Add comments explaining VB6 origins
8. **Handle Edge Cases**: VB6 has many quirks; handle them gracefully

## Getting Help

- Review existing converters (Rust, JavaScript) for examples
- Check the core traits documentation
- Look at vb6parse documentation for AST structure
- Join discussions in the project repository
