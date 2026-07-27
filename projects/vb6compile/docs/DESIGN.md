# vb6compile Design Document

## Overview

`vb6compile` (command-line tool: `vb6c`) is an ahead-of-time compiler that transforms VB6 source code into native executables or other target languages. It uses `vb6core` for shared runtime functionality and supports multiple backend code generators.

## Goals

1. **Native Performance**: Generate code as fast or faster than VB6.exe
2. **Cross-Platform**: Compile to multiple targets (Windows, Linux, macOS, Web)
3. **Optimization**: Apply modern compiler optimizations
4. **Correctness**: Preserve exact VB6 semantics
5. **Maintainability**: Generate readable, debuggable code

## Architecture

### Component Structure

```
vb6compile/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library interface
│   ├── pipeline/
│   │   ├── mod.rs
│   │   ├── parse.rs         # Parsing stage
│   │   ├── analyze.rs       # Semantic analysis
│   │   ├── lower.rs         # AST → IR lowering
│   │   ├── optimize.rs      # IR optimization
│   │   └── codegen.rs       # Backend dispatch
│   ├── backend/
│   │   ├── mod.rs           # Backend trait
│   │   ├── rust/
│   │   │   ├── mod.rs
│   │   │   ├── codegen.rs   # Rust code generator
│   │   │   ├── types.rs     # Type mappings
│   │   │   └── stdlib.rs    # Stdlib call generation
│   │   ├── llvm/
│   │   │   ├── mod.rs
│   │   │   ├── codegen.rs   # LLVM IR generator
│   │   │   ├── types.rs     # LLVM type mappings
│   │   │   └── intrinsics.rs
│   │   └── javascript/
│   │       ├── mod.rs
│   │       ├── codegen.rs   # JS code generator
│   │       └── runtime.rs   # JS runtime helpers
│   ├── optimizer/
│   │   ├── mod.rs
│   │   ├── constant_fold.rs
│   │   ├── dead_code.rs
│   │   ├── inline.rs
│   │   ├── specialization.rs
│   │   └── loop_opt.rs
│   ├── linker.rs            # Link generated code
│   └── cli.rs               # CLI argument parsing
├── tests/
│   ├── codegen_tests.rs
│   ├── optimization_tests.rs
│   └── integration/
└── benches/
    └── codegen.rs
```

## Compilation Pipeline

### Stage 1: Parsing

```rust
use vb6parse::parsers::*;

pub struct Parser {
    options: ParseOptions,
}

impl Parser {
    pub fn parse_file(&self, path: &Path) -> Result<AST> {
        // Use vb6parse to create AST
        let source = std::fs::read_to_string(path)?;
        
        if path.extension() == Some("bas") {
            parse_module(&source)
        } else if path.extension() == Some("cls") {
            parse_class(&source)
        } else if path.extension() == Some("frm") {
            parse_form(&source)
        } else if path.extension() == Some("vbp") {
            parse_project(path)
        } else {
            Err(anyhow!("Unknown file type"))
        }
    }
}
```

### Stage 2: Semantic Analysis

```rust
use vb6semantic::SemanticAnalyzer;

pub struct Analyzer {
    analyzer: SemanticAnalyzer,
}

impl Analyzer {
    pub fn analyze(&mut self, ast: &AST) -> Result<AnalysisResult> {
        // Run semantic analysis
        let result = self.analyzer.analyze_project(ast)?;
        
        // Check for errors
        if !result.errors.is_empty() {
            return Err(CompileError::SemanticErrors(result.errors));
        }
        
        Ok(result)
    }
}
```

### Stage 3: IR Lowering

Transform AST to vb6core IR:

