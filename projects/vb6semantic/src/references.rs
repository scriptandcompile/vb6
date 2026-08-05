//! Project reference resolution for VB6 projects.
//!
//! A VB6 `.vbp` file can reference external libraries (COM type libraries such as
//! "Visual Basic For Applications" or "OLE Automation") and sub-projects. On a
//! Windows install the IDE locates these through the registry; that information is
//! not available on other platforms.
//!
//! This module provides a registerable reference-resolution mechanism so that
//! third parties can handle references with their own code. The flow is:
//!
//! 1. The analyzer walks the project's `Reference=` lines and turns each one into
//!    an owned [`ReferenceInfo`] (guid, path, description).
//! 2. It consults the [`ReferenceRegistry`], which holds a list of
//!    [`ReferenceResolver`] implementations. The first resolver whose
//!    [`ReferenceResolver::can_handle`] returns `true` is asked to populate a
//!    [`ScopeKind::Reference`] scope with the symbols the library exposes.
//! 3. If no resolver handles the reference, the analyzer records an
//!    "unresolved reference" warning instead of failing, so projects can be
//!    analyzed on machines that do not have the referenced library installed.
//!
//! Linux builds (or any build without a Windows registry) can register resolvers
//! that supply symbols from bundled data files, such as the
//! [`ManifestReferenceResolver`], which loads symbols from a JSON manifest.
//!
//! # Examples
//!
//! ```rust, no_run
//! use vb6semantic::{
//!     references::{ReferenceResolver, ReferenceRegistry, StaticReferenceResolver},
//!     Symbol, SymbolKind, TypeInfo, Visibility,
//! };
//! use std::collections::HashMap;
//!
//! let mut registry = ReferenceRegistry::new();
//!
//! // A third-party resolver supplies symbols for the "OLE Automation" library.
//! let resolver = StaticReferenceResolver::new(
//!     "ole-automation",
//!     vec!["OLE Automation".to_string()],
//!     vec![Symbol {
//!         name: "Now".to_string(),
//!         kind: SymbolKind::Function,
//!         type_info: TypeInfo::new(vb6semantic::VBType::Date),
//!         visibility: Visibility::Public,
//!         location: vb6semantic::SourceLocation {
//!             file: "<reference>".to_string(),
//!             line: 1,
//!             column: 1,
//!         },
//!         scope_id: 0,
//!         attributes: HashMap::new(),
//!     }],
//! );
//! registry.register(Box::new(resolver));
//! ```

use crate::error::{Result, SemanticError, SourceLocation};
use crate::scope::{ScopeKind, ScopeManager};
use crate::symbols::{Symbol, SymbolKind, Visibility};
use crate::types::{TypeInfo, VBType};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::path::Path;
use vb6parse::files::project::ProjectReference;

/// Owned description of a project reference, decoupled from `vb6parse` lifetimes.
///
/// This is the value passed to [`ReferenceResolver`] implementations and stored
/// in the analysis result.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReferenceInfo {
    /// The GUID of the referenced type library, if this is a compiled reference
    /// (e.g. `00020430-0000-0000-C000-000000000046`).
    pub guid: Option<String>,
    /// The path from the `Reference=` line (e.g. `C:\Windows\System32\stdole2.tlb`
    /// or the sub-project file name).
    pub path: String,
    /// The human-readable description (e.g. "OLE Automation").
    pub description: String,
    /// Whether this is a sub-project reference rather than a compiled library.
    pub is_subproject: bool,
}

impl ReferenceInfo {
    /// Build an owned [`ReferenceInfo`] from a parsed project reference
    pub fn from_project_reference(reference: &ProjectReference) -> ReferenceInfo {
        match reference {
            ProjectReference::Compiled {
                uuid,
                path,
                description,
                ..
            } => ReferenceInfo {
                guid: Some(uuid.to_string()),
                path: (*path).to_string(),
                description: (*description).to_string(),
                is_subproject: false,
            },
            ProjectReference::SubProject { path } => ReferenceInfo {
                guid: None,
                path: (*path).to_string(),
                description: String::new(),
                is_subproject: true,
            },
        }
    }

