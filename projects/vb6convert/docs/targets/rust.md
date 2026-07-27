# Rust Conversion Backend

## Overview

The Rust conversion backend transforms VB6 projects into safe, idiomatic Rust code. Rust is an excellent target for VB6 conversion due to its strong type system, memory safety, and performance characteristics.

## Goals

1. **Type Safety**: Convert VB6's dynamic types to Rust's static type system
2. **Memory Safety**: Eliminate VB6's unsafe memory practices
3. **Performance**: Generate efficient Rust code
4. **Idiomaticity**: Produce code that feels natural to Rust developers
5. **Maintainability**: Generate readable, well-documented code

## Type Mapping

### Primitive Types

| VB6 Type | Rust Type | Notes |
|----------|-----------|-------|
| Integer | i16 | 16-bit signed integer |
| Long | i32 | 32-bit signed integer |
| Single | f32 | 32-bit floating point |
| Double | f64 | 64-bit floating point |
| String | String | Heap-allocated string |
| Boolean | bool | Native boolean |
| Byte | u8 | 8-bit unsigned integer |
| Currency | Decimal | Requires rust_decimal crate |
| Date | chrono::DateTime | Requires chrono crate |

### Complex Types

| VB6 Type | Rust Type | Notes |
|----------|-----------|-------|
| Variant | Variant enum | Custom enum to represent any type |
| Object | Box<dyn Any> | Type-erased object |
| Array | Vec<T> | Dynamic array |
| Fixed Array | [T; N] | Fixed-size array |
| Collection | HashMap<String, Variant> | Key-value collection |
| User Type | struct | Custom struct |
| Enum | enum | Rust enum |

### Custom Variant Type

```rust
#[derive(Debug, Clone)]
pub enum Variant {
    Empty,
    Null,
    Integer(i16),
    Long(i32),
    Single(f32),
    Double(f64),
    String(String),
    Boolean(bool),
    Date(chrono::DateTime<chrono::Utc>),
    Object(Box<dyn std::any::Any>),
    Array(Vec<Variant>),
    Error(String),
}

impl Variant {
    pub fn as_i32(&self) -> Result<i32> {
        match self {
            Variant::Integer(i) => Ok(*i as i32),
            Variant::Long(l) => Ok(*l),
            Variant::String(s) => s.parse().map_err(|_| ConversionError::TypeMismatch),
            _ => Err(ConversionError::TypeMismatch),
        }
    }
    
    // Additional conversion methods...
}
```

## Module Conversion

### VB6 Module Structure

```vb6
' Module1.bas
Option Explicit

Public Const APP_NAME As String = "MyApp"
Private m_counter As Integer

Public Sub Initialize()
    m_counter = 0
End Sub

Public Function GetNextId() As Integer
    m_counter = m_counter + 1
    GetNextId = m_counter
End Function
```

### Converted Rust

```rust
// module1.rs
//! Converted from VB6 Module1.bas

/// Application name constant
pub const APP_NAME: &str = "MyApp";

/// Counter state
static mut M_COUNTER: i16 = 0;

/// Initialize the module
pub fn initialize() {
    unsafe {
        M_COUNTER = 0;
    }
}

/// Get next ID
pub fn get_next_id() -> i16 {
    unsafe {
        M_COUNTER += 1;
        M_COUNTER
    }
}
```

**Note**: This uses `unsafe` for global mutable state. A better approach would be:

```rust
use std::sync::Mutex;
use lazy_static::lazy_static;

pub const APP_NAME: &str = "MyApp";

lazy_static! {
    static ref COUNTER: Mutex<i16> = Mutex::new(0);
}

pub fn initialize() {
    *COUNTER.lock().unwrap() = 0;
}

pub fn get_next_id() -> i16 {
    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;
    *counter
}
```

## Class Conversion

### VB6 Class

