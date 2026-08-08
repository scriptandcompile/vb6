//! vb6harness: differential test harness for the VB6 interpreter and compiler.
//!
//! Runs a corpus of VB6 test modules through each engine (`vb6interpret`,
//! `vb6compile`, and the legacy `VB6.exe`) and compares their `Print` output
//! against committed golden files.

mod compare;
mod engines;
mod golden;
mod report;
mod runner;
mod suite;

use anyhow::{Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use engines::Engine;
use engines::compiler::CompilerEngine;
use engines::interpreter::InterpreterEngine;
use engines::vb6::Vb6Engine;
use runner::Runner;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Default relative location of the test suite, resolved against the
/// workspace root.
const DEFAULT_SUITE_DIR: &str = "tests/suite";

/// Default relative location of the committed golden files.
const DEFAULT_GOLDEN_DIR: &str = "tests/golden";

/// Floating-point tolerance used when comparing numeric output lines.
pub const DEFAULT_TOLERANCE: f64 = 1e-12;

#[derive(Parser)]
#[command(name = "vb6harness")]
#[command(about = "Differential test harness for the VB6 interpreter and compiler", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the test suite and compare engine output against the goldens.
    Run {
        /// Test suite directory (absolute, or relative to the workspace root).
        #[arg(long)]
        suite: Option<PathBuf>,

        /// Golden file directory (absolute, or relative to the workspace root).
        #[arg(long)]
        golden_dir: Option<PathBuf>,

        /// Run only tests whose path contains this substring.
        #[arg(long)]
        test: Option<String>,

        /// Run only tests in this category.
        #[arg(long)]
        category: Option<String>,

        /// Also run against the legacy VB6 compiler (Windows or Wine).
        #[arg(long)]
        vb6: bool,

        /// Path to VB6.exe (defaults to the VB6_PATH environment variable).
        #[arg(long, env = "VB6_PATH")]
        vb6_path: Option<PathBuf>,

        /// Also run the compiler engine (skipped until vb6compile emits output).
        #[arg(long)]
        compiler: bool,

        /// Write a JUnit XML report to this path.
        #[arg(long)]
        junit: Option<PathBuf>,

        /// Print every test result, not just failures.
        #[arg(long)]
        verbose: bool,
    },

    /// Write golden files from an engine's output.
    UpdateGolden {
        /// Test suite directory (absolute, or relative to the workspace root).
        #[arg(long)]
        suite: Option<PathBuf>,

        /// Golden file directory (absolute, or relative to the workspace root).
        #[arg(long)]
        golden_dir: Option<PathBuf>,

        /// Engine whose output becomes the golden files.
        #[arg(long, value_enum, default_value_t = GoldenEngine::Interpreter)]
        engine: GoldenEngine,

        /// Path to VB6.exe (only used with `--engine vb6`).
        #[arg(long, env = "VB6_PATH")]
        vb6_path: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum GoldenEngine {
    Interpreter,
    Compiler,
    Vb6,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run {
            suite,
            golden_dir,
            test,
            category,
            vb6,
            vb6_path,
            compiler,
            junit,
            verbose,
        } => {
            let workspace = workspace_root();
            let suite_dir = resolve(suite.as_deref(), &workspace, DEFAULT_SUITE_DIR);
            let golden_dir = resolve(golden_dir.as_deref(), &workspace, DEFAULT_GOLDEN_DIR);

            let tests = suite::discover(&suite_dir, test.as_deref(), category.as_deref())?;
            if tests.is_empty() {
                bail!("No tests found in {}", suite_dir.display());
            }

            let mut engines: Vec<Box<dyn Engine>> = vec![Box::new(InterpreterEngine)];
            if vb6 {
                let work_dir = workspace.join("target").join("harness-work");
                engines.push(Box::new(Vb6Engine::new(vb6_path, work_dir)));
            }
            if compiler {
                engines.push(Box::new(CompilerEngine));
            }

            let runner = Runner {
                engines,
                golden_dir,
                tolerance: DEFAULT_TOLERANCE,
            };
            let started = Instant::now();
            let outcomes = runner.run(&tests)?;
            let report = report::Report::new(outcomes, started.elapsed());

            println!("{}", report.summary());
            if verbose {
                for outcome in report.outcomes() {
                    println!("{}", report::render_outcome(outcome));
                }
            }
            let failures: Vec<_> = report.outcomes().iter().filter(|o| !o.passed()).collect();
            if !failures.is_empty() {
                eprintln!("{}", report::render_failures(&failures));
                if let Some(path) = junit {
                    report.write_junit(&path)?;
                }
                std::process::exit(1);
            }
            if let Some(path) = junit {
                report.write_junit(&path)?;
                println!("JUnit report: {}", path.display());
            }
        }
        Commands::UpdateGolden {
            suite,
            golden_dir,
            engine,
            vb6_path,
        } => {
            let workspace = workspace_root();
            let suite_dir = resolve(suite.as_deref(), &workspace, DEFAULT_SUITE_DIR);
            let golden_dir = resolve(golden_dir.as_deref(), &workspace, DEFAULT_GOLDEN_DIR);

            let tests = suite::discover(&suite_dir, None, None)?;
            if tests.is_empty() {
                bail!("No tests found in {}", suite_dir.display());
            }

            for test in &tests {
                let lines = match engine {
                    GoldenEngine::Interpreter => runner::capture_lines(&InterpreterEngine, test)?,
                    GoldenEngine::Compiler => {
                        bail!("The compiler engine is not implemented yet");
                    }
                    GoldenEngine::Vb6 => {
                        let work_dir = workspace.join("target").join("harness-work");
                        runner::capture_lines(&Vb6Engine::new(vb6_path.clone(), work_dir), test)?
                    }
                };
                golden::save(&golden_dir, &test.stem, &lines)?;
            }
            println!(
                "Updated {} golden file(s) in {}",
                tests.len(),
                golden_dir.display()
            );
        }
    }
    Ok(())
}

/// The workspace root: the ancestor of the harness crate that contains the
/// root `Cargo.toml` and this crate. Overridable via `VB6_WORKSPACE_ROOT`.
fn workspace_root() -> PathBuf {
    if let Ok(root) = std::env::var("VB6_WORKSPACE_ROOT") {
        return PathBuf::from(root);
    }
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .ancestors()
        .find(|dir| dir.join("Cargo.toml").is_file() && dir.join("tests").join("harness").is_dir())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| manifest.to_path_buf())
}