    /// A short, human-readable name for this reference used in diagnostics
    pub fn display_name(&self) -> String {
        if !self.description.is_empty() {
            self.description.clone()
        } else if let Some(guid) = &self.guid {
            guid.clone()
        } else {
            self.path.clone()
        }
    }

    /// Whether a manifest/reference key matches this reference.
    ///
    /// Matching is case-insensitive and tolerant of the braces that surround guids
    /// in `.vbp` files. A key can be a description, a guid, a full path, or the
    /// file stem of a path.
    pub fn matches_key(key: &str, reference: &ReferenceInfo) -> bool {
        fn normalize(value: &str) -> String {
            value
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .to_ascii_lowercase()
        }

        // The file stem of a path, handling both '/' and '\' separators (paths in
        // .vbp files are Windows-style and must work on Linux too).
        fn path_stem(path: &str) -> Option<String> {
            let file_name = path.rsplit(['/', '\\']).next()?;
            if file_name.is_empty() {
                return None;
            }
            let stem = file_name
                .rsplit_once('.')
                .map(|(stem, _extension)| stem)
                .unwrap_or(file_name);
            Some(stem.to_string())
        }

        let key = normalize(key);
        if normalize(&reference.description) == key {
            return true;
        }
        if reference
            .guid
            .as_ref()
            .is_some_and(|guid| normalize(guid) == key)
        {
            return true;
        }
        if normalize(&reference.path) == key {
            return true;
        }
        // The bare file name (e.g. "stdole2.tlb") and its extensionless stem
        // (e.g. "stdole2") both match.
        if reference
            .path
            .rsplit(['/', '\\'])
            .next()
            .filter(|file_name| !file_name.is_empty())
            .is_some_and(|file_name| normalize(file_name) == key)
        {
            return true;
        }
        path_stem(&reference.path).is_some_and(|stem| normalize(&stem) == key)
    }
}

impl fmt::Display for ReferenceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Context handed to a [`ReferenceResolver`] so it can populate scopes.
///
/// The resolver receives the analyzer's [`ScopeManager`] and a set of helpers
/// that keep the scope structure tidy. Resolvers are expected to push a single
/// [`ScopeKind::Reference`] scope, add their symbols, then pop it.
pub struct ReferenceContext<'a> {
    scope_manager: &'a mut ScopeManager,
    file: &'a str,
}

impl<'a> ReferenceContext<'a> {
    pub(crate) fn new(scope_manager: &'a mut ScopeManager, file: &'a str) -> Self {
        Self {
            scope_manager,
            file,
        }
    }

    /// The name of the project file being analyzed (used as the source for
    /// reference-defined symbols).
    pub fn file(&self) -> &str {
        self.file
    }

    /// The current scope id in the underlying scope manager
    pub fn current_scope_id(&self) -> usize {
        self.scope_manager.current_scope_id()
    }

    /// Access to the underlying scope manager for advanced use
    pub fn scope_manager(&mut self) -> &mut ScopeManager {
        self.scope_manager
    }

    /// Create a new scope as a child of the current one.
    ///
    /// A `ScopeKind::Reference` scope is recorded in the project-wide reference
    /// search order; all other kinds create a plain child scope.
    pub fn push_scope(&mut self, kind: ScopeKind, name: String) -> usize {
        match kind {
            ScopeKind::Reference => self.scope_manager.push_reference_scope(name),
            _ => self.scope_manager.push_scope(kind, name),
        }
    }

    /// Create a new reference-library scope, recording it in the project-wide
    /// reference search order.
    pub fn push_reference_scope(&mut self, name: String) -> usize {
        self.scope_manager.push_reference_scope(name)
    }

