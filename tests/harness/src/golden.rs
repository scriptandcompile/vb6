//! Golden file loading and writing.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// The golden file path for a test module stem.
pub fn golden_path(golden_dir: &Path, stem: &str) -> std::path::PathBuf {
    golden_dir.join(format!("{stem}.txt"))
}

/// Load a golden file, or `None` when it does not exist.
pub fn load(golden_dir: &Path, stem: &str) -> Result<Option<Vec<String>>> {
    let path = golden_path(golden_dir, stem);
    if !path.exists() {
        return Ok(None);
    }
    let text =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
    Ok(Some(text.lines().map(str::to_string).collect()))
}

/// Write (or overwrite) a golden file from output lines.
pub fn save(golden_dir: &Path, stem: &str, lines: &[String]) -> Result<()> {
    fs::create_dir_all(golden_dir)
        .with_context(|| format!("Failed to create {}", golden_dir.display()))?;
    let text = if lines.is_empty() {
        String::new()
    } else {
        format!("{}\n", lines.join("\n"))
    };
    fs::write(golden_path(golden_dir, stem), text)
        .with_context(|| format!("Failed to write golden for {stem}"))
}
