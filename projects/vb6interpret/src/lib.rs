//! vb6-interpret: VB6 interpreter library
//!
//! A tree-walking interpreter that executes VB6 `.bas` modules directly from
//! the `vb6parse` concrete syntax tree, using `vb6runtime` values.

pub mod builtins;
pub mod error;
pub mod eval;
pub mod exec;
pub mod interpreter;
pub mod program;
pub mod scope;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use error::{RunError, RunResult};
pub use interpreter::Interpreter;
pub use scope::Scope;
pub use vb6runtime::{VBError, Value};

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
