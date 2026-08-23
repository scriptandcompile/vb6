//! `Print` / `Debug.Print` output emission, including `Print #filenumber`.

use vb6core::error::VBError;
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;
use vb6runtime::library::file as filefn;
use vb6runtime::VBVariant;

use crate::error::RunResult;
use crate::interpreter::Interpreter;

impl Interpreter {
    /// Emit `Debug.Print` / `Print` output.
    pub(crate) fn print_node(&mut self, node: &CstNode) -> RunResult<()> {
        // `Print #filenumber, ...` writes to the file backend instead of the
        // console buffer; the file number is a real expression node (unlike
        // `Open`/`Close`, which are parsed as flat tokens).
        let top_level: Vec<&CstNode> = node.significant_children().collect();
        let file_number = match top_level
            .iter()
            .position(|c| c.kind() == SyntaxKind::Octothorpe)
        {
            Some(hash_pos) => {
                let expr = top_level
                    .get(hash_pos + 1)
                    .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
                let number = self
                    .eval_expr(expr)?
                    .as_i16()
                    .map_err(|_| self.error_here(VBError::type_mismatch()))?;
                Some(number)
            }
            None => None,
        };

        let argument_list = node.first_child_by_kind(SyntaxKind::ArgumentList);
        let mut trailing_separator = false;
        let mut file_values: Vec<VBVariant> = Vec::new();
        if let Some(list) = argument_list {
            let significant: Vec<&CstNode> = list.significant_children().collect();
            for child in &significant {
                match child.kind() {
                    SyntaxKind::Argument => {
                        let value = match child.first_non_whitespace_child() {
                            Some(expr) => self.eval_expr(expr)?,
                            None => VBVariant::Empty,
                        };
                        if file_number.is_some() {
                            file_values.push(value);
                        } else {
                            self.current_output.push_str(&value.as_string()?);
                        }
                    }
                    SyntaxKind::Comma => {
                        if file_number.is_none() {
                            self.current_output.push('\t');
                        }
                    }
                    SyntaxKind::Semicolon => {}
                    _ => {}
                }
            }
            trailing_separator = matches!(
                significant.last().map(|c| c.kind()),
                Some(SyntaxKind::Comma | SyntaxKind::Semicolon)
            );
        }

        if let Some(number) = file_number {
            filefn::print::print_statement(number, &file_values, !trailing_separator)
                .map_err(|e| self.error_here(e))?;
            return Ok(());
        }

        if !trailing_separator {
            self.output.push(std::mem::take(&mut self.current_output));
        }

        Ok(())
    }
}
