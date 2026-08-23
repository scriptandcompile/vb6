//! Simple builtin statements: clock (`Date`/`Time`), window/process
//! interaction (`AppActivate`, `SendKeys`), graphics (`SavePicture`), and
//! in-place string mutation (`LSet`/`RSet`/`Mid`/`MidB` assignment forms).

use vb6core::error::{err_number, VBError, VBResult};
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;
use vb6runtime::value::{VBLong, VBString};
use vb6runtime::VBVariant;

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
            .ok_or_else(|| self.error_here(vb6core::error::VBError::invalid_procedure_call()))?;
        let expr = significant
            .get(eq_index + 1)
            .ok_or_else(|| self.error_here(vb6core::error::VBError::invalid_procedure_call()))?;
        let value = self.eval_expr(expr)?;
        // Always set the mock clock so `Date` reads correctly.
        vb6runtime::library::datetime::date_statement::date_statement(&value)
            .map_err(|e| self.error_here(e))?;
        // When the real clock is allowed, also write the OS clock and clear the mock offset.
        if self.allow_system_time {
            let serial = value.as_date_serial().map_err(|e| self.error_here(e))?;
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
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let args: Vec<&CstNode> = significant
            .iter()
            .skip(1) // the AppActivate keyword
            .copied()
            .filter(|c| c.kind() != SyntaxKind::Comma)
            .collect();
        if args.is_empty() || args.len() > 2 {
            return Err(self.error_here(VBError::invalid_procedure_call()));
        }
        let title = self.eval_expr(args[0])?;
        let wait = match args.get(1) {
            Some(expr) => {
                // Simple builtin statements keep their arguments unwrapped,
                // so a literal `True`/`False` arrives as a bare keyword
                // token rather than an expression node.
                let value = if matches!(
                    expr.kind(),
                    SyntaxKind::TrueKeyword | SyntaxKind::FalseKeyword
                ) {
                    self.eval_literal(expr)?
                } else {
                    self.eval_expr(expr)?
                };
                value.as_bool()?
            }
            None => false,
        };
        let title = title.as_string().map_err(|e| self.error_here(e))?;
        vb6runtime::library::interaction::app_activate::app_activate(
            &vb6runtime::value::VBString::from(title),
            wait,
        )
        .map_err(|e| self.error_here(e))?;
        Ok(())
    }

    /// `SendKeys string[, wait]`: send keystrokes to the active window.
    ///
    /// The keystroke expression is converted to its string form; `wait`
    /// defaults to `False`. Malformed key strings raise VB6 error 5.
    pub(crate) fn exec_send_keys(&mut self, node: &CstNode) -> RunResult<()> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let args: Vec<&CstNode> = significant
            .iter()
            .skip(1) // the SendKeys keyword
            .copied()
            .filter(|c| c.kind() != SyntaxKind::Comma)
            .collect();
        if args.is_empty() || args.len() > 2 {
            return Err(self.error_here(VBError::invalid_procedure_call()));
        }
        let keys = self.eval_expr(args[0])?;
        let wait = match args.get(1) {
            Some(expr) => {
                // Simple builtin statements keep their arguments unwrapped,
                // so a literal `True`/`False` arrives as a bare keyword
                // token rather than an expression node.
                let value = if matches!(
                    expr.kind(),
                    SyntaxKind::TrueKeyword | SyntaxKind::FalseKeyword
                ) {
                    self.eval_literal(expr)?
                } else {
                    self.eval_expr(expr)?
                };
                value.as_bool()?
            }
            None => false,
        };
        let keys = keys.as_string().map_err(|e| self.error_here(e))?;
        vb6runtime::library::interaction::sendkeys::send_keys(
            &vb6runtime::value::VBString::from(keys),
            wait,
        )
        .map_err(|e| self.error_here(e))?;
        Ok(())
    }

    /// `SavePicture picture, filename`: save a picture object to a bitmap
    /// file, overwriting any existing file.
    pub(crate) fn exec_save_picture(&mut self, node: &CstNode) -> RunResult<()> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let args: Vec<&CstNode> = significant
            .iter()
            .skip(1) // the SavePicture keyword
            .copied()
            .filter(|c| c.kind() != SyntaxKind::Comma)
            .collect();
        if args.len() != 2 {
            return Err(self.error_here(VBError::invalid_procedure_call()));
        }
        let picture = self.eval_simple_operand(args[0])?;
        let filename = self.eval_simple_operand(args[1])?;
        vb6runtime::library::graphics::savepicture::save_picture(&picture, &filename)
            .map_err(|e| self.error_here(e))?;
        Ok(())
    }

    /// `Time = expr` statement.
    pub(crate) fn exec_time_statement(&mut self, node: &CstNode) -> RunResult<()> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let eq_index = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::EqualityOperator)
            .ok_or_else(|| self.error_here(vb6core::error::VBError::invalid_procedure_call()))?;
        let expr = significant
            .get(eq_index + 1)
            .ok_or_else(|| self.error_here(vb6core::error::VBError::invalid_procedure_call()))?;
        let value = self.eval_expr(expr)?;
        vb6runtime::library::datetime::time_statement::time_statement(&value)
            .map_err(|e| self.error_here(e))?;
        if self.allow_system_time {
            let serial = value.as_date_serial().map_err(|e| self.error_here(e))?;
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
    ///
    /// Like `Open`/`Close`, these statements keep their operands as flat
    /// tokens rather than nested expression nodes, so the source may be a
    /// single identifier-like token, a literal, or a wrapped expression;
    /// compound flat-token expressions are not evaluated. The alignment
    /// width is the target's current length.
    pub(crate) fn exec_alignment_set(
        &mut self,
        node: &CstNode,
        align: fn(&VBString, &VBString) -> VBResult<VBString>,
    ) -> RunResult<()> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let eq_index = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::EqualityOperator)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        // Target variable: the first identifier-like token after the
        // statement keyword (index 0) and before the `=`.
        let target = significant[1..eq_index]
            .iter()
            .find(|c| program::is_identifier_like(c))
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let name = target.text().trim().to_string();
        if significant.len() != eq_index + 2 {
            return Err(self.error_here(VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                "LSet/RSet support only a single source expression",
            )));
        }
        let value = self.eval_flat_operand(significant[eq_index + 1])?;
        let value = VBString::try_from(&value).map_err(|e| self.error_here(e))?;
        let current = match self.lookup(&name) {
            Some(current) => VBString::try_from(current).map_err(|e| self.error_here(e))?,
            None => VBString::from(""),
        };
        let aligned = align(&current, &value).map_err(|e| self.error_here(e))?;
        self.set_variable(&name, VBVariant::from(aligned));
        Ok(())
    }

    /// `Mid(target, start[, length]) = string` and the byte-oriented `MidB`
    /// form: overwrite the target variable in place via `apply` and store
    /// the result back.
    ///
    /// Like other simple builtin statements, operands arrive as flat tokens,
    /// shaped exactly like the `Mid` function call followed by an
    /// assignment: a parenthesized argument list naming the target variable,
    /// its start position, and optional length, then the replacement
    /// expression.
    pub(crate) fn exec_mid_set(
        &mut self,
        node: &CstNode,
        apply: fn(&VBString, &VBLong, Option<&VBLong>, &VBString) -> VBResult<VBString>,
    ) -> RunResult<()> {
        const ARITY_MESSAGE: &str = "Mid expects Mid(target, start[, length]) = string";
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let eq_index = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::EqualityOperator)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let lhs = &significant[1..eq_index];
        if lhs.len() < 2
            || lhs[0].kind() != SyntaxKind::LeftParenthesis
            || lhs[lhs.len() - 1].kind() != SyntaxKind::RightParenthesis
        {
            return Err(self.error_here(VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                ARITY_MESSAGE,
            )));
        }
        let inner = &lhs[1..lhs.len() - 1];

        // Split the argument list on commas into `target, start[, length]`.
        let mut parts: Vec<Vec<&CstNode>> = vec![Vec::new()];
        for child in inner {
            if child.kind() == SyntaxKind::Comma {
                parts.push(Vec::new());
            } else {
                parts.last_mut().expect("parts is never empty").push(child);
            }
        }
        if !(2..=3).contains(&parts.len()) || parts.iter().any(|part| part.len() != 1) {
            return Err(self.error_here(VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                ARITY_MESSAGE,
            )));
        }
        // First argument names the target variable.
        let target = parts[0][0];
        if !program::is_identifier_like(target) {
            return Err(self.error_here(VBError::invalid_procedure_call()));
        }
        let name = target.text().trim().to_string();
        let start = VBLong::try_from(&self.eval_flat_operand(parts[1][0])?)
            .map_err(|e| self.error_here(e))?;
        let length = match parts.get(2) {
            Some(part) => Some(
                VBLong::try_from(&self.eval_flat_operand(part[0])?)
                    .map_err(|e| self.error_here(e))?,
            ),
            None => None,
        };

        if significant.len() != eq_index + 2 {
            return Err(self.error_here(VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                "Mid/MidB support only a single source expression",
            )));
        }
        let value = self.eval_flat_operand(significant[eq_index + 1])?;
        let value = VBString::try_from(&value).map_err(|e| self.error_here(e))?;
        let current = match self.lookup(&name) {
            Some(current) => VBString::try_from(current).map_err(|e| self.error_here(e))?,
            None => VBString::from(""),
        };
        let updated =
            apply(&current, &start, length.as_ref(), &value).map_err(|e| self.error_here(e))?;
        self.set_variable(&name, VBVariant::from(updated));
        Ok(())
    }
}
