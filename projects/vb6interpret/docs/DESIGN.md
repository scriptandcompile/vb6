# vb6interpret Design Document

## Overview

`vb6interpret` is a VB6 interpreter that executes VB6 code directly without ahead-of-time compilation. It provides both a REPL for interactive use and batch execution for scripts and projects.

## Goals

1. **Rapid Iteration**: Fast edit-run cycle for development and testing
2. **Full VB6 Compatibility**: Execute all valid VB6 code correctly
3. **Interactive Development**: REPL for experimentation
4. **Debugging Support**: Step-through debugging with inspection
5. **Reference Implementation**: Validate compiler correctness

## Architecture

### Component Structure

```
vb6interpret/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library interface
│   ├── engine/
│   │   ├── mod.rs           # Interpreter engine
│   │   ├── executor.rs      # IR execution
│   │   ├── stack.rs         # Operand stack
│   │   └── optimizer.rs     # Runtime optimizations
│   ├── repl/
│   │   ├── mod.rs           # REPL implementation
│   │   ├── commands.rs      # REPL commands
│   │   ├── completion.rs    # Tab completion
│   │   └── history.rs       # Command history
│   ├── debugger/
│   │   ├── mod.rs           # Debugger interface
│   │   ├── breakpoint.rs    # Breakpoint management
│   │   ├── step.rs          # Step execution
│   │   └── inspect.rs       # Variable inspection
│   ├── pipeline/
│   │   ├── mod.rs
│   │   ├── parse.rs         # Parsing stage
│   │   ├── analyze.rs       # Semantic analysis
│   │   ├── lower.rs         # AST to IR lowering
│   │   └── optimize.rs      # IR optimization
│   └── cli.rs               # CLI argument parsing
├── tests/
│   ├── interpreter_tests.rs
│   ├── repl_tests.rs
│   └── integration/
└── benches/
    └── interpreter.rs
```

## Core Components

### 1. Interpreter Engine (`engine/`)

The heart of the interpreter - executes IR instructions.

```rust
pub struct InterpreterEngine {
    /// Runtime context (from vb6core)
    context: RuntimeContext,
    
    /// Operand stack for expression evaluation
    operand_stack: Vec<Value>,
    
    /// Instruction pointer
    ip: usize,
    
    /// Current function being executed
    current_function: Option<IRFunction>,
    
    /// Loaded modules
    modules: HashMap<String, IRModule>,
    
    /// Debugger state
    debugger: Option<DebuggerState>,
    
    /// Performance statistics
    stats: ExecutionStats,
}

impl InterpreterEngine {
    pub fn new() -> Self;
    
    /// Load and prepare a module for execution
    pub fn load_module(&mut self, module: IRModule) -> Result<()>;
    
    /// Execute a function
    pub fn call_function(&mut self, name: &str, args: Vec<Value>) -> Result<Value>;
    
    /// Execute one instruction
    pub fn step(&mut self) -> Result<StepResult>;
    
    /// Run until completion or breakpoint
    pub fn run(&mut self) -> Result<Value>;
    
    /// Set breakpoint
    pub fn set_breakpoint(&mut self, location: Location) -> BreakpointId;
    
    /// Get current execution state
    pub fn get_state(&self) -> ExecutionState;
}

pub enum StepResult {
    Continue,
    BreakpointHit(BreakpointId),
    FunctionReturn(Value),
    Error(RuntimeError),
    Finished,
}
```

### 2. Instruction Execution (`engine/executor.rs`)

Core instruction execution logic:

```rust
impl InterpreterEngine {
    fn execute_instruction(&mut self, instr: &Instruction) -> Result<()> {
        match instr {
            Instruction::Add(left, right) => {
                let l = self.eval_value(left)?;
                let r = self.eval_value(right)?;
                let result = vb6_core::ops::add(&l, &r)?;
                self.operand_stack.push(result);
            }
            
            Instruction::Call { function, arguments, result } => {
                let args = arguments.iter()
                    .map(|arg| self.eval_value(arg))
                    .collect::<Result<Vec<_>>>()?;
                
                let return_value = self.call_function(function, args)?;
                
                if let Some(var) = result {
                    self.context.set_local(var, return_value);
                }
            }
            
            Instruction::JumpIfFalse(condition, label) => {
                let cond = self.eval_value(condition)?;
                if !cond.to_boolean()? {
                    self.jump_to_label(label)?;
                }
            }
            
            Instruction::Return(value) => {
                let ret_val = if let Some(v) = value {
                    self.eval_value(v)?
                } else {
                    Value::Empty
                };
                
                return Err(Return(ret_val));  // Use error for control flow
            }
            
            // ... handle all other instructions
        }
        
        Ok(())
    }
    
    fn eval_value(&mut self, value: &Value) -> Result<Value> {
        match value {
            Value::Variable(name) => {
                // Look up in current scope
                self.context.get_local(name)
                    .or_else(|_| self.context.get_global(name))
                    .cloned()
            }
            _ => Ok(value.clone())
        }
    }
}
```

