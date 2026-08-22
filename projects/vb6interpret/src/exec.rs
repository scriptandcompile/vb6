//! Statement execution over the CST.
//!
//! Statements are dispatched by [`SyntaxKind`] and executed directly against
//! the tree. Line numbers are tracked by counting newlines: each block walks
//! its raw children, so nested bodies receive accurate start lines without
//! accumulating loop iterations.

use vb6core::error::{err_number, VBError, VBResult};
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;
use vb6runtime::library::file as filefn;
use vb6runtime::state::file as file_state;
use vb6runtime::value::VBString;
use vb6runtime::{ArrayValue, VBVariant};

use crate::error::{RunError, RunResult};
use crate::eval::ArithmaticOperator;
use crate::interpreter::{Flow, Interpreter};

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
use crate::program::{identifier_name, is_identifier_like, is_statement_kind, type_from_keyword};

/// Number of `\n` characters in a node's text span.
pub(crate) fn count_newlines(node: &CstNode) -> usize {
    node.text().matches('\n').count()
}

impl Interpreter {
    /// Execute every statement in a block (a `StatementList` or the module
    /// root). `start_line` is the 1-based line the block's first statement is
    /// on.
    pub(crate) fn exec_statements(
        &mut self,
        parent: &CstNode,
        start_line: usize,
    ) -> RunResult<Flow> {
        let mut line = start_line;
        for child in parent.children() {
            match child.kind() {
                SyntaxKind::Newline => line += 1,
                SyntaxKind::Whitespace | SyntaxKind::EndOfLineComment | SyntaxKind::RemComment => {}
                SyntaxKind::LabelStatement => {
                    // Labels matter only for GoTo/GoSub, which are unsupported.
                }
                kind if is_statement_kind(kind) => {
                    self.current_stmt_line = line;
                    // Loops emit their own element-level trace snapshots, so
                    // skip the generic whole-line snapshot to avoid a
                    // duplicate highlight of the loop line at entry.
                    if self.record_debug_snapshots
                        && matches!(
                            kind,
                            SyntaxKind::ForStatement
                                | SyntaxKind::DoStatement
                                | SyntaxKind::WhileStatement
                        )
                    {
                        self.step_without_snapshot()?;
                    } else {
                        self.step()?;
                    }
                    let flow = self.exec_stmt(child, line)?;
                    if flow != Flow::Next {
                        return Ok(flow);
                    }
                    line += count_newlines(child);
                }
                _ => {}
            }
        }
        Ok(Flow::Next)
    }

