//! vb6format: VB6 source code formatter
//!
//! Formats VB6 source code according to configurable style rules. Parses source
//! into a concrete syntax tree (CST) and applies formatting passes to produce
//! consistently formatted output.

#![warn(missing_docs)]
/// Formatting context for tracking state during formatting passes.
pub mod context;
mod cst_formatter;
mod passes;
/// CST rewrite utilities for transforming the parsed tree.
pub mod rewrite;
/// Formatting configuration (indentation, keyword case, blank lines).
pub mod settings;

pub use settings::FmtSettings;

use anyhow::Result;
use vb6parse::ConcreteSyntaxTree;

/// Format a parsed concrete syntax tree according to the given settings.
pub fn fmt_cst(cst: ConcreteSyntaxTree, settings: &FmtSettings) -> Result<String> {
    let formatter = cst_formatter::CstFormatter::new(cst, settings);
    Ok(formatter.format())
}

/// Parse VB6 source code and format it according to the given settings.
pub fn fmt_source(source: &str, settings: &FmtSettings) -> Result<String> {
    let parse_result = ConcreteSyntaxTree::from_text("fmt_input", source);
    let (cst_opt, _failures) = parse_result.unpack();
    let cst = cst_opt.ok_or_else(|| anyhow::anyhow!("Failed to parse source code"))?;
    fmt_cst(cst, settings)
}
