//! vb6-interpret: VB6 interpreter CLI
//!
//! Execute VB6 code directly without compilation.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
            println!("TODO: Run {}", path.display());
            println!("  Initial vars: {:?}", set);
            println!("  Timeout: {}s", timeout);
        }
        Some(Commands::Repl) | None => {
            println!("TODO: Start REPL");
        }
        Some(Commands::Debug { path, r#break }) => {
            println!("TODO: Debug {}", path.display());
            println!("  Breakpoints: {:?}", r#break);
        }
        Some(Commands::Check { path }) => {
            println!("TODO: Check {}", path.display());
        }
    }

    Ok(())
}
