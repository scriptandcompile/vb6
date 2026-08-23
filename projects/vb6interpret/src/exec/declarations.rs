//! Declaration statements: `Dim`, `Const`, `ReDim`, and `Erase`.

use vb6core::error::VBError;
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;
use vb6runtime::{ArrayValue, VBVariant};

use super::coerce;
use crate::error::RunResult;
use crate::interpreter::Interpreter;
use crate::program::{is_identifier_like, type_from_keyword};

impl Interpreter {
    /// `Dim` / `Const` declaration, including array bounds and multiple
    /// declarations separated by commas.
    pub(crate) fn exec_dim(&mut self, node: &CstNode) -> RunResult<()> {
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
    pub(crate) fn exec_redim(&mut self, node: &CstNode) -> RunResult<()> {
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
    pub(crate) fn exec_erase(&mut self, node: &CstNode) -> RunResult<()> {
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
}
