//! Semantic analyzer for VB6 code. This module performs semantic analysis on the parsed CST
//! to build symbol tables, resolve names, check types, and report errors.
//!
//! The main entry point is the `SemanticAnalyzer` struct, which provides methods to analyze
//! project files, module files, class files, and form files. The analyzer maintains a scope
//! manager to handle symbol resolution and a type checker for validating type usage. Errors and
//! warnings are collected during analysis and can be retrieved after the process is complete.
//!
//! The `NameResolver` struct is used for resolving symbol references within the current
//! scope context. It provides methods to resolve simple names and qualified names, as well
//! as checking symbol accessibility.
//!
//! The `ScopeManager` struct manages the hierarchy of scopes and symbols, allowing for lookups
//! and scope transitions during analysis. The `TypeChecker` struct provides methods for checking
//! type compatibility and assignment validity.
//!
//! Overall, this module is responsible for the core semantic analysis logic that ensures the VB6 code
//! is semantically correct and provides meaningful error messages for any issues found.
//!
//! # Examples
//!
//! ```rust, no_run
//! use vb6semantic::SemanticAnalyzer;
//! use vb6parse::io::SourceFile;
//! use vb6parse::files::ProjectFile;
//!
//! let mut analyzer = SemanticAnalyzer::new();
//! let project_source = SourceFile::from_file("MyProject.vbp").expect("Failed to read project file");
//! let (project_opt, failures) = ProjectFile::parse(&project_source).unpack();
//! if !failures.is_empty() {
//!     eprintln!("Failed to parse project file: {:?}", failures);
//!     return;
//! }
//!
//! let project = project_opt.expect("Project file should have parsed successfully");
//!
//! let analysis_result = analyzer.analyze_project(&project).expect("Failed to analyze project");
//! println!("Analysis completed with {} errors and {} warnings", analysis_result.errors.len(), analysis_result.warnings.len());
//! ```

use crate::error::{Result, SemanticError, SourceLocation};
use crate::scope::{ScopeKind, ScopeManager};
use crate::symbols::{Symbol, SymbolKind, Visibility};
use crate::types::{TypeChecker, TypeInfo, TypeKind};
use std::collections::HashMap;
use vb6parse::parsers::SyntaxKind;
use vb6parse::parsers::cst::CstNode;

/// Main semantic analyzer that processes VB6 code
pub struct SemanticAnalyzer {
    /// Scope manager for symbol resolution
    scope_manager: ScopeManager,

    /// Type checker for type validation
    #[allow(dead_code)]
    type_checker: TypeChecker,

    /// Current file being analyzed
    current_file: Option<String>,

    /// Interfaces implemented by the current class
    implements: Vec<String>,

    /// Collected errors
    errors: Vec<SemanticError>,

