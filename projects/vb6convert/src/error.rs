use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Failed to parse VB6 source: {0}")]
    ParseError(String),

    #[error("Unsupported VB6 feature: {0}")]
    UnsupportedFeature(String),

    #[error("Code generation error: {0}")]
    CodeGenError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Conversion not implemented for target: {0}")]
    NotImplemented(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, ConversionError>;