    /// Pop the current scope, returning to its parent
    pub fn pop_scope(&mut self) -> Result<()> {
        self.scope_manager.pop_scope()
    }

    /// Add a symbol to the current scope.
    ///
    /// The symbol's `scope_id` is overwritten with the current scope and
    /// duplicate definitions are tolerated (the first definition wins), so
    /// overlapping reference definitions do not abort analysis.
    pub fn add_symbol(&mut self, mut symbol: Symbol) -> Result<()> {
        symbol.scope_id = self.scope_manager.current_scope_id();
        match self.scope_manager.add_symbol(symbol) {
            Ok(()) => Ok(()),
            Err(SemanticError::DuplicateSymbol { .. }) => Ok(()),
            Err(other) => Err(other),
        }
    }
}

/// A third-party handler that supplies symbols for one or more project references.
///
/// Implementors decide which references they serve via [`ReferenceResolver::can_handle`]
/// and populate the scope hierarchy through the [`ReferenceContext`]. Registration
/// happens through [`ReferenceRegistry::register`] or
/// [`crate::SemanticAnalyzer::register_reference_resolver`].
pub trait ReferenceResolver {
    /// A unique name for this resolver, used in diagnostics.
    fn name(&self) -> &str;

    /// Whether this resolver can handle the given reference.
    fn can_handle(&self, reference: &ReferenceInfo) -> bool;

    /// Populate scopes with the symbols exposed by the given reference.
    fn resolve(&mut self, reference: &ReferenceInfo, context: &mut ReferenceContext) -> Result<()>;
}

/// A user-modifiable list of reference resolvers.
///
/// Resolvers are consulted in registration order; the first one that reports it
/// can handle a reference is used for it.
#[derive(Default)]
pub struct ReferenceRegistry {
    resolvers: Vec<Box<dyn ReferenceResolver>>,
}

impl ReferenceRegistry {
    /// Create an empty registry
    pub fn new() -> Self {
        Self {
            resolvers: Vec::new(),
        }
    }

    /// Register a resolver, appending it to the list
    pub fn register(&mut self, resolver: Box<dyn ReferenceResolver>) {
        self.resolvers.push(resolver);
    }

    /// The registered resolvers, in registration order
    pub fn resolvers(&self) -> &[Box<dyn ReferenceResolver>] {
        &self.resolvers
    }

    /// Whether no resolvers are registered
    pub fn is_empty(&self) -> bool {
        self.resolvers.is_empty()
    }

    /// Attempt to resolve a reference against the registered resolvers.
    ///
    /// Returns `Ok(true)` if a resolver handled the reference, `Ok(false)` if no
    /// resolver matched, and `Err` if a matching resolver failed.
    pub fn resolve(
        &mut self,
        reference: &ReferenceInfo,
        scopes: &mut ScopeManager,
        file: &str,
    ) -> Result<bool> {
        for resolver in &mut self.resolvers {
            if !resolver.can_handle(reference) {
                continue;
            }

            let mut context = ReferenceContext::new(scopes, file);
            resolver.resolve(reference, &mut context)?;
            return Ok(true);
        }
        Ok(false)
    }
}

/// A resolver that provides a fixed set of symbols for any reference matching
/// one of its keys.
///
/// This is the building block for third-party reference support: construct one
/// per library you want to support and register it. Keys match against the
/// reference's description, guid, path, or path file stem (see
/// [`ReferenceInfo::matches_key`]).
pub struct StaticReferenceResolver {
    name: String,
    keys: Vec<String>,
    symbols: Vec<Symbol>,
}

impl StaticReferenceResolver {
    /// Create a resolver that serves `symbols` for any reference matching one of `keys`
    pub fn new(name: impl Into<String>, keys: Vec<String>, symbols: Vec<Symbol>) -> Self {
        Self {
            name: name.into(),
            keys,
            symbols,
        }
    }
}

impl ReferenceResolver for StaticReferenceResolver {
    fn name(&self) -> &str {
        &self.name
    }

