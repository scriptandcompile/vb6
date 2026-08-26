//! File I/O statements: `Open` and `Close`.

use vb6core::error::VBError;
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;
use vb6runtime::library::file as filefn;
use vb6runtime::state::file as file_state;

use crate::error::RunResult;
use crate::interpreter::{Flow, Interpreter};

impl Interpreter {
    /// `Open pathname For mode [Access access] [lock] As [#]filenumber [Len=reclength]`.
    pub(crate) fn exec_open(&mut self, node: &CstNode) -> RunResult<Flow> {
        let sig_children: Vec<&CstNode> = node.significant_children().collect();

        // Pathname: first expression node after OpenKeyword
        let pathname_node = sig_children.iter().skip(1).find(|c| {
            matches!(
                c.kind(),
                SyntaxKind::StringLiteralExpression | SyntaxKind::IdentifierExpression
            )
        });
        let path_value = self
            .eval_expr(
                pathname_node
                    .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?,
            )
            .map_err(|_| self.error_here(VBError::invalid_procedure_call(), None))?;

        // Mode: first KeywordClause after ForKeyword
        let mode_keyword = sig_children
            .iter()
            .skip_while(|c| c.kind() != SyntaxKind::ForKeyword)
            .skip(1)
            .find(|c| c.kind() == SyntaxKind::KeywordClause)
            .and_then(|clause| clause.first_child_by_kind(SyntaxKind::Identifier));
        let mode = match mode_keyword {
            Some(k) => match k.kind() {
                SyntaxKind::InputKeyword => file_state::OpenMode::Input,
                SyntaxKind::OutputKeyword => file_state::OpenMode::Output,
                SyntaxKind::AppendKeyword => file_state::OpenMode::Append,
                SyntaxKind::BinaryKeyword => file_state::OpenMode::Binary,
                SyntaxKind::RandomKeyword => file_state::OpenMode::Random,
                _ => file_state::OpenMode::Random,
            },
            None => file_state::OpenMode::Random,
        };

        // Access clause: second KeywordClause after ForKeyword (contains AccessKeyword)
        let access_kw_clauses: Vec<&CstNode> = sig_children
            .iter()
            .skip_while(|c| c.kind() != SyntaxKind::ForKeyword)
            .skip(1)
            .filter(|c| c.kind() == SyntaxKind::KeywordClause)
            .copied()
            .collect();
        let access = match access_kw_clauses.get(1) {
            Some(clause) => {
                let mut can_read = false;
                let mut can_write = false;
                for child in clause.significant_children() {
                    match child.kind() {
                        SyntaxKind::ReadKeyword => can_read = true,
                        SyntaxKind::WriteKeyword => can_write = true,
                        _ => {}
                    }
                }
                match (can_read, can_write) {
                    (true, false) => file_state::AccessMode::Read,
                    (false, true) => file_state::AccessMode::Write,
                    _ => file_state::AccessMode::ReadWrite,
                }
            }
            None => file_state::AccessMode::ReadWrite,
        };

        // Lock clause: third KeywordClause after ForKeyword (contains LockKeyword or "Shared")
        let lock = match access_kw_clauses.get(2) {
            Some(clause) => {
                let mut locks_read = false;
                let mut locks_write = false;
                for child in clause.significant_children() {
                    match child.kind() {
                        SyntaxKind::ReadKeyword => locks_read = true,
                        SyntaxKind::WriteKeyword => locks_write = true,
                        _ => {}
                    }
                }
                match (locks_read, locks_write) {
                    (true, true) => file_state::LockMode::LockReadWrite,
                    (true, false) => file_state::LockMode::LockRead,
                    (false, true) => file_state::LockMode::LockWrite,
                    _ => file_state::LockMode::Shared,
                }
            }
            None => file_state::LockMode::Shared,
        };

        // Filenumber: first ExpressionClause
        let filenumber_clause = node.first_child_by_kind(SyntaxKind::ExpressionClause);
        let filenumber_expr =
            filenumber_clause.and_then(|c| c.first_child_by_kind(SyntaxKind::IdentifierExpression));
        let file_number = self
            .eval_filenumber(filenumber_expr)
            .map_err(|_| self.error_here(VBError::type_mismatch(), None))?;

        // Len clause: last ExpressionClause (if two, the second is Len)
        let len_clauses: Vec<&CstNode> = node
            .children_by_kind(SyntaxKind::ExpressionClause)
            .collect();
        let record_length = match len_clauses.len() {
            2 => {
                let clause = len_clauses[1];
                let expr = clause
                    .first_child_by_kind(SyntaxKind::NumericLiteralExpression)
                    .or_else(|| clause.first_child_by_kind(SyntaxKind::LiteralExpression));
                match expr {
                    Some(e) => self.eval_literal(e)?.as_i32().unwrap_or(0),
                    None => 0,
                }
            }
            _ => 0,
        };

        filefn::open::open_file(&path_value, mode, access, lock, file_number, record_length)
            .map_err(|e| self.error_here(e, None))?;

        Ok(Flow::Next)
    }

    /// Evaluate a filenumber expression, handling `#1` (Octothorpe + literal) and
    /// bare identifiers (`fileNum`).
    fn eval_filenumber(&mut self, node: Option<&CstNode>) -> RunResult<i16> {
        let Some(node) = node else {
            return Err(self.error_here(VBError::invalid_procedure_call(), None));
        };

        // Look for an IntegerLiteral child (handles `#1` prefix case)
        if let Some(int_lit) = node.first_child_by_kind(SyntaxKind::IntegerLiteral) {
            return Ok(self.eval_literal(int_lit)?.as_i16()?);
        }

        // Handle a bare Identifier (variable name like `fileNum`)
        if node.first_child_by_kind(SyntaxKind::Identifier).is_some() {
            return Ok(self.eval_expr(node)?.as_i16()?);
        }

        // Fallback: try evaluating the node as a literal
        Ok(self.eval_literal(node)?.as_i16()?)
    }

    /// `Close [[#]filenumber] [, [#]filenumber] ...`; closes all open files
    /// if the list is empty.
    pub(crate) fn exec_close(&mut self, node: &CstNode) -> RunResult<Flow> {
        let mut file_numbers: Vec<i16> = Vec::new();

        for arg in node.children_by_kind(SyntaxKind::Argument) {
            for child in arg.significant_children() {
                if matches!(
                    child.kind(),
                    SyntaxKind::StringLiteralExpression
                        | SyntaxKind::NumericLiteralExpression
                        | SyntaxKind::IdentifierExpression
                        | SyntaxKind::Octothorpe
                ) {
                    let value = self
                        .eval_expr(child)
                        .map_err(|_| self.error_here(VBError::type_mismatch(), None))?;
                    file_numbers.push(value.as_i16()?);
                    break;
                }
            }
        }

        filefn::close::close_files(&file_numbers).map_err(|e| self.error_here(e, None))?;

        Ok(Flow::Next)
    }
}
