# VB6 Compiler/Interpreter Test Harness

## Overview

The test harness validates the correctness of `vb6interpret` and `vb6compile` by comparing their output against the legacy VB6 compiler (VB6.exe). This ensures semantic compatibility and helps catch regressions.

## Goals

1. **Correctness Verification**: Ensure our implementations produce identical results to VB6
2. **Regression Testing**: Detect when changes break existing functionality
3. **Coverage Tracking**: Identify which VB6 features are tested
4. **Performance Benchmarking**: Compare execution speed
5. **Continuous Integration**: Automate testing in CI/CD pipeline

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Test Harness                          │
│                                                         │
│  ┌──────────────┐         ┌─────────────────────────┐ │
│  │ Test Suite   │────────▶│  Test Runner            │ │
│  │ (.vbp, .bas) │         │                         │ │
│  └──────────────┘         │  ┌───────────────────┐  │ │
│                           │  │ VB6.exe (legacy)  │  │ │
│                           │  └─────────┬─────────┘  │ │
│                           │            │            │ │
│                           │  ┌─────────▼─────────┐  │ │
│                           │  │ vb6interpret     │  │ │
│                           │  └─────────┬─────────┘  │ │
│                           │            │            │ │
│                           │  ┌─────────▼─────────┐  │ │
│                           │  │ vb6compile       │  │ │
│                           │  └─────────┬─────────┘  │ │
│                           │            │            │ │
│                           │   ┌────────▼────────┐   │ │
│                           │   │  Result Compare │   │ │
│                           │   └────────┬────────┘   │ │
│                           └────────────┼────────────┘ │
│                                        │              │
│                               ┌────────▼────────┐     │
│                               │  Test Report    │     │
│                               └─────────────────┘     │
└─────────────────────────────────────────────────────────┘
```

## Test Suite Structure

```
tests/
├── harness/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs           # Test runner CLI
│   │   ├── runner.rs         # Test execution logic
│   │   ├── vb6.rs            # VB6.exe wrapper
│   │   ├── compare.rs        # Result comparison
│   │   └── report.rs         # Report generation
│   └── config.toml           # Configuration
├── suite/
│   ├── basic/
│   │   ├── arithmetic.bas    # Basic arithmetic tests
│   │   ├── strings.bas       # String operations
│   │   ├── arrays.bas        # Array operations
│   │   └── ...
│   ├── stdlib/
│   │   ├── string_funcs.bas  # String function tests
│   │   ├── math_funcs.bas    # Math function tests
│   │   ├── date_funcs.bas    # Date/time tests
│   │   └── ...
│   ├── control_flow/
│   │   ├── if_then.bas       # If statements
│   │   ├── select_case.bas   # Select Case
│   │   ├── loops.bas         # Loops
│   │   └── ...
│   ├── oop/
│   │   ├── classes.cls       # Class tests
│   │   ├── properties.cls    # Property tests
│   │   ├── events.cls        # Event tests
│   │   └── ...
│   ├── projects/
│   │   ├── simple_app/
│   │   │   ├── project.vbp
│   │   │   ├── main.bas
│   │   │   └── module1.bas
│   │   └── ...
│   └── edge_cases/
│       ├── variant_ops.bas   # Variant edge cases
│       ├── error_handling.bas
│       └── ...
└── expected/
    ├── arithmetic.txt        # Expected outputs
    ├── strings.txt
    └── ...
```

## Test Format

### Test File Header

Each test file includes metadata:

```vb
' TEST: Arithmetic Operations
' CATEGORY: basic
' DESCRIPTION: Test basic integer and floating-point arithmetic
' EXPECTED: arithmetic.txt
' TIMEOUT: 5
' VB6_VERSION: 6.0

Sub Main()
    ' Test code here
    Print 2 + 2
    Print 10 - 5
    Print 3 * 4
    Print 15 / 3
End Sub
```

### Expected Output Format

```
4
5
12
5
```

### Test Metadata

```toml
# arithmetic.toml
name = "Arithmetic Operations"
category = "basic"
description = "Test basic integer and floating-point arithmetic"
expected_output = "arithmetic.txt"
timeout_seconds = 5
vb6_version = "6.0"
skip_interpret = false
skip_compile = false
known_issues = []
```

## Test Runner Implementation

### Main Runner (`runner.rs`)

```rust
use std::process::Command;
use std::path::Path;

