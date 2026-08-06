//! vb6-interpret: VB6 interpreter CLI
//!
//! Execute VB6 code directly without compilation.

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use vb6interpret::Interpreter;
use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;
use vb6runtime::Value;

#[derive(Parser)]
#[command(name = "vb6-interpret")]
#[command(about = "Visual Basic 6 Interpreter", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Show execution trace
    #[arg(short, long)]
    trace: bool,

    /// Enable profiling
    #[arg(short, long)]
    profile: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a VB6 file or project
    Run {
        /// Path to file or project
        path: PathBuf,

        /// Set initial variables (VAR=VALUE)
        #[arg(long)]
        set: Vec<String>,

        /// Execution timeout in seconds (0 = no timeout)
        #[arg(long, default_value = "0")]
        timeout: u64,
    },

    /// Start interactive REPL
    Repl,

    /// Run with debugger
    Debug {
        /// Path to file or project
        path: PathBuf,

        /// Initial breakpoints
        #[arg(long)]
        r#break: Vec<String>,
    },

    /// Check syntax without executing
    Check {
        /// Path to file or project
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { path, set, timeout }) => {
            let path = expand_tilde(&path);
            let source_file = SourceFile::from_file(&path)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
                .with_context(|| format!("Failed to read {}", path.display()))?;
            let module = ModuleFile::parse(&source_file).unwrap_or_fail();

            let mut interpreter = Interpreter::new();
            if timeout > 0 {
                interpreter.set_step_limit(u64::MAX);
            }
            for assignment in &set {
                let (name, raw) = assignment.split_once('=').ok_or_else(|| {
                    anyhow::anyhow!("Invalid --set '{assignment}' (expected VAR=VALUE)")
                })?;
                interpreter.set_global(name, parse_value(raw));
            }

            let started = Instant::now();
            let result = interpreter.run_module(&module);
            if cli.trace {
                eprintln!("{:?} statements executed", interpreter.steps());
            }
            let timed_out = match result {
                Ok(()) => timeout > 0 && started.elapsed() > Duration::from_secs(timeout),
                Err(error) => {
                    print_output(&interpreter);
                    let message = error.to_string();
                    eprintln!("Runtime error: {message}");
                    std::process::exit(1);
                }
            };
            print_output(&interpreter);
            if timed_out {
                bail!("Execution timed out after {}s", timeout);
            }
        }
        Some(Commands::Repl) | None => {
            println!("TODO: Start REPL");
        }
        Some(Commands::Debug { path, r#break }) => {
            let path = expand_tilde(&path);
            println!("TODO: Debug {}", path.display());
            println!("  Breakpoints: {:?}", r#break);
        }
        Some(Commands::Check { path }) => {
            let path = expand_tilde(&path);
            let source_file = SourceFile::from_file(&path)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
                .with_context(|| format!("Failed to read {}", path.display()))?;
            let module = ModuleFile::parse(&source_file).unwrap_or_fail();
            if cli.verbose {
                println!("Parsed {} OK", module.name);
            } else {
                println!("OK");
            }
        }
    }

    Ok(())
}

/// Write the interpreter's captured output to stdout, ensuring the output ends
/// with a newline (needed even when the last `Print` used a trailing `;`).
fn print_output(interpreter: &Interpreter) {
    let text = interpreter.output_text();
    print!("{text}");
    if !text.is_empty() && !text.ends_with('\n') {
        println!();
    }
    let _ = std::io::stdout().flush();
}

/// Expand a leading `~`/`~/` into the user's home directory, matching what a
/// shell would do for unquoted paths. Bash does not expand `~` inside double
/// quotes, so CLI users often pass it through literally.
fn expand_tilde(path: &Path) -> PathBuf {
    let s = path.to_string_lossy();
    let home = std::env::var_os("HOME");
    match s.strip_prefix("~/") {
        Some(rest) => home
            .map(|h| PathBuf::from(h).join(rest))
            .unwrap_or_else(|| path.to_path_buf()),
        None if s == "~" => home
            .map(PathBuf::from)
            .unwrap_or_else(|| path.to_path_buf()),
        None => path.to_path_buf(),
    }
}

/// Parse a `--set` value into a runtime value (Long, Double, or String).
fn parse_value(raw: &str) -> Value {
    if let Ok(long) = raw.parse::<i64>() {
        return Value::from_i64(long);
    }
    if let Ok(double) = raw.parse::<f64>() {
        return Value::from_double(double);
    }
    Value::from_string(raw.to_string())
}
