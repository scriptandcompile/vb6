//! Simple builtin statements: clock (`Date`/`Time`), window/process
//! interaction (`AppActivate`, `SendKeys`), graphics (`SavePicture`), and
//! in-place string mutation (`LSet`/`RSet`/`Mid`/`MidB` assignment forms).

use vb6core::error::{VBError, VBResult, err_number};
use vb6parse::parsers::SyntaxKind;
use vb6parse::parsers::cst::CstNode;
use vb6runtime::VBVariant;
use vb6runtime::value::{VBLong, VBString};

use super::super::program;
use crate::error::RunResult;
use crate::interpreter::Interpreter;

/// Convert a VB6 date serial (days since 1899-12-30) to a [`jiff::Timestamp`].
///
/// The serial is the integer part (date) plus the fractional part (time).
/// Interpreted in the system's local time zone, matching VB6 semantics.
/// Returns `None` if the serial is out of the representable range.
fn serial_to_timestamp(serial: f64) -> Option<jiff::Timestamp> {
    let base = jiff::civil::Date::new(1899, 12, 30).ok()?;
    let days = serial.floor();
    let date = base
        .checked_add(jiff::SignedDuration::from_secs((days * 86400.0) as i64))
        .ok()?;
    let fraction = serial.fract();
    let total_secs = (fraction * 86_400.0).round() as i64;
    let h = (total_secs / 3600) as i8;
    let m = ((total_secs % 3600) / 60) as i8;
    let s = (total_secs % 60) as i8;
    let dt = date.at(h, m, s, 0);
    let zoned = dt.to_zoned(jiff::tz::TimeZone::system()).ok()?;
    Some(zoned.timestamp())
}

/// Convert a time-only serial (fractional part of a date serial) to a
/// [`jiff::Timestamp`] using today's date, interpreted in the system's
/// local time zone, matching VB6 semantics.
fn time_serial_to_timestamp(serial: f64) -> Option<jiff::Timestamp> {
    let ts = vb6runtime::state::clock::get();
    let tz = jiff::tz::TimeZone::system();
    let zoned = jiff::Zoned::new(ts, tz.clone());
    let d = zoned.date();
    let fraction = serial.fract();
    let total_secs = (fraction * 86_400.0).round() as i64;
    let h = (total_secs / 3600) as i8;
    let m = ((total_secs % 3600) / 60) as i8;
    let s = (total_secs % 60) as i8;
    let dt = d.at(h, m, s, 0);
    let zoned = dt.to_zoned(tz).ok()?;
    Some(zoned.timestamp())
}

