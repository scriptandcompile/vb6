mod cst_formatter;
pub mod settings;

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

    deduplicate_blank_lines(&mut output, line_ending);

    Ok(output)
}

fn deduplicate_blank_lines(output: &mut String, le: &str) {
    let lines: Vec<&str> = if le == "\r\n" {
        output.split("\r\n").collect()
    } else {
        output.split('\n').collect()
    };

    let mut result = String::with_capacity(output.len());
    let mut prev_blank = false;

    for (i, line) in lines.iter().enumerate() {
        let is_trailing = line.is_empty() && i == lines.len() - 1;
        if is_trailing {
            break;
        }
        if line.is_empty() {
            if !prev_blank {
                result.push_str(le);
                prev_blank = true;
            }
        } else {
            result.push_str(line);
            result.push_str(le);
            prev_blank = false;
        }
    }
    *output = result;
}
