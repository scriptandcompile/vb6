//! vb6compile: VB6 compiler pipeline and CLI
//!
//! This library provides the compiler infrastructure for compiling VB6 to native code
//! or other target languages.
//!
//! # Status
//!
//! This crate is in the planning phase. The source code is currently a stub.

#![warn(missing_docs)]

// TODO: Implement these modules (currently in design phase)
// pub mod pipeline;
// pub mod backend;
// pub mod optimizer;
// pub mod linker;

// pub use pipeline::CompilationPipeline;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
