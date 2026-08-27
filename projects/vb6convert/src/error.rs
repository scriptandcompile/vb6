use thiserror::Error;

#[derive(Error, Debug)]
/// Error types that can occur during VB6-to-target conversion.
pub enum ConversionError {
    /// Failed to parse VB6 source code.
    #[error("Failed to parse VB6 source: {0}")]
    ParseError(String),

    /// A VB6 feature used in the project is not supported by the target converter.
    #[error("Unsupported VB6 feature: {0}")]
    UnsupportedFeature(String),

    /// Error during code generation for the target language.
    #[error("Code generation error: {0}")]
    CodeGenError(String),

    /// I/O error while reading/writing files.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid or missing conversion configuration.
    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    /// The conversion target is not yet implemented.
    #[error("Conversion not implemented for target: {0}")]
    NotImplemented(String),

    /// Validation of converted code or input failed.
    #[error("Validation failed: {0}")]
    ValidationError(String),

    /// Catch-all error wrapping any [`anyhow::Error`].
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Shorthand for a conversion result that carries [`ConversionError`].
pub type Result<T> = std::result::Result<T, ConversionError>;