    fn can_handle(&self, reference: &ReferenceInfo) -> bool {
        self.keys
            .iter()
            .any(|key| ReferenceInfo::matches_key(key, reference))
    }

    fn resolve(&mut self, reference: &ReferenceInfo, context: &mut ReferenceContext) -> Result<()> {
        context.push_reference_scope(reference.display_name());
        for symbol in &self.symbols {
            context.add_symbol(symbol.clone())?;
        }
        context.pop_scope()
    }
}

/// A resolver that loads reference symbols from a JSON manifest.
///
/// The manifest maps reference keys (description, guid, path, or path file stem)
/// to lists of symbol descriptions, which makes it possible to ship reference
/// data for Linux builds without writing any Rust code:
///
/// ```json
/// {
///   "references": {
///     "OLE Automation": [
///       { "name": "Now", "kind": "function", "type": "Date" },
///       { "name": "vbCrLf", "kind": "constant", "type": "String" }
///     ]
///   }
/// }
/// ```
///
/// `kind` defaults to `variable` and `type` defaults to `Variant`. Valid kinds
/// are `constant`, `variable`, `sub_procedure`, `function`, `property_get`,
/// `property_let`, `property_set`, `class`, `module`, `form`, `control`,
/// `enum`, `enum_member`, `user_type`, `type_member`, `parameter`, and `label`.
pub struct ManifestReferenceResolver {
    entries: BTreeMap<String, Vec<ManifestSymbol>>,
}

impl ManifestReferenceResolver {
    /// Create a resolver from the contents of a JSON manifest
    pub fn from_json(json: &str) -> Result<Self> {
        let manifest: Manifest = serde_json::from_str(json).map_err(|error| {
            SemanticError::AnalysisError(format!("Failed to parse reference manifest: {error}"))
        })?;
        Ok(Self {
            entries: manifest.references,
        })
    }

    /// Create a resolver from a JSON manifest file on disk
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let contents = std::fs::read_to_string(path.as_ref()).map_err(|error| {
            SemanticError::FileReadError {
                file: path.as_ref().display().to_string(),
                message: error.to_string(),
            }
        })?;
        Self::from_json(&contents)
    }
}

impl ReferenceResolver for ManifestReferenceResolver {
    fn name(&self) -> &str {
        "manifest"
    }

    fn can_handle(&self, reference: &ReferenceInfo) -> bool {
        self.entries
            .keys()
            .any(|key| ReferenceInfo::matches_key(key, reference))
    }

    fn resolve(&mut self, reference: &ReferenceInfo, context: &mut ReferenceContext) -> Result<()> {
        let Some((_key, specs)) = self
            .entries
            .iter()
            .find(|(key, _)| ReferenceInfo::matches_key(key, reference))
        else {
            return Err(SemanticError::AnalysisError(format!(
                "Reference resolver 'manifest' has no symbols for reference '{}'",
                reference.display_name()
            )));
        };

        context.push_reference_scope(reference.display_name());
        for spec in specs {
            context.add_symbol(spec.to_symbol(&SourceLocation {
                file: context.file().to_string(),
                line: 1,
                column: 1,
            }))?;
        }
        context.pop_scope()
    }
}

/// Deserialized JSON reference manifest
#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    /// Map of reference key to symbol descriptions
    pub references: BTreeMap<String, Vec<ManifestSymbol>>,
}

/// A symbol description inside a reference manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSymbol {
    /// The symbol name
    pub name: String,
    /// The kind of symbol, defaults to `variable`
    #[serde(default)]
    pub kind: ManifestSymbolKind,
    /// The type name (e.g. `Long`, `String`, `Variant`), defaults to `Variant`
    #[serde(default = "ManifestSymbol::default_type_name")]
    pub r#type: String,
    /// Whether the symbol is an array, defaults to `false`
    #[serde(default)]
    pub is_array: bool,
    /// The symbol visibility, defaults to `public`
    #[serde(default)]
    pub visibility: ManifestVisibility,
}

