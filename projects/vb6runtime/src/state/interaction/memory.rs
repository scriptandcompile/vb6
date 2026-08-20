//! In-memory interaction backend for WASM and tests.
//!
//! Stores injectable command-line arguments and provides no-op
//! implementations for OS-dependent operations.

use super::backend::InteractionBackend;

/// In-memory interaction backend.
///
/// Command-line arguments are stored in a `Vec` and can be set via
/// [`set_command_args`]. `Beep` and `DoEvents` are silent no-ops.
pub struct MemoryBackend {
    /// The injected command-line arguments.
    command_args: Vec<String>,
}

impl MemoryBackend {
    /// Create a new empty in-memory backend.
    pub fn new() -> Self {
        Self {
            command_args: Vec::new(),
        }
    }

    /// Create a backend with pre-set command-line arguments.
    pub fn with_args(args: Vec<String>) -> Self {
        Self { command_args: args }
    }

    /// Replace the command-line arguments.
    pub fn set_command_args(&mut self, args: Vec<String>) {
        self.command_args = args;
    }
}

impl Default for MemoryBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractionBackend for MemoryBackend {
    fn command_args(&self) -> Vec<String> {
        self.command_args.clone()
    }

    fn do_events(&self) -> i16 {
        0
    }

    fn beep(&self) {
        // No-op in the memory backend.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_by_default() {
        let backend = MemoryBackend::new();
        assert!(backend.command_args().is_empty());
    }

    #[test]
    fn with_args_returns_injected() {
        let backend = MemoryBackend::with_args(vec!["--debug".into(), "file.txt".into()]);
        assert_eq!(backend.command_args(), vec!["--debug", "file.txt"]);
    }

    #[test]
    fn set_command_args_replaces() {
        let mut backend = MemoryBackend::new();
        backend.set_command_args(vec!["/server:localhost".into()]);
        assert_eq!(backend.command_args(), vec!["/server:localhost"]);
    }

    #[test]
    fn do_events_returns_zero() {
        let backend = MemoryBackend::new();
        assert_eq!(backend.do_events(), 0i16);
    }
}