### 3. REPL Implementation (`repl/`)

Interactive VB6 shell:

```rust
pub struct Repl {
    /// Interpreter engine
    engine: InterpreterEngine,
    
    /// Line editor
    editor: rustyline::Editor<ReplHelper>,
    
    /// Command history
    history: Vec<String>,
    
    /// Current context (for multi-line input)
    context: ReplContext,
}

impl Repl {
    pub fn new() -> Result<Self>;
    
    /// Start REPL loop
    pub fn run(&mut self) -> Result<()> {
        self.print_banner();
        
        loop {
            let line = self.read_line()?;
            
            match self.process_line(&line) {
                Ok(ReplAction::Continue) => continue,
                Ok(ReplAction::Exit) => break,
                Ok(ReplAction::Value(v)) => println!("{}", v),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        
        Ok(())
    }
    
    fn process_line(&mut self, line: &str) -> Result<ReplAction> {
        // Check for REPL commands
        if line.starts_with(':') {
            return self.execute_command(line);
        }
        
        // Try to parse as statement
        if let Ok(stmt) = parse_statement(line) {
            self.execute_statement(stmt)
        } else {
            // Try as expression
            if let Ok(expr) = parse_expression(line) {
                let value = self.evaluate_expression(expr)?;
                Ok(ReplAction::Value(value))
            } else {
                Err(anyhow!("Invalid input"))
            }
        }
    }
    
    fn execute_command(&mut self, cmd: &str) -> Result<ReplAction> {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        
        match parts[0] {
            ":quit" => Ok(ReplAction::Exit),
            ":vars" => {
                self.print_variables();
                Ok(ReplAction::Continue)
            }
            ":load" => {
                if let Some(file) = parts.get(1) {
                    self.load_file(file)?;
                }
                Ok(ReplAction::Continue)
            }
            ":help" => {
                self.print_help();
                Ok(ReplAction::Continue)
            }
            _ => Err(anyhow!("Unknown command: {}", parts[0]))
        }
    }
}

pub enum ReplAction {
    Continue,
    Exit,
    Value(Value),
}
```

### 4. Debugger (`debugger/`)

Step-through debugging support:

```rust
pub struct Debugger {
    /// Breakpoints
    breakpoints: HashMap<BreakpointId, Breakpoint>,
    
    /// Next breakpoint ID
    next_id: BreakpointId,
    
    /// Step mode
    step_mode: StepMode,
    
    /// Watch expressions
    watches: Vec<String>,
}

pub struct Breakpoint {
    pub id: BreakpointId,
    pub location: Location,
    pub condition: Option<String>,
    pub hit_count: usize,
    pub enabled: bool,
}

pub enum Location {
    Function(String),
    Line(String, usize),
    Address(usize),
}

pub enum StepMode {
    Off,
    StepOver,   // Step to next line, don't enter functions
    StepInto,   // Step into function calls
    StepOut,    // Run until function returns
}

impl Debugger {
    pub fn set_breakpoint(&mut self, location: Location) -> BreakpointId;
    pub fn remove_breakpoint(&mut self, id: BreakpointId);
    pub fn should_break(&mut self, location: &Location) -> bool;
    pub fn add_watch(&mut self, expression: String);
    pub fn evaluate_watches(&self, context: &RuntimeContext) -> Vec<(String, Value)>;
}
```

### 5. Execution Pipeline (`pipeline/`)

Transform VB6 source to executable IR:

```rust
pub struct ExecutionPipeline {
    parser: Parser,
    analyzer: SemanticAnalyzer,
    lowerer: IrLowerer,
    optimizer: IrOptimizer,
}

impl ExecutionPipeline {
    pub fn compile(&mut self, source: &str) -> Result<IRModule> {
        // 1. Parse
        let ast = self.parser.parse(source)?;
        
        // 2. Semantic analysis
        let symbols = self.analyzer.analyze(&ast)?;
        
        // 3. Lower to IR
        let mut ir = self.lowerer.lower(&ast, &symbols)?;
        
        // 4. Optimize
        self.optimizer.optimize(&mut ir)?;
        
        Ok(ir)
    }
    
    pub fn compile_file(&mut self, path: &Path) -> Result<IRModule> {
        let source = std::fs::read_to_string(path)?;
        self.compile(&source)
    }
    
    pub fn compile_project(&mut self, project_path: &Path) -> Result<Vec<IRModule>> {
        let project = vb6parse::parsers::parse_project(project_path)?;
        
        let mut modules = Vec::new();
        
        // Compile each module
        for module_ref in project.modules() {
            let module = self.compile_file(&module_ref.path)?;
            modules.push(module);
        }
        
        Ok(modules)
    }
}
```

## Optimization Strategies

### Interpreter-Level Optimizations

