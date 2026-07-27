/// Core traits for VB6 to target language conversion
///
/// This module defines the trait hierarchy that all conversion backends must implement.
/// Different targets (Rust, JavaScript, etc.) should implement these traits to provide
/// conversion functionality.
use crate::error::Result;
use crate::types::*;

// Type aliases for vb6parse types - these represent the parsed VB6 structures
// Using vb6parse::files types as these are the actual parsed representations
pub type Project<'a> = vb6parse::files::ProjectFile<'a>;
pub type Module = vb6parse::files::ModuleFile;
pub type Class = vb6parse::files::ClassFile;
pub type Form = vb6parse::language::Form;

/// Main conversion trait that all target converters must implement
///
/// This trait defines the high-level interface for converting an entire VB6 project
/// to a target language or framework.
pub trait ProjectConverter: Send + Sync {
    /// Get the name of this converter (e.g., "rust", "javascript", "tauri")
    fn name(&self) -> &str;

    /// Get a description of this converter
    fn description(&self) -> &str;

    /// Convert an entire VB6 project
    fn convert_project(
        &self,
        project: &Project<'_>,
        config: &ConversionConfig,
    ) -> Result<ConversionResult>;

    /// Check if this converter supports a specific VB6 feature
    fn supports_feature(&self, feature: VB6Feature) -> bool;

    /// Get list of required dependencies for the target
    fn required_dependencies(&self) -> Vec<Dependency>;
}

/// Trait for converting individual VB6 modules
pub trait ModuleConverter: Send + Sync {
    /// Convert a VB6 module (.bas file) to target language
    fn convert_module(&self, module: &Module, config: &ConversionConfig) -> Result<String>;

    /// Get the file extension for the converted module
    fn file_extension(&self) -> &str;
}

/// Trait for converting VB6 classes
pub trait ClassConverter: Send + Sync {
    /// Convert a VB6 class (.cls file) to target language
    fn convert_class(&self, class: &Class, config: &ConversionConfig) -> Result<String>;

    /// Get the file extension for the converted class
    fn file_extension(&self) -> &str;
}

/// Trait for converting VB6 forms
pub trait FormConverter: Send + Sync {
    /// Convert a VB6 form (.frm file) to target UI representation
    fn convert_form(&self, form: &Form, config: &ConversionConfig) -> Result<FormOutput>;

    /// Convert form layout to target format
    fn convert_layout(&self, form: &Form) -> Result<String>;

    /// Convert form code-behind
    fn convert_code_behind(&self, form: &Form, config: &ConversionConfig) -> Result<String>;
}

/// Output from form conversion (may include multiple files)
#[derive(Debug, Clone)]
pub struct FormOutput {
    /// Layout file (HTML, XAML, etc.)
    pub layout: Option<ConvertedFile>,

    /// Code-behind file
    pub code_behind: ConvertedFile,

    /// Styling file (CSS, etc.)
    pub styling: Option<ConvertedFile>,

    /// Additional assets (images, icons, etc.)
    pub assets: Vec<ConvertedFile>,
}

/// Represents a converted file
#[derive(Debug, Clone)]
pub struct ConvertedFile {
    /// Suggested filename
    pub filename: String,

    /// File content
    pub content: String,

    /// File type
    pub file_type: FileType,
}

/// Trait for converting VB6 controls to target UI elements
pub trait ControlConverter: Send + Sync {
    /// Convert a VB6 control to target UI element
    fn convert_control(&self, control: &VB6Control, config: &ConversionConfig) -> Result<String>;

    /// Map VB6 control properties to target properties
    fn map_properties(&self, control: &VB6Control) -> Result<ControlProperties>;

    /// Map VB6 control events to target events
    fn map_events(&self, control: &VB6Control) -> Result<Vec<EventMapping>>;
}

/// Trait for expression and statement conversion
pub trait ExpressionConverter: Send + Sync {
    /// Convert a VB6 expression to target language
    fn convert_expression(&self, expr: &str, context: &ConversionContext) -> Result<String>;

    /// Convert a VB6 statement to target language
    fn convert_statement(&self, stmt: &str, context: &ConversionContext) -> Result<String>;
}

/// Trait for type system conversion
pub trait TypeConverter: Send + Sync {
    /// Convert a VB6 type to target type
    fn convert_type(&self, vb6_type: &VB6Type) -> Result<String>;

    /// Check if type conversion is lossless
    fn is_lossless_conversion(&self, vb6_type: &VB6Type) -> bool;
}

/// Context information during conversion
#[derive(Debug, Clone)]
pub struct ConversionContext {
    /// Current file being converted
    pub current_file: String,

    /// Scope information
    pub scope: ScopeInfo,

    /// Available imports/dependencies
    pub imports: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ScopeInfo {
    /// Variables in current scope
    pub variables: Vec<String>,

    /// Functions/procedures in scope
    pub functions: Vec<String>,

    /// Parent scope (if any)
    pub parent: Option<Box<ScopeInfo>>,
}

/// Represents a VB6 control (simplified)
#[derive(Debug, Clone)]
pub struct VB6Control {
    pub control_type: String,
    pub name: String,
    pub properties: std::collections::HashMap<String, String>,
}

/// Properties of a converted control
#[derive(Debug, Clone)]
pub struct ControlProperties {
    pub properties: std::collections::HashMap<String, String>,
}

/// Mapping of a VB6 event to target event
#[derive(Debug, Clone)]
pub struct EventMapping {
    pub vb6_event: String,
    pub target_event: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
}

/// VB6 type information
#[derive(Debug, Clone, PartialEq)]
pub enum VB6Type {
    Integer,
    Long,
    Single,
    Double,
    String,
    Boolean,
    Variant,
    Object,
    Date,
    Currency,
    Byte,
    Custom(String),
}

/// VB6 features that may or may not be supported by a converter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VB6Feature {
    // Language features
    OptionExplicit,
    OptionBase,
    GoTo,
    OnError,
    WithBlock,
    SelectCase,

    // Forms and controls
    Forms,
    MdiForm,
    UserControls,
    PropertyPages,

    // Controls
    StandardControls,
    ActiveXControls,
    CustomControls,

    // Data access
    AdoDatabase,
    DaoDatabase,
    DataEnvironment,

    // Advanced features
    ApiCalls,
    DllImports,
    LateBinding,
    Arrays,
    Collections,
    Classes,
    Interfaces,

    // File operations
    FileSystemAccess,
    BinaryFiles,
    TextFiles,

    // Graphics
    Printing,
    Graphics,
}

/// Dependency information for the target platform
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub description: String,
}
