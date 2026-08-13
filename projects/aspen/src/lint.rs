//! `aspen lint`: per-file rules, selected by code.
//!
//! Separate from `check`, which needs a `.vbp` and runs the whole semantic
//! analysis. This walks files, parses each one on its own and applies the
//! selected rules, which keeps it fast enough for a pre-commit hook.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rayon::prelude::*;

use vb6format::lint::{Diagnostic, Fixability, LintSettings, RULES};
use vb6parse::io::SourceFile;

pub struct LintCommand {
    pub project_path: PathBuf,
    pub select: Vec<String>,
    pub ignore: Vec<String>,
    pub explain: bool,
}

/// The `[lint]` section of `.aspen.toml`, alongside the `[fmt]` section that
/// is already read from the same file.
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct LintConfig {
    #[serde(default)]
    pub select: Vec<String>,
    #[serde(default)]
    pub ignore: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct AspenConfig {
    lint: Option<LintConfig>,
}

/// Reads the `[lint]` section next to the given path, then from the working
/// directory, mirroring how the formatter finds its own settings.
#[must_use]
pub fn load_lint_settings(project_path: &Path) -> LintConfig {
    let config_root = if project_path.is_dir() {
        project_path.to_path_buf()
    } else {
        project_path
            .parent()
            .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
    };

    let candidates = [
        config_root.join(".aspenfmt.toml"),
        config_root.join(".aspen.toml"),
        PathBuf::from(".aspenfmt.toml"),
        PathBuf::from(".aspen.toml"),
    ];

    for path in &candidates {
        if !path.exists() {
            continue;
        }

        let Ok(contents) = std::fs::read_to_string(path) else {
            continue;
        };

        match toml::from_str::<AspenConfig>(&contents) {
            Ok(config) => return config.lint.unwrap_or_default(),
            Err(e) => {
                // A malformed config that is silently ignored looks exactly
                // like a config that selected nothing.
                eprintln!("Ignoring {}: {}", path.display(), e);
            }
        }
    }

    LintConfig::default()
}

pub fn lint_subcommand(cmd: LintCommand) -> Result<()> {
    if cmd.explain {
        println!(
            "{:<6} {:<24} {:<8} {:<8} {}",
            "CODE", "NAME", "DEFAULT", "FIX", "SUMMARY"
        );
        for rule in RULES {
            println!(
                "{:<6} {:<24} {:<8} {:<8} {}",
                rule.code,
                rule.name,
                if rule.default_on { "on" } else { "off" },
                match rule.fixability {
                    Fixability::Safe => "safe",
                    Fixability::Unsafe => "unsafe",
                    Fixability::None => "none",
                },
                rule.summary
            );
        }
        return Ok(());
    }

    let unknown: Vec<&String> = cmd
        .select
        .iter()
        .chain(cmd.ignore.iter())
        .filter(|code| {
            !RULES
                .iter()
                .any(|rule| rule.code.starts_with(code.as_str()))
        })
        .collect();

    if !unknown.is_empty() {
        // A typo in a rule code must not quietly select nothing.
        anyhow::bail!(
            "unknown rule code(s): {}. `aspen lint --explain` lists them.",
            unknown
                .iter()
                .map(|code| code.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    let settings = LintSettings::from_selection(&cmd.select, &cmd.ignore);
    let files = collect_files(&cmd.project_path);

    if files.is_empty() {
        println!("No VB6 source files found.");
        return Ok(());
    }

    let results: Vec<(PathBuf, Result<Vec<Diagnostic>>)> = files
        .par_iter()
        .map(|path| (path.clone(), lint_file(path, &settings)))
        .collect();

    let mut found = 0usize;
    let mut failed = 0usize;

    for (path, result) in &results {
        match result {
            Ok(diagnostics) => {
                for diagnostic in diagnostics {
                    println!(
                        "{}:{}:{}: {} {}",
                        path.display(),
                        diagnostic.line,
                        diagnostic.column,
                        diagnostic.code,
                        diagnostic.message
                    );
                    found += 1;
                }
            }
            Err(e) => {
                eprintln!("Error linting {}: {}", path.display(), e);
                failed += 1;
            }
        }
    }

    println!("{} finding(s) in {} file(s).", found, files.len());

    if failed > 0 {
        eprintln!("{} of {} files could not be read.", failed, files.len());
    }

    // ruff's convention: 1 means the run worked and found something, 2 means
    // the run itself failed. Keeping them apart lets CI tell "your code has a
    // problem" from "the tool could not look".
    if failed > 0 {
        std::process::exit(2);
    }

    if found > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn lint_file(path: &Path, settings: &LintSettings) -> Result<Vec<Diagnostic>> {
    let source = SourceFile::from_file(path)
        .map_err(|e| anyhow::anyhow!("{}", e))
        .with_context(|| format!("Failed to read {}", path.display()))?;

    Ok(vb6format::lint_source(source.as_ref(), settings))
}

fn collect_files(path: &Path) -> Vec<PathBuf> {
    if path.is_file() {
        return if is_source_file(path) {
            vec![path.to_path_buf()]
        } else {
            Vec::new()
        };
    }

    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| path.is_file() && is_source_file(path))
        .collect()
}

fn is_source_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("bas" | "cls" | "frm")
    )
}