impl Interpreter {
    /// `Date = expr`: set the system date.
    pub(crate) fn exec_date_statement(&mut self, node: &CstNode) -> RunResult<()> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let eq_index = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::EqualityOperator)
            .ok_or_else(|| {
                self.error_here(vb6core::error::VBError::invalid_procedure_call(), None)
            })?;
        let expr = significant.get(eq_index + 1).ok_or_else(|| {
            self.error_here(vb6core::error::VBError::invalid_procedure_call(), None)
        })?;
        let value = self.eval_expr(expr)?;
        // Always set the mock clock so `Date` reads correctly.
        vb6runtime::library::datetime::date_statement::date_statement(&value)
            .map_err(|e| self.error_here(e, None))?;
        // When the real clock is allowed, also write the OS clock and clear the mock offset.
        if self.allow_system_time {
            let serial = value
                .as_date_serial()
                .map_err(|e| self.error_here(e, None))?;
            if let Some(ts) = serial_to_timestamp(serial) {
                if let Err(_e) = vb6runtime::state::clock::system_set(ts) {
                    // Best-effort: the real clock write may fail due to
                    // permissions.  The mock clock still has the correct value.
                }
                vb6runtime::state::clock::reset();
            }
        }
        Ok(())
    }

    /// `AppActivate title[, wait]`: bring a matching window to the foreground.
    ///
    /// The title expression is converted to its string form (a numeric
    /// Shell task ID becomes its decimal digits, which platform backends
    /// may resolve to a window); `wait` defaults to `False`.
    pub(crate) fn exec_app_activate(&mut self, node: &CstNode) -> RunResult<()> {
        let arg_list = node
            .children_by_kind(SyntaxKind::ArgumentList)
            .next()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;

        let args: Vec<&CstNode> = arg_list.children_by_kind(SyntaxKind::Argument).collect();
        if args.is_empty() || args.len() > 2 {
            return Err(self.error_here(VBError::invalid_procedure_call(), None));
        }
        let title_node = args[0]
            .significant_children()
            .next()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;
        let title = self.eval_expr(title_node)?;
        let wait = match args.get(1) {
            Some(arg) => {
                let expr_node = arg
                    .significant_children()
                    .next()
                    .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;
                let value = if matches!(
                    expr_node.kind(),
                    SyntaxKind::TrueKeyword | SyntaxKind::FalseKeyword
                ) {
                    self.eval_literal(expr_node)?
                } else {
                    self.eval_expr(expr_node)?
                };
                value.as_bool()?
            }
            None => false,
        };
        let title = title.as_string().map_err(|e| self.error_here(e, None))?;
        vb6runtime::library::interaction::app_activate::app_activate(
            &vb6runtime::value::VBString::from(title),
            wait,
        )
        .map_err(|e| self.error_here(e, None))?;
        Ok(())
    }

    /// `SendKeys string[, wait]`: send keystrokes to the active window.
    ///
    /// The keystroke expression is converted to its string form; `wait`
    /// defaults to `False`. Malformed key strings raise VB6 error 5.
    pub(crate) fn exec_send_keys(&mut self, node: &CstNode) -> RunResult<()> {
        let arg_list = node
            .children_by_kind(SyntaxKind::ArgumentList)
            .next()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;

        let args: Vec<&CstNode> = arg_list.children_by_kind(SyntaxKind::Argument).collect();
        if args.is_empty() || args.len() > 2 {
            return Err(self.error_here(VBError::invalid_procedure_call(), None));
        }
        let keys_node = args[0]
            .significant_children()
            .next()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;
        let keys = self.eval_expr(keys_node)?;
        let wait = match args.get(1) {
            Some(arg) => {
                let expr_node = arg
                    .significant_children()
                    .next()
                    .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;
                let value = if matches!(
                    expr_node.kind(),
                    SyntaxKind::TrueKeyword | SyntaxKind::FalseKeyword
                ) {
                    self.eval_literal(expr_node)?
                } else {
                    self.eval_expr(expr_node)?
                };
                value.as_bool()?
            }
            None => false,
        };
        let keys = keys.as_string().map_err(|e| self.error_here(e, None))?;
        vb6runtime::library::interaction::sendkeys::send_keys(
            &vb6runtime::value::VBString::from(keys),
            wait,
        )
        .map_err(|e| self.error_here(e, None))?;
        Ok(())
    }

    /// `SavePicture picture, filename`: save a picture object to a bitmap
    /// file, overwriting any existing file.
    pub(crate) fn exec_save_picture(&mut self, node: &CstNode) -> RunResult<()> {
        let arg_list = node
            .children_by_kind(SyntaxKind::ArgumentList)
            .next()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;

        let args: Vec<&CstNode> = arg_list.children_by_kind(SyntaxKind::Argument).collect();
        if args.len() != 2 {
            return Err(self.error_here(VBError::invalid_procedure_call(), None));
        }

        // Extract the expression from each Argument node.
        let picture_node = args[0]
            .significant_children()
            .next()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;
        let filename_node = args[1]
            .significant_children()
            .next()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;

        let picture = self.eval_expr(picture_node)?;
        let filename = self.eval_expr(filename_node)?;
        vb6runtime::library::graphics::savepicture::save_picture(&picture, &filename)
            .map_err(|e| self.error_here(e, None))?;
        Ok(())
    }

    /// `Time = expr` statement.
    pub(crate) fn exec_time_statement(&mut self, node: &CstNode) -> RunResult<()> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let eq_index = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::EqualityOperator)
            .ok_or_else(|| {
                self.error_here(vb6core::error::VBError::invalid_procedure_call(), None)
            })?;
        let expr = significant.get(eq_index + 1).ok_or_else(|| {
            self.error_here(vb6core::error::VBError::invalid_procedure_call(), None)
        })?;
        let value = self.eval_expr(expr)?;
        vb6runtime::library::datetime::time_statement::time_statement(&value)
            .map_err(|e| self.error_here(e, None))?;
        if self.allow_system_time {
            let serial = value
                .as_date_serial()
                .map_err(|e| self.error_here(e, None))?;
            if let Some(ts) = time_serial_to_timestamp(serial) {
                if let Err(_e) = vb6runtime::state::clock::system_set(ts) {
                    // Best-effort.
                }
                vb6runtime::state::clock::reset();
            }
        }
        Ok(())
    }

    /// `LSet stringvar = string` / `RSet stringvar = string`: align `string`
    /// within `stringvar` (left or right per `align`) and store the result
    /// back.
    pub(crate) fn exec_alignment_set(
        &mut self,
        node: &CstNode,
        align: fn(&VBString, &VBString) -> VBResult<VBString>,
    ) -> RunResult<()> {
        // Locate the BinaryExpression child (the `target = value` portion).
        let binary = node
            .children()
            .iter()
            .find(|c| c.kind() == SyntaxKind::BinaryExpression)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;
        let binary_children: Vec<&CstNode> = binary.significant_children().collect();
        let eq_index = binary_children
            .iter()
            .position(|c| c.kind() == SyntaxKind::EqualityOperator)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;
        // Target variable: the expression before the `=`.
        let target_expr: Vec<&CstNode> = binary_children[..eq_index]
            .iter()
            .copied()
            .filter(|c| c.kind() == SyntaxKind::IdentifierExpression)
            .collect();
        let (Some(target_node), None) = (target_expr.first(), target_expr.get(1)) else {
            return Err(self.error_here(VBError::invalid_procedure_call(), None));
        };
        let name = program::identifier_name(target_node);
        // Value expression: the expression after the `=`.
        let value_expr: Vec<&CstNode> = binary_children[eq_index + 1..]
            .iter()
            .copied()
            .filter(|c| !matches!(c.kind(), SyntaxKind::Whitespace | SyntaxKind::Newline))
            .collect();
        let Some(value) = value_expr.first() else {
            return Err(self.error_here(VBError::invalid_procedure_call(), None));
        };
        let value = self.eval_expr(value)?;
        let value = VBString::try_from(&value).map_err(|e| self.error_here(e, None))?;
        let current = match self.lookup(&name) {
            Some(current) => VBString::try_from(current).map_err(|e| self.error_here(e, None))?,
            None => VBString::from(""),
        };
        let aligned = align(&current, &value).map_err(|e| self.error_here(e, None))?;
        self.set_variable(&name, VBVariant::from(aligned));
        Ok(())
    }

    /// `Mid(target, start[, length]) = string` and the byte-oriented `MidB`
    /// form: overwrite the target variable in place via `apply` and store
    /// the result back.
    pub(crate) fn exec_mid_set(
        &mut self,
        node: &CstNode,
        apply: fn(&VBString, &VBLong, Option<&VBLong>, &VBString) -> VBResult<VBString>,
    ) -> RunResult<()> {
        const ARITY_MESSAGE: &str = "Mid expects Mid(target, start[, length]) = string";

        let arg_list = node
            .children_by_kind(SyntaxKind::ArgumentList)
            .next()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;

        let args: Vec<&CstNode> = arg_list.children_by_kind(SyntaxKind::Argument).collect();
        if !(2..=3).contains(&args.len()) {
            return Err(self.error_here(
                VBError::with_description(err_number::INVALID_PROCEDURE_CALL, ARITY_MESSAGE),
                None,
            ));
        }

        // First argument names the target variable.
        if !program::is_identifier_like(args[0]) {
            return Err(self.error_here(VBError::invalid_procedure_call(), None));
        }
        let name = args[0].text().trim().to_string();

        // Second argument: start position.
        let start_expr = args[1]
            .significant_children()
            .next()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;
        let start =
            VBLong::try_from(&self.eval_expr(start_expr)?).map_err(|e| self.error_here(e, None))?;

        // Third argument (optional): length.
        let length = if let Some(len_arg) = args.get(2) {
            let len_expr = len_arg
                .significant_children()
                .next()
                .ok_or_else(|| self.error_here(VBError::invalid_procedure_call(), None))?;
            Some(
                VBLong::try_from(&self.eval_expr(len_expr)?)
                    .map_err(|e| self.error_here(e, None))?,
            )
        } else {
            None
        };

        // Replacement expression: the expression after the `=` operator.
        let sig: Vec<&CstNode> = node.significant_children().collect();
        let eq_pos = sig
            .iter()
            .position(|c| c.kind() == SyntaxKind::EqualityOperator);
        let value_node = match eq_pos.and_then(|i| sig.get(i + 1)) {
            Some(node) => node,
            None => {
                return Err(self.error_here(
                    VBError::with_description(
                        err_number::INVALID_PROCEDURE_CALL,
                        "Mid/MidB requires a replacement expression after '='",
                    ),
                    None,
                ));
            }
        };
        let value = self.eval_expr(value_node)?;
        let value = VBString::try_from(&value).map_err(|e| self.error_here(e, None))?;

        let current = match self.lookup(&name) {
            Some(current) => VBString::try_from(current).map_err(|e| self.error_here(e, None))?,
            None => VBString::from(""),
        };
        let updated = apply(&current, &start, length.as_ref(), &value)
            .map_err(|e| self.error_here(e, None))?;
        self.set_variable(&name, VBVariant::from(updated));
        Ok(())
    }
}
