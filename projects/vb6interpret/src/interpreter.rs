//! The interpreter engine.
//!
//! [`Interpreter`] owns the global scope, the call frame stack, and the
//! loaded program. Statements and expressions are executed directly against
//! the CST (see [`crate::eval`] and [`crate::exec`]).

use std::collections::HashMap;

use vb6core::error::VBError;
use vb6parse::files::ModuleFile;
use vb6runtime::Value;

use crate::error::{RunError, RunResult};
use crate::program::Procedure;
use crate::scope::Scope;

/// The maximum execution step budget before the interpreter aborts.
pub const DEFAULT_STEP_LIMIT: u64 = 10_000_000;

/// A single active procedure call.
#[derive(Debug)]
pub(crate) struct Frame {
    /// The procedure name (original casing).
    pub(crate) name: String,
    /// Whether the procedure returns a value (Function).
    pub(crate) is_function: bool,
    /// Local variables for this call.
    pub(crate) locals: Scope,
    /// The value written to the function name (the return value).
    pub(crate) return_value: Option<Value>,
}

/// Control-flow signal produced by statement execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Flow {
    /// Proceed with the next statement.
    Next,
    /// `Exit For` / `Exit Do` / `Exit While`.
    BreakLoop,
    /// `Exit Sub` / `Exit Function` / end of procedure body.
    Return,
    /// `End` statement: terminate the whole program.
    Terminate,
}

/// The VB6 interpreter.
#[derive(Debug)]
pub struct Interpreter {
    pub(crate) globals: Scope,
    pub(crate) frames: Vec<Frame>,
    pub(crate) procedures: HashMap<String, Procedure>,
    /// Completed lines of `Debug.Print`/`Print` output.
    pub(crate) output: Vec<String>,
    /// The partially accumulated current output line (before a newline).
    pub(crate) current_output: String,
    /// The module name from `Attribute VB_Name`.
    pub(crate) module_name: String,
    /// Total statement budget; aborts with an error when exhausted.
    pub(crate) step_limit: u64,
    /// Statements executed so far.
    pub(crate) steps: u64,
    /// Whether an `End` statement terminated the program.
    pub(crate) terminated: bool,
    /// 1-based line of the statement currently executing.
    pub(crate) current_stmt_line: usize,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    /// Create a fresh interpreter.
    pub fn new() -> Self {
        Self {
            globals: Scope::new(),
            frames: Vec::new(),
            procedures: HashMap::new(),
            output: Vec::new(),
            current_output: String::new(),
            module_name: String::new(),
            step_limit: DEFAULT_STEP_LIMIT,
            steps: 0,
            terminated: false,
            current_stmt_line: 1,
        }
    }

    /// Set the maximum number of statements the interpreter will execute
    /// before aborting (prevents infinite loops). Defaults to
    /// [`DEFAULT_STEP_LIMIT`].
    pub fn set_step_limit(&mut self, limit: u64) {
        self.step_limit = limit;
    }

    /// Reset all runtime state (globals, frames, output, program).
    pub fn clear(&mut self) {
        let step_limit = self.step_limit;
        *self = Self::new();
        self.step_limit = step_limit;
    }

    /// Execute a VB6 module from source text.
    pub fn run_source(&mut self, source: &str) -> RunResult<()> {
        let source_file = vb6parse::io::SourceFile::from_string("module.bas", source);
        let module = ModuleFile::parse(&source_file).unwrap_or_fail();
        self.run_module(&module)
    }

    /// Execute a parsed module.
    pub fn run_module(&mut self, module: &ModuleFile) -> RunResult<()> {
        self.clear();
        let root = module.cst.to_root_node();
        let program = crate::program::build_program(&root, &module.name);
        self.module_name = module.name.clone();
        self.procedures = program.procedures;

        // Module-level statements (Dim/Const/Option/...) execute first.
        self.exec_statements(&program.root, 1)?;
        if self.terminated {
            return Ok(());
        }

        // Run the entry procedure.
        let entry = program.entry.clone();
        if let Some(procedure) = self.procedures.get(&entry).cloned() {
            if procedure.is_function {
                self.call_function(&procedure.name, Vec::new())?;
            } else {
                self.call_sub(&procedure.name, Vec::new())?;
            }
        }
        Ok(())
    }

    /// The completed `Debug.Print`/`Print` output lines.
    pub fn output(&self) -> &[String] {
        &self.output
    }

