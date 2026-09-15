//! vb6compile: VB6 transpiler and build orchestrator
//!
//! This library transforms VB6 source code into Rust and delegates compilation to
//! `rustc` via `cargo`. It uses `vb6convert` as the transpiler library and links
//! generated binaries against `vb6runtime`.
//!
//! ## Pipeline
//!
//! 1. Parse CLI arguments and resolve the VB6 project (`.vbp`)
//! 2. Delegate transpilation to `vb6convert::convert_project()` — produces Rust source
//!    files for modules (`.bas` → `pub fn`), classes (`.cls` → `struct` + `impl`),
//!    and forms (`.frm` → code-behind + layout integration)
//! 3. Write a `Cargo.toml` that depends on `vb6runtime`
//! 4. Invoke `cargo build` to produce a native executable
//!
//! ## Forms
//!
//! VB6 forms are not compiled into HTML/CSS. They are converted to Rust code that
//! calls `vb6runtime::layout::load_form()` at runtime. The layout engine renders
//! forms through a platform-abstract `Renderer` trait — WASM (`WebSysRenderer`) or
//! Tauri (`TauriRenderer`). The generated code delegates rendering to the host.
//!
//! # Status
//!
//! This crate is in the design phase. The CLI (`main.rs`) is implemented but the
//! transpilation and build orchestration modules are stubs.
//!
//! See [docs/DESIGN.md](../docs/DESIGN.md) for the full design.

#![warn(missing_docs)]

// TODO: Implement these modules (currently in design phase)
// pub mod pipeline;
// pub mod config;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
