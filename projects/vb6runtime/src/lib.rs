//! vb6runtime: VB6 runtime library for value system, type conversions, and standard library
//!
//! This library provides the runtime execution infrastructure for VB6 programs, including:
//!
//! - Value system (all VB6 types)
//! - Type definitions and conversions
//! - Runtime execution context
//! - Standard library implementations (string, math, date, file I/O, etc.)
//! - Array and Variant support
//! - Error handling (On Error Resume/GoTo)
//!
//! # Example
//!
//! ```rust,ignore
//! // Note: This example is for future reference when modules are implemented
//! use vb6runtime::value::Value;
//! use vb6runtime::stdlib;
//!
//! // Create values
//! let text = Value::String("Hello World".to_string());
//! let num = Value::Integer(42);
//!
//! // Use standard library functions
//! let left_part = stdlib::string::left(&text.to_string().unwrap(), 5);
//! assert_eq!(left_part.unwrap(), "Hello");
//! ```

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

// TODO: Implement these modules (currently in design phase)
// pub mod array;
// pub mod conversion;
// pub mod error;
// pub mod object;
// pub mod runtime;
// pub mod stdlib;
// pub mod types;
// pub mod value;
// pub mod variant;

// Re-export commonly used types (commented out until modules are implemented)
// pub use error::{RuntimeError, Result};
// pub use runtime::RuntimeContext;
// pub use types::VBType;
// pub use value::Value;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
