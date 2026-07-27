# Testing Framework for vb6convert

## Overview

The vb6convert testing framework provides multiple layers of testing to ensure conversion accuracy and reliability:

1. **Unit Tests**: Test individual components and converters
2. **Integration Tests**: Test complete project conversions
3. **Validation Harness**: Compare converted code against reference implementations
4. **Regression Tests**: Ensure changes don't break existing functionality

## Testing Layers

### Layer 1: Unit Tests

Unit tests validate individual converter components in isolation.

#### Module Converter Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_simple_sub() {
        let converter = MyConverter::new();
        let vb6_code = r#"
            Sub DoSomething()
                MsgBox "Hello"
            End Sub
        "#;
        
        let result = converter.convert_subroutine(vb6_code);
        assert!(result.is_ok());
        
        let converted = result.unwrap();
        // Assert expected output
    }

    #[test]
    fn test_type_conversion() {
        let converter = MyConverter::new();
        assert_eq!(converter.convert_type(&VB6Type::Integer).unwrap(), "i32");
        assert_eq!(converter.convert_type(&VB6Type::String).unwrap(), "String");
    }
}
```

### Layer 2: Integration Tests

Integration tests validate complete project conversions using test VB6 projects.

#### Test Project Structure

```
tests/
  fixtures/
    simple_module/
      Project1.vbp
      Module1.bas
      expected/
        Module1.rs      # Expected Rust output
        Module1.js      # Expected JS output
    
    simple_form/
      Project2.vbp
      Form1.frm
      Form1.frx
      expected/
        ...
    
    complex_project/
      ...
```

#### Integration Test Example

```rust
#[test]
#[cfg(feature = "rust-code")]
fn test_simple_module_to_rust() {
    use vb6convert::*;
    use std::path::PathBuf;
    
    let project_path = PathBuf::from("tests/fixtures/simple_module/Project1.vbp");
    let output_dir = tempfile::tempdir().unwrap();
    
    let config = ConversionConfig {
        target: "rust".to_string(),
        output_dir: output_dir.path().to_path_buf(),
        source_project: project_path.clone(),
        ..Default::default()
    };
    
    let registry = ConverterRegistry::default();
    let converter = registry.get("rust").unwrap();
    
    // Parse the VB6 project
    let project = vb6parse::parsers::parse_project(&project_path).unwrap();
    
    // Convert
    let result = converter.convert_project(&project, &config);
    assert!(result.is_ok());
    
    let conversion_result = result.unwrap();
    
    // Verify output files were created
    assert!(conversion_result.generated_files.len() > 0);
    
    // Compare against expected output
    let expected = std::fs::read_to_string(
        "tests/fixtures/simple_module/expected/Module1.rs"
    ).unwrap();
    
    let actual = std::fs::read_to_string(
        output_dir.path().join("Module1.rs")
    ).unwrap();
    
    assert_eq!(normalize_whitespace(&expected), normalize_whitespace(&actual));
}

