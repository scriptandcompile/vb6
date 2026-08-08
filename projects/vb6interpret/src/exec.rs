//! Statement execution over the CST.
//!
//! Statements are dispatched by [`SyntaxKind`] and executed directly against
//! the tree. Line numbers are tracked by counting newlines: each block walks
//! its raw children, so nested bodies receive accurate start lines without
//! accumulating loop iterations.

use vb6core::error::{err_number, VBError, VBResult};
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;
use vb6runtime::{ArrayValue, VBVariant};

use crate::error::{RunError, RunResult};
use crate::eval::ArithmaticOperator;
use crate::interpreter::{Flow, Interpreter};
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
                    self.step()?;
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
            SyntaxKind::ExitStatement => self.exec_exit(node),
            SyntaxKind::EndStatement => {
                self.terminated = true;
                Ok(Flow::Terminate)
            }
            SyntaxKind::StopStatement => Ok(Flow::Next),
            SyntaxKind::BeepStatement => Ok(Flow::Next),
            SyntaxKind::OptionStatement
            | SyntaxKind::TypeStatement
            | SyntaxKind::EnumStatement
            | SyntaxKind::DeclareStatement => Ok(Flow::Next),
            SyntaxKind::EraseStatement => {
                self.exec_erase(node)?;
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

        let name = node
            .first_child_by_kind(SyntaxKind::IdentifierExpression)
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
        if let Some(step_idx) = significant
            .iter()
            .position(|c| c.kind() == SyntaxKind::StepKeyword)
        {
            if let Some(step_node) = significant.get(step_idx + 1) {
                step = self.eval_expr(step_node)?;
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

        let mut counter = start;
        loop {
            self.step()?;
            let current = counter.as_f64()?;
            let end_value = end.as_f64()?;
            let done = if step_f >= 0.0 {
                current > end_value
            } else {
                current < end_value
            };
            if done {
                break;
            }
            self.set_variable(&name, counter.clone());
            if let Some(idx) = body_index {
                let flow = self.exec_statements(children[idx], body_line)?;
                match flow {
                    Flow::Next => {}
                    Flow::BreakLoop => break,
                    Flow::Return | Flow::Terminate => return Ok(flow),
                }
            }
            counter = self.arith(counter, step.clone(), ArithmaticOperator::Add)?;
        }
        self.set_variable(&name, counter);
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
        let loop_index = significant
            .iter()
            .position(|part| part.kind() == SyntaxKind::LoopKeyword);

        // Pre-test: `Do While cond` / `Do Until cond`.
        let mut pre_test: Option<(bool, &CstNode)> = None;
        for part in &significant[do_index + 1..] {
            match part.kind() {
                SyntaxKind::WhileKeyword | SyntaxKind::UntilKeyword => {
                    if let Some(next) = part.next_significant(significant.as_slice()) {
                        let invert = part.kind() == SyntaxKind::UntilKeyword;
                        pre_test = Some((invert, next));
                    }
                    break;
                }
                SyntaxKind::StatementList => break,
                _ => {}
            }
        }

        // Post-test: `Loop While cond` / `Loop Until cond`.
        let mut post_test: Option<(bool, &CstNode)> = None;
        if let Some(li) = loop_index {
            let after: Vec<&CstNode> = significant[li + 1..].to_vec();
            for part in &after {
                match part.kind() {
                    SyntaxKind::WhileKeyword | SyntaxKind::UntilKeyword => {
                        if let Some(next) = part.next_significant(&after) {
                            let invert = part.kind() == SyntaxKind::UntilKeyword;
                            post_test = Some((invert, next));
                        }
                        break;
                    }
                    _ => {}
                }
            }
        }

        loop {
            self.step()?;
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
            if let Some((invert, cond)) = post_test {
                let b = self.eval_expr(cond)?.as_bool()?;
                if b == invert {
                    break;
                }
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
            self.step()?;
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
    fn in_range(&mut self, selector: &VBVariant, low: &VBVariant, high: &VBVariant) -> RunResult<bool> {
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
                if let Some(first) = args.first() {
                    let text = first.as_string()?;
                    self.emit(text, true);
                }
                Ok(Flow::Next)
            }
            "beep" => Ok(Flow::Next),
            _ => Err(self.error_here(VBError::new(35))), // Sub or Function not defined
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
        let argument_list = node.first_child_by_kind(SyntaxKind::ArgumentList);
        let mut trailing_separator = false;
        if let Some(list) = argument_list {
            let significant: Vec<&CstNode> = list.significant_children().collect();
            for child in &significant {
                match child.kind() {
                    SyntaxKind::Argument => {
                        if let Some(expr) = child.first_non_whitespace_child() {
                            let value = self.eval_expr(expr)?;
                            let text = value.as_string()?;
                            self.current_output.push_str(&text);
                        }
                    }
                    SyntaxKind::Comma => self.current_output.push('\t'),
                    SyntaxKind::Semicolon => {}
                    _ => {}
                }
            }
            trailing_separator = matches!(
                significant.last().map(|c| c.kind()),
                Some(SyntaxKind::Comma | SyntaxKind::Semicolon)
            );
        }

        if !trailing_separator {
            self.output.push(std::mem::take(&mut self.current_output));
        }

        Ok(())
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
        vb6core::types::VBType::String => {
            value.as_string().map(VBVariant::from_string).unwrap_or(value)
        }
        vb6core::types::VBType::Boolean => value.as_bool().map(VBVariant::Boolean).unwrap_or(value),
        vb6core::types::VBType::Date => value.as_date_serial().map(VBVariant::Date).unwrap_or(value),
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
