//! The legacy `VB6.exe` engine.
//!
//! Corpus modules are written VB6-native (`Print #1, <expr>` for output), so
//! the compiled source is the committed file. The engine only builds from a
//! temporary copy that renames the corpus entry point:
//!
//! - `Sub Main` is renamed to `Sub TestMain` so a generated `startup.bas`
//!   module can open/close the output file around the call.
//!
//! The committed test files are never modified.
//!
//! Requires Windows (`VB6.exe /make`) or a Wine prefix running VB6.

use super::{Engine, EngineId, EngineOutput, EngineRun, run_with_timeout};
use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

/// The name of the output file the generated startup module writes to.
const OUTPUT_FILE: &str = "out.txt";

/// The generated module that wraps the (renamed) test entry point with file
/// open/close.
const STARTUP_MODULE: &str = "\
Attribute VB_Name = \"HarnessStartup\"
Option Explicit

Public Sub Main()
    Open \"out.txt\" For Output As #1
    TestMain
    Close #1
End Sub
";

/// The `VB6.exe` engine. `work_dir` is where temporary build projects live.
pub struct Vb6Engine {
    vb6_path: Option<PathBuf>,
    work_dir: PathBuf,
}

impl Vb6Engine {
    pub fn new(vb6_path: Option<PathBuf>, work_dir: PathBuf) -> Self {
        Self { vb6_path, work_dir }
    }

    /// Rewrite a committed test module into a VB6-buildable copy.
    fn rewrite_source(&self, source: &str) -> String {
        source
            .lines()
            .map(rewrite_line)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Engine for Vb6Engine {
    fn id(&self) -> EngineId {
        EngineId::Vb6
    }

    fn run(&self, module_path: &Path, timeout: Duration) -> Result<EngineRun> {
        let started = Instant::now();
        let Some(vb6_path) = &self.vb6_path else {
            return Ok(EngineRun::Skipped(
                "VB6 not configured (pass --vb6-path or set VB6_PATH)".to_string(),
            ));
        };

        let stem = module_path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("non-UTF8 test file name")?;
        let dir = self.work_dir.join(stem);
        fs::create_dir_all(&dir).with_context(|| format!("Failed to create {}", dir.display()))?;

        let source = fs::read_to_string(module_path)
            .with_context(|| format!("Failed to read {}", module_path.display()))?;
        let rewritten = self.rewrite_source(&source);
        fs::write(dir.join("test.bas"), rewritten)?;
        fs::write(dir.join("startup.bas"), STARTUP_MODULE)?;
        fs::write(dir.join("test.vbp"), project_file())?;

        // Compile the temporary project.
        let mut make = if cfg!(windows) {
            let mut command = Command::new(vb6_path);
            command.arg("/make").arg(dir.join("test.vbp"));
            command
        } else {
            let mut command = Command::new("wine");
            command.arg(vb6_path).arg("/make").arg(dir.join("test.vbp"));
            command
        };
        let status = run_with_timeout(&mut make, timeout)?;
        if status != Some(0) {
            return Ok(EngineRun::Skipped(format!(
                "VB6 compilation failed (exit {status:?}) for {}",
                module_path.display()
            )));
        }

        // Run the produced executable from the build directory. On non-Windows
        // hosts the PE binary must be executed through a Wine prefix.
        let exe = dir.join("test.exe");
        let run_duration = timeout.max(Duration::from_secs(30));
        let mut run = if cfg!(windows) {
            let mut command = Command::new(&exe);
            command.current_dir(&dir);
            command
        } else {
            let mut command = Command::new("wine");
            command.arg(&exe).current_dir(&dir);
            command
        };
        let exit = run_with_timeout(&mut run, run_duration)?;
        if exit != Some(0) {
            return Err(anyhow!(
                "VB6 program exited with {exit:?} for {}",
                module_path.display()
            ));
        }

        let text = fs::read_to_string(dir.join(OUTPUT_FILE))
            .with_context(|| format!("Missing output file in {}", dir.display()))?;
        let lines = text.lines().map(str::to_string).collect();

        Ok(EngineRun::Output(EngineOutput {
            lines,
            error: None,
            duration: started.elapsed(),
        }))
    }
}

/// Rename the entry point (`Sub Main` -> `Sub TestMain`) so a generated
/// startup module can wrap it. Everything else is left verbatim.
fn rewrite_line(line: &str) -> String {
    let upper = line.trim_start().to_ascii_uppercase();

    if upper.starts_with("SUB MAIN") {
        let mut rewritten = line.to_string();
        if let Some(index) = rewritten.find("Main") {
            rewritten.replace_range(index..index + "Main".len(), "TestMain");
        }
        return rewritten;
    }

    line.to_string()
}

/// Minimal `.vbp` for a standard EXE with a `Sub Main` startup.
fn project_file() -> &'static str {
    "\
Type=Exe
Module=test.bas
Module=startup.bas
Startup=\"Main\"
Name=\"test\"
StartMode=0
Unattended=0
Retained=0
"
}