    /// Collected warnings
    warnings: Vec<String>,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer instance
    pub fn new() -> Self {
        Self {
            scope_manager: ScopeManager::new(),
            type_checker: TypeChecker::new(),
            current_file: None,
            implements: Vec::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Analyze a VB6 project file
    pub fn analyze_project(
        &mut self,
        project: &vb6parse::files::ProjectFile,
    ) -> Result<AnalysisResult> {
        // Create project-level scope
        let _project_scope = self
            .scope_manager
            .push_scope(ScopeKind::Global, project.properties.name.to_string());

        // TODO: Analyze project references and external dependencies

        for module_reference in project.modules() {
            self.analyze_module_reference(module_reference)?;
        }

        for class_reference in project.classes() {
            self.analyze_class_reference(class_reference)?;
        }

        for form_file_name in project.forms() {
            self.analyze_form_path(form_file_name)?;
        }

        // TODO: Analyze any additional project-level constructs if necessary

        // For now, return empty result
        Ok(AnalysisResult {
            scope_manager: self.scope_manager.clone(),
            errors: self.errors.clone(),
            warnings: self.warnings.clone(),
        })
    }

    /// Analyze a module reference from the project file reference
    pub fn analyze_module_reference(
        &mut self,
        module_reference: &vb6parse::files::project::ProjectModuleReference,
    ) -> Result<()> {
        let source_file =
            vb6parse::io::SourceFile::from_file(module_reference.path).map_err(|e| {
                crate::error::SemanticError::FileReadError {
                    file: module_reference.path.to_string(),
                    message: e.to_string(),
                }
            })?;

        let (module_opt, failures) = vb6parse::files::ModuleFile::parse(&source_file).unpack();
        if let Some(module) = module_opt {
            self.analyze_module(&module)?;
        } else if !failures.is_empty() {
            let diagnostics = failures
                .into_iter()
                .map(|failure| vb6parse::errors::ErrorDetails {
                    source_name: failure.source_name.clone(),
                    source_content: Box::leak(failure.source_content.to_string().into_boxed_str()),
                    error_offset: failure.error_offset,
                    line_start: failure.line_start,
                    line_end: failure.line_end,
                    kind: failure.kind,
                    severity: failure.severity,
                    labels: failure.labels,
                    notes: failure.notes,
                })
                .collect();
            return Err(crate::error::SemanticError::FileParseError {
                file: module_reference.path.to_string(),
                diagnostics,
            });
        }
        Ok(())
    }

    /// Analyze a module file
    pub fn analyze_module(&mut self, module: &vb6parse::files::ModuleFile) -> Result<()> {
        self.current_file = Some(module.name.clone());

        // Create module scope
        let module_scope = self
            .scope_manager
            .push_scope(ScopeKind::Global, module.name.clone());

        // Register the module itself as a symbol
        self.register_self_symbol(
            module.name.clone(),
            SymbolKind::Module,
            TypeInfo::new(TypeKind::Class(module.name.clone())),
            Visibility::Public,
            module_scope,
        )?;

        // Process module-level declarations
        let root = module.cst.to_root_node();
        self.process_statements(&root, root.children())?;

        self.scope_manager.pop_scope()?;
        Ok(())
    }

    /// Analyze a class reference from the project file class reference
    pub fn analyze_class_reference(
        &mut self,
        class_reference: &vb6parse::files::project::ProjectClassReference,
    ) -> Result<()> {
        let source_file =
            vb6parse::io::SourceFile::from_file(class_reference.path).map_err(|e| {
                crate::error::SemanticError::FileReadError {
                    file: class_reference.path.to_string(),
                    message: e.to_string(),
                }
            })?;

        let (class_opt, failures) = vb6parse::files::ClassFile::parse(&source_file).unpack();
        if let Some(class) = class_opt {
            self.analyze_class(&class)?;
        } else if !failures.is_empty() {
            let diagnostics = failures
                .into_iter()
                .map(|failure| vb6parse::errors::ErrorDetails {
                    source_name: failure.source_name.clone(),
                    source_content: Box::leak(failure.source_content.to_string().into_boxed_str()),
                    error_offset: failure.error_offset,
                    line_start: failure.line_start,
                    line_end: failure.line_end,
                    kind: failure.kind,
                    severity: failure.severity,
                    labels: failure.labels,
                    notes: failure.notes,
                })
                .collect();
            return Err(crate::error::SemanticError::FileParseError {
                file: class_reference.path.to_string(),
                diagnostics,
            });
        }
        Ok(())
    }

    /// Analyze a class file
    pub fn analyze_class(&mut self, class: &vb6parse::files::ClassFile) -> Result<()> {
        let class_name = class.header.attributes.name.clone();
        self.current_file = Some(class_name.clone());

        // Create class scope
        let class_scope = self
            .scope_manager
            .push_scope(ScopeKind::Class, class_name.clone());

        // Register the class itself as a symbol
        self.register_self_symbol(
            class_name.clone(),
            SymbolKind::Class,
            TypeInfo::new(TypeKind::Class(class_name.clone())),
            Visibility::Public,
            class_scope,
        )?;

        // Process class members (methods, properties, events, declarations)
        let root = class.cst.to_root_node();
        self.process_statements(&root, root.children())?;

        self.scope_manager.pop_scope()?;
        Ok(())
    }

    /// Analyze a form file by its path
    pub fn analyze_form_path(&mut self, form_reference_path: &str) -> Result<()> {
        let source_file =
            vb6parse::io::SourceFile::from_file(form_reference_path).map_err(|e| {
                crate::error::SemanticError::FileReadError {
                    file: form_reference_path.to_string(),
                    message: e.to_string(),
                }
            })?;

        let (form_opt, failures) = vb6parse::files::FormFile::parse(&source_file).unpack();
        if let Some(form) = form_opt {
            self.analyze_form(&form)?;
        } else if !failures.is_empty() {
            let diagnostics = failures
                .into_iter()
                .map(|failure| vb6parse::errors::ErrorDetails {
                    source_name: failure.source_name.clone(),
                    source_content: Box::leak(failure.source_content.to_string().into_boxed_str()),
                    error_offset: failure.error_offset,
                    line_start: failure.line_start,
                    line_end: failure.line_end,
                    kind: failure.kind,
                    severity: failure.severity,
                    labels: failure.labels,
                    notes: failure.notes,
                })
                .collect();
            return Err(crate::error::SemanticError::FileParseError {
                file: form_reference_path.to_string(),
                diagnostics,
            });
        }
        Ok(())
    }

    /// Analyze a form file
    pub fn analyze_form(&mut self, form: &vb6parse::files::FormFile) -> Result<()> {
        let form_name = form.form.name().to_string();
        self.current_file = Some(form_name.clone());

        // Create form scope (forms are like classes)
        let form_scope = self
            .scope_manager
            .push_scope(ScopeKind::Class, form_name.clone());

        // Register the form itself as a symbol
        self.register_self_symbol(
            form_name.clone(),
            SymbolKind::Form,
            TypeInfo::new(TypeKind::Class(form_name.clone())),
            Visibility::Public,
            form_scope,
        )?;

        // Register controls and menus as symbols in the form scope
        for control in form.form.controls() {
            self.register_control(control)?;
        }
        for menu in form.form.menus() {
            self.register_menu(menu)?;
        }

        // Process the form code section (event handlers, module-level declarations)
        let root = form.cst.to_root_node();
        self.process_statements(&root, root.children())?;

        self.scope_manager.pop_scope()?;
        Ok(())
    }

    /// Add a symbol to the current scope
    pub fn add_symbol(&mut self, symbol: Symbol) -> Result<()> {
        match self.scope_manager.add_symbol(symbol) {
            Ok(()) => Ok(()),
            Err(e) => {
                self.errors.push(e.clone());
                Err(e)
            }
        }
    }

    /// Lookup a symbol
    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.scope_manager.lookup(name)
    }

    /// Get the scope manager (for inspection)
    pub fn scope_manager(&self) -> &ScopeManager {
        &self.scope_manager
    }

    /// Get collected errors
    pub fn errors(&self) -> &[crate::error::SemanticError] {
        &self.errors
    }

    /// Get collected warnings
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Add a warning
    pub fn add_warning(&mut self, message: String) {
        self.warnings.push(message);
    }

    /// Create a source location for current file
    fn make_location(&self, line: usize, column: usize) -> SourceLocation {
        SourceLocation {
            file: self
                .current_file
                .clone()
                .unwrap_or_else(|| "<unknown>".to_string()),
            line,
            column,
        }
    }

    /// Register the symbol representing the analyzed file itself (module, class, or form)
    fn register_self_symbol(
        &mut self,
        name: String,
        kind: SymbolKind,
        type_info: TypeInfo,
        visibility: Visibility,
        scope_id: usize,
    ) -> Result<()> {
        self.add_symbol(Symbol {
            name,
            kind,
            type_info,
            visibility,
            location: self.make_location(1, 1),
            scope_id,
            attributes: HashMap::new(),
        })
    }

    /// Process a sequence of statements at module/class/form level
    fn process_statements(&mut self, root: &CstNode, statements: &[CstNode]) -> Result<()> {
        for statement in statements {
            if statement.is_token() || Self::is_trivia(statement.kind()) {
                continue;
            }
            let line = 1 + Self::preceding_newlines(root, statement);
            self.process_statement(statement, line)?;
        }
        Ok(())
    }

    /// Dispatch a single module/class-level statement
    fn process_statement(&mut self, statement: &CstNode, line: usize) -> Result<()> {
        match statement.kind() {
            SyntaxKind::DimStatement => self.process_dim_statement(statement, line)?,
            SyntaxKind::TypeStatement => self.process_type_statement(statement, line)?,
            SyntaxKind::EnumStatement => self.process_enum_statement(statement, line)?,
            SyntaxKind::DefTypeStatement => self.process_deftype_statement(statement)?,
            SyntaxKind::SubStatement
            | SyntaxKind::FunctionStatement
            | SyntaxKind::PropertyStatement => self.process_procedure(statement, line)?,
            SyntaxKind::DeclareStatement => self.process_declare_statement(statement, line)?,
            SyntaxKind::EventStatement => self.process_event_statement(statement, line)?,
            SyntaxKind::ImplementsStatement => self.process_implements_statement(statement)?,
            _ => {}
        }
        Ok(())
    }

    /// Process a `Dim` or `Const` statement (both use the `DimStatement` syntax kind)
    fn process_dim_statement(&mut self, statement: &CstNode, line: usize) -> Result<()> {
        let is_const = statement
            .children()
            .iter()
            .any(|c| c.kind() == SyntaxKind::ConstKeyword);
        let visibility = Self::visibility_from_statement(statement).unwrap_or(Visibility::Private);
        let scope_id = self.scope_manager.current_scope_id();

        for item in Self::parse_declaration_list(statement) {
            let mut attributes = HashMap::new();
            if is_const {
                attributes.insert("const".to_string(), "true".to_string());
            }
            if item.is_array {
                attributes.insert("array".to_string(), "true".to_string());
            }
            if item.with_events {
                attributes.insert("withevents".to_string(), "true".to_string());
            }
            if let Some(value) = item.value {
                attributes.insert("value".to_string(), value);
            }
            let mut type_info = item.type_info;
            if item.is_array {
                type_info.is_array = true;
            }
            self.add_symbol(Symbol {
                name: item.name,
                kind: if is_const {
                    SymbolKind::Constant
                } else {
                    SymbolKind::Variable
                },
                type_info,
                visibility,
                location: self.make_location(line, 1),
                scope_id,
                attributes,
            })?;
        }
        Ok(())
    }

    /// Process a `Type` definition, registering the type and its members
    fn process_type_statement(&mut self, statement: &CstNode, line: usize) -> Result<()> {
        let name = Self::first_identifier(statement).to_string();
        if name.is_empty() {
            return Ok(());
        }
        let visibility = Self::visibility_from_statement(statement).unwrap_or(Visibility::Private);

        // The type itself lives in the module/class scope; its members live in the type scope
        let scope_id = self.scope_manager.current_scope_id();
        self.add_symbol(Symbol {
            name: name.clone(),
            kind: SymbolKind::UserType,
            type_info: TypeInfo::new(TypeKind::UserType(name.clone())),
            visibility,
            location: self.make_location(line, 1),
            scope_id,
            attributes: HashMap::new(),
        })?;

        let type_scope = self.scope_manager.push_scope(ScopeKind::Type, name);
        if let Some(list) = statement.first_child_by_kind(SyntaxKind::StatementList) {
            let member_line = line + Self::preceding_newlines(statement, list);
            for item in Self::parse_type_members(list) {
                let mut type_info = item.type_info;
                if item.is_array {
                    type_info.is_array = true;
                }
                self.add_symbol(Symbol {
                    name: item.name,
                    kind: SymbolKind::TypeMember,
                    type_info,
                    visibility: Visibility::Private,
                    location: self.make_location(member_line, 1),
                    scope_id: type_scope,
                    attributes: HashMap::new(),
                })?;
            }
        }

        self.scope_manager.pop_scope()?;
        Ok(())
    }

    /// Process an `Enum` definition, registering the enum and its members
    fn process_enum_statement(&mut self, statement: &CstNode, line: usize) -> Result<()> {
        let name = Self::first_identifier(statement).to_string();
        if name.is_empty() {
            return Ok(());
        }
        let visibility = Self::visibility_from_statement(statement).unwrap_or(Visibility::Private);

        // The enum itself lives in the module/class scope; its members live in the enum scope
        let scope_id = self.scope_manager.current_scope_id();
        self.add_symbol(Symbol {
            name: name.clone(),
            kind: SymbolKind::Enum,
            type_info: TypeInfo::new(TypeKind::Enum(name.clone())),
            visibility,
            location: self.make_location(line, 1),
            scope_id,
            attributes: HashMap::new(),
        })?;

        let enum_scope = self.scope_manager.push_scope(ScopeKind::Enum, name.clone());

        if let Some(list) = statement.first_child_by_kind(SyntaxKind::StatementList) {
            let member_line = line + Self::preceding_newlines(statement, list);
            let mut line_tokens: Vec<&CstNode> = Vec::new();
            for child in list.children() {
                if child.kind() == SyntaxKind::Newline {
                    if !line_tokens.is_empty() {
                        self.register_enum_member(
                            &line_tokens,
                            name.clone(),
                            enum_scope,
                            member_line,
                        )?;
                        line_tokens.clear();
                    }
                } else if !Self::is_trivia(child.kind()) {
                    line_tokens.push(child);
                }
            }
            if !line_tokens.is_empty() {
                self.register_enum_member(&line_tokens, name.clone(), enum_scope, member_line)?;
            }
        }

        self.scope_manager.pop_scope()?;
        Ok(())
    }

    /// Register a single enum member from one line of significant tokens
    fn register_enum_member(
        &mut self,
        tokens: &[&CstNode],
        enum_name: String,
        enum_scope: usize,
        line: usize,
    ) -> Result<()> {
        let Some(first) = tokens.first() else {
            return Ok(());
        };
        let member_name = first.text().to_string();

        let mut attributes = HashMap::new();
        let mut i = 1;
        if i < tokens.len() && tokens[i].kind() == SyntaxKind::EqualityOperator {
            i += 1;
            let mut parts = Vec::new();
            while i < tokens.len() {
                parts.push(tokens[i].text().to_string());
                i += 1;
            }
            attributes.insert("value".to_string(), parts.concat());
        }

        self.add_symbol(Symbol {
            name: member_name,
            kind: SymbolKind::EnumMember,
            type_info: TypeInfo::new(TypeKind::Enum(enum_name)),
            visibility: Visibility::Private,
            location: self.make_location(line, 1),
            scope_id: enum_scope,
            attributes,
        })
    }

    /// Process a `Sub`, `Function`, or `Property` procedure declaration
    fn process_procedure(&mut self, statement: &CstNode, line: usize) -> Result<()> {
        let name = Self::first_identifier(statement).to_string();
        if name.is_empty() {
            return Ok(());
        }
        let kind = Self::procedure_symbol_kind(statement);
        let visibility = Self::visibility_from_statement(statement).unwrap_or(Visibility::Public);
        let type_info = match kind {
            SymbolKind::Function => TypeInfo::new(TypeKind::Function {
                return_type: Box::new(Self::procedure_return_type(statement)),
            }),
            SymbolKind::PropertyGet => Self::procedure_return_type(statement),
            _ => TypeInfo::new(TypeKind::Sub),
        };

        let scope_id = self.scope_manager.current_scope_id();

        // Property Get/Let/Set accessors share a name in VB6 and must be merged
        // into a single property symbol rather than treated as duplicates.
        if matches!(
            kind,
            SymbolKind::PropertyGet | SymbolKind::PropertyLet | SymbolKind::PropertySet
        ) && self.scope_manager.lookup_in_scope(scope_id, &name).is_some()
        {
            if let Some(scope) = self.scope_manager.get_scope_mut(scope_id)
                && let Some(existing) = scope.symbols.get_mut(&name)
            {
                let accessor = match kind {
                    SymbolKind::PropertyGet => "get",
                    SymbolKind::PropertyLet => "let",
                    _ => "set",
                };
                let entry = existing
                    .attributes
                    .entry("accessors".to_string())
                    .or_insert_with(String::new);
                if !entry.is_empty() {
                    entry.push(',');
                }
                entry.push_str(accessor);
                if kind == SymbolKind::PropertyGet {
                    existing.kind = SymbolKind::PropertyGet;
                    existing.type_info = type_info;
                }
            }
            self.register_parameters(statement, name, line)?;
            return Ok(());
        }

        let mut attributes = HashMap::new();
        if matches!(
            kind,
            SymbolKind::PropertyGet | SymbolKind::PropertyLet | SymbolKind::PropertySet
        ) {
            let accessor = match kind {
                SymbolKind::PropertyGet => "get",
                SymbolKind::PropertyLet => "let",
                _ => "set",
            };
            attributes.insert("accessors".to_string(), accessor.to_string());
        }

        self.add_symbol(Symbol {
            name: name.clone(),
            kind,
            type_info,
            visibility,
            location: self.make_location(line, 1),
            scope_id,
            attributes,
        })?;

        self.register_parameters(statement, name, line)?;
        Ok(())
    }

    /// Register a procedure's parameters in a new procedure scope
    fn register_parameters(
        &mut self,
        statement: &CstNode,
        name: String,
        line: usize,
    ) -> Result<()> {
        let procedure_scope = self.scope_manager.push_scope(ScopeKind::Procedure, name);
        if let Some(param_list) = statement.first_child_by_kind(SyntaxKind::ParameterList) {
            for param in self.parse_parameter_list(param_list, procedure_scope, line)? {
                self.add_symbol(param)?;
            }
        }
        self.scope_manager.pop_scope()?;
        Ok(())
    }

    /// Process an `Declare` (external API) declaration
    fn process_declare_statement(&mut self, statement: &CstNode, line: usize) -> Result<()> {
        let name = Self::first_identifier(statement).to_string();
        if name.is_empty() {
            return Ok(());
        }
        let is_function = statement
            .children()
            .iter()
            .any(|c| c.kind() == SyntaxKind::FunctionKeyword);
        let kind = if is_function {
            SymbolKind::Function
        } else {
            SymbolKind::SubProcedure
        };
        let type_info = if is_function {
            TypeInfo::new(TypeKind::Function {
                return_type: Box::new(Self::procedure_return_type(statement)),
            })
        } else {
            TypeInfo::new(TypeKind::Sub)
        };

        let mut attributes = HashMap::new();
        attributes.insert("declare".to_string(), "true".to_string());

        let scope_id = self.scope_manager.current_scope_id();
        self.add_symbol(Symbol {
            name: name.clone(),
            kind,
            type_info,
            visibility: Self::visibility_from_statement(statement).unwrap_or(Visibility::Public),
            location: self.make_location(line, 1),
            scope_id,
            attributes,
        })?;

        self.register_parameters(statement, name, line)?;
        Ok(())
    }

    /// Register a `Public Event` declaration
    fn process_event_statement(&mut self, statement: &CstNode, line: usize) -> Result<()> {
        let name = Self::first_identifier(statement).to_string();
        if name.is_empty() {
            return Ok(());
        }
        let mut attributes = HashMap::new();
        attributes.insert("event".to_string(), "true".to_string());
        self.add_symbol(Symbol {
            name,
            kind: SymbolKind::SubProcedure,
            type_info: TypeInfo::new(TypeKind::Sub),
            visibility: Self::visibility_from_statement(statement).unwrap_or(Visibility::Public),
            location: self.make_location(line, 1),
            scope_id: self.scope_manager.current_scope_id(),
            attributes,
        })?;
        Ok(())
    }

    /// Record an `Implements <Interface>` clause
    fn process_implements_statement(&mut self, statement: &CstNode) -> Result<()> {
        let mut after_implements = false;
        for child in statement.children() {
            if child.kind() == SyntaxKind::ImplementsKeyword {
                after_implements = true;
                continue;
            }
            if after_implements && child.kind() == SyntaxKind::Identifier {
                self.implements.push(child.text().to_string());
                break;
            }
        }
        Ok(())
    }

    /// Record the implicit type ranges declared by a `DefType` statement
    fn process_deftype_statement(&mut self, statement: &CstNode) -> Result<()> {
        let letters = Self::def_type_letters(statement);
        if letters.is_empty() {
            return Ok(());
        }
        let type_name = Self::def_type_keyword_name(statement);
        let scope_id = self.scope_manager.current_scope_id();
        let self_name = self.scope_manager.get_scope(scope_id).map(|s| s.name.clone());
        if let Some(self_name) = self_name
            && let Some(scope) = self.scope_manager.get_scope_mut(scope_id)
            && let Some(symbol) = scope.symbols.get_mut(&self_name)
        {
            let entry = symbol
                .attributes
                .entry("deftype".to_string())
                .or_insert_with(String::new);
            if !entry.is_empty() {
                entry.push_str(", ");
            }
            entry.push_str(&format!("{type_name} {letters}"));
        }
        Ok(())
    }

    /// Register a control (and its children) as a symbol in the current form scope
    fn register_control(
        &mut self,
        control: &vb6parse::language::Control,
    ) -> Result<()> {
        let mut attributes = HashMap::new();
        attributes.insert("control".to_string(), control.kind().to_string());
        if control.index() != 0 {
            attributes.insert("index".to_string(), control.index().to_string());
        }
        if !control.tag().is_empty() {
            attributes.insert("tag".to_string(), control.tag().to_string());
        }
        self.add_symbol(Symbol {
            name: control.name().to_string(),
            kind: SymbolKind::Control,
            type_info: TypeInfo::object(),
            visibility: Visibility::Public,
            location: self.make_location(1, 1),
            scope_id: self.scope_manager.current_scope_id(),
            attributes,
        })?;

        // Recursively register controls nested inside containers
        match control.kind() {
            vb6parse::language::ControlKind::Frame { controls, .. } => {
                for child in controls {
                    self.register_control(child)?;
                }
            }
            vb6parse::language::ControlKind::PictureBox { controls, .. } => {
                for child in controls {
                    self.register_control(child)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Register a menu (and its sub-menus) as a symbol in the current form scope
    fn register_menu(&mut self, menu: &vb6parse::language::MenuControl) -> Result<()> {
        let mut attributes = HashMap::new();
        attributes.insert("menu".to_string(), "true".to_string());
        if menu.index() != 0 {
            attributes.insert("index".to_string(), menu.index().to_string());
        }
        self.add_symbol(Symbol {
            name: menu.name().to_string(),
            kind: SymbolKind::Control,
            type_info: TypeInfo::object(),
            visibility: Visibility::Public,
            location: self.make_location(1, 1),
            scope_id: self.scope_manager.current_scope_id(),
            attributes,
        })?;
        for sub in menu.sub_menus() {
            self.register_menu(sub)?;
        }
        Ok(())
    }

    /// Get the first identifier in a statement (the declared name)
    fn first_identifier(node: &CstNode) -> &str {
        node.children()
            .iter()
            .find(|c| c.kind() == SyntaxKind::Identifier)
            .map(|c| c.text())
            .unwrap_or("")
    }

    /// Get the explicit visibility modifier of a statement, if any
    fn visibility_from_statement(statement: &CstNode) -> Option<Visibility> {
        for child in statement.children() {
            match child.kind() {
                SyntaxKind::PrivateKeyword => return Some(Visibility::Private),
                SyntaxKind::PublicKeyword => return Some(Visibility::Public),
                SyntaxKind::FriendKeyword => return Some(Visibility::Friend),
                _ => {}
            }
        }
        None
    }

    /// Determine the symbol kind of a `Sub`/`Function`/`Property` statement
    fn procedure_symbol_kind(statement: &CstNode) -> SymbolKind {
        match statement.kind() {
            SyntaxKind::FunctionStatement => SymbolKind::Function,
            SyntaxKind::PropertyStatement => {
                if statement
                    .children()
                    .iter()
                    .any(|c| c.kind() == SyntaxKind::LetKeyword)
                {
                    SymbolKind::PropertyLet
                } else if statement
                    .children()
                    .iter()
                    .any(|c| c.kind() == SyntaxKind::SetKeyword)
                {
                    SymbolKind::PropertySet
                } else {
                    SymbolKind::PropertyGet
                }
            }
            _ => SymbolKind::SubProcedure,
        }
    }

    /// Extract the `As <type>` clause from a procedure declaration
    fn procedure_return_type(statement: &CstNode) -> TypeInfo {
        let tokens: Vec<&CstNode> = Self::significant_children(statement).collect();
        for (index, token) in tokens.iter().enumerate() {
            if token.kind() == SyntaxKind::AsKeyword {
                let mut j = index + 1;
                return Self::parse_type_from_tokens(&tokens, &mut j);
            }
        }
        TypeInfo::variant()
    }

    /// Map a type-suffix token (e.g. `$`, `%`, `&`) to its `TypeInfo`
    fn type_suffix_type(kind: SyntaxKind) -> Option<TypeInfo> {
        Some(match kind {
            SyntaxKind::DollarSign => TypeInfo::string(),
            SyntaxKind::Percent => TypeInfo::integer(),
            SyntaxKind::Ampersand => TypeInfo::long(),
            SyntaxKind::ExclamationMark => TypeInfo::new(TypeKind::Single),
            SyntaxKind::AtSign => TypeInfo::new(TypeKind::Currency),
            _ => return None,
        })
    }

    /// Parse a type expression starting at `tokens[*index]`, advancing the index past the type
    fn parse_type_from_tokens(tokens: &[&CstNode], index: &mut usize) -> TypeInfo {
        if *index >= tokens.len() {
            return TypeInfo::unknown();
        }
        match tokens[*index].kind() {
            SyntaxKind::NewKeyword => {
                *index += 1;
                TypeInfo::new(TypeKind::Class(Self::join_type_name(tokens, index)))
            }
            SyntaxKind::IntegerKeyword => {
                *index += 1;
                TypeInfo::integer()
            }
            SyntaxKind::LongKeyword => {
                *index += 1;
                TypeInfo::long()
            }
            SyntaxKind::SingleKeyword => {
                *index += 1;
                TypeInfo::new(TypeKind::Single)
            }
            SyntaxKind::DoubleKeyword => {
                *index += 1;
                TypeInfo::new(TypeKind::Double)
            }
            SyntaxKind::CurrencyKeyword => {
                *index += 1;
                TypeInfo::new(TypeKind::Currency)
            }
            SyntaxKind::StringKeyword => {
                *index += 1;
                TypeInfo::string()
            }
            SyntaxKind::BooleanKeyword => {
                *index += 1;
                TypeInfo::boolean()
            }
            SyntaxKind::ByteKeyword => {
                *index += 1;
                TypeInfo::new(TypeKind::Byte)
            }
            SyntaxKind::DateKeyword => {
                *index += 1;
                TypeInfo::new(TypeKind::Date)
            }
            SyntaxKind::VariantKeyword => {
                *index += 1;
                TypeInfo::variant()
            }
            SyntaxKind::ObjectKeyword => {
                *index += 1;
                TypeInfo::object()
            }
            SyntaxKind::Identifier => {
                TypeInfo::new(TypeKind::UserType(Self::join_type_name(tokens, index)))
            }
            _ => TypeInfo::unknown(),
        }
    }

    /// Consume `identifier (. identifier)*` tokens and return the joined name
    fn join_type_name(tokens: &[&CstNode], index: &mut usize) -> String {
        let mut parts = Vec::new();
        while *index < tokens.len() {
            match tokens[*index].kind() {
                SyntaxKind::Identifier | SyntaxKind::PeriodOperator => {
                    parts.push(tokens[*index].text().to_string());
                    *index += 1;
                }
                _ => break,
            }
        }
        parts.concat()
    }

    /// Parse the declarators of a `Dim`/`Const` statement (comma-separated)
    fn parse_declaration_list(node: &CstNode) -> Vec<DeclaredItem> {
        let tokens: Vec<&CstNode> = Self::significant_children(node).collect();
        let mut items = Vec::new();

        let mut i = 0;
        while i < tokens.len()
            && matches!(
                tokens[i].kind(),
                SyntaxKind::DimKeyword
                    | SyntaxKind::ConstKeyword
                    | SyntaxKind::PrivateKeyword
                    | SyntaxKind::PublicKeyword
                    | SyntaxKind::FriendKeyword
                    | SyntaxKind::StaticKeyword
            )
        {
            i += 1;
        }

        Self::parse_comma_separated_declarators(&tokens, &mut i, &mut items);
        items
    }

    /// Parse the members of a `Type` statement (newline-separated)
    fn parse_type_members(list: &CstNode) -> Vec<DeclaredItem> {
        let mut items = Vec::new();
        let mut line_tokens: Vec<&CstNode> = Vec::new();

        for child in list.children() {
            if child.kind() == SyntaxKind::Newline {
                if !line_tokens.is_empty() {
                    let mut i = 0;
                    Self::parse_comma_separated_declarators(&line_tokens, &mut i, &mut items);
                    line_tokens.clear();
                }
            } else if !Self::is_trivia(child.kind()) {
                line_tokens.push(child);
            }
        }
        if !line_tokens.is_empty() {
            let mut i = 0;
            Self::parse_comma_separated_declarators(&line_tokens, &mut i, &mut items);
        }
        items
    }

    /// Parse comma-separated declarators starting at `*index` into `items`
    fn parse_comma_separated_declarators(
        tokens: &[&CstNode],
        index: &mut usize,
        items: &mut Vec<DeclaredItem>,
    ) {
        while *index < tokens.len() {
            let Some(item) = Self::parse_single_declarator(tokens, index) else {
                break;
            };
            items.push(item);
            if *index < tokens.len() && tokens[*index].kind() == SyntaxKind::Comma {
                *index += 1;
            } else {
                break;
            }
        }
    }

    /// Parse a single declarator starting at `tokens[*index]`
    fn parse_single_declarator(tokens: &[&CstNode], index: &mut usize) -> Option<DeclaredItem> {
        let mut with_events = false;
        while *index < tokens.len() && tokens[*index].kind() == SyntaxKind::WithEventsKeyword {
            with_events = true;
            *index += 1;
        }
        if *index >= tokens.len() {
            return None;
        }

        let name = tokens[*index].text().to_string();
        *index += 1;

        let mut type_info = None;
        let mut is_array = false;

        // Array bounds
        if *index < tokens.len() && tokens[*index].kind() == SyntaxKind::LeftParenthesis {
            is_array = true;
            let mut depth = 1;
            *index += 1;
            while *index < tokens.len() && depth > 0 {
                if tokens[*index].kind() == SyntaxKind::LeftParenthesis {
                    depth += 1;
                } else if tokens[*index].kind() == SyntaxKind::RightParenthesis {
                    depth -= 1;
                    if depth == 0 {
                        *index += 1;
                        break;
                    }
                }
                *index += 1;
            }
        }

        // Type suffix
        if *index < tokens.len()
            && let Some(suffix_type) = Self::type_suffix_type(tokens[*index].kind())
        {
            type_info = Some(suffix_type);
            *index += 1;
        }

        // As <type>
        if *index < tokens.len() && tokens[*index].kind() == SyntaxKind::AsKeyword {
            *index += 1;
            type_info = Some(Self::parse_type_from_tokens(tokens, index));
        }

        // Const value
        let mut value = None;
        if *index < tokens.len() && tokens[*index].kind() == SyntaxKind::EqualityOperator {
            *index += 1;
            let mut parts = Vec::new();
            while *index < tokens.len() && tokens[*index].kind() != SyntaxKind::Comma {
                parts.push(tokens[*index].text().to_string());
                *index += 1;
            }
            value = Some(parts.concat());
        }

        Some(DeclaredItem {
            name,
            type_info: type_info.unwrap_or_else(TypeInfo::variant),
            is_array,
            with_events,
            value,
        })
    }

    /// Extract parameter symbols from a `ParameterList` node
    fn parse_parameter_list(
        &self,
        list: &CstNode,
        procedure_scope: usize,
        line: usize,
    ) -> Result<Vec<Symbol>> {
        let tokens: Vec<&CstNode> = Self::significant_children(list).collect();
        let mut symbols = Vec::new();

        let mut i = 0;
        if i < tokens.len() && tokens[i].kind() == SyntaxKind::LeftParenthesis {
            i += 1;
        }

        while i < tokens.len() && tokens[i].kind() != SyntaxKind::RightParenthesis {
            let mut optional = false;
            let mut by_ref = false;
            let mut param_array = false;

            // Modifiers (Optional/ByVal/ByRef/ParamArray)
            loop {
                if i >= tokens.len() {
                    break;
                }
                match tokens[i].kind() {
                    SyntaxKind::OptionalKeyword => {
                        optional = true;
                        i += 1;
                    }
                    SyntaxKind::ByRefKeyword => {
                        by_ref = true;
                        i += 1;
                    }
                    SyntaxKind::ByValKeyword => {
                        i += 1;
                    }
                    SyntaxKind::ParamArrayKeyword => {
                        param_array = true;
                        i += 1;
                    }
                    SyntaxKind::LeftParenthesis => {
                        i += 1;
                    }
                    _ => break,
                }
            }

            if i >= tokens.len() || tokens[i].kind() == SyntaxKind::RightParenthesis {
                break;
            }

            let name = tokens[i].text().to_string();
            i += 1;

            // Array parameter
            let mut is_array = false;
            if i < tokens.len() && tokens[i].kind() == SyntaxKind::LeftParenthesis {
                is_array = true;
                let mut depth = 1;
                i += 1;
                while i < tokens.len() && depth > 0 {
                    if tokens[i].kind() == SyntaxKind::LeftParenthesis {
                        depth += 1;
                    } else if tokens[i].kind() == SyntaxKind::RightParenthesis {
                        depth -= 1;
                        if depth == 0 {
                            i += 1;
                            break;
                        }
                    }
                    i += 1;
                }
            }

            let mut type_info = None;
            if i < tokens.len()
                && let Some(suffix_type) = Self::type_suffix_type(tokens[i].kind())
            {
                type_info = Some(suffix_type);
                i += 1;
            }
            if i < tokens.len() && tokens[i].kind() == SyntaxKind::AsKeyword {
                i += 1;
                type_info = Some(Self::parse_type_from_tokens(&tokens, &mut i));
            }

            // Default value
            let mut default_value = None;
            if i < tokens.len() && tokens[i].kind() == SyntaxKind::EqualityOperator {
                i += 1;
                let mut parts = Vec::new();
                while i < tokens.len()
                    && tokens[i].kind() != SyntaxKind::Comma
                    && tokens[i].kind() != SyntaxKind::RightParenthesis
                {
                    parts.push(tokens[i].text().to_string());
                    i += 1;
                }
                default_value = Some(parts.concat());
            }

            let mut type_info = type_info.unwrap_or_else(TypeInfo::variant);
            type_info.is_reference = by_ref;
            if is_array {
                type_info.is_array = true;
            }

            let mut attributes = HashMap::new();
            if optional {
                attributes.insert("optional".to_string(), "true".to_string());
            }
            if param_array {
                attributes.insert("paramarray".to_string(), "true".to_string());
            }
            if let Some(value) = default_value {
                attributes.insert("default".to_string(), value);
            }

            symbols.push(Symbol {
                name,
                kind: SymbolKind::Parameter,
                type_info,
                visibility: Visibility::Private,
                location: self.make_location(line, 1),
                scope_id: procedure_scope,
                attributes,
            });

            if i < tokens.len() && tokens[i].kind() == SyntaxKind::Comma {
                i += 1;
            }
        }

        Ok(symbols)
    }

    /// Get the name of the `Def*` keyword of a `DefType` statement
    fn def_type_keyword_name(statement: &CstNode) -> String {
        statement
            .children()
            .iter()
            .find(|c| Self::is_def_type_keyword(c.kind()))
            .map(|c| c.text().to_string())
            .unwrap_or_default()
    }

    /// Returns true if the kind is a `Def*` keyword (`DefInt`, `DefStr`, ...)
    fn is_def_type_keyword(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::DefBoolKeyword
                | SyntaxKind::DefByteKeyword
                | SyntaxKind::DefIntKeyword
                | SyntaxKind::DefLngKeyword
                | SyntaxKind::DefCurKeyword
                | SyntaxKind::DefSngKeyword
                | SyntaxKind::DefDblKeyword
                | SyntaxKind::DefDecKeyword
                | SyntaxKind::DefDateKeyword
                | SyntaxKind::DefStrKeyword
                | SyntaxKind::DefObjKeyword
                | SyntaxKind::DefVarKeyword
        )
    }

    /// Collect the letter range text of a `DefType` statement (e.g. `A-Z`, `A,B`)
    fn def_type_letters(statement: &CstNode) -> String {
        let mut parts = Vec::new();
        for child in statement.children() {
            match child.kind() {
                SyntaxKind::Identifier => parts.push(child.text().to_string()),
                SyntaxKind::SubtractionOperator => parts.push("-".to_string()),
                SyntaxKind::Comma => parts.push(",".to_string()),
                _ => {}
            }
        }
        parts.concat()
    }

    /// Returns true if the syntax kind is trivia (whitespace, newlines, comments, line continuations)
    fn is_trivia(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::Whitespace
                | SyntaxKind::Newline
                | SyntaxKind::Underscore
                | SyntaxKind::EndOfLineComment
                | SyntaxKind::RemComment
        )
    }

    /// Iterate over the direct children of `node` that are not trivia
    fn significant_children(node: &CstNode) -> impl Iterator<Item = &CstNode> {
        node.children()
            .iter()
            .filter(|c| !Self::is_trivia(c.kind()))
    }

    /// Count the Newline tokens in a subtree
    fn count_newlines(node: &CstNode) -> usize {
        if node.kind() == SyntaxKind::Newline {
            return 1;
        }
        node.children().iter().map(Self::count_newlines).sum()
    }

    /// Returns true if `target` is `node` or a descendant of `node`
    fn contains(node: &CstNode, target: &CstNode) -> bool {
        if std::ptr::eq(node, target) {
            return true;
        }
        node.children().iter().any(|c| Self::contains(c, target))
    }

    /// Count the Newline tokens that occur before `target` within `ancestor`'s subtree
    fn preceding_newlines(ancestor: &CstNode, target: &CstNode) -> usize {
        if std::ptr::eq(ancestor, target) {
            return 0;
        }
        let mut count = 0;
        for child in ancestor.children() {
            if std::ptr::eq(child, target) {
                return count;
            }
            if Self::contains(child, target) {
                return count + Self::preceding_newlines(child, target);
            }
            count += Self::count_newlines(child);
        }
        count
    }
}

/// A single declarator parsed from a `Dim`/`Const` statement (or a `Type` member)
struct DeclaredItem {
    name: String,
    type_info: TypeInfo,
    is_array: bool,
    with_events: bool,
    value: Option<String>,
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of semantic analysis
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Final scope manager with all symbols
    pub scope_manager: ScopeManager,

    /// Errors found during analysis
    pub errors: Vec<crate::error::SemanticError>,

    /// Warnings generated
    pub warnings: Vec<String>,
}

impl AnalysisResult {
    /// Check if analysis was successful (no errors)
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbols::{SymbolKind, Visibility};
    use crate::types::TypeInfo;
    use std::{collections::HashMap, fs};
    use tempfile::tempdir;

    #[test]
    fn analyze_empty_project() {
        let mut analyzer = SemanticAnalyzer::new();
        let project = vb6parse::files::ProjectFile::default();

        let result = analyzer
            .analyze_project(&project)
            .expect("Analysis should succeed");

        assert!(result.is_successful());
        assert_eq!(result.error_count(), 0);
        assert_eq!(result.warning_count(), 0);
    }

    #[test]
    fn add_symbol_and_lookup_in_current_scope() {
        let mut analyzer = SemanticAnalyzer::new();
        let child_scope_id = analyzer
            .scope_manager
            .push_scope(ScopeKind::Procedure, "proc".to_string());

        let symbol = Symbol {
            name: "counter".to_string(),
            kind: SymbolKind::Variable,
            type_info: TypeInfo::integer(),
            visibility: Visibility::Private,
            location: SourceLocation {
                file: "Module1.bas".to_string(),
                line: 1,
                column: 1,
            },
            scope_id: child_scope_id,
            attributes: HashMap::new(),
        };

        analyzer.add_symbol(symbol).expect("Symbol should be added");

        let resolved = analyzer
            .lookup_symbol("counter")
            .expect("Symbol should be resolvable");
        assert_eq!(resolved.name, "counter");
        assert_eq!(resolved.kind, SymbolKind::Variable);
    }

    #[test]
    fn duplicate_symbol_records_an_error() {
        let mut analyzer = SemanticAnalyzer::new();

        let first = Symbol {
            name: "value".to_string(),
            kind: SymbolKind::Variable,
            type_info: TypeInfo::integer(),
            visibility: Visibility::Private,
            location: SourceLocation {
                file: "Module1.bas".to_string(),
                line: 1,
                column: 1,
            },
            scope_id: analyzer.scope_manager.global_scope_id(),
            attributes: HashMap::new(),
        };
        let second = Symbol {
            name: "value".to_string(),
            kind: SymbolKind::Variable,
            type_info: TypeInfo::integer(),
            visibility: Visibility::Private,
            location: SourceLocation {
                file: "Module1.bas".to_string(),
                line: 2,
                column: 1,
            },
            scope_id: analyzer.scope_manager.global_scope_id(),
            attributes: HashMap::new(),
        };

        analyzer
            .add_symbol(first)
            .expect("First symbol should be added");
        let duplicate = analyzer.add_symbol(second);

        assert!(duplicate.is_err());
        assert!(matches!(
            analyzer.errors().first(),
            Some(crate::error::SemanticError::DuplicateSymbol { .. })
        ));
    }

    #[test]
    fn analyze_module_reference_reports_missing_files() {
        let mut analyzer = SemanticAnalyzer::new();
        let module_reference = vb6parse::files::project::ProjectModuleReference {
            name: "Missing",
            path: "/tmp/does-not-exist.bas",
        };

        let error = analyzer
            .analyze_module_reference(&module_reference)
            .expect_err("Missing files should fail analysis");

        assert!(matches!(
            error,
            crate::error::SemanticError::FileReadError { .. }
        ));
    }

    #[test]
    fn analyze_module_reference_sets_current_file_for_valid_module() {
        let temp_dir = tempdir().expect("Temporary directory should be created");
        let module_path = temp_dir.path().join("Module1.bas");
        fs::write(
            &module_path,
            "Attribute VB_Name = \"Module1\"\nOption Explicit\n",
        )
        .unwrap();

        let mut analyzer = SemanticAnalyzer::new();
        let module_reference = vb6parse::files::project::ProjectModuleReference {
            name: "Module1",
            path: module_path
                .to_str()
                .expect("Module path should be valid UTF-8"),
        };

        analyzer
            .analyze_module_reference(&module_reference)
            .expect("Valid module should be analyzed");

        assert_eq!(analyzer.current_file.as_deref(), Some("Module1"));
        assert_eq!(
            analyzer
                .scope_manager
                .get_scopes_by_kind(ScopeKind::Global)
                .len(),
            2
        );
    }

    fn symbol_in_global_scopes<'a>(
        analyzer: &'a SemanticAnalyzer,
        name: &str,
    ) -> Option<&'a Symbol> {
        analyzer
            .scope_manager()
            .get_scopes_by_kind(ScopeKind::Global)
            .iter()
            .find_map(|scope| scope.symbols.get(name))
    }

    fn symbol_in_scope_kind<'a>(
        analyzer: &'a SemanticAnalyzer,
        kind: ScopeKind,
        name: &str,
    ) -> Option<&'a Symbol> {
        analyzer
            .scope_manager()
            .get_scopes_by_kind(kind)
            .iter()
            .find_map(|scope| scope.symbols.get(name))
    }

    #[test]
    fn analyze_module_collects_declarations() {
        let temp_dir = tempdir().expect("Temporary directory should be created");
        let module_path = temp_dir.path().join("Module1.bas");
        fs::write(
            &module_path,
            r#"Attribute VB_Name = "Module1"
Option Explicit

Private Const APP_NAME = "Test"
Private m_counter As Long
Dim g_values(10) As Integer

Public Type Customer
    Name As String
    Id As Long
End Type

Private Enum Status
    Inactive = 0
    Active
End Enum

Private Sub Initialize()
End Sub

Public Function GetCount() As Long
End Function

Public Sub Increment(ByVal amount As Long, ByRef total As Long)
End Sub
"#,
        )
        .unwrap();

        let source = vb6parse::io::SourceFile::from_file(&module_path).unwrap();
        let (module_opt, failures) = vb6parse::files::ModuleFile::parse(&source).unpack();
        assert!(failures.is_empty(), "Parse failures: {:?}", failures);
        let module = module_opt.expect("Module should parse");

        let mut analyzer = SemanticAnalyzer::new();
        analyzer
            .analyze_module(&module)
            .expect("Analysis should succeed");
        assert!(
            analyzer.errors().is_empty(),
            "Analysis produced errors: {:?}",
            analyzer.errors()
        );

        // The module itself
        let module_symbol =
            symbol_in_global_scopes(&analyzer, "Module1").expect("Module symbol");
        assert_eq!(module_symbol.kind, SymbolKind::Module);

        // Constants
        let app_name = symbol_in_global_scopes(&analyzer, "APP_NAME").expect("Const symbol");
        assert_eq!(app_name.kind, SymbolKind::Constant);
        assert_eq!(
            app_name.attributes.get("const").map(String::as_str),
            Some("true")
        );

        // Variables and arrays
        let counter = symbol_in_global_scopes(&analyzer, "m_counter").expect("Var symbol");
        assert_eq!(counter.kind, SymbolKind::Variable);
        assert_eq!(counter.type_info.kind, TypeKind::Long);

        let g_values = symbol_in_global_scopes(&analyzer, "g_values").expect("Array symbol");
        assert!(g_values.type_info.is_array);
        assert_eq!(g_values.type_info.kind, TypeKind::Integer);

        // User-defined type and its members
        let customer = symbol_in_global_scopes(&analyzer, "Customer").expect("Type symbol");
        assert_eq!(customer.kind, SymbolKind::UserType);
        let name_member = symbol_in_scope_kind(&analyzer, ScopeKind::Type, "Name")
            .expect("Type member");
        assert_eq!(name_member.kind, SymbolKind::TypeMember);
        assert_eq!(name_member.type_info.kind, TypeKind::String);
        let id_member = symbol_in_scope_kind(&analyzer, ScopeKind::Type, "Id").expect("Type member");
        assert_eq!(id_member.type_info.kind, TypeKind::Long);

        // Enum and its members
        let status = symbol_in_global_scopes(&analyzer, "Status").expect("Enum symbol");
        assert_eq!(status.kind, SymbolKind::Enum);
        let inactive =
            symbol_in_scope_kind(&analyzer, ScopeKind::Enum, "Inactive").expect("Enum member");
        assert_eq!(inactive.kind, SymbolKind::EnumMember);
        assert_eq!(
            inactive.attributes.get("value").map(String::as_str),
            Some("0")
        );
        let active = symbol_in_scope_kind(&analyzer, ScopeKind::Enum, "Active").expect("Enum member");
        assert_eq!(active.kind, SymbolKind::EnumMember);

        // Procedures
        let initialize = symbol_in_global_scopes(&analyzer, "Initialize").expect("Sub symbol");
        assert_eq!(initialize.kind, SymbolKind::SubProcedure);
        assert_eq!(initialize.visibility, Visibility::Private);

        let get_count =
            symbol_in_global_scopes(&analyzer, "GetCount").expect("Function symbol");
        assert_eq!(get_count.kind, SymbolKind::Function);
        assert_eq!(get_count.visibility, Visibility::Public);
        assert!(matches!(
            get_count.type_info.kind,
            TypeKind::Function { ref return_type } if return_type.kind == TypeKind::Long
        ));

        // Parameters
        let amount = symbol_in_scope_kind(&analyzer, ScopeKind::Procedure, "amount")
            .expect("Parameter symbol");
        assert_eq!(amount.kind, SymbolKind::Parameter);
        assert_eq!(amount.type_info.kind, TypeKind::Long);
        assert!(!amount.type_info.is_reference);
        let total = symbol_in_scope_kind(&analyzer, ScopeKind::Procedure, "total")
            .expect("Parameter symbol");
        assert_eq!(total.kind, SymbolKind::Parameter);
        assert_eq!(total.type_info.kind, TypeKind::Long);
        assert!(total.type_info.is_reference);
    }

    #[test]
    fn analyze_class_collects_members() {
        let temp_dir = tempdir().expect("Temporary directory should be created");
        let class_path = temp_dir.path().join("Counter.cls");
        fs::write(
            &class_path,
            r#"VERSION 1.0 CLASS
BEGIN
  MultiUse = -1  'True
  Persistable = 0  'NotPersistable
  DataBindingBehavior = 0  'vbNone
  DataSourceBehavior = 0  'vbNone
  MTSTransactionMode = 0  'NotAnMTSObject
END
Attribute VB_Name = "Counter"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = True
Attribute VB_PredeclaredId = False
Attribute VB_Exposed = False

Private m_value As Long

Public Property Get Value() As Long
    Value = m_value
End Property

Public Property Let Value(v As Long)
    m_value = v
End Property

Public Sub Increment()
End Sub

Public Event StatusChanged(NewStatus As String)

Implements TaskInterface
"#,
        )
        .unwrap();

        let source = vb6parse::io::SourceFile::from_file(&class_path).unwrap();
        let (class_opt, failures) = vb6parse::files::ClassFile::parse(&source).unpack();
        assert!(failures.is_empty(), "Parse failures: {:?}", failures);
        let class = class_opt.expect("Class should parse");

        let mut analyzer = SemanticAnalyzer::new();
        analyzer
            .analyze_class(&class)
            .expect("Analysis should succeed");
        assert!(
            analyzer.errors().is_empty(),
            "Analysis produced errors: {:?}",
            analyzer.errors()
        );

        // The class itself
        let class_symbol =
            symbol_in_scope_kind(&analyzer, ScopeKind::Class, "Counter").expect("Class symbol");
        assert_eq!(class_symbol.kind, SymbolKind::Class);

        // Class-level variable
        let m_value = symbol_in_scope_kind(&analyzer, ScopeKind::Class, "m_value")
            .expect("Class variable symbol");
        assert_eq!(m_value.kind, SymbolKind::Variable);
        assert_eq!(m_value.type_info.kind, TypeKind::Long);

        // Property Get + Let are merged into a single symbol
        let value = symbol_in_scope_kind(&analyzer, ScopeKind::Class, "Value").expect("Property");
        assert_eq!(value.kind, SymbolKind::PropertyGet);
        assert_eq!(value.type_info.kind, TypeKind::Long);
        assert_eq!(
            value.attributes.get("accessors").map(String::as_str),
            Some("get,let")
        );

        // Method
        let increment =
            symbol_in_scope_kind(&analyzer, ScopeKind::Class, "Increment").expect("Method symbol");
        assert_eq!(increment.kind, SymbolKind::SubProcedure);

        // Event
        let event = symbol_in_scope_kind(&analyzer, ScopeKind::Class, "StatusChanged")
            .expect("Event symbol");
        assert_eq!(
            event.attributes.get("event").map(String::as_str),
            Some("true")
        );

        // Implements clause was recorded
        assert_eq!(analyzer.implements, vec!["TaskInterface"]);
    }

    #[test]
    fn analyze_form_collects_controls_and_handlers() {
        let temp_dir = tempdir().expect("Temporary directory should be created");
        let form_path = temp_dir.path().join("Form1.frm");
        fs::write(
            &form_path,
            r#"VERSION 5.00
Begin VB.Form Form1
   Caption = "Test Form"
   Begin VB.CommandButton Command1
      Caption = "Click Me"
   End
   Begin VB.Menu mnuFile
      Caption = "&File"
      Begin VB.Menu mnuNew
         Caption = "&New"
      End
   End
End
Attribute VB_Name = "Form1"

Private Sub Command1_Click()
End Sub
"#,
        )
        .unwrap();

        let source = vb6parse::io::SourceFile::from_file(&form_path).unwrap();
        let (form_opt, failures) = vb6parse::files::FormFile::parse(&source).unpack();
        assert!(failures.is_empty(), "Parse failures: {:?}", failures);
        let form = form_opt.expect("Form should parse");

        let mut analyzer = SemanticAnalyzer::new();
        analyzer
            .analyze_form(&form)
            .expect("Analysis should succeed");
        assert!(
            analyzer.errors().is_empty(),
            "Analysis produced errors: {:?}",
            analyzer.errors()
        );

        // The form itself
        let form_symbol =
            symbol_in_scope_kind(&analyzer, ScopeKind::Class, "Form1").expect("Form symbol");
        assert_eq!(form_symbol.kind, SymbolKind::Form);

        // Controls
        let command = symbol_in_scope_kind(&analyzer, ScopeKind::Class, "Command1")
            .expect("Control symbol");
        assert_eq!(command.kind, SymbolKind::Control);
        assert_eq!(
            command.attributes.get("control").map(String::as_str),
            Some("CommandButton")
        );

        // Menus (including sub-menus)
        let menu = symbol_in_scope_kind(&analyzer, ScopeKind::Class, "mnuFile").expect("Menu symbol");
        assert_eq!(
            menu.attributes.get("menu").map(String::as_str),
            Some("true")
        );
        let sub_menu =
            symbol_in_scope_kind(&analyzer, ScopeKind::Class, "mnuNew").expect("Sub-menu symbol");
        assert_eq!(sub_menu.kind, SymbolKind::Control);

        // Event handler from the code section
        let handler = symbol_in_scope_kind(&analyzer, ScopeKind::Class, "Command1_Click")
            .expect("Handler symbol");
        assert_eq!(handler.kind, SymbolKind::SubProcedure);
    }
}
