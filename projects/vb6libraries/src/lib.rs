//! vb6libraries: VB6 library detection and mapping (win32, office, third-party)
//!
//! This crate provides modular support for common VB6 third-party libraries,
//! Windows API calls, Office automation, and database integrations.
//!
//! # Features
//!
//! ## Windows APIs
//!
//! Feature flags: `win32-core`, `win32-gdi`, `win32-shell`, `win32-registry`, `win32-networking`
//!
//! ```toml
//! vb6libraries = { features = ["win32-core", "win32-gdi"] }
//! ```
//!
//! ## Office Automation
//!
//! Feature flags: `excel-automation`, `word-automation`, `outlook-automation`
//!
//! ```toml
//! vb6libraries = { features = ["excel-automation"] }
//! ```
//!
//! ## Database Libraries
//!
//! Feature flags: `dao`, `ado`, `sql-server`, `oracle-db`, `mysql-db`, `access-db`
//!
//! ```toml
//! vb6libraries = { features = ["ado", "sql-server"] }
//! ```
//!
//! ## UI Controls
//!
//! Feature flags: `mscomctl`, `datagrid`, `richtextbox`, `webview`
//!
//! ```toml
//! vb6libraries = { features = ["mscomctl", "datagrid"] }
//! ```
//!
//! # Usage
//!
//! This crate is used by `vb6codegen`, `vb6convert`, and `vb6compile` to:
//!
//! - Detect which libraries a VB6 project uses
//! - Map VB6 library calls to modern equivalents
//! - Generate appropriate code for each library
//! - Handle cross-cutting concerns (libraries affecting backend and frontend)

#![warn(missing_docs)]


/// Library detection and analysis
pub mod detection {
    use std::collections::HashSet;

    /// Represents the libraries used by a VB6 project
    #[derive(Debug, Clone, Default)]
    pub struct LibraryUsage {
        /// Win32 API calls detected
        pub win32_calls: HashSet<String>,
        /// Office automation usage
        pub office_usage: OfficeUsage,
        /// Database libraries used
        pub database_libs: DatabaseUsage,
        /// UI controls used
        pub ui_controls: HashSet<String>,
        /// Third-party controls used
        pub third_party: HashSet<String>,
    }

    /// Which Office applications are used by a VB6 project.
    #[derive(Debug, Clone, Default)]
    pub struct OfficeUsage {
        /// Microsoft Excel COM automation.
        pub excel: bool,
        /// Microsoft Word COM automation.
        pub word: bool,
        /// Microsoft Outlook COM automation.
        pub outlook: bool,
        /// Microsoft Access database automation.
        pub access: bool,
    }

    /// Which database technologies are used by a VB6 project.
    #[derive(Debug, Clone, Default)]
    pub struct DatabaseUsage {
        /// DAO (Data Access Objects) is used.
        pub dao: bool,
        /// ADO (ActiveX Data Objects) is used.
        pub ado: bool,
        /// RDO (Remote Data Objects) is used.
        pub rdo: bool,
        /// ODBC connections are used.
        pub odbc: bool,
    }

    impl LibraryUsage {
        /// Create a new empty [`LibraryUsage`].
        pub fn new() -> Self {
            Self::default()
        }

        /// Recommend Cargo features based on detected usage
        pub fn recommend_features(&self) -> Vec<String> {
            let mut features = Vec::new();

            if !self.win32_calls.is_empty() {
                features.push("win32-apis".to_string());
            }

            if self.office_usage.excel {
                features.push("excel-automation".to_string());
            }
            if self.office_usage.word {
                features.push("word-automation".to_string());
            }
            if self.office_usage.outlook {
                features.push("outlook-automation".to_string());
            }

            if self.database_libs.dao || self.database_libs.ado {
                features.push("database-support".to_string());
            }

            if !self.ui_controls.is_empty() {
                features.push("ui-controls".to_string());
            }

            features
        }
    }
}

/// Common traits for library integration
pub mod traits {
    /// Trait for mapping VB6 API calls to modern equivalents
    pub trait ApiMapper {
        /// Map a VB6 API call to target code
        fn map_call(&self, api_name: &str, args: &[String]) -> Option<String>;

        /// Get imports required for this API
        fn get_imports(&self) -> Vec<String>;
    }

    /// Trait for library-specific code generation hooks
    pub trait CodegenHook {
        /// Generate initialization code
        fn generate_init(&self) -> Option<String>;

        /// Generate cleanup/teardown code
        fn generate_cleanup(&self) -> Option<String>;

        /// Whether this hook affects backend code
        fn affects_backend(&self) -> bool;

        /// Whether this hook affects frontend code
        fn affects_frontend(&self) -> bool;
    }

    /// Trait for cross-cutting library concerns
    pub trait CrossCuttingLibrary: ApiMapper + CodegenHook {
        /// Generate backend-specific code
        fn generate_backend(&self, context: &BackendContext) -> String;

        /// Generate frontend-specific code
        fn generate_frontend(&self, context: &FrontendContext) -> String;
    }

    /// Context for backend code generation.
    pub struct BackendContext {
        /// The backend target to generate code for.
        pub target: BackendTarget,
    }

    /// Target backend language for code generation.
    pub enum BackendTarget {
        /// Rust backend (e.g., via `vb6codegen`).
        Rust,
        /// LLVM-based backend.
        Llvm,
    }

    /// Context for frontend code generation.
    pub struct FrontendContext {
        /// The frontend UI framework to target.
        pub framework: FrontendFramework,
    }

    /// Front-end UI frameworks for converted VB6 forms.
    pub enum FrontendFramework {
        /// React (JavaScript/TypeScript).
        React,
        /// Vue (JavaScript/TypeScript).
        Vue,
        /// Svelte (JavaScript).
        Svelte,
        /// Leptos (Rust/WASM).
        Leptos,
        /// Yew (Rust/WASM).
        Yew,
        /// Flutter (Dart).
        Flutter,
        /// egui (Rust/-native).
        Egui,
    }
}

// Placeholder modules (to be implemented)

#[cfg(feature = "win32-core")]
pub mod win32 {
    //! Windows API integration
    //! Maps VB6 Win32 API calls to Rust `windows` crate
}

#[cfg(any(feature = "mscomctl", feature = "datagrid"))]
pub mod ui {
    //! UI control mapping
    //! Maps VB6 UI controls to modern UI components
}

#[cfg(feature = "office-automation")]
pub mod office {
    //! Office automation
    //! Maps VB6 Office COM automation to Rust office libraries
}

#[cfg(feature = "database-common")]
pub mod database {
    //! Database library integration
    //! Maps DAO/ADO to modern Rust database drivers
}

#[cfg(any(feature = "crystalreports", feature = "devexpress"))]
pub mod thirdparty {
    //! Third-party control integration
    //! Maps commercial VB6 controls to modern equivalents
}