impl ManifestSymbol {
    fn default_type_name() -> String {
        "Variant".to_string()
    }

    fn to_symbol(&self, location: &SourceLocation) -> Symbol {
        let mut type_info = TypeInfo::new(type_kind_from_name(&self.r#type));
        type_info.is_array = self.is_array;
        Symbol {
            name: self.name.clone(),
            kind: self.kind.into(),
            type_info,
            visibility: self.visibility.into(),
            location: location.clone(),
            scope_id: 0,
            attributes: HashMap::new(),
        }
    }
}

/// Manifest symbol kind (serde string form)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestSymbolKind {
    /// Constant declaration
    Constant,
    /// Variable declaration
    #[default]
    Variable,
    /// Sub procedure
    SubProcedure,
    /// Function
    Function,
    /// Property Get
    PropertyGet,
    /// Property Let
    PropertyLet,
    /// Property Set
    PropertySet,
    /// Class
    Class,
    /// Module
    Module,
    /// Form
    Form,
    /// Control on a form
    Control,
    /// Enum
    Enum,
    /// Enum member
    EnumMember,
    /// User-defined type
    UserType,
    /// Type member
    TypeMember,
    /// Parameter
    Parameter,
    /// Label
    Label,
}

impl From<ManifestSymbolKind> for SymbolKind {
    fn from(kind: ManifestSymbolKind) -> Self {
        match kind {
            ManifestSymbolKind::Constant => SymbolKind::Constant,
            ManifestSymbolKind::Variable => SymbolKind::Variable,
            ManifestSymbolKind::SubProcedure => SymbolKind::SubProcedure,
            ManifestSymbolKind::Function => SymbolKind::Function,
            ManifestSymbolKind::PropertyGet => SymbolKind::PropertyGet,
            ManifestSymbolKind::PropertyLet => SymbolKind::PropertyLet,
            ManifestSymbolKind::PropertySet => SymbolKind::PropertySet,
            ManifestSymbolKind::Class => SymbolKind::Class,
            ManifestSymbolKind::Module => SymbolKind::Module,
            ManifestSymbolKind::Form => SymbolKind::Form,
            ManifestSymbolKind::Control => SymbolKind::Control,
            ManifestSymbolKind::Enum => SymbolKind::Enum,
            ManifestSymbolKind::EnumMember => SymbolKind::EnumMember,
            ManifestSymbolKind::UserType => SymbolKind::UserType,
            ManifestSymbolKind::TypeMember => SymbolKind::TypeMember,
            ManifestSymbolKind::Parameter => SymbolKind::Parameter,
            ManifestSymbolKind::Label => SymbolKind::Label,
        }
    }
}

/// Manifest visibility (serde string form)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestVisibility {
    /// Public symbol
    #[default]
    Public,
    /// Private symbol
    Private,
    /// Friend symbol
    Friend,
    /// Global symbol
    Global,
}

impl From<ManifestVisibility> for Visibility {
    fn from(visibility: ManifestVisibility) -> Self {
        match visibility {
            ManifestVisibility::Public => Visibility::Public,
            ManifestVisibility::Private => Visibility::Private,
            ManifestVisibility::Friend => Visibility::Friend,
            ManifestVisibility::Global => Visibility::Global,
        }
    }
}

