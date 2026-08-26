//! # `DeleteSetting` Statement
//!
//! Deletes a section or key setting from an application's entry in the Windows registry.
//!
//! ## Syntax
//!
//! ```vb
//! DeleteSetting appname [, section [, key]]
//! ```
//!
//! ## Parts
//!
//! - **appname**: Required. String expression containing the name of the application or project.
//! - **section**: Optional. String expression containing the name of the section to delete.
//! - **key**: Optional. String expression containing the name of the key to delete.
//!
//! ## Examples
//!
//! ```vb
//! DeleteSetting "MyApp", "Startup"            ' Deletes entire Startup section
//! DeleteSetting "MyApp", "Startup", "Left"    ' Deletes Left key from Startup section
//! ```
//!
//! ## References
//!
//! - [DeleteSetting Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/deletesetting-statement)

use crate::Token;
use crate::parsers::cst::Parser;
use crate::parsers::syntaxkind::SyntaxKind;

impl Parser<'_> {
    /// Parses a `DeleteSetting` statement.
    ///
    /// Deletes a section or key setting from an application's registry entry.
    ///
    /// ## Syntax
    ///
    /// ```vb
    /// DeleteSetting appname [, section [, key]]
    /// ```
    ///
    /// ## Reference
    ///
    /// [DeleteSetting Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/deletesetting-statement)
    pub(crate) fn parse_delete_setting_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::DeleteSettingStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.at_token(Token::Newline) && !self.is_at_end() {
            self.parse_separated_expressions(Token::Comma, SyntaxKind::ArgumentList);
        }

        self.builder.finish_node();
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn deletesetting_with_section_only() {
        // Test DeleteSetting with appname and section (deletes entire section)
        let source = r#"
Sub Test()
    DeleteSetting "MyApp", "Startup"
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            "../../../../snapshots/syntax/statements/system_interaction/delete_setting",
        );
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn deletesetting_with_key() {
        // Test DeleteSetting with appname, section, and key
        let source = r#"
Sub Test()
    DeleteSetting "MyApp", "Startup", "Left"
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            "../../../../snapshots/syntax/statements/system_interaction/delete_setting",
        );
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn deletesetting_with_app_productname() {
        // Test DeleteSetting using App.ProductName
        let source = r#"
Sub Test()
    DeleteSetting App.ProductName, "FileFilter"
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            "../../../../snapshots/syntax/statements/system_interaction/delete_setting",
        );
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn deletesetting_with_constants() {
        // Test DeleteSetting with constants
        let source = r#"
Sub Test()
    DeleteSetting REGISTRY_KEY, "Settings", "frmPost.Left"
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            "../../../../snapshots/syntax/statements/system_interaction/delete_setting",
        );
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn deletesetting_multiple_calls() {
        // Test multiple DeleteSetting calls
        let source = r#"
Sub Test()
    DeleteSetting REGISTRY_KEY, "Settings", "frmPost.Left"
    DeleteSetting REGISTRY_KEY, "Settings", "frmPost.Top"
    DeleteSetting REGISTRY_KEY, "Settings", "frmPost.Height"
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            "../../../../snapshots/syntax/statements/system_interaction/delete_setting",
        );
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn deletesetting_with_variables() {
        // Test DeleteSetting with variables
        let source = r#"
Sub Test()
    Dim appName As String
    Dim sectionName As String
    appName = "MyApp"
    sectionName = "Settings"
    DeleteSetting appName, sectionName
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            "../../../../snapshots/syntax/statements/system_interaction/delete_setting",
        );
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn deletesetting_in_loop() {
        // Test DeleteSetting in a loop
        let source = r#"
Sub Test()
    Dim i As Integer
    For i = 1 To 10
        DeleteSetting "MyApp", "Item" & i
    Next i
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            "../../../../snapshots/syntax/statements/system_interaction/delete_setting",
        );
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn deletesetting_with_concatenation() {
        // Test DeleteSetting with string concatenation
        let source = r#"
Sub Test()
    DeleteSetting "MyApp", "Section" & Num, "Key" & Index
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            "../../../../snapshots/syntax/statements/system_interaction/delete_setting",
        );
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn deletesetting_in_if_statement() {
        // Test DeleteSetting in conditional
        let source = r#"
Sub Test()
    If ResetSettings Then
        DeleteSetting "MyApp", "Preferences"
    End If
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            "../../../../snapshots/syntax/statements/system_interaction/delete_setting",
        );
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn deletesetting_with_function_call() {
        // Test DeleteSetting with function call as argument
        let source = r"
Sub Test()
    DeleteSetting GetAppName(), GetSection(), GetKey()
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            "../../../../snapshots/syntax/statements/system_interaction/delete_setting",
        );
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn deletesetting_with_parentheses() {
        // Test DeleteSetting with parentheses around arguments
        let source = r#"
Sub Test()
    DeleteSetting ("MyApp"), ("Settings"), ("WindowState")
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            "../../../../snapshots/syntax/statements/system_interaction/delete_setting",
        );
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn deletesetting_with_error_handling() {
        // Test DeleteSetting with error handling
        let source = r#"
Sub Test()
    On Error Resume Next
    DeleteSetting "MyApp", "Settings"
    If Err Then MsgBox "Error deleting setting"
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            "../../../../snapshots/syntax/statements/system_interaction/delete_setting",
        );
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }
}
