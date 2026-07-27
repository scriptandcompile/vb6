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

// Language-specific conversion modules (feature-gated)
#[cfg(feature = "rust-code")]
pub mod rust;

#[cfg(feature = "js-code")]
pub mod javascript;

#[cfg(feature = "dart")]
pub mod dart;

#[cfg(feature = "html")]
pub mod html;

#[cfg(feature = "css")]
pub mod css;

#[cfg(feature = "tauri")]
pub mod tauri;

#[cfg(feature = "svelte")]
pub mod svelte;

#[cfg(feature = "react")]
pub mod react;

#[cfg(feature = "vue")]
pub mod vue;

#[cfg(feature = "flutter")]
pub mod flutter;

#[cfg(feature = "test-harness")]
pub mod testing;