```vb6
' Person.cls
Option Explicit

Private m_name As String
Private m_age As Integer

Public Property Get Name() As String
    Name = m_name
End Property

Public Property Let Name(ByVal value As String)
    m_name = value
End Property

Public Property Get Age() As Integer
    Age = m_age
End Property

Public Property Let Age(ByVal value As Integer)
    If value >= 0 Then
        m_age = value
    End If
End Property

Public Sub Greet()
    MsgBox "Hello, " & m_name
End Sub
```

### Converted Rust

```rust
// person.rs
//! Converted from VB6 Person.cls

#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i16,
}

impl Person {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            age: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }

    pub fn age(&self) -> i16 {
        self.age
    }

    pub fn set_age(&mut self, value: i16) {
        if value >= 0 {
            self.age = value;
        }
    }

    pub fn greet(&self) {
        println!("Hello, {}", self.name);
    }
}

impl Default for Person {
    fn default() -> Self {
        Self::new()
    }
}
```

## Statement Conversion

### Control Flow

| VB6 | Rust |
|-----|------|
| `If...Then...Else...End If` | `if...else` |
| `Select Case...End Select` | `match` or `if/else if` |
| `For...Next` | `for...in` loop |
| `For Each...Next` | `for...in` iterator |
| `While...Wend` | `while` loop |
| `Do While...Loop` | `while` loop |
| `Do Until...Loop` | `while !condition` |
| `Exit Sub/Function` | `return` |
| `GoTo` | `loop` with labels (discouraged) |

### Examples

**If Statement**

```vb6
If x > 0 Then
    MsgBox "Positive"
ElseIf x < 0 Then
    MsgBox "Negative"
Else
    MsgBox "Zero"
End If
```

```rust
if x > 0 {
    println!("Positive");
} else if x < 0 {
    println!("Negative");
} else {
    println!("Zero");
}
```

**Select Case**

```vb6
Select Case value
    Case 1
        result = "One"
    Case 2, 3
        result = "Two or Three"
    Case 4 To 10
        result = "Four to Ten"
    Case Else
        result = "Other"
End Select
```

```rust
let result = match value {
    1 => "One",
    2 | 3 => "Two or Three",
    4..=10 => "Four to Ten",
    _ => "Other",
};
```

**For Loop**

```vb6
For i = 1 To 10
    Debug.Print i
Next i
```

```rust
for i in 1..=10 {
    println!("{}", i);
}
```

**For Each**

```vb6
For Each item In collection
    Debug.Print item.Name
Next item
```

```rust
for item in &collection {
    println!("{}", item.name());
}
```

## Error Handling

VB6's error handling translates to Rust's Result type:

### VB6 Error Handling

```vb6
Public Function ReadFile(path As String) As String
    On Error GoTo ErrorHandler
    
    Dim fso As New FileSystemObject
    Dim file As TextStream
    Set file = fso.OpenTextFile(path, ForReading)
    ReadFile = file.ReadAll
    file.Close
    Exit Function

ErrorHandler:
    MsgBox "Error: " & Err.Description
    ReadFile = ""
End Function
```

### Rust Error Handling

```rust
use std::fs;
use std::io;

pub fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

// Or with custom error type:
pub fn read_file_safe(path: &str) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("Error: {}", e))
}
```

## API and External Function Calls

VB6's `Declare` statements for Windows API calls:

### VB6 API Declaration

```vb6
Private Declare Function MessageBox Lib "user32" Alias "MessageBoxA" _
    (ByVal hwnd As Long, ByVal lpText As String, _
     ByVal lpCaption As String, ByVal uType As Long) As Long
```

### Rust FFI

```rust
#[cfg(windows)]
use winapi::um::winuser::{MessageBoxA, MB_OK};
use std::ffi::CString;

#[cfg(windows)]
pub fn message_box(text: &str, caption: &str) -> i32 {
    unsafe {
        let text = CString::new(text).unwrap();
        let caption = CString::new(caption).unwrap();
        MessageBoxA(
            std::ptr::null_mut(),
            text.as_ptr(),
            caption.as_ptr(),
            MB_OK,
        )
    }
}

#[cfg(not(windows))]
pub fn message_box(text: &str, caption: &str) -> i32 {
    eprintln!("[{}] {}", caption, text);
    0
}
```

