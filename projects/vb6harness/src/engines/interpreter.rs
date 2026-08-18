//! The `vb6interpret` engine, run in-process through its library API.
//!
//! Corpus modules are VB6-native and write output with `Print #1, <expr>`,
//! assuming file #1 is already open (see `engines::vb6`'s generated startup
//! module). The interpreter itself no longer treats `Print #1` as console
//! output, so this engine mirrors that convention: it opens an in-memory
//! file as #1 before running, then reads it back afterward.

use super::{Engine, EngineId, EngineOutput, EngineRun};
use anyhow::{Context, Result};
use std::path::Path;
use std::time::{Duration, Instant};
use vb6interpret::Interpreter;
use vb6runtime::state::file as file_state;
use vb6runtime::state::file::memory::MemoryBackend;
use vb6runtime::state::file::{AccessMode, LockMode, OpenMode};

/// The file name the harness's implicit `#1` output file is opened as.
const OUTPUT_FILE: &str = "out.txt";

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
        interpreter.set_file_backend(Box::new(MemoryBackend::new()));
        file_state::open_file(
            Path::new(OUTPUT_FILE),
            OpenMode::Output,
            AccessMode::ReadWrite,
            LockMode::Shared,
            0,
            1,
        )
        .context("Failed to open harness output file as #1")?;

        let started = Instant::now();
        let result = interpreter.run_source(&source);
        let duration = started.elapsed();

        let _ = file_state::close_file(1);
        let content = file_state::list_memory_files()
            .ok()
            .and_then(|files| {
                files
                    .into_iter()
                    .find(|(path, _, _)| path.ends_with(OUTPUT_FILE))
            })
            .and_then(|(_, _, content)| content)
            .unwrap_or_default();

        // Most corpus modules write via `Print #1` (opened above to mirror
        // real VB6), but a few exercise bare `Print`/`Debug.Print`, which are
        // console-only extensions with no file backing.
        let text = if content.is_empty() {
            interpreter.output_text().to_string()
        } else {
            String::from_utf8_lossy(&content).into_owned()
        };
        let lines = text.split('\n').map(str::to_string).collect();
        let error = result.err().map(|e| e.to_string());
        Ok(EngineRun::Output(EngineOutput {
            lines,
            error,
            duration,
        }))
    }
}
