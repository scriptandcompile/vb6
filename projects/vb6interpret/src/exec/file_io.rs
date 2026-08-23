//! File I/O statements: `Open` and `Close`.
//!
//! Unlike most statements, these are parsed as flat token runs rather than
//! nested expression nodes, so their arguments go through
//! [`Interpreter::eval_flat_token`].

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
        let children: Vec<&CstNode> = node.significant_children().collect();
        let mut idx = 0;

        if children.first().map(|c| c.kind()) == Some(SyntaxKind::OpenKeyword) {
            idx += 1;
        }

        let path_tok = children
            .get(idx)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let path_value = self.eval_flat_token(path_tok)?;
        idx += 1;

        let mut mode = file_state::OpenMode::Random;
        if children.get(idx).map(|c| c.kind()) == Some(SyntaxKind::ForKeyword) {
            idx += 1;
            if let Some(mode_tok) = children.get(idx) {
                mode = match mode_tok.kind() {
                    SyntaxKind::InputKeyword => file_state::OpenMode::Input,
                    SyntaxKind::OutputKeyword => file_state::OpenMode::Output,
                    SyntaxKind::AppendKeyword => file_state::OpenMode::Append,
                    SyntaxKind::BinaryKeyword => file_state::OpenMode::Binary,
                    _ => file_state::OpenMode::Random,
                };
                idx += 1;
            }
        }

        let mut access = file_state::AccessMode::ReadWrite;
        if children.get(idx).map(|c| c.kind()) == Some(SyntaxKind::AccessKeyword) {
            idx += 1;
            let mut can_read = false;
            let mut can_write = false;
            while idx < children.len() && children[idx].kind() != SyntaxKind::AsKeyword {
                match children[idx].kind() {
                    SyntaxKind::ReadKeyword => can_read = true,
                    SyntaxKind::WriteKeyword => can_write = true,
                    _ => {}
                }
                idx += 1;
            }
            access = match (can_read, can_write) {
                (true, false) => file_state::AccessMode::Read,
                (false, true) => file_state::AccessMode::Write,
                _ => file_state::AccessMode::ReadWrite,
            };
        }

        // Optional lock clause (`Shared`, `Lock Read`, `Lock Write`, `Lock Read Write`).
        let mut lock = file_state::LockMode::Shared;
        if idx < children.len() && children[idx].kind() != SyntaxKind::AsKeyword {
            let mut locks_read = false;
            let mut locks_write = false;
            while idx < children.len() && children[idx].kind() != SyntaxKind::AsKeyword {
                match children[idx].kind() {
                    SyntaxKind::ReadKeyword => locks_read = true,
                    SyntaxKind::WriteKeyword => locks_write = true,
                    _ => {}
                }
                idx += 1;
            }
            lock = match (locks_read, locks_write) {
                (true, true) => file_state::LockMode::LockReadWrite,
                (true, false) => file_state::LockMode::LockRead,
                (false, true) => file_state::LockMode::LockWrite,
                _ => file_state::LockMode::Shared,
            };
        }

        if children.get(idx).map(|c| c.kind()) == Some(SyntaxKind::AsKeyword) {
            idx += 1;
        }
        if children.get(idx).map(|c| c.kind()) == Some(SyntaxKind::Octothorpe) {
            idx += 1;
        }
        let number_tok = children
            .get(idx)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let file_number = self
            .eval_flat_token(number_tok)?
            .as_i16()
            .map_err(|_| self.error_here(VBError::type_mismatch()))?;
        idx += 1;

        let mut record_length = 0i32;
        if children.get(idx).map(|c| c.kind()) == Some(SyntaxKind::LenKeyword) {
            idx += 1;
            if children.get(idx).map(|c| c.kind()) == Some(SyntaxKind::EqualityOperator) {
                idx += 1;
            }
            if let Some(len_tok) = children.get(idx) {
                record_length = self.eval_flat_token(len_tok)?.as_i32().unwrap_or(0);
            }
        }

        filefn::open::open_file(&path_value, mode, access, lock, file_number, record_length)
            .map_err(|e| self.error_here(e))?;

        Ok(Flow::Next)
    }

    /// `Close [[#]filenumber] [, [#]filenumber] ...`; closes all open files
    /// if the list is empty.
    pub(crate) fn exec_close(&mut self, node: &CstNode) -> RunResult<Flow> {
        let children: Vec<&CstNode> = node.significant_children().collect();
        let mut file_numbers: Vec<i16> = Vec::new();

        for child in &children {
            match child.kind() {
                SyntaxKind::IntegerLiteral | SyntaxKind::Identifier => {
                    let number = self
                        .eval_flat_token(child)?
                        .as_i16()
                        .map_err(|_| self.error_here(VBError::type_mismatch()))?;
                    file_numbers.push(number);
                }
                _ => {}
            }
        }

        filefn::close::close_files(&file_numbers).map_err(|e| self.error_here(e))?;

        Ok(Flow::Next)
    }
}
