mod cst_formatter;
mod directives;
pub mod settings;
mod type_enum;

pub use settings::FmtSettings;

use anyhow::Result;
use vb6parse::ConcreteSyntaxTree;

pub fn fmt_source(source: &str, settings: &FmtSettings) -> Result<String> {
    let line_ending = cst_formatter::detect_line_ending(source);

    let parse_result = ConcreteSyntaxTree::from_text("fmt_input", source);
    let (cst_opt, _failures) = parse_result.unpack();
    let cst = cst_opt.ok_or_else(|| anyhow::anyhow!("Failed to parse source code"))?;
    let root = cst.to_root_node();

    let mut output = String::with_capacity(source.len());
    {
        let mut formatter = cst_formatter::CstFormatter::new(&mut output, line_ending, settings);
        formatter.walk_node(&root);
    }

    directives::post_process_directives(&mut output, settings, line_ending);
    type_enum::post_process_type_enum(&mut output, settings, line_ending);

    Ok(output)
}
