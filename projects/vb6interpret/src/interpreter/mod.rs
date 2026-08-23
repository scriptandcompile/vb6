//! The interpreter engine.
//!
//! [`Interpreter`] owns the global scope, the call frame stack, and the
//! loaded program. Statements and expressions are executed directly against
//! the CST (see [`crate::eval`] and [`crate::exec`]).
//!
//! Submodules hold the host-facing configuration knobs ([`config`]), the
//! debugger snapshot machinery ([`snapshots`], re-exported below so existing
//! `crate::interpreter::{DebugSnapshot, DebugVariable}` paths keep working),
//! and the built-in constant registration.

mod builtin_constants;
mod config;
mod snapshots;

pub use snapshots::{DebugSnapshot, DebugVariable};

use std::collections::HashMap;

use vb6core::error::err_number;
use vb6core::error::VBError;
use vb6parse::files::ModuleFile;
use vb6runtime::state::environment as env_state;
use vb6runtime::state::resources as resources_state;
use vb6runtime::state::settings as settings_state;
use vb6runtime::VBVariant;

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
    pub(crate) return_value: Option<VBVariant>,
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
    /// Number of leading source lines omitted from the CST, such as VB6
    /// header attributes.
    pub(crate) source_line_offset: usize,
    /// Total statement budget; aborts with an error when exhausted.
    pub(crate) step_limit: u64,
    /// Statements executed so far.
    pub(crate) steps: u64,
    /// Whether an `End` statement terminated the program.
    pub(crate) terminated: bool,
    /// 1-based line of the statement currently executing.
    pub(crate) current_stmt_line: usize,
    /// Byte range `[start, end)` of the specific source element being
    /// executed, when a snapshot should target a sub-line element. Consumed
    /// by the next snapshot capture.
    pub(crate) current_stmt_range: Option<(u32, u32)>,
    /// Optional debugger pause budget. When set, execution pauses before the
    /// next statement once `steps` reaches the configured value.
    pub(crate) pause_after_steps: Option<u64>,
    /// Whether execution should record statement-boundary snapshots.
    pub(crate) record_debug_snapshots: bool,
    /// Statement-boundary snapshots captured during the current run.
    pub(crate) debug_snapshots: Vec<DebugSnapshot>,
    /// Environment variables installed via [`Interpreter::set_environment`],
    /// written into the shared runtime snapshot at the start of every run.
    pub(crate) environment: HashMap<String, String>,
    /// Settings staged via [`Interpreter::set_setting`], written into the
    /// shared settings store at the start of every run.
    pub(crate) settings: Vec<(String, String, String, String)>,
    /// Whether the `Date` and `Time` statements are allowed to modify the
    /// real system clock.  When `false`, these statements write to an
    /// internal mock clock that advances in real time from the set point.
    /// Defaults to `true`.
    pub(crate) allow_system_time: bool,
    /// Optional initial date to set on the mock clock at the start of a run.
    /// Only used when `allow_system_time` is `false`.
    pub(crate) initial_date: Option<vb6runtime::civil::Date>,
    /// Optional initial time to set on the mock clock at the start of a run.
    /// Only used when `allow_system_time` is `false`.
    pub(crate) initial_time: Option<vb6runtime::civil::Time>,
    /// Path of the `.res` file staged via [`Interpreter::set_resource_file`],
    /// linked into the shared runtime at the start of every run so the
    /// `LoadRes*` functions can read it. `None` means the program has no
    /// resource file, as a VB6 project that never added one.
    pub(crate) resource_file: Option<String>,
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
            source_line_offset: 0,
            step_limit: DEFAULT_STEP_LIMIT,
            steps: 0,
            terminated: false,
            current_stmt_line: 1,
            current_stmt_range: None,
            pause_after_steps: None,
            record_debug_snapshots: false,
            debug_snapshots: Vec::new(),
            environment: HashMap::new(),
            settings: Vec::new(),
            allow_system_time: true,
            initial_date: None,
            initial_time: None,
            resource_file: None,
        }
    }

    /// Set the maximum number of statements the interpreter will execute
    /// before aborting (prevents infinite loops). Defaults to
    /// [`DEFAULT_STEP_LIMIT`].
    pub fn set_step_limit(&mut self, limit: u64) {
        self.step_limit = limit;
    }

    /// Reset all interpreter state (globals, frames, output, program).
    pub fn clear(&mut self) {
        let step_limit = self.step_limit;
        let pause_after_steps = self.pause_after_steps;
        let record_debug_snapshots = self.record_debug_snapshots;
        let environment = std::mem::take(&mut self.environment);
        let settings = std::mem::take(&mut self.settings);
        let allow_system_time = self.allow_system_time;
        let initial_date = self.initial_date;
        let initial_time = self.initial_time;
        let resource_file = std::mem::take(&mut self.resource_file);
        *self = Self::new();
        self.step_limit = step_limit;
        self.pause_after_steps = pause_after_steps;
        self.record_debug_snapshots = record_debug_snapshots;
        self.environment = environment;
        self.settings = settings;
        self.allow_system_time = allow_system_time;
        self.initial_date = initial_date;
        self.initial_time = initial_time;
        self.resource_file = resource_file;
    }

    /// Pause execution before the next statement once this many statements
    /// have completed.
    pub fn set_pause_after_steps(&mut self, pause_after_steps: Option<u64>) {
        self.pause_after_steps = pause_after_steps;
    }

    /// Enable or disable statement-boundary snapshot recording.
    pub fn set_record_debug_snapshots(&mut self, enabled: bool) {
        self.record_debug_snapshots = enabled;
        self.debug_snapshots.clear();
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
        // Register built-in VB6 data constants (vbCrLf, vbTab, etc.)
        self.register_builtin_constants();
        // Apply the interpreter's environment overrides on top of the shared
        // runtime snapshot so `Environ$` sees them during this run.
        for (name, value) in &self.environment {
            env_state::set_env(name, value);
        }
        // Apply staged settings on top of the shared settings store so
        // `GetSetting` sees them during this run. Failures are ignored when
        // there is no store location (e.g. wasm) — staged settings still win
        // in [`Interpreter::get_setting`].
        for (appname, section, key, value) in &self.settings {
            let _ = settings_state::set(appname, section, key, value);
        }
        // Link the resource file so `LoadRes*` can read it. Re-linking drops
        // any cached parse, so a run always sees the file's current contents
        // even if the host rewrote it between runs.
        match &self.resource_file {
            Some(path) => resources_state::set_file(path),
            None => resources_state::clear(),
        }
        // Configure the clock. When the real clock is allowed the mock clock
        // stays at offset zero and reads from the system directly. When it
        // is disabled, snapshot the real time and apply any initial overrides.
        if !self.allow_system_time {
            vb6runtime::state::clock::reset();
            if let Some(date) = self.initial_date {
                vb6runtime::state::clock::set_date(date);
            }
            if let Some(time) = self.initial_time {
                vb6runtime::state::clock::set_time(time);
            }
        } else {
            vb6runtime::state::clock::reset();
        }
        let root = module.cst.to_root_node();
        let program = crate::program::build_program(&root, &module.name);
        self.module_name = module.name.clone();
        self.source_line_offset = module.line_offset;
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
    pub fn global(&self, name: &str) -> Option<&VBVariant> {
        self.globals.get(name)
    }

    /// Declare or overwrite a global variable before execution.
    pub fn set_global(&mut self, name: &str, value: VBVariant) {
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

    /// The line of the statement about to execute, or the most recently
    /// executed line after completion.
    pub fn current_line(&self) -> usize {
        self.current_stmt_line + self.source_line_offset
    }

    /// The currently executing procedure, when inside a procedure body.
    pub fn current_procedure(&self) -> Option<&str> {
        self.frames.last().map(|frame| frame.name.as_str())
    }

    /// The current frame locals, when inside a procedure body.
    pub fn current_locals(&self) -> Option<&Scope> {
        self.frames.last().map(|frame| &frame.locals)
    }

    /// Recorded statement-boundary snapshots for the current run.
    pub fn debug_snapshots(&self) -> &[DebugSnapshot] {
        &self.debug_snapshots
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
        if self.record_debug_snapshots {
            self.capture_debug_snapshot();
        }
        self.advance_steps()
    }

    /// Charge one step against the budget without capturing a snapshot.
    ///
    /// Used for statements that emit their own element-level trace snapshots
    /// (loops), avoiding a duplicate whole-line highlight at their start.
    pub(crate) fn step_without_snapshot(&mut self) -> RunResult<()> {
        self.advance_steps()
    }

    /// Charge one step against the execution budget, marking the snapshot
    /// with a sub-line cursor over `range`.
    ///
    /// When debug snapshots are not being recorded this behaves exactly like
    /// [`Interpreter::step`], so plain executions are unaffected.
    pub(crate) fn step_marked(&mut self, range: Option<(u32, u32)>) -> RunResult<()> {
        if self.record_debug_snapshots {
            self.current_stmt_range = range;
            self.capture_debug_snapshot();
        }
        self.advance_steps()
    }

    /// Enforce the pause budget, increment the step counter, and enforce the
    /// statement limit.
    fn advance_steps(&mut self) -> RunResult<()> {
        if self
            .pause_after_steps
            .is_some_and(|pause_after_steps| self.steps >= pause_after_steps)
        {
            return Err(RunError::debug_pause()
                .at_line(self.current_stmt_line)
                .in_procedure(&self.current_procedure_name()));
        }

        self.steps += 1;
        if self.steps > self.step_limit {
            return Err(RunError::err_number(28).at_line(self.current_stmt_line));
            // Out of stack space
        }
        Ok(())
    }

    /// Invoke a Sub procedure, returning its control-flow signal.
    pub(crate) fn call_sub(&mut self, name: &str, args: Vec<VBVariant>) -> RunResult<Flow> {
        let procedure = self.lookup_procedure(name)?;
        let body = procedure.body.clone();
        let body_line = procedure.line + 1;
        let entry_line = procedure.line;
        let end_line = procedure.end_line;
        self.push_frame(procedure, args)?;

        if self.record_debug_snapshots {
            self.current_stmt_line = entry_line;
            self.capture_debug_snapshot();
        }

        let result = match body {
            Some(body_node) => self.exec_statements(&body_node, body_line),
            None => Ok(Flow::Next),
        };

        let normal_end = matches!(&result, Ok(Flow::Next));
        if normal_end {
            // A normal return leaves the reported position on the
            // procedure's closing line so the final highlight lands on
            // `End Sub` rather than the last body statement. In trace mode
            // this is also captured as a snapshot.
            self.current_stmt_line = end_line;
            if self.record_debug_snapshots {
                self.capture_debug_snapshot();
            }
        }
        self.frames.pop();

        match result {
            Ok(Flow::Terminate) => Ok(Flow::Terminate),
            Ok(_) => Ok(Flow::Next),
            Err(e) => Err(e),
        }
    }

    /// Invoke a Function procedure, returning its result value.
    pub(crate) fn call_function(
        &mut self,
        name: &str,
        args: Vec<VBVariant>,
    ) -> RunResult<VBVariant> {
        let procedure = self.lookup_procedure(name)?;
        let return_type = procedure.return_type.clone();
        let body = procedure.body.clone();
        let body_line = procedure.line + 1;
        let entry_line = procedure.line;
        let end_line = procedure.end_line;
        self.push_frame(procedure, args)?;

        if self.record_debug_snapshots {
            self.current_stmt_line = entry_line;
            self.capture_debug_snapshot();
        }

        let result = match body {
            Some(body_node) => self.exec_statements(&body_node, body_line),
            None => Ok(Flow::Next),
        };
        let return_value = self.frames.last().and_then(|f| f.return_value.clone());
        let normal_end = matches!(&result, Ok(Flow::Next));
        if normal_end {
            // Same as `call_sub`: a normal return leaves the reported
            // position on the procedure's `End Function` line.
            self.current_stmt_line = end_line;
            if self.record_debug_snapshots {
                self.capture_debug_snapshot();
            }
        }
        self.frames.pop();

        match result {
            Ok(Flow::Terminate) => {
                self.terminated = true;
                Ok(return_value.unwrap_or_else(|| VBVariant::default_for_type(&return_type)))
            }
            Ok(_) => Ok(return_value.unwrap_or_else(|| VBVariant::default_for_type(&return_type))),
            Err(e) => Err(e),
        }
    }

    /// Fetch a procedure by name, or error 35.
    fn lookup_procedure(&self, name: &str) -> RunResult<Procedure> {
        self.procedures
            .get(&name.to_lowercase())
            .cloned()
            .ok_or_else(|| {
                RunError::new(VBError::new(err_number::SUB_OR_FUNCTION_NOT_DEFINED))
                    .at_line(self.current_stmt_line)
                    .in_procedure(&self.current_procedure_name())
            })
    }

    /// Push a frame for `procedure`, binding its parameters.
    fn push_frame(&mut self, procedure: Procedure, args: Vec<VBVariant>) -> RunResult<()> {
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
                        VBVariant::default_for_type(&param.ty)
                    } else {
                        return Err(RunError::new(VBError::new(
                            err_number::WRONG_NUMBER_OF_ARGUMENTS,
                        ))
                        .at_line(self.current_stmt_line));
                    }
                }
            };
            frame.locals.declare(&param.name, value);
        }
        self.frames.push(frame);
        Ok(())
    }
}
