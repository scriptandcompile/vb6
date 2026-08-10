//! Runtime state statements.
//!
//! This module contains parsers for VB6 statements that control runtime state:
//! - System time (Date, Time)
//! - Error handling (Error)
//! - Random number generation (Randomize)

pub mod date;
pub mod error;
pub mod randomize;
pub mod time;
