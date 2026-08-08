//! Suite execution: run each test through each engine and compare against the
//! committed goldens.

use crate::compare::{Comparer, Diff};
use crate::engines::{Engine, EngineId, EngineOutput, EngineRun};
use crate::golden;
use crate::suite::TestFile;
use anyhow::{Result, bail};
use std::path::PathBuf;
use std::time::Duration;

/// The outcome of one engine for one test.
#[derive(Debug, Clone)]
pub struct EngineOutcome {
    pub engine: EngineId,
    /// Reason this engine was skipped, when applicable.
    pub skipped: Option<String>,
    /// Engine error text, when the program failed to run.
    pub error: Option<String>,
    pub duration: Duration,
    /// Mismatching lines against the golden.
    pub diffs: Vec<Diff>,
}

/// The outcome of one test across all engines.
#[derive(Debug, Clone)]
pub struct TestOutcome {
    pub name: String,
    pub path: PathBuf,
    pub category: String,
    pub known_issue: Option<String>,
    pub engines: Vec<EngineOutcome>,
}

impl TestOutcome {
    /// Whether the test passes: every engine that produced output matched the
    /// golden, and no engine reported an error. Skipped engines do not fail.
    pub fn passed(&self) -> bool {
        self.engines
            .iter()
            .all(|outcome| outcome.error.is_none() && outcome.diffs.is_empty())
    }
}

/// Runs a suite against a set of engines.
pub struct Runner {
    pub engines: Vec<Box<dyn Engine>>,
    pub golden_dir: PathBuf,
    /// Default numeric tolerance used when comparing output lines.
    pub tolerance: f64,
}

impl Runner {
    pub fn run(&self, tests: &[TestFile]) -> Result<Vec<TestOutcome>> {
        let mut outcomes = Vec::new();
        for test in tests {
            let golden = golden::load(&self.golden_dir, &test.stem)?.ok_or_else(|| {
                anyhow::anyhow!(
                    "Missing golden for {} (run `vb6harness update-golden`)",
                    test.path.display()
                )
            })?;
            let mut engine_outcomes = Vec::new();
            for engine in &self.engines {
                let id = engine.id();
                let skip_reason = match id {
                    EngineId::Interpreter => test.meta.skip_interpreter.clone(),
                    EngineId::Compiler => test.meta.skip_compiler.clone(),
                    EngineId::Vb6 => test.meta.skip_vb6.clone(),
                };
                if let Some(reason) = skip_reason {
                    engine_outcomes.push(EngineOutcome {
                        engine: id,
                        skipped: Some(reason),
                        error: None,
                        duration: Duration::ZERO,
                        diffs: Vec::new(),
                    });
                    continue;
                }
                match engine.run(&test.path, test.meta.timeout)? {
                    EngineRun::Skipped(reason) => engine_outcomes.push(EngineOutcome {
                        engine: id,
                        skipped: Some(reason),
                        error: None,
                        duration: Duration::ZERO,
                        diffs: Vec::new(),
                    }),
                    EngineRun::Output(output) => {
                        let tolerance = test.meta.tolerance.unwrap_or(self.tolerance);
                        let comparer = Comparer::new(tolerance);
                        let diffs = if output.error.is_some() {
                            Vec::new()
                        } else {
                            comparer.compare(&golden, &output.lines)
                        };
                        engine_outcomes.push(EngineOutcome {
                            engine: id,
                            skipped: None,
                            error: output.error,
                            duration: output.duration,
                            diffs,
                        });
                    }
                }
            }
            outcomes.push(TestOutcome {
                name: test.meta.name.clone(),
                path: test.path.clone(),
                category: test.meta.category.clone(),
                known_issue: test.meta.known_issue.clone(),
                engines: engine_outcomes,
            });
        }
        Ok(outcomes)
    }
}

/// Run `test` with `engine`, returning its output lines (or an error when the
/// engine skipped or the program failed). Used by `update-golden`.
pub fn capture_lines(engine: &dyn Engine, test: &TestFile) -> Result<Vec<String>> {
    match engine.run(&test.path, test.meta.timeout)? {
        EngineRun::Skipped(reason) => bail!("engine skipped: {reason}"),
        EngineRun::Output(EngineOutput { lines, error, .. }) => {
            if let Some(error) = error {
                bail!("engine failed: {error}");
            }
            Ok(lines)
        }
    }
}
