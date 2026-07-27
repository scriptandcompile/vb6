# Getting Started with vb6convert Development

Welcome! This guide will help you get started with contributing to vb6convert.

## Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- Basic understanding of VB6 (helpful but not required)
- Familiarity with the target language you want to implement

## Quick Start

### 1. Build the Project

```bash
cd /path/to/vb6
cargo build -p vb6convert
```

### 2. Run Tests

```bash
cargo test -p vb6convert
```

### 3. Try the CLI

```bash
# List available targets
cargo run -p vb6convert -- targets

# This will show "not implemented" but demonstrates the CLI works
cargo run -p vb6convert -- analyze path/to/project.vbp
```

## Project Structure

```
vb6convert/
├── docs/                          # 📚 Start here!
│   ├── ARCHITECTURE.md            # System design and structure
│   ├── IMPLEMENTATION_GUIDE.md    # Step-by-step how-to
│   ├── TESTING.md                 # Testing strategy
│   ├── ROADMAP.md                 # Implementation plan
│   └── targets/                   # Target-specific guides
│       ├── rust.md
│       ├── javascript.md
│       └── tauri.md
│
├── src/
│   ├── lib.rs                     # Library entry point
│   ├── main.rs                    # CLI entry point
│   ├── traits.rs                  # 🔑 Core trait definitions
│   ├── types.rs                   # Common types
│   ├── error.rs                   # Error handling
│   ├── converters.rs              # Converter registry
│   ├── analysis.rs                # Project analysis
│   └── validation.rs              # Validation framework
│
└── tests/                         # Integration tests
```

## What to Read First

1. **[ARCHITECTURE.md](ARCHITECTURE.md)** - Understand the overall design (15 min read)
2. **[traits.rs](../src/traits.rs)** - See what you need to implement (10 min read)
3. **[IMPLEMENTATION_GUIDE.md](IMPLEMENTATION_GUIDE.md)** - Step-by-step instructions (20 min read)
4. **Target guide** - Read the guide for your target language (15 min read)
   - [rust.md](targets/rust.md) for Rust
   - [javascript.md](targets/javascript.md) for JavaScript/TypeScript
   - [tauri.md](targets/tauri.md) for Tauri

**Total time**: ~1 hour to understand the codebase

## Choose Your Path

### Path A: Implement a Converter 🚀

**Best for**: Adding a new conversion target (Rust, JavaScript, Dart, etc.)

1. Read [IMPLEMENTATION_GUIDE.md](IMPLEMENTATION_GUIDE.md)
2. Follow the checklist for implementing `ProjectConverter`
3. Start with simple module conversion
4. Add tests as you go
5. Gradually add more features

**What you'll implement**:
- Module converter (functions, variables, constants)
- Class converter (properties, methods)
- Expression converter (arithmetic, logic, strings)
- Statement converter (if/for/while)
- Type converter (VB6 → target types)

**Time estimate**: 2-4 weeks for basic functionality

### Path B: Improve the Framework 🔧

**Best for**: Enhancing core functionality

Areas to work on:
- **Project Analyzer**: Detect VB6 features and complexity
- **Validation Framework**: Compare converted code with original
- **CLI Improvements**: Better error messages, progress bars
- **Testing Infrastructure**: Test harness, fixtures

**Time estimate**: 1-2 weeks per component

### Path C: Add Test Cases 🧪

**Best for**: Contributing without deep coding

What we need:
- Real-world VB6 code samples
- Expected conversion outputs
- Edge cases and corner cases
- Performance benchmarks

**Time estimate**: Ongoing, bite-sized contributions

### Path D: Documentation & Examples 📝

**Best for**: Improving developer experience

Areas to improve:
- Tutorial content
- Code examples
- API documentation
- Migration guides
- Blog posts / articles

**Time estimate**: A few hours to a few days

## Example: Adding a Simple Converter

Let's walk through adding a minimal "example" converter:

### Step 1: Add Feature Flag

Edit `Cargo.toml`:

```toml
[features]
example-target = []
```

### Step 2: Create Module

Create `src/example.rs`:

