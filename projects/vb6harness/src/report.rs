//! Test report generation: human-readable summary and JUnit XML.

use crate::runner::TestOutcome;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::time::Duration;

/// Aggregated results for one run.
pub struct Report {
    outcomes: Vec<TestOutcome>,
    duration: Duration,
}

impl Report {
    pub fn new(outcomes: Vec<TestOutcome>, duration: Duration) -> Self {
        Self { outcomes, duration }
    }

    pub fn outcomes(&self) -> &[TestOutcome] {
        &self.outcomes
    }

    pub fn summary(&self) -> String {
        let total = self.outcomes.len();
        let passed = self
            .outcomes
            .iter()
            .filter(|outcome| outcome.passed())
            .count();
        let skipped = self
            .outcomes
            .iter()
            .flat_map(|outcome| &outcome.engines)
            .filter(|engine| engine.skipped.is_some())
            .count();
        format!(
            "{passed}/{total} tests passed ({} failed, {skipped} skipped) in {:.1}s",
            total - passed,
            self.duration.as_secs_f64()
        )
    }

    /// Write a JUnit XML report.
    pub fn write_junit(&self, path: &Path) -> Result<()> {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<testsuites>\n");
        for outcome in &self.outcomes {
            xml.push_str(&format!(
                "  <testcase classname=\"{}\" name=\"{}\">\n",
                escape(&outcome.category),
                escape(&outcome.name)
            ));
            if !outcome.passed() {
                xml.push_str("    <failure>");
                for engine in &outcome.engines {
                    if let Some(error) = &engine.error {
                        xml.push_str(&format!("{}: {}\n", engine.engine.label(), escape(error)));
                    }
                    for diff in &engine.diffs {
                        xml.push_str(&format!(
                            "{} line {}: expected {:?} got {:?}\n",
                            engine.engine.label(),
                            diff.line,
                            diff.expected,
                            diff.actual
                        ));
                    }
                }
                xml.push_str("    </failure>\n");
            }
            xml.push_str("  </testcase>\n");
        }
        xml.push_str("</testsuites>\n");
        fs::write(path, xml)?;
        Ok(())
    }
}

/// Render one outcome as a single status line.
pub fn render_outcome(outcome: &TestOutcome) -> String {
    let status = if outcome.passed() { "PASS" } else { "FAIL" };
    let mut line = format!("{status} {} ({})", outcome.name, outcome.path.display());
    for engine in &outcome.engines {
        let engine = if let Some(reason) = &engine.skipped {
            format!("{} skipped ({reason})", engine.engine.label())
        } else if engine.error.is_some() || !engine.diffs.is_empty() {
            format!(
                "{} failed ({:.0}ms)",
                engine.engine.label(),
                engine.duration.as_millis()
            )
        } else {
            format!(
                "{} ok ({:.0}ms)",
                engine.engine.label(),
                engine.duration.as_millis()
            )
        };
        line.push_str(&format!(" | {engine}"));
    }
    line
}

/// Render detailed failure output.
pub fn render_failures(outcomes: &[&TestOutcome]) -> String {
    let mut text = String::new();
    for outcome in outcomes {
        text.push_str(&format!(
            "\nFAIL: {} ({})\n",
            outcome.name,
            outcome.path.display()
        ));
        if let Some(issue) = &outcome.known_issue {
            text.push_str(&format!("  known issue: {issue}\n"));
        }
        for engine in &outcome.engines {
            if let Some(error) = &engine.error {
                text.push_str(&format!("  [{}] error: {}\n", engine.engine.label(), error));
            }
            for diff in &engine.diffs {
                text.push_str(&format!(
                    "  [{}] line {}: expected {:?}, got {:?}\n",
                    engine.engine.label(),
                    diff.line,
                    diff.expected,
                    diff.actual
                ));
            }
        }
    }
    text
}

/// Minimal XML escaping for element text.
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
