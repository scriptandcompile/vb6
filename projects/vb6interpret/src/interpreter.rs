//! The interpreter engine.
//!
//! [`Interpreter`] owns the global scope, the call frame stack, and the
//! loaded program. Statements and expressions are executed directly against
//! the CST (see [`crate::eval`] and [`crate::exec`]).

use std::collections::HashMap;

use vb6core::error::VBError;
use vb6parse::files::ModuleFile;
use vb6runtime::state::environment as env_state;
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

/// A formatted debug snapshot of a variable at a statement boundary.
#[derive(Debug, Clone)]
pub struct DebugVariable {
    pub name: String,
    pub type_name: String,
    pub value: String,
}

/// A debugger-oriented snapshot captured at a statement boundary.
#[derive(Debug, Clone)]
pub struct DebugSnapshot {
    pub steps: u64,
    pub current_line: usize,
    pub current_procedure: Option<String>,
    pub stack_depth: usize,
    pub globals: Vec<DebugVariable>,
    pub locals: Vec<DebugVariable>,
    pub output_lines: Vec<String>,
    pub output_text: String,
    pub terminated: bool,
    /// Byte range `[start, end)` of the specific source element being
    /// executed, when the snapshot targets a sub-line element such as a
    /// loop's counter, step, or `Next`. `None` means the whole line.
    pub cursor_range: Option<(u32, u32)>,
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
        }
    }

    /// Set the maximum number of statements the interpreter will execute
    /// before aborting (prevents infinite loops). Defaults to
    /// [`DEFAULT_STEP_LIMIT`].
    pub fn set_step_limit(&mut self, limit: u64) {
        self.step_limit = limit;
    }

    /// Assign an environment variable before the next run.
    ///
    /// `Environ$`/`Environ` read these values during execution, on top of the
    /// process environment. The assignment survives [`Interpreter::clear`] and
    /// is re-applied at the start of every run, so it can be configured once
    /// before calling [`Interpreter::run_source`] or [`Interpreter::run_module`].
    pub fn set_environment(&mut self, name: &str, value: &str) {
        self.environment.insert(name.to_string(), value.to_string());
    }

    /// Clear all environment variables installed with [`Interpreter::set_environment`].
    pub fn clear_environment(&mut self) {
        for name in self.environment.keys() {
            env_state::remove_env(name);
        }
        self.environment.clear();
    }

    /// Assign an application setting before the next run.
    ///
    /// `GetSetting` reads these values during execution, on top of any values
    /// already present in the settings store (or on disk). The assignment
    /// survives [`Interpreter::clear`] and is re-applied at the start of every
    /// run, so it can be configured once before calling
    /// [`Interpreter::run_source`] or [`Interpreter::run_module`]. A setting
    /// staged later overrides an earlier one with the same
    /// `(appname, section, key)`.
    pub fn set_setting(&mut self, appname: &str, section: &str, key: &str, value: &str) {
        self.settings.push((
            appname.to_string(),
            section.to_string(),
            key.to_string(),
            value.to_string(),
        ));
    }

    /// The value for `(appname, section, key)`, or `None` when unset.
    ///
    /// Staged settings win over values already in the store; among staged
    /// settings the most recently staged value wins.
    pub fn get_setting(&self, appname: &str, section: &str, key: &str) -> Option<String> {
        for (a, s, k, v) in self.settings.iter().rev() {
            if a.eq_ignore_ascii_case(appname)
                && s.eq_ignore_ascii_case(section)
                && k.eq_ignore_ascii_case(key)
            {
                return Some(v.clone());
            }
        }
        settings_state::get(appname, section, key)
    }

    /// Remove a single setting, both staged and from the store.
    pub fn remove_setting(&mut self, appname: &str, section: &str, key: &str) {
        self.settings.retain(|(a, s, k, _)| {
            !(a.eq_ignore_ascii_case(appname)
                && s.eq_ignore_ascii_case(section)
                && k.eq_ignore_ascii_case(key))
        });
        let _ = settings_state::remove_key(appname, section, key);
    }

    /// Remove every setting staged with [`Interpreter::set_setting`], both
    /// staged and from the store.
    pub fn clear_settings(&mut self) {
        for (appname, section, key, _) in &self.settings {
            let _ = settings_state::remove_key(appname, section, key);
        }
        self.settings.clear();
    }

    /// Redirect the settings store to `root` for this interpreter.
    ///
    /// Equivalent to [`vb6runtime::state::settings::set_store_root`], scoped
    /// to the interpreter for convenience.
    pub fn set_settings_store_root(&self, root: impl Into<std::path::PathBuf>) {
        settings_state::set_store_root(root);
    }

    /// Set the active settings backend for this interpreter.
    ///
    /// Equivalent to [`vb6runtime::state::settings::set_backend`], scoped
    /// to the interpreter for convenience. After switching, all settings
    /// are reloaded from the new backend.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use vb6interpret::Interpreter;
    /// use vb6runtime::state::settings::memory::MemoryBackend;
    ///
    /// let mut interp = Interpreter::new();
    /// interp.set_settings_backend(Box::new(MemoryBackend::new()));
    /// ```
    pub fn set_settings_backend(
        &self,
        backend: Box<dyn vb6runtime::state::settings::backend::SettingsBackend>,
    ) {
        settings_state::set_backend(backend);
    }

    /// Reset the settings backend to the platform default.
    ///
    /// Equivalent to [`vb6runtime::state::settings::reset_backend`], scoped
    /// to the interpreter for convenience.
    pub fn reset_settings_backend(&self) {
        settings_state::reset_backend();
    }

    /// Set the active file backend for this interpreter.
    ///
    /// Equivalent to [`vb6runtime::state::file::set_backend`], scoped
    /// to the interpreter for convenience.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use vb6interpret::Interpreter;
    /// use vb6runtime::state::file::memory::MemoryBackend;
    ///
    /// let mut interp = Interpreter::new();
    /// interp.set_file_backend(Box::new(MemoryBackend::new()));
    /// ```
    pub fn set_file_backend(&self, backend: Box<dyn vb6runtime::state::file::FileBackend>) {
        vb6runtime::state::file::set_backend(backend);
    }

    /// Reset the file backend to the platform default.
    ///
    /// Equivalent to [`vb6runtime::state::file::reset_backend`], scoped
    /// to the interpreter for convenience.
    pub fn reset_file_backend(&self) {
        vb6runtime::state::file::reset_backend();
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
        *self = Self::new();
        self.step_limit = step_limit;
        self.pause_after_steps = pause_after_steps;
        self.record_debug_snapshots = record_debug_snapshots;
        self.environment = environment;
        self.settings = settings;
        self.allow_system_time = allow_system_time;
        self.initial_date = initial_date;
        self.initial_time = initial_time;
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

    /// Control whether `Date` and `Time` statements may modify the real
    /// system clock.
    ///
    /// - `true` (default): statements write to the real system clock.
    /// - `false`: statements write to an internal mock clock that advances
    ///   in real time from the set point.  The real clock is never touched.
    ///
    /// When set to `false`, the current real date/time is captured as the
    /// mock clock's starting point (unless overridden with
    /// [`set_initial_date`] or [`set_initial_time`]).
    pub fn set_allow_system_time(&mut self, allowed: bool) {
        self.allow_system_time = allowed;
    }

    /// Whether `Date` and `Time` statements may modify the real system clock.
    pub fn allow_system_time(&self) -> bool {
        self.allow_system_time
    }

    /// Set an initial date for the mock clock at the start of a run.
    ///
    /// Automatically disables real-clock writes (equivalent to calling
    /// [`set_allow_system_time(false)`](Self::set_allow_system_time)).
    /// When set, the mock clock starts at this date (preserving the current
    /// time-of-day) instead of the real system date.
    pub fn set_initial_date(&mut self, date: vb6runtime::civil::Date) {
        self.allow_system_time = false;
        self.initial_date = Some(date);
    }

    /// Clear any initial date override.
    pub fn clear_initial_date(&mut self) {
        self.initial_date = None;
    }

    /// Set an initial time for the mock clock at the start of a run.
    ///
    /// Automatically disables real-clock writes (equivalent to calling
    /// [`set_allow_system_time(false)`](Self::set_allow_system_time)).
    /// When set, the mock clock starts at this time (preserving the current
    /// date) instead of the real system time.
    pub fn set_initial_time(&mut self, time: vb6runtime::civil::Time) {
        self.allow_system_time = false;
        self.initial_time = Some(time);
    }

    /// Set both the initial date and time for the mock clock at the start
    /// of a run.
    ///
    /// Automatically disables real-clock writes.  This is a convenience
    /// shorthand for calling [`set_initial_date`] and [`set_initial_time`]
    /// together.
    pub fn set_initial_date_time(
        &mut self,
        date: vb6runtime::civil::Date,
        time: vb6runtime::civil::Time,
    ) {
        self.allow_system_time = false;
        self.initial_date = Some(date);
        self.initial_time = Some(time);
    }

    /// Clear any initial time override.
    pub fn clear_initial_time(&mut self) {
        self.initial_time = None;
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

    /// Capture the current interpreter state as a debugger snapshot.
    ///
    /// Consumes any pending `current_stmt_range`, so sub-line cursors are
    /// attached to exactly one snapshot and never leak into later ones.
    pub fn capture_debug_snapshot(&mut self) {
        let current_procedure = self.current_procedure().map(str::to_string);
        let globals = scope_to_debug_variables(&self.globals);
        let locals = self
            .current_locals()
            .map(scope_to_debug_variables)
            .unwrap_or_default();
        let cursor_range = self.current_stmt_range.take();

        let snapshot = DebugSnapshot {
            steps: self.steps,
            current_line: self.current_line(),
            current_procedure,
            stack_depth: self.frames.len(),
            globals,
            locals,
            output_lines: self.output().to_vec(),
            output_text: self.output_text(),
            terminated: self.is_terminated(),
            cursor_range,
        };

        let should_push = self.debug_snapshots.last().is_none_or(|last| {
            last.steps != snapshot.steps
                || last.current_line != snapshot.current_line
                || last.current_procedure != snapshot.current_procedure
                || last.output_text != snapshot.output_text
                || last.terminated != snapshot.terminated
                || last.cursor_range != snapshot.cursor_range
        });

        if should_push {
            self.debug_snapshots.push(snapshot);
        }
    }

    /// Capture the post-run state for the trace's final step.
    ///
    /// Drops the capture when it duplicates the previous snapshot in every way
    /// except the call stack, which happens right after a procedure exits
    /// normally: the `End Sub` snapshot already represents the finished state.
    pub fn capture_final_debug_snapshot(&mut self) {
        let previous = self.debug_snapshots.last().cloned();
        self.capture_debug_snapshot();
        if let (Some(before), Some(after)) = (previous, self.debug_snapshots.last()) {
            if before.steps == after.steps
                && before.current_line == after.current_line
                && before.output_text == after.output_text
                && before.terminated == after.terminated
            {
                self.debug_snapshots.pop();
            }
        }
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

    /// Write output text, honoring the in-progress line buffer.
    pub(crate) fn emit(&mut self, text: String, newline: bool) {
        self.current_output.push_str(&text);
        if newline {
            self.output.push(std::mem::take(&mut self.current_output));
        }
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
                RunError::new(VBError::new(35))
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

fn scope_to_debug_variables(scope: &Scope) -> Vec<DebugVariable> {
    let mut variables: Vec<DebugVariable> = scope
        .iter()
        .map(|(name, value)| DebugVariable {
            name: name.to_string(),
            type_name: value.type_of().to_string(),
            value: format!("{value}"),
        })
        .collect();
    variables.sort_by(|left, right| left.name.cmp(&right.name));
    variables
}