    /// Execute a single statement.
    pub(crate) fn exec_stmt(&mut self, node: &CstNode, line: usize) -> RunResult<Flow> {
        match node.kind() {
            SyntaxKind::AssignmentStatement => {
                self.exec_assignment(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::LetStatement | SyntaxKind::SetStatement => {
                self.exec_assignment(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::DimStatement | SyntaxKind::ConstStatement => {
                self.exec_dim(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::ReDimStatement => {
                self.exec_redim(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::IfStatement => self.exec_if(node, line),
            SyntaxKind::ForStatement => self.exec_for(node, line),
            SyntaxKind::ForEachStatement => Err(self.unsupported(node, "For Each")),
            SyntaxKind::DoStatement => self.exec_do(node, line),
            SyntaxKind::WhileStatement => self.exec_while(node, line),
            SyntaxKind::SelectCaseStatement => self.exec_select(node, line),
            SyntaxKind::CallStatement => self.exec_call(node),
            SyntaxKind::PrintStatement => {
                self.print_node(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::OpenStatement => {
                self.exec_open(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::CloseStatement => {
                self.exec_close(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::ExitStatement => self.exec_exit(node),
            SyntaxKind::EndStatement => {
                self.terminated = true;
                Ok(Flow::Terminate)
            }
            SyntaxKind::StopStatement => {
                vb6runtime::library::interaction::stop::stop();
                if self.record_debug_snapshots {
                    // Development environment: suspend execution (break
                    // mode) without closing files or clearing variables.
                    return Err(RunError::debug_pause()
                        .at_line(line)
                        .in_procedure(&self.current_procedure_name()));
                }
                // Compiled executable: `Stop` acts like `End`.
                self.terminated = true;
                Ok(Flow::Terminate)
            }
            SyntaxKind::BeepStatement => {
                vb6runtime::library::interaction::beep::beep();
                Ok(Flow::Next)
            }
            SyntaxKind::AppActivateStatement => {
                self.exec_app_activate(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::SendKeysStatement => {
                self.exec_send_keys(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::OptionStatement
            | SyntaxKind::TypeStatement
            | SyntaxKind::EnumStatement
            | SyntaxKind::DeclareStatement => Ok(Flow::Next),
            SyntaxKind::EraseStatement => {
                self.exec_erase(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::DateStatement => {
                self.exec_date_statement(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::TimeStatement => {
                self.exec_time_statement(node)?;
                Ok(Flow::Next)
            }
            SyntaxKind::LSetStatement => {
                self.exec_alignment_set(
                    node,
                    vb6runtime::library::string::lset_statement::lset_statement,
                )?;
                Ok(Flow::Next)
            }
            SyntaxKind::RSetStatement => {
                self.exec_alignment_set(
                    node,
                    vb6runtime::library::string::rset_statement::rset_statement,
                )?;
                Ok(Flow::Next)
            }
            SyntaxKind::OnErrorStatement
            | SyntaxKind::GoSubStatement
            | SyntaxKind::GotoStatement
            | SyntaxKind::ReturnStatement
            | SyntaxKind::ResumeStatement
            | SyntaxKind::OnGoToStatement
            | SyntaxKind::OnGoSubStatement => Err(self.unsupported(node, "control-flow statement")),
            other => Err(self.unsupported(node, &format!("statement {other:?}"))),
        }
    }

    /// `Dim` / `Const` declaration, including array bounds and multiple
    /// declarations separated by commas.
    fn exec_dim(&mut self, node: &CstNode) -> RunResult<()> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let is_const = significant
            .first()
            .is_some_and(|c| c.kind() == SyntaxKind::ConstKeyword);

        let mut index = 1; //if is_const { 1 } else { 1 }; // skip Dim/Const keyword

        let mut first = true;
        while index < significant.len() {
            if !first {
                // Skip separator commas between declarations.
                if significant[index].kind() == SyntaxKind::Comma {
                    index += 1;
                }
            }
            first = false;

            if index >= significant.len() || !is_identifier_like(significant[index]) {
                break;
            }
            let name = significant[index].text().trim().to_string();
            index += 1;

            // Optional array bounds: `name ( ... )`.
            let mut bounds: Vec<vb6runtime::ArrayDimension> = Vec::new();
            if index < significant.len() && significant[index].kind() == SyntaxKind::LeftParenthesis
            {
                index += 1;
                while index < significant.len()
                    && significant[index].kind() != SyntaxKind::RightParenthesis
                {
                    let mut dim_parts = Vec::new();
                    while index < significant.len()
                        && significant[index].kind() != SyntaxKind::Comma
                        && significant[index].kind() != SyntaxKind::RightParenthesis
                    {
                        dim_parts.push(significant[index]);
                        index += 1;
                    }
                    bounds.push(self.parse_dimension(&dim_parts)?);
                    if index < significant.len() && significant[index].kind() == SyntaxKind::Comma {
                        index += 1;
                    }
                }
                if index < significant.len() {
                    index += 1; // RightParenthesis
                }
            }

            // Optional `As <type>`.
            let mut ty = vb6core::types::VBType::Variant;
            if index < significant.len() && significant[index].kind() == SyntaxKind::AsKeyword {
                index += 1;
                if index < significant.len() {
                    if let Some(parsed) = type_from_keyword(significant[index]) {
                        ty = parsed;
                    }
                    index += 1;
                }
            }

            if is_const {
                // `Const name [As type] = value`
                let eq = significant[index..]
                    .iter()
                    .position(|c| c.kind() == SyntaxKind::EqualityOperator);
                if let Some(eq) = eq {
                    let value_idx = index + eq + 1;
                    if let Some(value_node) = significant.get(value_idx) {
                        let value = self.eval_expr(value_node)?;
                        let value = coerce(value, &ty);
                        self.declare_in(&name, value);
                    }
                }
            } else if !bounds.is_empty() {
                let array =
                    ArrayValue::new_fixed(ty.clone(), &bounds).map_err(|e| self.error_here(e))?;
                self.declare_in(&name, VBVariant::Array(array));
            } else {
                let value = VBVariant::default_for_type(&ty);
                self.declare_in(&name, value);
            }

            if index < significant.len() && significant[index].kind() == SyntaxKind::Comma {
                index += 1;
            }
        }
        Ok(())
    }

    /// `ReDim name(bounds) [As type]`, rebuilding the array.
    fn exec_redim(&mut self, node: &CstNode) -> RunResult<()> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let mut index = 0;
        if significant
            .first()
            .is_some_and(|c| c.kind() == SyntaxKind::ReDimKeyword)
        {
            index = 1;
        }
        if index >= significant.len() || !is_identifier_like(significant[index]) {
            return Ok(());
        }
        let name = significant[index].text().trim().to_string();
        index += 1;

        let mut bounds: Vec<vb6runtime::ArrayDimension> = Vec::new();
        if index < significant.len() && significant[index].kind() == SyntaxKind::LeftParenthesis {
            index += 1;
            while index < significant.len()
                && significant[index].kind() != SyntaxKind::RightParenthesis
            {
                if significant[index].kind() == SyntaxKind::PreserveKeyword {
                    index += 1;
                    continue;
                }
                let mut dim_parts = Vec::new();
                while index < significant.len()
                    && significant[index].kind() != SyntaxKind::Comma
                    && significant[index].kind() != SyntaxKind::RightParenthesis
                {
                    dim_parts.push(significant[index]);
                    index += 1;
                }
                bounds.push(self.parse_dimension(&dim_parts)?);
                if index < significant.len() && significant[index].kind() == SyntaxKind::Comma {
                    index += 1;
                }
            }
            if index < significant.len() {
                index += 1; // RightParenthesis
            }
        }

        // Keep the existing element type unless a new type is declared.
        let mut ty = self
            .lookup(&name)
            .and_then(|v| v.as_array().ok())
            .map(|a| a.element_type().clone())
            .unwrap_or(vb6core::types::VBType::Variant);
        if index < significant.len() && significant[index].kind() == SyntaxKind::AsKeyword {
            index += 1;
            if index < significant.len() {
                if let Some(parsed) = type_from_keyword(significant[index]) {
                    ty = parsed;
                }
            }
        }

        let array = ArrayValue::new_fixed(ty, &bounds).map_err(|e| self.error_here(e))?;
        self.set_variable(&name, VBVariant::Array(array));
        Ok(())
    }

    /// Parse one dimension's bounds: `expr` or `expr To expr`.
    fn parse_dimension(&mut self, parts: &[&CstNode]) -> RunResult<vb6runtime::ArrayDimension> {
        if parts.is_empty() {
            return Err(self.error_here(VBError::invalid_procedure_call()));
        }
        if let Some(to_index) = parts
            .iter()
            .position(|part| part.kind() == SyntaxKind::ToKeyword)
        {
            let lower = parts[..to_index]
                .last()
                .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
            let upper = parts[to_index + 1..]
                .first()
                .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
            let lo = self.eval_expr(lower)?.as_i32()?;
            let hi = self.eval_expr(upper)?.as_i32()?;
            Ok(vb6runtime::ArrayDimension::new(lo, hi))
        } else {
            let upper = parts
                .last()
                .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
            let hi = self.eval_expr(upper)?.as_i32()?;
            // A single bound uses 0-based indexing (`Dim a(5)` -> 0 To 5).
            Ok(vb6runtime::ArrayDimension::new(0, hi))
        }
    }

    /// `Erase name`: release a dynamic array (fixed arrays reset to defaults).
    fn exec_erase(&mut self, node: &CstNode) -> RunResult<()> {
        let name = node
            .first_child_by_kind(SyntaxKind::Identifier)
            .map(|t| t.text().trim().to_string())
            .unwrap_or_default();
        if let Some(VBVariant::Array(array)) = self.lookup(&name) {
            let element_type = array.element_type().clone();
            let dynamic = ArrayValue::new_dynamic(element_type);
            self.set_variable(&name, VBVariant::Array(dynamic));
        }
        Ok(())
    }

    /// `Date = expr`: set the system date.
    fn exec_date_statement(&mut self, node: &CstNode) -> RunResult<()> {
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
    fn exec_app_activate(&mut self, node: &CstNode) -> RunResult<()> {
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
    fn exec_send_keys(&mut self, node: &CstNode) -> RunResult<()> {
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

    /// `Time = expr` statement.
    fn exec_time_statement(&mut self, node: &CstNode) -> RunResult<()> {
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
    fn exec_alignment_set(
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
            .find(|c| is_identifier_like(c))
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

    /// Evaluate one operand of a flat-token statement: an identifier-like
    /// token names a variable (`Empty` when undeclared, literal keywords
    /// such as `True` included), anything else evaluates as an expression
    /// node.
    fn eval_flat_operand(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        if is_identifier_like(node) {
            let name = node.text().trim();
            if let Some(value) = self.lookup(name) {
                return Ok(value.clone());
            }
            return match node.kind() {
                SyntaxKind::TrueKeyword => Ok(VBVariant::Boolean(true)),
                SyntaxKind::FalseKeyword => Ok(VBVariant::Boolean(false)),
                SyntaxKind::NullKeyword => Ok(VBVariant::Null),
                SyntaxKind::NothingKeyword => Ok(VBVariant::Nothing),
                _ => Ok(VBVariant::Empty),
            };
        }
        self.eval_expr(node)
    }

    /// Assignment, `Set`, or `Let`: `lhs = rhs`.
    fn exec_assignment(&mut self, node: &CstNode) -> RunResult<()> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        // LHS may be preceded by `Let`/`Set` keywords.
        let lhs = significant
            .iter()
            .find(|c| {
                matches!(
                    c.kind(),
                    SyntaxKind::IdentifierExpression
                        | SyntaxKind::CallExpression
                        | SyntaxKind::MemberAccessExpression
                )
            })
            .copied()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        // RHS is the last significant child (a `Set` LHS object may have `New`).
        let rhs = significant
            .last()
            .copied()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let value = self.eval_expr(rhs)?;
        self.assign(lhs, value)
    }

    /// Write a value into a variable, array element, or function result.
    pub(crate) fn assign(&mut self, lhs: &CstNode, value: VBVariant) -> RunResult<()> {
        match lhs.kind() {
            SyntaxKind::IdentifierExpression => {
                let name = identifier_name(lhs);

                // Assigning to the Function's name sets its return value.
                if let Some(frame) = self.frames.last() {
                    if frame.is_function && name.to_lowercase() == frame.name.to_lowercase() {
                        if let Some(frame) = self.frames.last_mut() {
                            frame.return_value = Some(value);
                        }
                        return Ok(());
                    }
                }
                self.set_variable(&name, value);
                Ok(())
            }
            SyntaxKind::CallExpression => {
                let significant: Vec<&CstNode> = lhs.significant_children().collect();
                let name = significant
                    .iter()
                    .find(|c| is_identifier_like(c))
                    .map(|c| c.text().trim().to_string())
                    .unwrap_or_default();
                let argument_list = significant
                    .iter()
                    .find(|c| c.kind() == SyntaxKind::ArgumentList);
                let args = match argument_list {
                    Some(list) => self.eval_args(list)?,
                    None => Vec::new(),
                };
                let indices: Vec<i32> = args
                    .iter()
                    .map(|arg| arg.as_i32())
                    .collect::<VBResult<_>>()?;

                let existing = self
                    .lookup(&name)
                    .cloned()
                    .ok_or_else(|| self.error_here(VBError::subscript_out_of_range()))?;
                if let VBVariant::Array(mut array) = existing {
                    array.set(&indices, value)?;
                    self.set_variable(&name, VBVariant::Array(array));
                    Ok(())
                } else {
                    Err(self.error_here(VBError::type_mismatch()))
                }
            }
            SyntaxKind::MemberAccessExpression => Err(self.unsupported(lhs, "member assignment")),
            _ => Err(self.error_here(VBError::invalid_procedure_call())),
        }
    }

    /// `If` statement (block or single-line form).
    fn exec_if(&mut self, node: &CstNode, line: usize) -> RunResult<Flow> {
        let children: Vec<&CstNode> = node.children().iter().collect();

        // Find the ThenKeyword; the condition is the last significant child
        // before it.
        let then_index = children
            .iter()
            .position(|c| c.kind() == SyntaxKind::ThenKeyword)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let cond = children[..then_index]
            .iter()
            .rev()
            .find(|c| c.is_significant())
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let cond_true = self.eval_expr(cond)?.as_bool()?;

        let is_block = node
            .first_child_by_kind(SyntaxKind::StatementList)
            .is_some();
        if !is_block {
            // Single-line form: statements after Then, optionally an Else.
            let mut matched = cond_true;
            let mut executed = false;
            for child in &children[then_index + 1..] {
                if child.kind() == SyntaxKind::ElseKeyword {
                    matched = !cond_true;
                    continue;
                }
                if !matched || !is_statement_kind(child.kind()) {
                    continue;
                }
                let flow = self.exec_stmt(child, line)?;
                if flow != Flow::Next {
                    return Ok(flow);
                }
                executed = true;
            }
            let _ = executed;
            return Ok(Flow::Next);
        }

        // Block form: walk children, tracking the line, executing the body of
        // the first matching branch only.
        let mut cur_line = line;
        let mut taken = false;
        for child in &children[then_index + 1..] {
            match child.kind() {
                SyntaxKind::Newline => cur_line += 1,
                SyntaxKind::StatementList => {
                    if cond_true && !taken {
                        let flow = self.exec_statements(child, cur_line)?;
                        if flow != Flow::Next {
                            return Ok(flow);
                        }
                        taken = true;
                    }
                }
                SyntaxKind::ElseIfClause => {
                    if !cond_true && !taken {
                        let clause_children: Vec<&CstNode> = child.children().iter().collect();
                        if let Some(ti) = clause_children
                            .iter()
                            .position(|c| c.kind() == SyntaxKind::ThenKeyword)
                        {
                            let elseif_cond = clause_children[..ti]
                                .iter()
                                .rev()
                                .find(|c| c.is_significant())
                                .ok_or_else(|| {
                                    self.error_here(VBError::invalid_procedure_call())
                                })?;
                            let elseif_true = self.eval_expr(elseif_cond)?.as_bool()?;
                            if elseif_true {
                                let flow = self.exec_body_of(child, cur_line)?;
                                if flow != Flow::Next {
                                    return Ok(flow);
                                }
                                taken = true;
                            }
                        }
                    }
                    cur_line += count_newlines(child);
                }
                SyntaxKind::ElseClause => {
                    if !taken {
                        let flow = self.exec_body_of(child, cur_line)?;
                        if flow != Flow::Next {
                            return Ok(flow);
                        }
                        taken = true;
                    }
                    cur_line += count_newlines(child);
                }
                _ => {}
            }
        }
        Ok(Flow::Next)
    }

    /// Execute the first `StatementList` inside a clause, starting at `line`.
    fn exec_body_of(&mut self, clause: &CstNode, line: usize) -> RunResult<Flow> {
        let mut cur_line = line;
        for child in clause.children() {
            match child.kind() {
                SyntaxKind::Newline => cur_line += 1,
                SyntaxKind::StatementList => {
                    return self.exec_statements(child, cur_line);
                }
                _ => {}
            }
        }
        Ok(Flow::Next)
    }

    /// `For var = start To end [Step step] ... Next [var]`.
    fn exec_for(&mut self, node: &CstNode, line: usize) -> RunResult<Flow> {
        let children: Vec<&CstNode> = node.children().iter().collect();
        let significant: Vec<&CstNode> = node.significant_children().collect();

        let counter = node.first_child_by_kind(SyntaxKind::IdentifierExpression);
        let name = counter
            .and_then(|e| e.first_child_by_kind(SyntaxKind::Identifier))
            .map(|t| t.text().trim().to_string())
            .unwrap_or_default();

        let eq_index = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::EqualityOperator);
        let to_index = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::ToKeyword);
        let (Some(eq_index), Some(to_index)) = (eq_index, to_index) else {
            return Err(self.error_here(VBError::invalid_procedure_call()));
        };

        let start_node = significant
            .get(eq_index + 1)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let end_node = significant
            .get(to_index + 1)
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;

        let mut step = VBVariant::from_long(1);
        let mut step_cursor = None;
        if let Some(step_idx) = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::StepKeyword)
        {
            if let Some(step_node) = significant.get(step_idx + 1) {
                step = self.eval_expr(step_node)?;
                step_cursor = Some((significant[step_idx].start_offset(), step_node.end_offset()));
            }
        }

        let start = self.eval_expr(start_node)?;
        let end = self.eval_expr(end_node)?;
        let step_f = step.as_f64()?;
        if step_f == 0.0 {
            return Err(self.error_here(VBError::invalid_procedure_call()));
        }

        let body_index = children
            .iter()
            .position(|c| c.kind() == SyntaxKind::StatementList);
        let body_line = match body_index {
            Some(idx) => {
                line + children[..idx]
                    .iter()
                    .map(|c| count_newlines(c))
                    .sum::<usize>()
            }
            None => line,
        };

        // Sub-line cursor targets for the loop's own elements: the counter
        // assignment (`i = 1`), the end check (`To 100`), the step
        // (`Step 5`), and the `Next i` closing line.
        let start_cursor = counter.map(|c| (c.start_offset(), start_node.end_offset()));
        let to_cursor = Some((significant[to_index].start_offset(), end_node.end_offset()));
        let next_line = match children
            .iter()
            .position(|c| c.kind() == SyntaxKind::NextKeyword)
        {
            Some(next_idx) => {
                line + children[..next_idx]
                    .iter()
                    .map(|c| count_newlines(c))
                    .sum::<usize>()
            }
            None => line,
        };
        let next_cursor = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::NextKeyword)
            .map(|next_idx| {
                let next_kw = significant[next_idx];
                let next_end = significant
                    .get(next_idx + 1)
                    .filter(|c| c.kind() == SyntaxKind::Identifier)
                    .map(|c| c.end_offset())
                    .unwrap_or_else(|| next_kw.end_offset());
                (next_kw.start_offset(), next_end)
            });

        let mut counter_value = start;
        let mut first = true;
        loop {
            // On the first pass highlight the counter assignment; on later
            // passes highlight the check against the end value. Rewinding the
            // line to the loop top is only meaningful for trace snapshots; in
            // run mode it would leave the final highlight stuck on the loop
            // header instead of the `Next` line where the loop ends.
            if self.record_debug_snapshots {
                self.current_stmt_line = line;
            }
            let cursor = if first { start_cursor } else { to_cursor };
            first = false;
            self.step_marked(cursor)?;
            let current = counter_value.as_f64()?;
            let end_value = end.as_f64()?;
            let done = if step_f >= 0.0 {
                current > end_value
            } else {
                current < end_value
            };
            if done {
                break;
            }
            self.set_variable(&name, counter_value.clone());
            if let Some(cursor) = step_cursor {
                self.step_marked(Some(cursor))?;
            }
            if let Some(idx) = body_index {
                let flow = self.exec_statements(children[idx], body_line)?;
                match flow {
                    Flow::Next => {}
                    Flow::BreakLoop => break,
                    Flow::Return | Flow::Terminate => return Ok(flow),
                }
            }
            self.current_stmt_line = next_line;
            self.step_marked(next_cursor)?;
            counter_value = self.arith(counter_value, step.clone(), ArithmaticOperator::Add)?;
        }
        self.set_variable(&name, counter_value);
        Ok(Flow::Next)
    }

    /// `Do ... Loop` with optional `While`/`Until` pre- and post-tests.
    fn exec_do(&mut self, node: &CstNode, line: usize) -> RunResult<Flow> {
        let children: Vec<&CstNode> = node.children().iter().collect();
        let significant: Vec<&CstNode> = node.significant_children().collect();

        let body_index = children
            .iter()
            .position(|c| c.kind() == SyntaxKind::StatementList);
        let body_line = match body_index {
            Some(idx) => {
                line + children[..idx]
                    .iter()
                    .map(|c| count_newlines(c))
                    .sum::<usize>()
            }
            None => line,
        };

        let do_index = significant
            .iter()
            .position(|part| part.kind() == SyntaxKind::DoKeyword)
            .unwrap_or(0);
        let do_cursor = significant
            .get(do_index)
            .map(|part| (part.start_offset(), part.end_offset()));
        let loop_index = significant
            .iter()
            .position(|part| part.kind() == SyntaxKind::LoopKeyword);
        let loop_cursor = loop_index
            .and_then(|li| significant.get(li))
            .map(|part| (part.start_offset(), part.end_offset()));
        let loop_line = match children
            .iter()
            .position(|c| c.kind() == SyntaxKind::LoopKeyword)
        {
            Some(idx) => {
                line + children[..idx]
                    .iter()
                    .map(|c| count_newlines(c))
                    .sum::<usize>()
            }
            None => line,
        };

        // Pre-test: `Do While cond` / `Do Until cond`. The cursor spans the
        // `While`/`Until` keyword through the end of the condition.
        let mut pre_test: Option<(bool, &CstNode)> = None;
        let mut pre_cursor = None;
        for part in &significant[do_index + 1..] {
            match part.kind() {
                SyntaxKind::WhileKeyword | SyntaxKind::UntilKeyword => {
                    if let Some(next) = part.next_significant(significant.as_slice()) {
                        let invert = part.kind() == SyntaxKind::UntilKeyword;
                        pre_test = Some((invert, next));
                        pre_cursor = Some((part.start_offset(), next.end_offset()));
                    }
                    break;
                }
                SyntaxKind::StatementList => break,
                _ => {}
            }
        }

        // Post-test: `Loop While cond` / `Loop Until cond`. The cursor spans
        // the `While`/`Until` keyword through the end of the condition.
        let mut post_test: Option<(bool, &CstNode)> = None;
        let mut post_cursor = None;
        if let Some(li) = loop_index {
            let after: Vec<&CstNode> = significant[li + 1..].to_vec();
            for part in &after {
                match part.kind() {
                    SyntaxKind::WhileKeyword | SyntaxKind::UntilKeyword => {
                        if let Some(next) = part.next_significant(&after) {
                            let invert = part.kind() == SyntaxKind::UntilKeyword;
                            post_test = Some((invert, next));
                            post_cursor = Some((part.start_offset(), next.end_offset()));
                        }
                        break;
                    }
                    _ => {}
                }
            }
        }

        loop {
            // Entry: highlight the pre-test condition, or the bare `Do`
            // keyword when the loop has no pre-test. Rewinding the line to
            // the loop top is only meaningful for trace snapshots; in run
            // mode it would leave the final highlight stuck on the `Do` line
            // instead of the `Loop` line where the loop ends.
            if self.record_debug_snapshots {
                self.current_stmt_line = line;
            }
            self.step_marked(pre_cursor.or(do_cursor))?;
            if let Some((invert, cond)) = pre_test {
                let condition = self.eval_expr(cond)?.as_bool()?;
                if condition == invert {
                    break;
                }
            }
            if let Some(idx) = body_index {
                let flow = self.exec_statements(children[idx], body_line)?;
                match flow {
                    Flow::Next => {}
                    Flow::BreakLoop => break,
                    Flow::Return | Flow::Terminate => return Ok(flow),
                }
            }
            // After the body: highlight the post-test condition, or the
            // `Loop` keyword when the loop has no post-test.
            self.current_stmt_line = loop_line;
            if post_test.is_some() {
                self.step_marked(post_cursor)?;
                if let Some((invert, cond)) = post_test {
                    let b = self.eval_expr(cond)?.as_bool()?;
                    if b == invert {
                        break;
                    }
                }
            } else if let Some(loop_c) = loop_cursor {
                self.step_marked(Some(loop_c))?;
            }
        }
        Ok(Flow::Next)
    }

    /// `While cond ... Wend`.
    fn exec_while(&mut self, node: &CstNode, line: usize) -> RunResult<Flow> {
        let children: Vec<&CstNode> = node.children().iter().collect();
        let significant: Vec<&CstNode> = node.significant_children().collect();

        let cond = significant
            .iter()
            .find(|c| !matches!(c.kind(), SyntaxKind::WhileKeyword))
            .copied()
            .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
        let cond_cursor = Some((cond.start_offset(), cond.end_offset()));
        let wend_cursor = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::WendKeyword)
            .map(|idx| {
                let wend = significant[idx];
                (wend.start_offset(), wend.end_offset())
            });
        let wend_line = match children
            .iter()
            .position(|c| c.kind() == SyntaxKind::WendKeyword)
        {
            Some(idx) => {
                line + children[..idx]
                    .iter()
                    .map(|c| count_newlines(c))
                    .sum::<usize>()
            }
            None => line,
        };

        let body_index = children
            .iter()
            .position(|c| c.kind() == SyntaxKind::StatementList);
        let body_line = match body_index {
            Some(idx) => {
                line + children[..idx]
                    .iter()
                    .map(|c| count_newlines(c))
                    .sum::<usize>()
            }
            None => line,
        };

        loop {
            // Rewinding the line to the loop top is only meaningful for trace
            // snapshots; in run mode it would leave the final highlight stuck
            // on the `While` header instead of the `Wend` line where the loop
            // ends.
            if self.record_debug_snapshots {
                self.current_stmt_line = line;
            }
            self.step_marked(cond_cursor)?;
            let b = self.eval_expr(cond)?.as_bool()?;
            if !b {
                break;
            }
            if let Some(idx) = body_index {
                let flow = self.exec_statements(children[idx], body_line)?;
                match flow {
                    Flow::Next => {}
                    Flow::BreakLoop => break,
                    Flow::Return | Flow::Terminate => return Ok(flow),
                }
            }
            self.current_stmt_line = wend_line;
            if let Some(cursor) = wend_cursor {
                self.step_marked(Some(cursor))?;
            }
        }
        Ok(Flow::Next)
    }

    /// `Select Case expr ... Case ... End Select`.
    fn exec_select(&mut self, node: &CstNode, line: usize) -> RunResult<Flow> {
        let children: Vec<&CstNode> = node.children().iter().collect();
        let significant: Vec<&CstNode> = node.significant_children().collect();

        let case_keyword = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::CaseKeyword);
        let selector = match case_keyword {
            Some(idx) => {
                let expr = significant
                    .get(idx + 1)
                    .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
                self.eval_expr(expr)?
            }
            None => VBVariant::Empty,
        };

        let mut cur_line = line;
        for child in children {
            match child.kind() {
                SyntaxKind::Newline => cur_line += 1,
                SyntaxKind::CaseClause | SyntaxKind::CaseElseClause => {
                    let is_else = child.kind() == SyntaxKind::CaseElseClause;
                    let matches = if is_else {
                        true
                    } else {
                        self.case_clause_matches(child, &selector)?
                    };
                    if matches {
                        let flow = self.exec_body_of(child, cur_line)?;
                        return Ok(flow);
                    }
                    cur_line += count_newlines(child);
                }
                _ => {}
            }
        }
        Ok(Flow::Next)
    }

    /// Whether a `CaseClause` matches the selector.
    fn case_clause_matches(&mut self, clause: &CstNode, selector: &VBVariant) -> RunResult<bool> {
        let significant: Vec<&CstNode> = clause.significant_children().collect();
        // Skip the leading CaseKeyword and stop at the body StatementList.
        let mut spec: Vec<&CstNode> = Vec::new();
        for c in &significant[1..] {
            if c.kind() == SyntaxKind::StatementList {
                break;
            }
            spec.push(c);
        }

        let mut index = 0;
        while index < spec.len() {
            match spec[index].kind() {
                SyntaxKind::Comma => index += 1,
                SyntaxKind::IsKeyword => {
                    let op = spec
                        .get(index + 1)
                        .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
                    let value_node = spec
                        .get(index + 2)
                        .ok_or_else(|| self.error_here(VBError::invalid_procedure_call()))?;
                    let value = self.eval_case_value(value_node)?;
                    let matched = self.apply_compare(selector, op.kind(), &value)?;
                    if matched {
                        return Ok(true);
                    }
                    index += 3;
                }
                SyntaxKind::ToKeyword => {
                    // Range: `low To high`.
                    let low = self.eval_case_value(spec[index - 1])?;
                    let high = self.eval_case_value(spec[index + 1])?;
                    if self.in_range(selector, &low, &high)? {
                        return Ok(true);
                    }
                    index += 2;
                }
                _ => {
                    let value = self.eval_case_value(spec[index])?;
                    let mut matched = selector == &value;
                    // A trailing `To` opens a range.
                    if index + 1 < spec.len() && spec[index + 1].kind() == SyntaxKind::ToKeyword {
                        let high = self.eval_case_value(spec[index + 2])?;
                        matched = self.in_range(selector, &value, &high)?;
                        index += 2;
                    }
                    if matched {
                        return Ok(true);
                    }
                    index += 1;
                }
            }
        }
        Ok(false)
    }

    /// Evaluate a case value, which may be a bare literal token or an
    /// expression node.
    fn eval_case_value(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        if is_literal_kind(node.kind()) {
            self.eval_literal(node)
        } else {
            self.eval_expr(node)
        }
    }

    /// `low <= selector <= high` (numeric or string range).
    fn in_range(
        &mut self,
        selector: &VBVariant,
        low: &VBVariant,
        high: &VBVariant,
    ) -> RunResult<bool> {
        let s = selector.as_f64()?;
        let lo = low.as_f64()?;
        let hi = high.as_f64()?;
        Ok(s >= lo && s <= hi)
    }

    /// Apply a comparison operator between the selector and a case value.
    fn apply_compare(
        &mut self,
        selector: &VBVariant,
        op: SyntaxKind,
        value: &VBVariant,
    ) -> RunResult<bool> {
        let a = selector.as_f64()?;
        let b = value.as_f64()?;
        Ok(match op {
            SyntaxKind::EqualityOperator => a == b,
            SyntaxKind::InequalityOperator => a != b,
            SyntaxKind::LessThanOperator => a < b,
            SyntaxKind::LessThanOrEqualOperator => a <= b,
            SyntaxKind::GreaterThanOperator => a > b,
            SyntaxKind::GreaterThanOrEqualOperator => a >= b,
            _ => return Err(self.error_here(VBError::invalid_procedure_call())),
        })
    }

    /// `Call` statement: `Debug.Print`, sub-procedure calls, and `MsgBox`.
    fn exec_call(&mut self, node: &CstNode) -> RunResult<Flow> {
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
            .find(|c| is_identifier_like(c) && c.kind() != SyntaxKind::CallKeyword)
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

    /// `Exit Sub|Function|For|Do|While`.
    fn exec_exit(&mut self, node: &CstNode) -> RunResult<Flow> {
        let significant: Vec<&CstNode> = node.significant_children().collect();
        let target = significant
            .iter()
            .find(|c| {
                matches!(
                    c.kind(),
                    SyntaxKind::SubKeyword
                        | SyntaxKind::FunctionKeyword
                        | SyntaxKind::ForKeyword
                        | SyntaxKind::DoKeyword
                        | SyntaxKind::WhileKeyword
                )
            })
            .map(|c| c.kind());
        Ok(match target {
            Some(SyntaxKind::ForKeyword | SyntaxKind::DoKeyword | SyntaxKind::WhileKeyword) => {
                Flow::BreakLoop
            }
            Some(SyntaxKind::SubKeyword | SyntaxKind::FunctionKeyword) => Flow::Return,
            _ => Flow::Next,
        })
    }

    /// Emit `Debug.Print` / `Print` output.
    fn print_node(&mut self, node: &CstNode) -> RunResult<()> {
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

    /// Evaluate a bare literal or `Identifier` token, as found in the flat
    /// token stream of `Open`/`Close` (which aren't parsed into nested
    /// expression nodes like other statements).
    fn eval_flat_token(&mut self, node: &CstNode) -> RunResult<VBVariant> {
        if node.kind() == SyntaxKind::Identifier {
            let name = node.text().trim().to_string();
            return Ok(self.lookup(&name).cloned().unwrap_or(VBVariant::Empty));
        }
        self.eval_literal(node)
    }

    /// `Open pathname For mode [Access access] [lock] As [#]filenumber [Len=reclength]`.
    fn exec_open(&mut self, node: &CstNode) -> RunResult<Flow> {
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
    fn exec_close(&mut self, node: &CstNode) -> RunResult<Flow> {
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

    /// Declare a variable in the current scope (globals at module level).
    pub(crate) fn declare_in(&mut self, name: &str, value: VBVariant) {
        if self.frames.is_empty() {
            self.globals.declare(name, value);
        } else if let Some(frame) = self.frames.last_mut() {
            frame.locals.declare(name, value);
        }
    }

    /// Set a variable, implicit-declaring it in the current scope if needed.
    pub(crate) fn set_variable(&mut self, name: &str, value: VBVariant) {
        if !self.frames.is_empty() {
            if let Some(frame) = self.frames.last_mut() {
                if frame.locals.set(name, value.clone()) {
                    return;
                }
            }
        }
        if self.globals.set(name, value.clone()) {
            return;
        }
        self.declare_in(name, value);
    }

    /// Unsupported-construct error.
    pub(crate) fn unsupported(&self, _node: &CstNode, what: &str) -> RunError {
        self.error_here(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            format!("{what} is not supported yet"),
        ))
    }

    /// Build an error tagged with the current source location.
    pub(crate) fn error_here(&self, error: VBError) -> RunError {
        RunError::new(error)
            .at_line(self.current_stmt_line)
            .in_procedure(&self.current_procedure_name())
    }
}

/// Whether a node kind is a literal token (usable as a case value or bound).
fn is_literal_kind(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::IntegerLiteral
            | SyntaxKind::LongLiteral
            | SyntaxKind::SingleLiteral
            | SyntaxKind::DoubleLiteral
            | SyntaxKind::CurrencyLiteral
            | SyntaxKind::DecimalLiteral
            | SyntaxKind::DateLiteral
            | SyntaxKind::StringLiteral
            | SyntaxKind::TrueKeyword
            | SyntaxKind::FalseKeyword
    )
}

/// Coerce a value to a static type following VB6 conversion semantics.
pub(crate) fn coerce(value: VBVariant, ty: &vb6core::types::VBType) -> VBVariant {
    match ty {
        vb6core::types::VBType::Byte => value.as_byte().map(VBVariant::Byte).unwrap_or(value),
        vb6core::types::VBType::Integer => value.as_i16().map(VBVariant::Integer).unwrap_or(value),
        vb6core::types::VBType::Long => value.as_i32().map(VBVariant::Long).unwrap_or(value),
        vb6core::types::VBType::Single => value.as_f32().map(VBVariant::Single).unwrap_or(value),
        vb6core::types::VBType::Double => value.as_f64().map(VBVariant::Double).unwrap_or(value),
        vb6core::types::VBType::Currency => value
            .as_currency_scaled()
            .map(VBVariant::Currency)
            .unwrap_or(value),
        vb6core::types::VBType::String => value
            .as_string()
            .map(VBVariant::from_string)
            .unwrap_or(value),
        vb6core::types::VBType::Boolean => value.as_bool().map(VBVariant::Boolean).unwrap_or(value),
        vb6core::types::VBType::Date => {
            value.as_date_serial().map(VBVariant::Date).unwrap_or(value)
        }
        _ => value,
    }
}

/// Trait to find the node following a keyword in a significant-child list.
trait NextSignificant {
    fn next_significant<'a>(&self, siblings: &[&'a CstNode]) -> Option<&'a CstNode>;
}

impl NextSignificant for CstNode {
    fn next_significant<'a>(&self, siblings: &[&'a CstNode]) -> Option<&'a CstNode> {
        siblings
            .iter()
            .position(|c| std::ptr::eq(*c, self))
            .and_then(|i| siblings.get(i + 1))
            .copied()
    }
}
