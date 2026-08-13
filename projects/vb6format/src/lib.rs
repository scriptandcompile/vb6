pub mod context;
mod cst_formatter;
pub mod lint;
mod passes;
pub mod rewrite;
pub mod settings;

pub use lint::{Diagnostic, Fixability, LintSettings, RULES, Rule, lint_source};
pub use settings::FmtSettings;

use anyhow::Result;
use vb6parse::ConcreteSyntaxTree;

pub fn fmt_cst(cst: ConcreteSyntaxTree, settings: &FmtSettings) -> Result<String> {
    let formatter = cst_formatter::CstFormatter::new(cst, settings);
    Ok(formatter.format())
}

pub fn fmt_source(source: &str, settings: &FmtSettings) -> Result<String> {
    let parse_result = ConcreteSyntaxTree::from_text("fmt_input", source);
    let (cst_opt, _failures) = parse_result.unpack();
    let cst = cst_opt.ok_or_else(|| anyhow::anyhow!("Failed to parse source code"))?;
    fmt_cst(cst, settings)
}