```rust
use vb6core::ir::*;

pub struct IrLowerer {
    current_function: Option<IRFunction>,
    label_counter: usize,
}

impl IrLowerer {
    pub fn lower_module(&mut self, module: &Module) -> Result<IRModule> {
        let mut ir_module = IRModule {
            name: module.name.clone(),
            functions: Vec::new(),
            globals: Vec::new(),
        };
        
        // Lower global variables
        for var in &module.variables {
            ir_module.globals.push(self.lower_variable(var)?);
        }
        
        // Lower functions and subs
        for func in &module.functions {
            ir_module.functions.push(self.lower_function(func)?);
        }
        
        Ok(ir_module)
    }
    
    fn lower_function(&mut self, func: &Function) -> Result<IRFunction> {
        let mut ir_func = IRFunction {
            name: func.name.clone(),
            parameters: func.parameters.iter()
                .map(|p| (p.name.clone(), p.typ.clone()))
                .collect(),
            return_type: func.return_type.clone(),
            locals: Vec::new(),
            instructions: Vec::new(),
        };
        
        self.current_function = Some(ir_func.clone());
        
        // Lower function body
        for stmt in &func.body {
            self.lower_statement(stmt, &mut ir_func)?;
        }
        
        Ok(ir_func)
    }
    
    fn lower_statement(&mut self, stmt: &Statement, func: &mut IRFunction) -> Result<()> {
        match stmt {
            Statement::Assignment { target, value } => {
                let val = self.lower_expression(value, func)?;
                func.instructions.push(Instruction::StoreLocal(
                    target.clone(),
                    val,
                ));
            }
            
            Statement::If { condition, then_branch, else_branch } => {
                let cond = self.lower_expression(condition, func)?;
                let else_label = self.new_label();
                let end_label = self.new_label();
                
                func.instructions.push(Instruction::JumpIfFalse(cond, else_label.clone()));
                
                // Then branch
                for stmt in then_branch {
                    self.lower_statement(stmt, func)?;
                }
                func.instructions.push(Instruction::Jump(end_label.clone()));
                
                // Else branch
                func.instructions.push(Instruction::Label(else_label));
                if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.lower_statement(stmt, func)?;
                    }
                }
                
                func.instructions.push(Instruction::Label(end_label));
            }
            
            // ... handle all statement types
            _ => todo!("Lower statement: {:?}", stmt),
        }
        
        Ok(())
    }
    
    fn new_label(&mut self) -> String {
        let label = format!("L{}", self.label_counter);
        self.label_counter += 1;
        label
    }
}
```

### Stage 4: Optimization

Apply optimization passes to IR:

```rust
pub struct Optimizer {
    level: OptLevel,
    passes: Vec<Box<dyn OptPass>>,
}

pub enum OptLevel {
    O0,  // No optimization
    O1,  // Basic optimization
    O2,  // Default optimization
    O3,  // Aggressive optimization
    Os,  // Size optimization
}

pub trait OptPass {
    fn name(&self) -> &str;
    fn run(&mut self, module: &mut IRModule) -> Result<OptStats>;
}

impl Optimizer {
    pub fn new(level: OptLevel) -> Self {
        let passes: Vec<Box<dyn OptPass>> = match level {
            OptLevel::O0 => vec![],
            OptLevel::O1 => vec![
                Box::new(ConstantFoldingPass),
                Box::new(DeadCodeEliminationPass),
            ],
            OptLevel::O2 => vec![
                Box::new(ConstantFoldingPass),
                Box::new(ConstantPropagationPass),
                Box::new(CommonSubexpressionPass),
                Box::new(DeadCodeEliminationPass),
                Box::new(InliningPass::new(100)),  // Inline functions < 100 instructions
            ],
            OptLevel::O3 => vec![
                Box::new(ConstantFoldingPass),
                Box::new(ConstantPropagationPass),
                Box::new(CommonSubexpressionPass),
                Box::new(DeadCodeEliminationPass),
                Box::new(InliningPass::new(500)),  // Aggressive inlining
                Box::new(LoopUnrollingPass),
                Box::new(SpecializationPass),
            ],
            OptLevel::Os => vec![
                Box::new(ConstantFoldingPass),
                Box::new(DeadCodeEliminationPass),
                Box::new(StringDedupPass),
            ],
        };
        
        Self { level, passes }
    }
    
    pub fn optimize(&mut self, module: &mut IRModule) -> Result<()> {
        for pass in &mut self.passes {
            log::debug!("Running optimization pass: {}", pass.name());
            let stats = pass.run(module)?;
            log::debug!("  {}", stats);
        }
        Ok(())
    }
}
```