fn normalize_whitespace(s: &str) -> String {
    // Helper to make whitespace-insensitive comparisons
    s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
```

### Layer 3: Validation Harness

The validation harness runs both the original VB6 code and converted code with identical inputs and compares outputs.

#### Harness Architecture

```
┌─────────────────┐
│  Test Inputs    │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
    ▼         ▼
┌─────┐   ┌──────┐
│ VB6 │   │Target│
│Code │   │ Code │
└──┬──┘   └───┬──┘
   │          │
   ▼          ▼
┌──────┐  ┌──────┐
│Output│  │Output│
│  A   │  │  B   │
└───┬──┘  └───┬──┘
    │         │
    └────┬────┘
         ▼
    ┌─────────┐
    │ Compare │
    └─────────┘
```

#### Validation Test Structure

```rust
#[cfg(feature = "test-harness")]
pub mod validation {
    use crate::testing::*;

    pub struct TestHarness {
        vb6_executor: VB6Executor,
        target_executor: Box<dyn TargetExecutor>,
    }

    impl TestHarness {
        pub fn new(target: &str) -> Result<Self> {
            Ok(Self {
                vb6_executor: VB6Executor::new()?,
                target_executor: Self::create_executor(target)?,
            })
        }

        pub fn run_comparison_test(&self, test_case: &TestCase) -> Result<ComparisonReport> {
            // Execute VB6 version
            let vb6_output = self.vb6_executor.execute(&test_case.vb6_code, &test_case.inputs)?;
            
            // Execute converted version
            let target_output = self.target_executor.execute(
                &test_case.converted_code, 
                &test_case.inputs
            )?;
            
            // Compare outputs
            let comparison = self.compare_outputs(&vb6_output, &target_output);
            
            Ok(ComparisonReport {
                test_name: test_case.name.clone(),
                passed: comparison.is_equivalent(),
                vb6_output,
                target_output,
                differences: comparison.differences,
            })
        }

        fn compare_outputs(&self, vb6: &ExecutionOutput, target: &ExecutionOutput) 
            -> OutputComparison 
        {
            // Compare with tolerance for floating point, timing, etc.
            todo!()
        }
    }

    pub trait TargetExecutor: Send + Sync {
        fn execute(&self, code: &str, inputs: &[TestInput]) -> Result<ExecutionOutput>;
    }

    // Implementations for different targets
    #[cfg(feature = "rust-code")]
    pub struct RustExecutor { /* ... */ }

    #[cfg(feature = "js-code")]
    pub struct JavaScriptExecutor { /* ... */ }
}
```

#### Test Case Definition

```rust
pub struct TestCase {
    pub name: String,
    pub description: String,
    pub vb6_code: String,
    pub converted_code: String,
    pub inputs: Vec<TestInput>,
    pub expected_behavior: ExpectedBehavior,
}

pub struct TestInput {
    pub name: String,
    pub value: TestValue,
}

pub enum TestValue {
    Integer(i32),
    String(String),
    Float(f64),
    Boolean(bool),
    Array(Vec<TestValue>),
}

pub enum ExpectedBehavior {
    IdenticalOutput,
    EquivalentOutput { tolerance: f64 },
    SimilarOutput { similarity_threshold: f64 },
    CustomValidator(fn(&ExecutionOutput, &ExecutionOutput) -> bool),
}
```

### Layer 4: Regression Tests

Regression tests ensure that changes don't break existing conversions.

```rust
#[test]
fn test_regression_suite() {
    // Run all known-good conversions
    let test_cases = load_regression_tests();
    
    for test_case in test_cases {
        let result = run_conversion(&test_case);
        assert!(
            result.is_ok(), 
            "Regression test failed: {}", 
            test_case.name
        );
    }
}
```

## Test Organization

```
vb6convert/
├── tests/
│   ├── fixtures/                    # Test VB6 projects
│   │   ├── simple_module/
│   │   ├── simple_form/
│   │   ├── class_with_properties/
│   │   ├── database_access/
│   │   └── api_calls/
│   │
│   ├── rust_conversion.rs           # Rust-specific tests
│   ├── javascript_conversion.rs     # JS-specific tests
│   ├── tauri_conversion.rs          # Tauri-specific tests
│   └── validation_harness.rs        # Validation tests
│
└── src/
    └── testing/                     # Test harness implementation
        ├── mod.rs
        ├── harness.rs
        ├── executors/
        │   ├── vb6.rs
        │   ├── rust.rs
        │   └── javascript.rs
        └── comparison.rs
```

## Writing Good Test Cases

### 1. Test One Thing at a Time

```rust
#[test]
fn test_integer_addition() {
    // Test ONLY integer addition
}

#[test]
fn test_string_concatenation() {
    // Test ONLY string concatenation
}
```

### 2. Use Descriptive Names

```rust
#[test]
fn test_for_loop_with_step_positive() { }

#[test]
fn test_for_loop_with_step_negative() { }

#[test]
fn test_for_loop_with_step_zero_should_fail() { }
```

### 3. Test Edge Cases

```rust
#[test]
fn test_divide_by_zero_handling() { }

#[test]
fn test_empty_string_handling() { }

#[test]
fn test_null_variant_handling() { }

#[test]
fn test_array_bounds_checking() { }
```

### 4. Cover All Features

For each VB6 feature you support, have tests:

- **Language Features**
  - Variables and constants
  - All data types
  - Operators (arithmetic, logical, comparison)
  - Control flow (If/Then, Select Case, For, While, Do)
  - Procedures and functions
  - Error handling

- **Object-Oriented Features**
  - Classes
  - Properties (Get, Let, Set)
  - Methods
  - Events
  - Interfaces (Implements)

- **Forms and UI**
  - Form layout
  - Standard controls
  - Events
  - Data binding

## Running Tests

```bash
# Run all tests with all features
cargo test --all-features

# Run tests for specific feature
cargo test --features rust-code

# Run specific test
cargo test test_simple_module_conversion

# Run with output
cargo test -- --nocapture

# Run validation harness (requires VB6 runtime or Wine)
cargo test --features test-harness
```

## Continuous Integration

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        feature:
          - rust-code
          - js-code
          - tauri
          - full
    
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
      - name: Run tests
        run: cargo test --features ${{ matrix.feature }}
```

## Test Fixture Guidelines

When creating test VB6 projects:

1. **Keep them simple**: Each test project should demonstrate one concept
2. **Document expected behavior**: Add comments explaining what should happen
3. **Provide expected outputs**: Include the expected converted code
4. **Cover variations**: Test normal case, edge cases, and error cases
5. **Real-world examples**: Include some real VB6 patterns from actual projects

## Validation Requirements

For a conversion to be considered "validated":

1. All unit tests pass
2. Integration tests pass for that target
3. At least 3 real-world test projects convert without errors
4. Validation harness shows >95% output equivalence
5. Performance is within acceptable bounds
6. Documentation is complete

## Performance Testing

```rust
#[test]
fn bench_large_project_conversion() {
    use std::time::Instant;
    
    let start = Instant::now();
    let result = convert_large_project();
    let duration = start.elapsed();
    
    assert!(result.is_ok());
    assert!(duration.as_secs() < 60, "Conversion took too long: {:?}", duration);
}
```

## Future Enhancements

- [ ] Automated fuzzing of converters
- [ ] Property-based testing with quickcheck
- [ ] Performance regression tracking
- [ ] Code coverage reporting
- [ ] Mutation testing
- [ ] Integration with Wine for VB6 execution on Linux
- [ ] Cloud-based test execution
- [ ] Visual diff tools for UI conversions
