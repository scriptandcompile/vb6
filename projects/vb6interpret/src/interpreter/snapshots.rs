//! Debugger-oriented state capture.
//!
//! [`Interpreter`] can record a [`DebugSnapshot`] at every statement
//! boundary while a program runs; hosts (the wasm debugger front end and
//! trace tests) consume the recorded snapshots to render step-by-step
//! program state.

use crate::interpreter::Interpreter;
use crate::scope::Scope;

/// A formatted debug snapshot of a variable at a statement boundary.
#[derive(Debug, Clone)]
pub struct DebugVariable {
    /// Variable name.
    pub name: String,
    /// Qualified type name as a string.
    pub type_name: String,
    /// Formatted value of the variable.
    pub value: String,
}

/// A debugger-oriented snapshot captured at a statement boundary.
#[derive(Debug, Clone)]
pub struct DebugSnapshot {
    /// Number of statements executed so far.
    pub steps: u64,
    /// Current line being executed (1-based).
    pub current_line: usize,
    /// Name of the currently executing procedure, if any.
    pub current_procedure: Option<String>,
    /// Current call stack depth.
    pub stack_depth: usize,
    /// Global variables visible at this point.
    pub globals: Vec<DebugVariable>,
    /// Local variables in the current scope.
    pub locals: Vec<DebugVariable>,
    /// Lines written to `Debug.Print` / Immediate window so far.
    pub output_lines: Vec<String>,
    /// All output concatenated into a single string.
    pub output_text: String,
    /// Whether the program has terminated.
    pub terminated: bool,
    /// Byte range `[start, end)` of the specific source element being
    /// executed, when the snapshot targets a sub-line element such as a
    /// loop's counter, step, or `Next`. `None` means the whole line.
    pub cursor_range: Option<(u32, u32)>,
}

impl Interpreter {
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
        if let (Some(before), Some(after)) = (previous, self.debug_snapshots.last())
            && before.steps == after.steps
            && before.current_line == after.current_line
            && before.output_text == after.output_text
            && before.terminated == after.terminated
        {
            self.debug_snapshots.pop();
        }
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