### Stage 5: Code Generation

Generate target code from IR:

```rust
pub trait CodeGenerator {
    fn generate_module(&mut self, module: &IRModule) -> Result<GeneratedCode>;
    fn generate_function(&mut self, function: &IRFunction) -> Result<String>;
    fn generate_instruction(&mut self, instr: &Instruction) -> Result<String>;
}

pub struct GeneratedCode {
    pub files: HashMap<PathBuf, String>,
    pub entry_point: Option<String>,
    pub dependencies: Vec<String>,
}

impl CodeGenerator for RustBackend {
    fn generate_module(&mut self, module: &IRModule) -> Result<GeneratedCode> {
        let mut output = String::new();
        
        // Generate module header
        output.push_str("// Generated by vb6c\n\n");
        output.push_str("use vb6core::prelude::*;\n\n");
        
        // Generate globals
        for (name, typ, init) in &module.globals {
            writeln!(output, "static mut {}: {} = {};",
                self.mangle_name(name),
                self.map_type(typ),
                self.generate_initializer(typ, init)?
            )?;
        }
        
        output.push('\n');
        
        // Generate functions
        for func in &module.functions {
            output.push_str(&self.generate_function(func)?);
            output.push('\n');
        }
        
        Ok(GeneratedCode {
            files: [(PathBuf::from("src/main.rs"), output)].into(),
            entry_point: Some("main".to_string()),
            dependencies: vec!["vb6core".to_string()],
        })
    }
    
    fn generate_function(&mut self, function: &IRFunction) -> Result<String> {
        let mut output = String::new();
        
        // Function signature
        write!(output, "pub fn {}(", self.mangle_name(&function.name))?;
        
        for (i, (name, typ)) in function.parameters.iter().enumerate() {
            if i > 0 {
                write!(output, ", ")?;
            }
            write!(output, "{}: {}", name, self.map_type(typ))?;
        }
        
        write!(output, ")")?;
        
        if let Some(ret_type) = &function.return_type {
            write!(output, " -> {}", self.map_type(ret_type))?;
        }
        
        writeln!(output, " {{")?;
        
        // Local variables
        for (name, typ) in &function.locals {
            writeln!(output, "    let mut {}: {} = {};",
                name,
                self.map_type(typ),
                self.default_value(typ)
            )?;
        }
        
        if !function.locals.is_empty() {
            writeln!(output)?;
        }
        
        // Instructions
        for instr in &function.instructions {
            writeln!(output, "    {}", self.generate_instruction(instr)?)?;
        }
        
        writeln!(output, "}}")?;
        
        Ok(output)
    }
}
```

## Backend Implementations

### Rust Backend

**Type Mappings**:
```rust
impl RustBackend {
    fn map_type(&self, vb_type: &VBType) -> String {
        match vb_type {
            VBType::Byte => "u8",
            VBType::Integer => "i16",
            VBType::Long => "i32",
            VBType::Single => "f32",
            VBType::Double => "f64",
            VBType::String => "String",
            VBType::Boolean => "bool",
            VBType::Variant => "vb6_core::Value",
            VBType::Object(Some(class)) => format!("Rc<dyn {}>", class),
            VBType::Object(None) => "Rc<dyn VbObject>",
            VBType::Array { element_type, .. } => {
                format!("vb6_core::Array<{}>", self.map_type(element_type))
            }
            VBType::UserDefined(name) => name.clone(),
        }.to_string()
    }
}
```

**Instruction Generation**:
```rust
impl RustBackend {
    fn generate_instruction(&mut self, instr: &Instruction) -> Result<String> {
        Ok(match instr {
            Instruction::Add(left, right) => {
                format!("{} + {}", 
                    self.generate_value(left)?,
                    self.generate_value(right)?)
            }
            
            Instruction::Call { function, arguments, result } => {
                let args = arguments.iter()
                    .map(|arg| self.generate_value(arg))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                    
                let call = format!("{}({})", function, args);
                
                if let Some(var) = result {
                    format!("{} = {};", var, call)
                } else {
                    format!("{};", call)
                }
            }
            
            Instruction::Label(label) => {
                format!("'{}:", label)  // Rust label
            }
            
            Instruction::Jump(label) => {
                format!("goto '{}; // Generated goto", label)
            }
            
            // ... more instructions
            _ => format!("/* TODO: {:?} */", instr),
        })
    }
}
```

