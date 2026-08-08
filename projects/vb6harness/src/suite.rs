//! Test suite discovery and metadata.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use walkdir::WalkDir;

/// Default timeout for a single engine run.
const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

/// Metadata parsed from a test module's header directives.
#[derive(Debug, Clone)]
pub struct TestMeta {
    pub name: String,
    pub category: String,
    pub timeout: Duration,
    pub tolerance: Option<f64>,
    pub known_issue: Option<String>,
    pub skip_vb6: Option<String>,
    pub skip_interpreter: Option<String>,
    pub skip_compiler: Option<String>,
}

/// A discovered test module.
#[derive(Debug, Clone)]
pub struct TestFile {
    /// Absolute path to the `.bas` file.
    pub path: PathBuf,
    /// File name without the extension (also the golden file name).
    pub stem: String,
    pub meta: TestMeta,
}

impl Default for TestMeta {
    fn default() -> Self {
        Self {
            name: String::new(),
            category: "misc".to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECONDS),
            tolerance: None,
            known_issue: None,
            skip_vb6: None,
            skip_interpreter: None,
            skip_compiler: None,
        }
    }
}

/// Discover `.bas` files under `suite_dir`, applying optional filters.
pub fn discover(
    suite_dir: &Path,
    test_filter: Option<&str>,
    category_filter: Option<&str>,
) -> Result<Vec<TestFile>> {
    let mut tests = Vec::new();
    for entry in WalkDir::new(suite_dir).follow_links(true) {
        let entry = entry
            .with_context(|| format!("Failed to walk suite directory {}", suite_dir.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        if path
            .extension()
            .is_none_or(|ext| !ext.eq_ignore_ascii_case("bas"))
        {
            continue;
        }

        if let Some(filter) = test_filter
            && !path.to_string_lossy().contains(filter)
        {
            continue;
        }

        let meta = read_meta(path)?;
        if let Some(category) = category_filter
            && meta.category != category
        {
            continue;
        }

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("non-UTF8 test file name")?
            .to_string();
        tests.push(TestFile {
            path: path.to_path_buf(),
            stem,
            meta,
        });
    }
    tests.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(tests)
}

/// Read a test module's header directives (comment lines before the first
/// procedure). Unknown directives are ignored so the files stay valid VB6.
fn read_meta(path: &Path) -> Result<TestMeta> {
    let source =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let mut meta = TestMeta::default();

    // Directives only appear before the first `Sub`/`Function` declaration.
    for line in source.lines().take_while(|line| {
        let trimmed = line.trim_start();
        !trimmed.starts_with("Sub ") && !trimmed.starts_with("Function ")
    }) {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix('\'') else {
            continue;
        };
        let Some((key, value)) = rest.split_once(':') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();
        match key.as_str() {
            "test" => meta.name = value.to_string(),
            "category" => meta.category = value.to_string(),
            "timeout" => {
                meta.timeout = Duration::from_secs(
                    value
                        .parse::<u64>()
                        .with_context(|| format!("Invalid TIMEOUT in {}", path.display()))?,
                );
            }
            "tolerance" => {
                meta.tolerance = Some(
                    value
                        .parse::<f64>()
                        .with_context(|| format!("Invalid TOLERANCE in {}", path.display()))?,
                );
            }
            "known_issue" => meta.known_issue = Some(value.to_string()),
            "skip_vb6" => meta.skip_vb6 = Some(value.to_string()),
            "skip_interpreter" => meta.skip_interpreter = Some(value.to_string()),
            "skip_compiler" => meta.skip_compiler = Some(value.to_string()),
            _ => {}
        }
    }

    if meta.name.is_empty() {
        meta.name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
    }
    Ok(meta)
}
