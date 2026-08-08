//! The `vb6interpret` engine, run in-process through its library API.

use super::{Engine, EngineId, EngineOutput, EngineRun};
use anyhow::{Context, Result};
use std::path::Path;
use std::time::{Duration, Instant};
use vb6interpret::Interpreter;

/// Runs test modules directly via the interpreter library.
pub struct InterpreterEngine;

impl Engine for InterpreterEngine {
    fn id(&self) -> EngineId {
        EngineId::Interpreter
    }

    fn run(&self, module_path: &Path, _timeout: Duration) -> Result<EngineRun> {
        let source = std::fs::read_to_string(module_path)
            .with_context(|| format!("Failed to read {}", module_path.display()))?;
        let mut interpreter = Interpreter::new();
        let started = Instant::now();
        let result = interpreter.run_source(&source);
        let duration = started.elapsed();
        let lines = interpreter
            .output_text()
            .split('\n')
            .map(str::to_string)
            .collect();
        let error = result.err().map(|e| e.to_string());
        Ok(EngineRun::Output(EngineOutput {
            lines,
            error,
            duration,
        }))
    }
}