### LLVM Backend

**Type Mappings**:
```rust
use inkwell::types::*;
use inkwell::context::Context;

impl LLVMBackend {
    fn map_type<'ctx>(&self, vb_type: &VBType, context: &'ctx Context) -> BasicTypeEnum<'ctx> {
        match vb_type {
            VBType::Byte => context.i8_type().into(),
            VBType::Integer => context.i16_type().into(),
            VBType::Long => context.i32_type().into(),
            VBType::Single => context.f32_type().into(),
            VBType::Double => context.f64_type().into(),
            VBType::Boolean => context.bool_type().into(),
            VBType::String => context.i8_type().ptr_type(AddressSpace::from(0)).into(),
            VBType::Variant => {
                // Variant is a struct with type tag and value union
                self.get_variant_type(context).into()
            }
            // ... more types
        }
    }
    
    fn get_variant_type<'ctx>(&self, context: &'ctx Context) -> StructType<'ctx> {
        // struct Variant { i16 type_tag; union { i64, f64, ptr } value; }
        context.struct_type(&[
            context.i16_type().into(),  // type tag
            context.i64_type().into(),  // value (large enough for any type)
        ], false)
    }
}
```

**Code Generation**:
```rust
impl LLVMBackend {
    fn generate_add(&mut self, left: &Value, right: &Value) -> Result<BasicValueEnum> {
        let lhs = self.generate_value(left)?;
        let rhs = self.generate_value(right)?;
        
        // Determine types and generate appropriate instruction
        match (lhs.get_type(), rhs.get_type()) {
            (BasicTypeEnum::IntType(_), BasicTypeEnum::IntType(_)) => {
                Ok(self.builder.build_int_add(
                    lhs.into_int_value(),
                    rhs.into_int_value(),
                    "add"
                )?.into())
            }
            (BasicTypeEnum::FloatType(_), BasicTypeEnum::FloatType(_)) => {
                Ok(self.builder.build_float_add(
                    lhs.into_float_value(),
                    rhs.into_float_value(),
                    "fadd"
                )?.into())
            }
            _ => {
                // Call runtime function for complex types
                self.call_runtime_function("vb6_add", &[lhs, rhs])
            }
        }
    }
}
```

### JavaScript Backend

**Type Mappings**:
```javascript
// All VB6 types map to JavaScript types
// No type annotations in pure JS, but TypeScript optional

// Byte, Integer, Long, Single, Double → number
// String → string
// Boolean → boolean
// Variant → any
// Object → object
// Array → Array
```

**Code Generation**:
```rust
impl JavaScriptBackend {
    fn generate_function(&mut self, function: &IRFunction) -> Result<String> {
        let mut output = String::new();
        
        // Function declaration
        write!(output, "function {}(", function.name)?;
        
        for (i, (name, _)) in function.parameters.iter().enumerate() {
            if i > 0 {
                write!(output, ", ")?;
            }
            write!(output, "{}", name)?;
        }
        
        writeln!(output, ") {{")?;
        
        // Local variables (initialize to defaults)
        for (name, typ) in &function.locals {
            writeln!(output, "    let {} = {};",
                name,
                self.default_value_js(typ)
            )?;
        }
        
        // Instructions
        for instr in &function.instructions {
            writeln!(output, "    {}", self.generate_instruction_js(instr)?)?;
        }
        
        writeln!(output, "}}")?;
        
        Ok(output)
    }
    
    fn generate_instruction_js(&mut self, instr: &Instruction) -> Result<String> {
        Ok(match instr {
            Instruction::Add(left, right) => {
                format!("{} + {}",
                    self.generate_value_js(left)?,
                    self.generate_value_js(right)?)
            }
            
            Instruction::Call { function, arguments, result } => {
                let args = arguments.iter()
                    .map(|arg| self.generate_value_js(arg))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                
                let call = format!("{}({})", function, args);
                
                if let Some(var) = result {
                    format!("{} = {};", var, call)
                } else {
                    format!("{};", call)
                }
            }
            
            // ... more instructions
            _ => format!("/* TODO: {:?} */", instr),
        })
    }
}
```