pub struct TestRunner {
    vb6_path: PathBuf,
    interpreter_path: PathBuf,
    compiler_path: PathBuf,
    wine_prefix: Option<PathBuf>,  // For running VB6 on Linux
}

#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub path: PathBuf,
    pub expected_output: Option<PathBuf>,
    pub timeout: Duration,
    pub category: String,
    pub skip_interpret: bool,
    pub skip_compile: bool,
}

#[derive(Debug)]
pub struct TestResult {
    pub test_name: String,
    pub vb6_result: ExecutionResult,
    pub interpret_result: ExecutionResult,
    pub compile_result: ExecutionResult,
    pub passed: bool,
    pub differences: Vec<Difference>,
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration: Duration,
    pub error: Option<String>,
}

impl TestRunner {
    pub fn run_test(&self, test: &TestCase) -> Result<TestResult> {
        // 1. Run with VB6.exe
        let vb6_result = self.run_with_vb6(test)?;
        
        // 2. Run with vb6interpret
        let interpret_result = if !test.skip_interpret {
            self.run_with_interpret(test)?
        } else {
            ExecutionResult::skipped()
        };
        
        // 3. Run with vb6compile
        let compile_result = if !test.skip_compile {
            self.run_with_compile(test)?
        } else {
            ExecutionResult::skipped()
        };
        
        // 4. Compare results
        let differences = self.compare_results(&vb6_result, &interpret_result, &compile_result)?;
        
        Ok(TestResult {
            test_name: test.name.clone(),
            vb6_result,
            interpret_result,
            compile_result,
            passed: differences.is_empty(),
            differences,
        })
    }
    
    fn run_with_vb6(&self, test: &TestCase) -> Result<ExecutionResult> {
        let start = Instant::now();
        
        // First, compile with VB6
        let compile_output = if cfg!(windows) {
            Command::new(&self.vb6_path)
                .arg("/make")
                .arg(&test.path)
                .output()?
        } else {
            // Use Wine on Linux/macOS
            Command::new("wine")
                .env("WINEPREFIX", self.wine_prefix.as_ref().unwrap())
                .arg(&self.vb6_path)
                .arg("/make")
                .arg(&test.path)
                .output()?
        };
        
        if !compile_output.status.success() {
            return Ok(ExecutionResult {
                stdout: String::new(),
                stderr: String::from_utf8_lossy(&compile_output.stderr).to_string(),
                exit_code: compile_output.status.code().unwrap_or(-1),
                duration: start.elapsed(),
                error: Some("Compilation failed".to_string()),
            });
        }
        
        // Execute the compiled program
        let exe_path = test.path.with_extension("exe");
        let run_output = Command::new(&exe_path)
            .output()?;
        
        Ok(ExecutionResult {
            stdout: String::from_utf8_lossy(&run_output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&run_output.stderr).to_string(),
            exit_code: run_output.status.code().unwrap_or(-1),
            duration: start.elapsed(),
            error: None,
        })
    }
    
    fn run_with_interpret(&self, test: &TestCase) -> Result<ExecutionResult> {
        let start = Instant::now();
        
        let output = Command::new(&self.interpreter_path)
            .arg("run")
            .arg(&test.path)
            .output()?;
        
        Ok(ExecutionResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            duration: start.elapsed(),
            error: None,
        })
    }
    
    fn run_with_compile(&self, test: &TestCase) -> Result<ExecutionResult> {
        let start = Instant::now();
        
        // Compile
        let compile_output = Command::new(&self.compiler_path)
            .arg("compile")
            .arg(&test.path)
            .arg("--out-dir")
            .arg("/tmp/vb6test")
            .output()?;
        
        if !compile_output.status.success() {
            return Ok(ExecutionResult {
                stdout: String::new(),
                stderr: String::from_utf8_lossy(&compile_output.stderr).to_string(),
                exit_code: compile_output.status.code().unwrap_or(-1),
                duration: start.elapsed(),
                error: Some("Compilation failed".to_string()),
            });
        }
        
        // Execute
        let exe_path = Path::new("/tmp/vb6test").join(
            test.path.file_stem().unwrap()
        );
        let run_output = Command::new(&exe_path)
            .output()?;
        
        Ok(ExecutionResult {
            stdout: String::from_utf8_lossy(&run_output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&run_output.stderr).to_string(),
            exit_code: run_output.status.code().unwrap_or(-1),
            duration: start.elapsed(),
            error: None,
        })
    }
}
```

### Result Comparison (`compare.rs`)

```rust
#[derive(Debug)]
pub enum Difference {
    StdoutMismatch {
        vb6: String,
        ours: String,
        line: usize,
    },
    ExitCodeMismatch {
        vb6: i32,
        ours: i32,
    },
    ErrorOccurred {
        implementation: String,
        error: String,
    },
    NumericPrecision {
        line: usize,
        vb6_value: f64,
        our_value: f64,
        difference: f64,
    },
}

