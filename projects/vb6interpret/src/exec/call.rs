//! The `Call` statement family: explicit `Call`, bare sub invocations,
//! `MsgBox`/`Beep`/`Shell` statement forms, and `Debug.Print` routing.

use vb6core::error::{err_number, VBError};
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;

use super::super::program;
use crate::error::RunResult;
use crate::interpreter::{Flow, Interpreter};

impl Interpreter {
    /// `Call` statement: `Debug.Print`, sub-procedure calls, and `MsgBox`.
    pub(crate) fn exec_call(&mut self, node: &CstNode) -> RunResult<Flow> {
        // Debug.Print ...
        let is_debug = node
            .first_child_by_kind(SyntaxKind::Identifier)
            .is_some_and(|i| i.text().trim().eq_ignore_ascii_case("debug"))
            && node.contains_kind(SyntaxKind::PrintKeyword);
        if is_debug {
            self.print_node(node)?;
            return Ok(Flow::Next);
        }

        let name = node
            .children()
            .iter()
            .find(|c| program::is_identifier_like(c) && c.kind() != SyntaxKind::CallKeyword)
            .map(|t| t.text().trim().to_string())
            .unwrap_or_default();

        let argument_list = node.first_child_by_kind(SyntaxKind::ArgumentList);
        let args = match argument_list {
            Some(list) => self.eval_args(list)?,
            None => Vec::new(),
        };

        // User-defined Sub.
        if self.procedures.contains_key(&name.to_lowercase()) {
            let flow = self.call_sub(&name, args)?;
            return Ok(flow);
        }

        match name.to_lowercase().as_str() {
            "msgbox" => {
                // Full `MsgBox` semantics: the interaction backend shows the
                // dialog (or records it) and the return value is discarded
                // because this form is a statement, not a function call.
                crate::builtins::call_builtin("msgbox", &args).map_err(|e| self.error_here(e))?;
                Ok(Flow::Next)
            }
            // `Beep` is a registered builtin Sub; a `Call` discards its
            // (always `Empty`) return value.
            "beep" => {
                crate::builtins::call_builtin("beep", &args).map_err(|e| self.error_here(e))?;
                Ok(Flow::Next)
            }
            // `Shell "prog"` statement form: the backend starts the program
            // (or records the request) and the task ID is discarded because
            // this form is a statement, not a function call.
            "shell" => {
                crate::builtins::call_builtin("shell", &args).map_err(|e| self.error_here(e))?;
                Ok(Flow::Next)
            }
            _ => Err(self.error_here(VBError::new(err_number::SUB_OR_FUNCTION_NOT_DEFINED))), // Sub or Function not defined
        }
    }
}