    /// The output as a single string with newlines between lines.
    pub fn output_text(&self) -> String {
        let mut text = self.output.join("\n");
        if !self.current_output.is_empty() {
            if !text.is_empty() {
                text.push('\n');
            }
            text.push_str(&self.current_output);
        }
        text
    }

    /// The module-level (global) scope.
    pub fn globals(&self) -> &Scope {
        &self.globals
    }

    /// Look up a global variable's value.
    pub fn global(&self, name: &str) -> Option<&Value> {
        self.globals.get(name)
    }

    /// Declare or overwrite a global variable before execution.
    pub fn set_global(&mut self, name: &str, value: Value) {
        self.globals.declare(name, value);
    }

    /// Whether an `End` statement terminated execution.
    pub fn is_terminated(&self) -> bool {
        self.terminated
    }

    /// The number of statements executed so far.
    pub fn steps(&self) -> u64 {
        self.steps
    }

    /// The name of the currently executing procedure (empty at module level).
    pub(crate) fn current_procedure_name(&self) -> String {
        self.frames
            .last()
            .map(|f| f.name.clone())
            .unwrap_or_default()
    }

    /// Charge one step against the execution budget.
    pub(crate) fn step(&mut self) -> RunResult<()> {
        self.steps += 1;
        if self.steps > self.step_limit {
            return Err(RunError::err_number(28).at_line(self.current_stmt_line));
            // Out of stack space
        }
        Ok(())
    }

    /// Write output text, honoring the in-progress line buffer.
    pub(crate) fn emit(&mut self, text: String, newline: bool) {
        self.current_output.push_str(&text);
        if newline {
            self.output.push(std::mem::take(&mut self.current_output));
        }
    }

    /// Invoke a Sub procedure, returning its control-flow signal.
    pub(crate) fn call_sub(&mut self, name: &str, args: Vec<Value>) -> RunResult<Flow> {
        let procedure = self.lookup_procedure(name)?;
        let body = procedure.body.clone();
        let body_line = procedure.line + 1;
        self.push_frame(procedure, args)?;
        let result = match body {
            Some(body_node) => self.exec_statements(&body_node, body_line),
            None => Ok(Flow::Next),
        };
        self.frames.pop();
        match result {
            Ok(Flow::Terminate) => Ok(Flow::Terminate),
            Ok(_) => Ok(Flow::Next),
            Err(e) => Err(e),
        }
    }

    /// Invoke a Function procedure, returning its result value.
    pub(crate) fn call_function(&mut self, name: &str, args: Vec<Value>) -> RunResult<Value> {
        let procedure = self.lookup_procedure(name)?;
        let return_type = procedure.return_type.clone();
        let body = procedure.body.clone();
        let body_line = procedure.line + 1;
        self.push_frame(procedure, args)?;
        let result = match body {
            Some(body_node) => self.exec_statements(&body_node, body_line),
            None => Ok(Flow::Next),
        };
        let return_value = self.frames.last().and_then(|f| f.return_value.clone());
        self.frames.pop();
        match result {
            Ok(Flow::Terminate) => {
                self.terminated = true;
                Ok(return_value.unwrap_or_else(|| Value::default_for_type(&return_type)))
            }
            Ok(_) => Ok(return_value.unwrap_or_else(|| Value::default_for_type(&return_type))),
            Err(e) => Err(e),
        }
    }

    /// Fetch a procedure by name, or error 35.
    fn lookup_procedure(&self, name: &str) -> RunResult<Procedure> {
        self.procedures
            .get(&name.to_lowercase())
            .cloned()
            .ok_or_else(|| {
                RunError::new(VBError::new(35))
                    .at_line(self.current_stmt_line)
                    .in_procedure(&self.current_procedure_name())
            })
    }

    /// Push a frame for `procedure`, binding its parameters.
    fn push_frame(&mut self, procedure: Procedure, args: Vec<Value>) -> RunResult<()> {
        let mut frame = Frame {
            name: procedure.name.clone(),
            is_function: procedure.is_function,
            locals: Scope::new(),
            return_value: None,
        };
        for (index, param) in procedure.params.iter().enumerate() {
            let value = match args.get(index) {
                Some(value) => crate::exec::coerce(value.clone(), &param.ty),
                None => {
                    if param.optional {
                        Value::default_for_type(&param.ty)
                    } else {
                        return Err(
                            RunError::new(VBError::new(450)).at_line(self.current_stmt_line)
                        );
                    }
                }
            };
            frame.locals.declare(&param.name, value);
        }
        self.frames.push(frame);
        Ok(())
    }
}