1. **Direct Threading**:
   ```rust
   // Instead of switch on instruction type
   // Use computed goto (via function pointers)
   type InstructionHandler = fn(&mut InterpreterEngine) -> Result<()>;
   
   const INSTRUCTION_TABLE: &[InstructionHandler] = &[
       execute_add,
       execute_sub,
       execute_mul,
       // ...
   ];
   ```

2. **Inline Caching**:
   ```rust
   // Cache property/method lookups
   struct InlineCache {
       last_type: Option<TypeId>,
       cached_offset: usize,
   }
   ```

3. **Type Specialization**:
   ```rust
   // Generate specialized versions for common types
   fn add_integers(left: i32, right: i32) -> i32;
   fn add_doubles(left: f64, right: f64) -> f64;
   // vs generic add(Value, Value) -> Value
   ```

4. **Constant Folding**:
   ```rust
   // At load time, evaluate constant expressions
   // 2 + 3 * 4 → 14 (at load)
   ```

### IR-Level Optimizations

1. **Dead Code Elimination**: Remove unreachable code
2. **Constant Propagation**: Replace variables with known values
3. **Common Subexpression Elimination**: Avoid duplicate computations
4. **Loop Optimizations**: Hoist invariants, unroll small loops

## Performance Targets

| Operation | Target Performance | Notes |
|-----------|-------------------|-------|
| Simple arithmetic | 2-3x slower than VB6 | Integer/Long operations |
| String operations | 1.5-2x slower | With UTF-8 conversion |
| Function calls | < 50ns overhead | vs direct call |
| Variable access | < 10ns | Hash lookup |
| REPL response | < 10ms | For typical command |
| Startup time | < 100ms | Load and start REPL |

## Error Handling

### Runtime Errors

```rust
pub enum RuntimeError {
    /// Type mismatch
    TypeMismatch {
        expected: VBType,
        got: VBType,
        location: Location,
    },
    
    /// Undefined variable
    UndefinedVariable {
        name: String,
        location: Location,
    },
    
    /// Division by zero
    DivisionByZero {
        location: Location,
    },
    
    /// Array subscript out of range
    SubscriptOutOfRange {
        index: i32,
        bounds: (i32, i32),
        location: Location,
    },
    
    /// VB6 runtime error (Err.Raise)
    VB6Error {
        number: i32,
        description: String,
        source: String,
    },
}
```

### Error Recovery

In REPL mode, recover from errors gracefully:
```rust
impl Repl {
    fn handle_error(&mut self, error: RuntimeError) {
        // Print error
        eprintln!("{}", error.display());
        
        // Reset interpreter state
        self.engine.reset_to_global_scope();
        
        // Continue REPL
    }
}
```

## Testing Strategy

### Unit Tests
- Individual instruction execution
- Value conversions
- Stack operations
- Error handling

### Integration Tests
- Complete VB6 programs
- REPL command sequences
- Debugger operations
- Multi-file projects

### Compatibility Tests
- Run VB6 test suite
- Compare output with VB6.exe
- Validate numeric precision
- Check string handling

### Performance Tests
- Benchmark against VB6
- Profile hot paths
- Memory usage tests
- Startup time tests

## CLI Design

```
vb6interpret 0.1.0
Visual Basic 6 Interpreter

USAGE:
    vb6interpret [OPTIONS] [COMMAND]

COMMANDS:
    run          Execute a VB6 file or project
    repl         Start interactive REPL (default)
    debug        Run with debugger attached
    check        Check syntax without executing
    help         Print help information

OPTIONS:
    -v, --verbose                 Verbose output
    -t, --trace                   Show execution trace
    -p, --profile                 Enable profiling
    --timeout <SECONDS>           Execution timeout [default: 0]
    --set <VAR=VALUE>...          Set initial variables
    --break <LOCATION>...         Set initial breakpoints
    -h, --help                    Print help
    -V, --version                 Print version

EXAMPLES:
    # Start REPL
    vb6interpret

    # Execute a script
    vb6interpret run script.bas

    # Execute with initial values
    vb6interpret run --set "x=10" --set "y=20" script.bas

    # Debug a program
    vb6interpret debug --break "Main" program.bas

    # Profile execution
    vb6interpret run --profile --trace program.bas
```

## Future Enhancements

### JIT Compilation
- Detect hot functions
- Compile to native code
- Fallback to interpreter for rare cases

### IDE Integration
- LSP server for IDE support
- DAP server for debugging
- Code completion using REPL context

### Advanced Debugging
- Time-travel debugging
- Record/replay
- Reverse execution
- Heapprofile

### Distribution
- Standalone executables
- Docker containers
- WebAssembly (browser-based REPL)

## Dependencies

- `vb6parse`: ^0.5.0 (parsing)
- `vb6semantic`: ^0.1.0 (analysis)
- `vb6core`: ^0.1.0 (runtime)
- `clap`: ^4.0 (CLI)
- `rustyline`: ^13.0 (REPL)
- `colored`: ^2.0 (terminal colors)
- `tracing`: ^0.1 (logging)
- `anyhow`: ^1.0 (error handling)

## License

MIT