```rust
use crate::traits::*;
use crate::types::*;
use crate::error::Result;

pub struct ExampleConverter;

impl ExampleConverter {
    pub fn new() -> Self {
        Self
    }
}

impl ProjectConverter for ExampleConverter {
    fn name(&self) -> &str {
        "example"
    }

    fn description(&self) -> &str {
        "Example converter for demonstration"
    }

    fn convert_project(&self, _project: &Project<'_>, _config: &ConversionConfig) 
        -> Result<ConversionResult> 
    {
        Ok(ConversionResult {
            generated_files: vec![],
            warnings: vec![],
            stats: ConversionStats::default(),
        })
    }

    fn supports_feature(&self, _feature: VB6Feature) -> bool {
        false  // Supports nothing yet
    }

    fn required_dependencies(&self) -> Vec<Dependency> {
        vec![]
    }
}
```

### Step 3: Register It

In `src/lib.rs`, add:

```rust
#[cfg(feature = "example-target")]
pub mod example;
```

In `src/converters.rs`, add:

```rust
#[cfg(feature = "example-target")]
{
    registry.register(Arc::new(crate::example::ExampleConverter::new()));
}
```

### Step 4: Test It

```bash
cargo build --features example-target
cargo run --features example-target -- targets
```

You should see "example" listed!

## Development Workflow

### Daily Development

```bash
# Make changes
# ...

# Check compilation
cargo check -p vb6convert

# Run tests
cargo test -p vb6convert

# Format code
cargo fmt

# Check for issues
cargo clippy -p vb6convert
```

### Before Committing

```bash
# Run all checks
cargo check --workspace
cargo test --workspace
cargo fmt --check
cargo clippy --workspace -- -D warnings

# Build documentation
cargo doc --no-deps --open
```

## Common Tasks

### Add a New Type Mapping

Edit `src/types.rs` or your converter's `types.rs` module.

### Add a New VB6 Feature

1. Add to `VB6Feature` enum in `src/traits.rs`
2. Implement detection in `src/analysis.rs`
3. Implement conversion in your converter

### Add a Test Case

Create a file in `tests/fixtures/`:

```
tests/fixtures/my_test/
  ├── Project.vbp
  ├── Module1.bas
  └── expected/
      └── module1.rs  (or .js, etc.)
```

Add test in `tests/my_converter.rs`:

```rust
#[test]
fn test_my_case() {
    // Test implementation
}
```

## Tips for Success

### Start Small
Don't try to implement everything at once. Start with:
1. Simple module with one function
2. Basic variable declarations
3. One control flow statement
4. Gradually expand

### Test Continuously
Write a test for each feature you implement. It's easier to debug small pieces than large systems.

### Read vb6parse Docs
The [vb6parse documentation](https://docs.rs/vb6parse) is essential for understanding the parsed VB6 structure.

### Ask for Help
- Open an issue for questions
- Join discussions
- Look at existing implementations for examples

### Document as You Go
Add comments, update docs, write examples. Future you (and others) will thank you!

## Resources

### Within This Project
- [ARCHITECTURE.md](ARCHITECTURE.md) - System design
- [IMPLEMENTATION_GUIDE.md](IMPLEMENTATION_GUIDE.md) - Implementation steps
- [TESTING.md](TESTING.md) - Testing guide
- [ROADMAP.md](ROADMAP.md) - Long-term plan
- [targets/](targets/) - Target-specific guides

### External
- [vb6parse docs](https://docs.rs/vb6parse) - Parser documentation
- [VB6 Language Reference](https://docs.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa266146(v=vs.60))
- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Cargo reference

## Getting Help

- **Issues**: Report bugs or request features
- **Discussions**: Ask questions, share ideas
- **Code Review**: Submit PRs for feedback
- **Documentation**: Improve docs through PRs

## Next Steps

1. ✅ Read [ARCHITECTURE.md](ARCHITECTURE.md)
2. ✅ Read [IMPLEMENTATION_GUIDE.md](IMPLEMENTATION_GUIDE.md)
3. ✅ Pick a converter to implement
4. ✅ Set up your development environment
5. ✅ Write your first test
6. ✅ Implement your first feature
7. ✅ Submit a PR!

Welcome to the project! 🎉
