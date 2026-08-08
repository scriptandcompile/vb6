//! The `vb6compile` engine.
//!
//! The compiler CLI is still a stub and emits no program output, so this
//! engine reports a skip until `vb6compile` can build and run executables.

use super::{Engine, EngineId, EngineRun};
use std::path::Path;
use std::time::Duration;

/// Placeholder engine for the future compiler. Always skipped for now.
pub struct CompilerEngine;

impl Engine for CompilerEngine {
    fn id(&self) -> EngineId {
        EngineId::Compiler
    }

    fn run(&self, _module_path: &Path, _timeout: Duration) -> anyhow::Result<EngineRun> {
        Ok(EngineRun::Skipped(
            "vb6compile does not emit program output yet".to_string(),
        ))
    }
}
