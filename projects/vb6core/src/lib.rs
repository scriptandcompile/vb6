//! vb6core: Core runtime and shared functionality for VB6 compiler and interpreter
//!
//! This library provides the foundational infrastructure shared between `vb6interpret`
//! and `vb6compile`. It includes:
//!
//! - Value system and type definitions
//! - Type conversion rules
//! - Intermediate representation (IR)
//! - Runtime execution context
//! - Standard library implementations
//! - Array and object support
//!
//! # Example
//!
//! ```rust,ignore
//! // Note: This example is for future reference when modules are implemented
//! use vb6core::value::Value;
//! use vb6core::stdlib;
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
// pub mod ir;
// pub mod object;
// pub mod runtime;
// pub mod stdlib;
// pub mod types;
// pub mod value;
// pub mod variant;

// Re-export commonly used types (commented out until modules are implemented)
// pub use error::{CoreError, Result};
// pub use runtime::RuntimeContext;
// pub use types::VBType;
// pub use value::Value;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