/// Map a manifest type name to a [`VBType`]. Unknown names become class types.
fn type_kind_from_name(name: &str) -> VBType {
    match name.to_ascii_lowercase().as_str() {
        "integer" => VBType::Integer,
        "long" => VBType::Long,
        "single" => VBType::Single,
        "double" => VBType::Double,
        "currency" => VBType::Currency,
        "string" => VBType::String,
        "boolean" => VBType::Boolean,
        "byte" => VBType::Byte,
        "date" => VBType::Date,
        "variant" => VBType::Variant,
        "object" => VBType::Object,
        "nothing" => VBType::Nothing,
        _ => VBType::Class(name.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SourceLocation;

    fn location() -> SourceLocation {
        SourceLocation {
            file: "<test>".to_string(),
            line: 1,
            column: 1,
        }
    }

    fn symbol(name: &str) -> Symbol {
        Symbol {
            name: name.to_string(),
            kind: SymbolKind::Constant,
            type_info: TypeInfo::new(VBType::String),
            visibility: Visibility::Public,
            location: location(),
            scope_id: 0,
            attributes: HashMap::new(),
        }
    }

    fn reference_info() -> ReferenceInfo {
        ReferenceInfo {
            guid: Some("00020430-0000-0000-C000-000000000046".to_string()),
            path: "C:\\Windows\\System32\\stdole2.tlb".to_string(),
            description: "OLE Automation".to_string(),
            is_subproject: false,
        }
    }

    #[test]
    fn reference_info_matches_key() {
        let info = reference_info();
        assert!(ReferenceInfo::matches_key("OLE Automation", &info));
        assert!(ReferenceInfo::matches_key("ole automation", &info));
        assert!(ReferenceInfo::matches_key(
            "{00020430-0000-0000-C000-000000000046}",
            &info
        ));
        assert!(ReferenceInfo::matches_key(
            "00020430-0000-0000-c000-000000000046",
            &info
        ));
        assert!(ReferenceInfo::matches_key(
            "C:\\Windows\\System32\\stdole2.tlb",
            &info
        ));
        assert!(ReferenceInfo::matches_key("stdole2.tlb", &info));
        assert!(ReferenceInfo::matches_key("stdole2", &info));
        assert!(!ReferenceInfo::matches_key("Nope", &info));
    }

    #[test]
    fn unhandled_reference_returns_false() {
        let mut registry = ReferenceRegistry::new();
        let mut scopes = ScopeManager::new();
        assert!(
            !registry
                .resolve(&reference_info(), &mut scopes, "test.vbp")
                .unwrap()
        );
    }

    #[test]
    fn static_resolver_populates_reference_scope() {
        let mut registry = ReferenceRegistry::new();
        registry.register(Box::new(StaticReferenceResolver::new(
            "test",
            vec!["OLE Automation".to_string()],
            vec![symbol("Now")],
        )));

        let mut scopes = ScopeManager::new();
        let handled = registry
            .resolve(&reference_info(), &mut scopes, "test.vbp")
            .unwrap();
        assert!(handled);

        let reference_scopes = scopes.get_scopes_by_kind(ScopeKind::Reference);
        assert_eq!(reference_scopes.len(), 1);
        assert!(reference_scopes[0].symbols.contains_key("Now"));
    }

    #[test]
    fn manifest_resolver_loads_symbols_from_json() {
        let json = r#"{
            "references": {
                "OLE Automation": [
                    { "name": "Now", "kind": "function", "type": "Date" },
                    { "name": "vbCrLf", "kind": "constant", "type": "String" },
                    { "name": "items", "kind": "variable", "type": "Long", "is_array": true }
                ]
            }
        }"#;
        let mut registry = ReferenceRegistry::new();
        registry.register(Box::new(
            ManifestReferenceResolver::from_json(json).unwrap(),
        ));

        let mut scopes = ScopeManager::new();
        let handled = registry
            .resolve(&reference_info(), &mut scopes, "test.vbp")
            .unwrap();
        assert!(handled);

        let scope = &scopes.get_scopes_by_kind(ScopeKind::Reference)[0];
        let now = &scope.symbols["Now"];
        assert_eq!(now.kind, SymbolKind::Function);
        assert_eq!(now.type_info.kind, VBType::Date);
        let crlf = &scope.symbols["vbCrLf"];
        assert_eq!(crlf.kind, SymbolKind::Constant);
        let items = &scope.symbols["items"];
        assert!(items.type_info.is_array);
        assert_eq!(items.type_info.kind, VBType::Long);
    }
}