## Project Structure

Converted Rust project structure:

```
converted_project/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Library root
│   ├── main.rs             # Entry point (if exe)
│   ├── modules/            # Converted modules
│   │   ├── mod.rs
│   │   ├── module1.rs
│   │   └── module2.rs
│   ├── classes/            # Converted classes
│   │   ├── mod.rs
│   │   ├── person.rs
│   │   └── account.rs
│   ├── forms/              # Converted forms (if using UI)
│   │   ├── mod.rs
│   │   └── main_form.rs
│   └── vb6_runtime/        # VB6 compatibility runtime
│       ├── mod.rs
│       ├── variant.rs
│       ├── strings.rs
│       └── collections.rs
└── tests/
    └── integration_tests.rs
```

### Generated Cargo.toml

```toml
[package]
name = "converted-vb6-project"
version = "0.1.0"
edition = "2021"

[dependencies]
# VB6 compatibility runtime
chrono = "0.4"
rust_decimal = "1.0"
lazy_static = "1.4"

# Windows-specific
[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winuser"] }

[dev-dependencies]
tempfile = "3.0"
```

## Conversion Challenges

### 1. Late Binding

VB6's late binding (Variant/Object types) doesn't translate directly to Rust:

```vb6
Dim obj As Object
Set obj = CreateObject("Excel.Application")
obj.Visible = True  ' Late binding
```

**Solution**: Use trait objects or enum dispatch:

```rust
trait ComObject {
    fn get_property(&self, name: &str) -> Result<Variant>;
    fn set_property(&mut self, name: &str, value: Variant) -> Result<()>;
    fn invoke_method(&mut self, name: &str, args: &[Variant]) -> Result<Variant>;
}

// Or use enum for known types
enum KnownObject {
    Excel(ExcelApplication),
    Word(WordApplication),
    Unknown(Box<dyn ComObject>),
}
```

### 2. ByRef Parameters

VB6 passes by reference by default:

```vb6
Sub Swap(ByRef a As Integer, ByRef b As Integer)
    Dim temp As Integer
    temp = a
    a = b
    b = temp
End Sub
```

```rust
fn swap(a: &mut i16, b: &mut i16) {
    std::mem::swap(a, b);
}
```

### 3. Default Properties

VB6 classes can have default properties. In Rust, implement `Deref`:

```rust
use std::ops::Deref;

impl Deref for MyClass {
    type Target = String;
    
    fn deref(&self) -> &Self::Target {
        &self.default_property
    }
}
```

### 4. Global State

VB6 modules can have module-level variables. Use:
- `lazy_static!` for initialization
- `Mutex` or `RwLock` for thread safety
- Consider refactoring to stateless functions

## Implementation Checklist

- [ ] Basic type conversion
- [ ] Module conversion
- [ ] Class conversion
- [ ] Property conversion (Get/Let/Set)
- [ ] Method conversion
- [ ] Expression conversion
- [ ] Statement conversion
- [ ] Control flow conversion
- [ ] Error handling conversion
- [ ] Array handling
- [ ] Collection handling
- [ ] API call conversion
- [ ] File I/O conversion
- [ ] String operations
- [ ] Date/Time operations
- [ ] Variant support
- [ ] Optional parameters
- [ ] ParamArray conversion
- [ ] Events and callbacks
- [ ] Implements/Interfaces
- [ ] Enums
- [ ] Constants
- [ ] Type declarations

## Testing Strategy

1. Unit test each conversion rule
2. Integration test complete modules
3. Compare behavior with test harness
4. Benchmark performance vs VB6
5. Test thread safety of generated code

## Future Enhancements

- [ ] Async/await for blocking operations
- [ ] Better error messages with source locations
- [ ] Optimization passes
- [ ] Generate documentation from VB6 comments
- [ ] Interactive migration assistant
- [ ] Incremental migration support
