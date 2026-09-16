#![warn(missing_docs)]
//! vb6interpret: VB6 interpreter and REPL
//!
//! A tree-walking interpreter that executes VB6 `.bas` modules directly from
//! the `vb6parse` concrete syntax tree, using `vb6runtime` values.

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub mod builtins;
pub mod error;
pub mod eval;
pub mod exec;
pub mod interpreter;
pub mod program;
pub mod project;
pub mod scope;

/// Tauri command handlers for form rendering (only compiled with `tauri` feature).
#[cfg(feature = "tauri")]
pub mod tauri_cmds;

/// Tauri background engine with IPC channels (only compiled with `tauri` feature).
#[cfg(feature = "tauri")]
pub mod tauri_engine;

#[cfg(feature = "tauri")]
pub use tauri_engine::{TauriCommand, TauriEngine, TauriEngineHandle, TauriResponse};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use error::{RunError, RunResult};
pub use interpreter::Interpreter;
pub use project::{LoadedClass, LoadedForm, LoadedModule, LoadedProject, StartupObject};
pub use scope::Scope;
pub use vb6runtime::{VBError, VBVariant};

/// Execute a VB6 module source string, capturing `Debug.Print` output.
///
/// Returns the captured output lines.
///
/// # Example
///
/// ```
/// use vb6interpret::run_source;
///
/// let output = run_source(
///     "Attribute VB_Name = \"M\"\n\
///      Sub Main()\n\
///          Debug.Print \"hello\"\n\
///      End Sub\n",
/// )
/// .unwrap();
///
/// assert_eq!(output, vec!["hello".to_string()]);
/// ```
pub fn run_source(source: &str) -> Result<Vec<String>, RunError> {
    let mut interpreter = Interpreter::new();
    interpreter.run_source(source)?;
    Ok(interpreter.output().to_vec())
}

/// Execute a loaded VB6 project, capturing `Debug.Print` output.
///
/// Returns the captured output lines.
pub fn run_project(project: &LoadedProject) -> Result<Vec<String>, RunError> {
    let mut interpreter = Interpreter::new();
    interpreter.run_project(project)?;
    Ok(interpreter.output().to_vec())
}