/// Resolve a path: absolute paths are used as-is, relative paths are resolved
/// against the workspace root, and absent paths fall back to `default_rel`.
fn resolve(path: Option<&Path>, workspace: &Path, default_rel: &str) -> PathBuf {
    match path {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => workspace.join(path),
        None => workspace.join(default_rel),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use runner::Runner;

    /// The committed golden files must match the interpreter's current output.
    /// This is the harness's CI gate: `cargo test --workspace` exercises it.
    #[test]
    fn golden_suite_matches_interpreter() {
        let workspace = workspace_root();
        let suite_dir = workspace.join(DEFAULT_SUITE_DIR);
        let golden_dir = workspace.join(DEFAULT_GOLDEN_DIR);

        let tests = suite::discover(&suite_dir, None, None).expect("discover tests");
        assert!(
            !tests.is_empty(),
            "no tests discovered in {}",
            suite_dir.display()
        );

        let runner = Runner {
            engines: vec![Box::new(InterpreterEngine)],
            golden_dir,
            tolerance: DEFAULT_TOLERANCE,
        };
        let outcomes = runner.run(&tests).expect("run suite");
        let failures: Vec<_> = outcomes.iter().filter(|o| !o.passed()).collect();
        assert!(
            failures.is_empty(),
            "{} golden test(s) failed:\n{}",
            failures.len(),
            report::render_failures(&failures)
        );
    }
}
