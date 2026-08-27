/// Core traits for VB6 to target language conversion
///
/// This module defines the trait hierarchy that all conversion backends must implement.
/// Different targets (Rust, JavaScript, etc.) should implement these traits to provide
/// conversion functionality.
use crate::error::Result;
use crate::types::*;

// Type aliases for vb6parse types - these represent the parsed VB6 structures
// Using vb6parse::files types as these are the actual parsed representations
/// Alias for a borrowed VB6 project file from the parser.
pub type Project<'a> = vb6parse::files::ProjectFile<'a>;
/// Alias for a VB6 standard module (.bas) file.
pub type Module = vb6parse::files::ModuleFile;
/// Alias for a VB6 class module (.cls) file.
pub type Class = vb6parse::files::ClassFile;
/// Alias for a VB6 form (.frm) parsed structure.
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

/// Scope information gathered during conversion (variable/function context).
#[derive(Debug, Clone)]
pub struct ScopeInfo {
    /// Names of variables in the current scope.
    pub variables: Vec<String>,

    /// Names of functions/procedures in the current scope.
    pub functions: Vec<String>,

    /// Parent scope (if any), forming a chain up to module scope.
    pub parent: Option<Box<ScopeInfo>>,
}

/// Represents a VB6 control (simplified).
#[derive(Debug, Clone)]
pub struct VB6Control {
    /// The control type (e.g., "CommandButton", "TextBox").
    pub control_type: String,
    /// The control's name as declared in VB6.
    pub name: String,
    /// Key-value pairs of the control's properties.
    pub properties: std::collections::HashMap<String, String>,
}

/// Properties of a converted control (key-value pairs mapped to target).
#[derive(Debug, Clone)]
pub struct ControlProperties {
    /// Mapped property names and values for the target framework.
    pub properties: std::collections::HashMap<String, String>,
}

/// Mapping from a VB6 event to a target-language event handler.
#[derive(Debug, Clone)]
pub struct EventMapping {
    /// The original VB6 event name (e.g., "Click").
    pub vb6_event: String,
    /// The target event name (e.g., "onClick").
    pub target_event: String,
    /// Parameters that the target event handler will receive.
    pub parameters: Vec<Parameter>,
}

/// A single parameter of an event or function in the target language.
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Parameter name.
    pub name: String,
    /// Parameter type as a string in the target language.
    pub param_type: String,
}

/// VB6 type information (used for type mapping during conversion).
#[derive(Debug, Clone, PartialEq)]
pub enum VB6Type {
    /// VB6 Integer (16-bit signed).
    Integer,
    /// VB6 Long (32-bit signed).
    Long,
    /// VB6 Single (32-bit float).
    Single,
    /// VB6 Double (64-bit float).
    Double,
    /// VB6 String (BSTR-like).
    String,
    /// VB6 Boolean (16-bit).
    Boolean,
    /// VB6 Variant.
    Variant,
    /// VB6 Object reference.
    Object,
    /// VB6 Date.
    Date,
    /// VB6 Currency.
    Currency,
    /// VB6 Byte (8-bit unsigned).
    Byte,
    /// User-defined type with the given name.
    Custom(String),
}

/// VB6 features that may or may not be supported by a converter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VB6Feature {
    // Language features
    /// Option Explicit statement usage.
    OptionExplicit,
    /// Option Base statement usage.
    OptionBase,
    /// Goto statement usage.
    GoTo,
    /// On Error GoTo error handling.
    OnError,
    /// With...End With blocks.
    WithBlock,
    /// Select Case blocks.
    SelectCase,

    // Forms and controls
    /// Standard form modules (.frm).
    Forms,
    /// MDI child/parent forms.
    MdiForm,
    /// User control (.ctl) modules.
    UserControls,
    /// Property page modules.
    PropertyPages,

    // Controls
    /// Standard VB6 controls (CommandButton, TextBox, etc.).
    StandardControls,
    /// ActiveX/OCX controls.
    ActiveXControls,
    /// Custom (designed) controls.
    CustomControls,

    // Data access
    /// ADO (ActiveX Data Objects) usage.
    AdoDatabase,
    /// DAO (Data Access Objects) usage.
    DaoDatabase,
    /// VB6 Data Environment usage.
    DataEnvironment,

    // Advanced features
    /// Windows API (Declare Function) calls.
    ApiCalls,
    /// DLL import declarations.
    DllImports,
    /// Late-bound COM calls (CreateObject, GetObject).
    LateBinding,
    /// Array operations (dynamic/static arrays).
    Arrays,
    /// VB6 Collection objects.
    Collections,
    /// Class modules with members.
    Classes,
    /// Class interfaces (Implements).
    Interfaces,

    // File operations
    /// FileSystemObject / file I/O.
    FileSystemAccess,
    /// Binary file operations (Open ... For Binary).
    BinaryFiles,
    /// Text file operations (Open ... For Output).
    TextFiles,

    // Graphics
    /// Printer object / PrintForm.
    Printing,
    /// Paint / Draw methods / PictureBox.
    Graphics,
}

/// Dependency information for the target platform.
#[derive(Debug, Clone)]
pub struct Dependency {
    /// Package or library name.
    pub name: String,
    /// Required version constraint (e.g., "1.0.0").
    pub version: Option<String>,
    /// Human-readable description of the dependency.
    pub description: String,
}
