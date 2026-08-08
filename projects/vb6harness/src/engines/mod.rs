//! Engine abstraction: each implementation (interpreter, compiler, legacy
//! VB6) produces `Print` output lines for a test module.

pub mod compiler;
pub mod interpreter;
pub mod vb6;

use std::path::Path;
use std::process::{Child, Command};
use std::time::{Duration, Instant};

/// Identifies an engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineId {
    Interpreter,
    Compiler,
    Vb6,
}

impl EngineId {
    /// Short display label.
    pub fn label(self) -> &'static str {
        match self {
            EngineId::Interpreter => "interpreter",
            EngineId::Compiler => "compiler",
            EngineId::Vb6 => "vb6",
        }
    }
}

/// The captured result of an engine run.
#[derive(Debug, Clone)]
pub struct EngineOutput {
    /// Output lines produced by the program.
    pub lines: Vec<String>,
    /// Engine error text, if the program failed to run.
    pub error: Option<String>,
    /// Wall-clock duration of the run.
    pub duration: Duration,
}

/// The outcome of an engine run: actual output, or an explicit skip.
#[derive(Debug)]
pub enum EngineRun {
    Output(EngineOutput),
    Skipped(String),
}

/// A runnable engine.
pub trait Engine {
    /// The engine identity.
    fn id(&self) -> EngineId;

    /// Run `module_path` and capture its output.
    fn run(&self, module_path: &Path, timeout: Duration) -> anyhow::Result<EngineRun>;
}

/// Spawn `command` and wait for it to finish, killing it after `timeout`.
/// Returns the exit code, or `None` if it timed out.
pub fn run_with_timeout(command: &mut Command, timeout: Duration) -> anyhow::Result<Option<i32>> {
    let started = Instant::now();
    let mut child: Child = command.spawn()?;
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status.code());
        }
        if started.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(None);
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}