pub struct ResultComparer {
    tolerance: f64,  // Floating-point comparison tolerance
    ignore_whitespace: bool,
}

impl ResultComparer {
    pub fn compare(&self, vb6: &ExecutionResult, ours: &ExecutionResult) -> Vec<Difference> {
        let mut diffs = Vec::new();
        
        // Compare exit codes
        if vb6.exit_code != ours.exit_code {
            diffs.push(Difference::ExitCodeMismatch {
                vb6: vb6.exit_code,
                ours: ours.exit_code,
            });
        }
        
        // Compare stdout
        diffs.extend(self.compare_output(&vb6.stdout, &ours.stdout));
        
        diffs
    }
    
    fn compare_output(&self, vb6: &str, ours: &str) -> Vec<Difference> {
        let mut diffs = Vec::new();
        
        let vb6_lines: Vec<&str> = vb6.lines().collect();
        let our_lines: Vec<&str> = ours.lines().collect();
        
        let max_lines = vb6_lines.len().max(our_lines.len());
        
        for i in 0..max_lines {
            let vb6_line = vb6_lines.get(i).copied().unwrap_or("");
            let our_line = our_lines.get(i).copied().unwrap_or("");
            
            if !self.lines_match(vb6_line, our_line) {
                diffs.push(Difference::StdoutMismatch {
                    vb6: vb6_line.to_string(),
                    ours: our_line.to_string(),
                    line: i + 1,
                });
            }
        }
        
        diffs
    }
    
    fn lines_match(&self, vb6: &str, ours: &str) -> bool {
        if self.ignore_whitespace {
            vb6.trim() == ours.trim()
        } else {
            // Try exact match first
            if vb6 == ours {
                return true;
            }
            
            // Try numeric comparison with tolerance
            if let (Ok(vb6_num), Ok(our_num)) = (vb6.parse::<f64>(), ours.parse::<f64>()) {
                (vb6_num - our_num).abs() < self.tolerance
            } else {
                false
            }
        }
    }
}
```

### Report Generation (`report.rs`)

```rust
pub struct TestReport {
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub results: Vec<TestResult>,
    pub duration: Duration,
}

