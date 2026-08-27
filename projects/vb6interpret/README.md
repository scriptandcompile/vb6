# vb6interpret

A Visual Basic 6 interpreter that executes VB6 code directly without compilation.

## Overview

`vb6interpret` provides an execution environment for VB6 programs, allowing them to run directly from source code. It includes a REPL for interactive experimentation and supports both script mode (single files) and project mode (full VB6 projects).

## Features

- **Direct Execution**: Run VB6 code without compilation
- **REPL Mode**: Interactive VB6 shell for experimentation
- **Script Mode**: Execute single .bas or .cls files
- **Standard Library**: Full VB6 standard function support
- **Forms Support**: Execute form-based applications *(future)*

## Architecture

```
┌───────────────────────────────────────┐
│           vb6interpret               │
│                                       │
│  ┌─────────┐      ┌─────────────┐   │
│  │  REPL   │      │   Debugger  │   │
│  └────┬────┘      └──────┬──────┘   │
│       │                  │           │
│       └──────┬───────────┘           │
│              │                       │
│     ┌────────▼─────────┐            │
│     │   Interpreter    │            │
│     │     Engine       │            │
│     └────────┬─────────┘            │
│              │                       │
└──────────────┼───────────────────────┘
               │
         ┌──────▼──────┐
         │  vb6runtime │  ← runtime values, I/O, standard library
         └─────────────┘
```

## Installation

```bash
# Install from source
cargo install --path .

# Or build and run
cargo build --release
./target/release/vb6interpret
```

## Usage

### Execute a Script

Run a single VB6 module file:

```bash
vb6interpret run script.bas
```

### Execute a Project

Run a complete VB6 project:

```bash
vb6interpret run MyProject.vbp
```

## Examples

### Simple Script

```vb
' hello.bas
Sub Main()
    Dim name As String
    name = InputBox("Enter your name:")
    MsgBox "Hello, " & name & "!"
End Sub
```

Run it:
```bash
vb6interpret run hello.bas
```

### Math Calculator

```vb
' calc.bas
Function Calculate(x As Double, y As Double, op As String) As Double
    Select Case op
        Case "+"
            Calculate = x + y
        Case "-"
            Calculate = x - y
        Case "*"
            Calculate = x * y
        Case "/"
            If y <> 0 Then
                Calculate = x / y
            Else
                Err.Raise 11, , "Division by zero"
            End If
    End Select
End Function

Sub Main()
    Print Calculate(10, 5, "+")   ' 15
    Print Calculate(10, 5, "-")   ' 5
    Print Calculate(10, 5, "*")   ' 50
    Print Calculate(10, 5, "/")   ' 2
End Sub
```

### Interactive Loop

```vb
Sub Main()
    Dim input As String
    Do
        input = InputBox("Enter command (quit to exit):")
        If LCase$(input) = "quit" Then Exit Do
        
        Print "You entered: " & input
    Loop
End Sub
```

## Performance

**Note:** The specific performance numbers below are not yet benchmarked. They represent rough estimates based on interpreter overhead.

The interpreter is optimized for:
- **Fast Startup**: Minimal overhead before execution
- **Efficient Execution**: Tree-walk interpretation with optimizations
- **Low Memory**: Conservative memory usage
- **Quick Iterations**: Fast REPL response times

Typical performance (unverified estimates):
- Simple arithmetic: ~2-3x slower than compiled VB6
- String operations: ~1.5-2x slower than compiled VB6
- Function calls: Minimal overhead
- REPL responsiveness: ~10ms per command

## Limitations

Current limitations (to be addressed):
- Forms and controls *(partially implemented)*
- COM objects *(future)*
- Binary file I/O *(partial)*
- API calls *(future)*
- Threading *(future)*
- IDE integration *(planned)*

## Implementation Details

### Execution Pipeline

1. **Parse**: Use vb6parse to create CST (Concrete Syntax Tree)
2. **Analyze**: Run semantic analysis
3. **Execute**: Walk the CST and evaluate using `vb6runtime` values

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Benchmarks
cargo bench
```

### Debugging the Interpreter

```bash
# Run with trace logging
RUST_LOG=debug vb6interpret run --trace script.bas

# Profile execution
cargo flamegraph -- run script.bas
```

## Future Enhancements

- [ ] Form execution
- [ ] JIT compilation for hot paths
- [ ] Parallel execution (where safe)
- [ ] GUI debugger
- [ ] IDE integration (LSP server)
- [ ] Web-based REPL
- [ ] Remote debugging
- [ ] Time-travel debugging
- [ ] Memory profiler
- [ ] Coverage analysis

## License

MIT License - see LICENSE file for details.
