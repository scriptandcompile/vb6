use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration for a conversion operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionConfig {
    /// Target language or framework
    pub target: String,

    /// Output directory
    pub output_dir: PathBuf,

    /// Source VB6 project path
    pub source_project: PathBuf,

    /// Additional options specific to the target
    pub target_options: HashMap<String, String>,

    /// Preserve comments
    pub preserve_comments: bool,

    /// Generate documentation
    pub generate_docs: bool,

    /// Apply code formatting
    pub format_output: bool,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            target: String::new(),
            output_dir: PathBuf::new(),
            source_project: PathBuf::new(),
            target_options: HashMap::new(),
            preserve_comments: true,
            generate_docs: true,
            format_output: true,
        }
    }
}

/// Result of a conversion operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    /// Files that were generated
    pub generated_files: Vec<GeneratedFile>,

    /// Warnings encountered during conversion
    pub warnings: Vec<ConversionWarning>,

    /// Statistics about the conversion
    pub stats: ConversionStats,
}

/// A file generated during conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    /// Path to the generated file (relative to output dir)
    pub path: PathBuf,

    /// Type of file (source, config, asset, etc.)
    pub file_type: FileType,

    /// Size in bytes
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Category of a file generated or processed during conversion.
pub enum FileType {
    /// Generated source code file.
    SourceCode,
    /// Configuration file (Cargo.toml, package.json, etc.).
    Configuration,
    /// Static asset (image, icon, etc.).
    Asset,
    /// Documentation file.
    Documentation,
    /// Test file.
    Test,
    /// Other / unclassified file.
    Other,
}

/// Warning encountered during conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionWarning {
    /// Warning message
    pub message: String,

    /// Location in source file
    pub location: Option<SourceLocation>,

    /// Severity level
    pub severity: WarningSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Severity level for conversion warnings.
pub enum WarningSeverity {
    /// Informational message.
    Info,
    /// Warning that may indicate a potential issue.
    Warning,
    /// Error-level issue.
    Error,
}

/// Location in a source file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    /// Path to the source file.
    pub file: PathBuf,
    /// Line number (1-based).
    pub line: usize,
    /// Column number (1-based).
    pub column: usize,
}

/// Statistics about the conversion
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConversionStats {
    /// Number of files processed
    pub files_processed: usize,

    /// Number of files generated
    pub files_generated: usize,

    /// Number of lines of code converted
    pub lines_converted: usize,

    /// Number of forms converted
    pub forms_converted: usize,

    /// Number of modules converted
    pub modules_converted: usize,

    /// Number of classes converted
    pub classes_converted: usize,

    /// Time taken (in milliseconds)
    pub duration_ms: u128,
}