impl TestReport {
    pub fn generate_html(&self, output_path: &Path) -> Result<()> {
        let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>VB6 Test Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .summary {{ background: #f0f0f0; padding: 15px; margin-bottom: 20px; }}
        .passed {{ color: green; }}
        .failed {{ color: red; }}
        .test {{ border: 1px solid #ccc; margin: 10px 0; padding: 10px; }}
        .diff {{ background: #ffe0e0; padding: 5px; margin: 5px 0; }}
    </style>
</head>
<body>
    <h1>VB6 Test Report</h1>
    <div class="summary">
        <p>Total: {}</p>
        <p class="passed">Passed: {}</p>
        <p class="failed">Failed: {}</p>
        <p>Duration: {:.2}s</p>
    </div>
    {}
</body>
</html>
        "#,
            self.total_tests,
            self.passed,
            self.failed,
            self.duration.as_secs_f64(),
            self.render_results()
        );
        
        std::fs::write(output_path, html)?;
        Ok(())
    }
    
    pub fn generate_junit_xml(&self, output_path: &Path) -> Result<()> {
        // Generate JUnit XML for CI integration
        let xml = format!(r#"
<?xml version="1.0" encoding="UTF-8"?>
<testsuites>
    <testsuite name="VB6 Tests" tests="{}" failures="{}" time="{}">
        {}
    </testsuite>
</testsuites>
        "#,
            self.total_tests,
            self.failed,
            self.duration.as_secs_f64(),
            self.render_testcases_xml()
        );
        
        std::fs::write(output_path, xml)?;
        Ok(())
    }
}
```

## VB6.exe Integration

### Windows

```rust
// Direct execution
Command::new("C:\\Program Files\\Microsoft Visual Studio\\VB98\\VB6.exe")
    .arg("/make")
    .arg("project.vbp")
    .output()?;
```

### Linux/macOS (Wine)

```rust
// Setup Wine environment
Command::new("wine")
    .env("WINEPREFIX", "/home/user/.wine_vb6")
    .arg("C:\\VB98\\VB6.exe")
    .arg("/make")
    .arg("project.vbp")
    .output()?;
```

### Docker Container

```dockerfile
FROM ubuntu:22.04

# Install Wine
RUN apt-get update && apt-get install -y wine wine64

# Copy VB6 installation
COPY vb6_install/ /opt/vb6/

# Setup Wine prefix
ENV WINEPREFIX=/opt/wine_vb6
RUN wine wineboot --init

# Install VB6 in Wine
RUN wine /opt/vb6/setup.exe /silent

CMD ["/bin/bash"]
```

## CI/CD Integration

### GitHub Actions

```yaml
name: VB6 Compatibility Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: windows-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup VB6
        run: |
          # Install VB6 (requires license)
          choco install vb6
      
      - name: Build harness
        run: cargo build --release -p test-harness
      
      - name: Run tests
        run: |
          cargo run --release -p test-harness -- \
            --vb6-path "C:\Program Files\Microsoft Visual Studio\VB98\VB6.exe" \
            --suite tests/suite \
            --report tests/report.html
      
      - name: Upload report
        uses: actions/upload-artifact@v3
        with:
          name: test-report
          path: tests/report.html
      
      - name: Check results
        run: |
          cargo run -p test-harness -- --check-results
```

## Usage

### Run All Tests

```bash
cargo run -p test-harness -- \
    --vb6-path "/path/to/VB6.exe" \
    --interpreter ../target/release/vb6interpret \
    --compiler ../target/release/vb6c \
    --suite tests/suite \
    --report report.html
```

### Run Specific Category

```bash
cargo run -p test-harness -- \
    --category basic \
    --suite tests/suite
```

### Run Single Test

```bash
cargo run -p test-harness -- \
    --test tests/suite/basic/arithmetic.bas
```

### Compare Only (No VB6)

```bash
cargo run -p test-harness -- \
    --no-vb6 \
    --expected tests/expected \
    --suite tests/suite
```

## Test Coverage

Track which VB6 features are tested:

```
Language Features:
  ✓ Arithmetic operators
  ✓ String operators
  ✓ Comparison operators
  ✓ Logical operators
  ✓ If/Then/Else
  ✓ Select Case
  ✓ For loops
  ✓ While loops
  ✓ Do loops
  ✓ Functions
  ✓ Subs
  ✓ Classes
  ⏳ Properties (partial)
  ⏳ Events (partial)
  ⏳ Forms (planned)
  ⏳ Controls (planned)

Standard Library:
  ✓ String functions (95%)
  ✓ Math functions (90%)
  ✓ Date functions (85%)
  ⏳ File I/O (50%)
  ⏳ Format functions (60%)
  ⏳ Conversion functions (80%)

Coverage: 72% of VB6 features
```

## Known Issues Tracking

```toml
[known_issues]

[[known_issues.list]]
test = "floating_point_precision"
issue = "Minor precision differences in trigonometric functions"
severity = "low"
workaround = "Increase tolerance to 1e-10"

[[known_issues.list]]
test = "variant_late_binding"
issue = "Late binding not yet implemented"
severity = "high"
status = "planned"
```

## Performance Benchmarking

```rust
pub struct PerformanceStats {
    pub vb6_time: Duration,
    pub interpret_time: Duration,
    pub compile_time: Duration,
}

impl TestRunner {
    pub fn benchmark(&self, test: &TestCase, iterations: usize) -> PerformanceStats {
        // Run each implementation multiple times and average
    }
}
```

Output:
```
Performance Results:
  VB6:         1.000x (baseline)
  Interpreter: 2.345x (2.35x slower)
  Compiled:    0.891x (11% faster)
```

## Future Enhancements

- [ ] Differential testing (fuzzing)
- [ ] Coverage-guided test generation
- [ ] Automatic issue filing for failures
- [ ] Performance regression detection
- [ ] Visual diff viewer for output
- [ ] Parallel test execution
- [ ] Test result database
- [ ] Historical trend analysis

## Dependencies

- `cargo`: For building Rust components
- `wine`: For running VB6 on Linux/macOS
- VB6.exe: Legacy compiler for validation
- `diff`: For text comparison
- `jq`: For JSON processing (optional)

## License

MIT
