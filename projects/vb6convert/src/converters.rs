/// Converter registry and factory
///
/// This module provides a central registry for all available converters
/// and a factory for creating converter instances.
use crate::error::{ConversionError, Result};
use crate::traits::ProjectConverter;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of available converters
pub struct ConverterRegistry {
    converters: HashMap<String, Arc<dyn ProjectConverter>>,
}

impl ConverterRegistry {
    pub fn new() -> Self {
        Self {
            converters: HashMap::new(),
        }
    }

    /// Register a converter
    pub fn register(&mut self, converter: Arc<dyn ProjectConverter>) {
        self.converters
            .insert(converter.name().to_string(), converter);
    }

    /// Get a converter by name
    pub fn get(&self, name: &str) -> Result<Arc<dyn ProjectConverter>> {
        self.converters
            .get(name)
            .cloned()
            .ok_or_else(|| ConversionError::NotImplemented(name.to_string()))
    }

    /// List all available converters
    pub fn list(&self) -> Vec<&str> {
        self.converters.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ConverterRegistry {
    fn default() -> Self {
        let registry = Self::new();

        // Register converters based on available features
        #[cfg(feature = "rust-code")]
        {
            // registry.register(Arc::new(crate::rust::RustConverter::new()));
        }

        #[cfg(feature = "js-code")]
        {
            // registry.register(Arc::new(crate::javascript::JavaScriptConverter::new()));
        }

        registry
    }
}