## Optimization Passes

### Constant Folding

```rust
pub struct ConstantFoldingPass;

impl OptPass for ConstantFoldingPass {
    fn run(&mut self, module: &mut IRModule) -> Result<OptStats> {
        let mut folded = 0;
        
        for func in &mut module.functions {
            for instr in &mut func.instructions {
                if let Some(folded_instr) = self.try_fold(instr) {
                    *instr = folded_instr;
                    folded += 1;
                }
            }
        }
        
        Ok(OptStats {
            name: "Constant Folding",
            changes: folded,
        })
    }
}

impl ConstantFoldingPass {
    fn try_fold(&self, instr: &Instruction) -> Option<Instruction> {
        match instr {
            Instruction::Add(Value::Integer(a), Value::Integer(b)) => {
                Some(Instruction::LoadConstant(Value::Integer(a + b)))
            }
            Instruction::Mul(Value::Integer(a), Value::Integer(b)) => {
                Some(Instruction::LoadConstant(Value::Integer(a * b)))
            }
            // ... more folding rules
            _ => None,
        }
    }
}
```

### Function Inlining

```rust
pub struct InliningPass {
    max_size: usize,
}

impl OptPass for InliningPass {
    fn run(&mut self, module: &mut IRModule) -> Result<OptStats> {
        let mut inlined = 0;
        
        // Build function size map
        let sizes: HashMap<String, usize> = module.functions.iter()
            .map(|f| (f.name.clone(), f.instructions.len()))
            .collect();
        
        // Find inlining candidates
        for func in &mut module.functions {
            for i in 0..func.instructions.len() {
                if let Instruction::Call { function, .. } = &func.instructions[i] {
                    if let Some(&size) = sizes.get(function) {
                        if size < self.max_size {
                            // Inline this call
                            self.inline_call(func, i, module)?;
                            inlined += 1;
                        }
                    }
                }
            }
        }
        
        Ok(OptStats {
            name: "Function Inlining",
            changes: inlined,
        })
    }
}
```

## Testing Strategy

### Unit Tests
- Type mapping correctness
- Instruction generation
- Optimization passes
- Code formatting

### Integration Tests
- Complete VB6 programs
- Cross-backend consistency
- Performance benchmarks

### Validation Tests
- Compare with VB6 compiler output
- Verify runtime behavior
- Check optimization correctness

## Performance Considerations

### Compilation Speed
- Parallel compilation of modules
- Incremental compilation cache
- Fast IR representation

### Runtime Performance
- Zero-cost abstractions where possible
- Inline standard library calls
- Optimize hot paths based on profile data

### Memory Usage
- Stream code generation (don't buffer entire output)
- Release IR after code generation
- Efficient symbol tables

## CLI Design

Full CLI specification in README.md

Key features:
- Multiple backends selectable via `--backend`
- Optimization levels via `-O0` to `-O3`
- Output control via `--emit`
- Cross-compilation via `--target`
- Debug symbols via `--debug` or `-g`

## Future Enhancements

- [ ] Profile-guided optimization
- [ ] Distributed compilation
- [ ] Custom backend plugins
- [ ] IDE integration (LSP)
- [ ] Hot reloading for faster development
- [ ] Whole-program optimization
- [ ] Dead code elimination across modules

## Dependencies

- `vb6parse`: ^0.5.0
- `vb6semantic`: ^0.1.0
- `vb6core`: ^0.1.0
- `clap`: ^4.0
- `heck`: ^0.4
- `indoc`: ^2.0
- `inkwell`: ^0.4 (optional, LLVM backend)
- `rustfmt-wrapper`: ^0.2

## License

MIT
