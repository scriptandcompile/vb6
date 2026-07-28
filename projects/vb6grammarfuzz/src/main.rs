//! Grammar-based fuzzer for vb6parse.
//!
//! Uses an ANTLR4 `.g4` grammar file to generate random VB6 source,
//! parses it with vb6parse, checks the CST for Error nodes, and
//! minimizes any failing inputs via delta debugging.

mod checker;
mod g4_parser;
mod generator;
mod reducer;

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::checker::check_source;
use crate::g4_parser::parse_g4;
use crate::generator::{Generator, GeneratorConfig};
use crate::reducer::reduce;

const EMBEDDED_GRAMMAR: &str =
    include_str!("../proleap-vb6-parser/src/main/antlr4/io/proleap/vb6/VisualBasic6.g4");

#[derive(Parser)]
#[command(name = "vb6grammarfuzz")]
#[command(about = "Grammar-based fuzzer for vb6parse")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate random VB6 source from a .g4 grammar file.
    Generate {
        /// Path to an ANTLR4 .g4 grammar file (default: embedded VisualBasic6.g4).
        #[arg(short, long)]
        grammar: Option<PathBuf>,
        /// Grammar rule to start generation from (default: "module").
        #[arg(short, long, default_value = "module")]
        start_rule: String,
        /// RNG seed for reproducibility.
        #[arg(short = 'S', long)]
        seed: Option<u64>,
        /// Maximum recursion depth.
        #[arg(long, default_value_t = 8)]
        max_depth: usize,
    },
    /// Check a VB6 source file for Error nodes in vb6parse's CST.
    Check {
        /// Path to the VB6 source file to check.
        #[arg(short, long)]
        file: PathBuf,
    },
    /// Reduce a VB6 source file to a minimal input that still triggers Error nodes.
    Reduce {
        /// Path to the VB6 source file to reduce.
        #[arg(short, long)]
        file: PathBuf,
        /// Path to write the reduced output (default: stdout).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Run the full fuzz loop: generate, check, and reduce.
    Fuzz {
        /// Path to an ANTLR4 .g4 grammar file (default: embedded VisualBasic6.g4).
        #[arg(short, long)]
        grammar: Option<PathBuf>,
        /// Grammar rule to start generation from.
        #[arg(short, long, default_value = "module")]
        start_rule: String,
        /// Number of iterations to run.
        #[arg(short, long, default_value_t = 1000)]
        iterations: usize,
        /// RNG seed for reproducibility.
        #[arg(short = 'S', long)]
        seed: Option<u64>,
        /// Maximum recursion depth for generation.
        #[arg(long, default_value_t = 8)]
        max_depth: usize,
        /// Directory to save failing (reduced) inputs.
        #[arg(long, default_value = "findings")]
        output_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate {
            grammar,
            start_rule,
            seed,
            max_depth,
        } => {
            let g4_text = load_grammar(grammar.as_deref())?;
            let grammar = parse_g4(&g4_text);

            let rng = match seed {
                Some(s) => SmallRng::seed_from_u64(s),
                None => SmallRng::from_rng(&mut rand::rng()),
            };

            let config = GeneratorConfig {
                max_depth,
                ..Default::default()
            };

            let source = Generator::new(&grammar, config, rng).generate(&start_rule);
            print!("{source}");
        }

        Command::Check { file } => {
            let source = fs::read_to_string(&file).context("Failed to read source file")?;
            let result = check_source(&source);

            if !result.parse_succeeded {
                if result.timed_out {
                    println!("Parse timed out.");
                } else {
                    println!(
                        "Parse failed (no CST produced). {} failure(s).",
                        result.failure_count
                    );
                }
                return Ok(());
            }

            if result.has_error {
                println!(
                    "Found {} Error node(s) ({} parse failure(s)):",
                    result.errors.len(),
                    result.failure_count
                );
                for (i, err) in result.errors.iter().enumerate() {
                    println!(
                        "  [{i}] text={:?}  path={:?}",
                        truncate(&err.text, 80),
                        err.path
                    );
                }
            } else {
                println!("No Error nodes. {} parse failure(s).", result.failure_count);
            }
        }

        Command::Reduce { file, output } => {
            let source = fs::read_to_string(&file).context("Failed to read source file")?;

            match reduce(&source) {
                Some(reduced) => {
                    if let Some(out_path) = output {
                        fs::write(&out_path, &reduced).context("Failed to write reduced output")?;
                        println!(
                            "Reduced {} → {} bytes, wrote to {}",
                            source.len(),
                            reduced.len(),
                            out_path.display()
                        );
                    } else {
                        println!(
                            "--- reduced input ({} → {} bytes) ---",
                            source.len(),
                            reduced.len()
                        );
                        println!("{reduced}");
                    }
                }
                None => {
                    println!("Source does not trigger Error nodes – nothing to reduce.");
                }
            }
        }

        Command::Fuzz {
            grammar,
            start_rule,
            iterations,
            seed,
            max_depth,
            output_dir,
        } => {
            let g4_text = load_grammar(grammar.as_deref())?;
            let grammar = parse_g4(&g4_text);

            let base_seed = seed.unwrap_or_else(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            });

            let config = GeneratorConfig {
                max_depth,
                ..Default::default()
            };

            fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

            let mut found = 0u64;
            let mut timeouts = 0u64;
            let mut seen_reduced: HashSet<String> = HashSet::new();

            println!("Starting grammar fuzz: {iterations} iterations, seed={base_seed}, start_rule={start_rule}");

            for i in 0..iterations {
                let iter_seed = base_seed.wrapping_add(i as u64);
                let rng = SmallRng::seed_from_u64(iter_seed);

                let source = Generator::new(&grammar, config.clone(), rng).generate(&start_rule);

                let result = check_source(&source);

                if result.timed_out {
                    timeouts += 1;
                    if timeouts <= 5 || timeouts.is_multiple_of(50) {
                        println!(
                            "[iter {i}] seed={iter_seed}: parse timed out ({} bytes) – skipping",
                            source.len()
                        );
                    }
                    continue;
                }

                if result.has_error {
                    found += 1;
                    let error_count = result.errors.len();
                    println!(
                        "[iter {i}] seed={iter_seed}: {error_count} Error node(s) in {} bytes – reducing...",
                        source.len()
                    );

                    let reduced = reduce(&source).unwrap_or_else(|| source.clone());

                    if seen_reduced.contains(&reduced) {
                        println!("  → duplicate (already seen this reduced form)");
                    } else {
                        seen_reduced.insert(reduced.clone());

                        // Save reduced version.
                        let filename = output_dir.join(format!("error_seed_{iter_seed}.bas"));
                        if let Err(e) = fs::write(&filename, &reduced) {
                            eprintln!("  Warning: could not write {}: {e}", filename.display());
                        } else {
                            println!(
                                "  → saved reduced input ({} bytes) to {}",
                                reduced.len(),
                                filename.display()
                            );
                        }

                        // Save original alongside for context.
                        let orig_filename =
                            output_dir.join(format!("error_seed_{iter_seed}_original.bas"));
                        let _ = fs::write(&orig_filename, &source);
                    }
                }

                if (i + 1) % 100 == 0 {
                    println!(
                        "[progress] {}/{iterations} iterations, {found} finding(s) so far",
                        i + 1
                    );
                }
            }

            println!(
                "Done. {found} finding(s) ({} unique) from {iterations} iterations ({timeouts} timed out).",
                seen_reduced.len()
            );
        }
    }

    Ok(())
}

fn load_grammar(path: Option<&std::path::Path>) -> Result<String> {
    match path {
        Some(p) => fs::read_to_string(p).context("Failed to read grammar file"),
        None => Ok(EMBEDDED_GRAMMAR.to_string()),
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}
