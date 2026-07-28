pub mod analysis;
pub mod converters;
pub mod error;
pub mod traits;
pub mod types;
pub mod validation;

// Re-export core types and traits
pub use error::{ConversionError, Result};
pub use traits::*;
pub use types::*;

// Language-specific conversion modules (feature-gated stubs)
#[cfg(feature = "rust-code")]
pub mod rust {
    //! Rust code generation
}

#[cfg(feature = "js-code")]
pub mod javascript {
    //! JavaScript / TypeScript code generation
}

#[cfg(feature = "dart")]
pub mod dart {
    //! Dart / Flutter code generation
}

#[cfg(feature = "html")]
pub mod html {
    //! HTML output generation
}

#[cfg(feature = "css")]
pub mod css {
    //! CSS output generation
}

#[cfg(feature = "tauri")]
pub mod tauri {
    //! Tauri framework integration
}

#[cfg(feature = "svelte")]
pub mod svelte {
    //! Svelte framework code generation
}

#[cfg(feature = "react")]
pub mod react {
    //! React framework code generation
}

#[cfg(feature = "vue")]
pub mod vue {
    //! Vue framework code generation
}

#[cfg(feature = "flutter")]
pub mod flutter {
    //! Flutter framework code generation
}

#[cfg(feature = "test-harness")]
pub mod testing {
    //! Test harness generation
}
