//! Trait abstracting over platform-specific user interaction operations.
//!
//! VB6 interaction functions (`Command$`, `DoEvents`, `Beep`) need a pluggable
//! backend to support both native platforms and WASM environments:
//!
//! - **Windows/Linux/macOS**: [`NativeBackend`](super::native::NativeBackend)
//!   uses the real process environment and OS primitives.
//! - **WASM/tests**: [`MemoryBackend`](super::memory::MemoryBackend)
//!   provides injectable, deterministic behavior.

/// Abstraction over user interaction operations.
///
/// Implementations must be `Send` so the backend can be shared across
/// threads via a `Mutex<Box<dyn InteractionBackend>>`.
pub trait InteractionBackend: Send {
    /// Get the command-line arguments passed to the program.
    ///
    /// Returns individual arguments (not including the program name).
    /// On native platforms this reads `std::env::args()`. On WASM it
    /// returns whatever the host injected.
    fn command_args(&self) -> Vec<String>;

    /// Yield execution to the operating system.
    ///
    /// Returns the number of open forms (VB6 stand-alone only; always 0
    /// for our interpreter). On native platforms this yields the thread.
    /// On WASM this is typically a no-op.
    fn do_events(&self) -> i16;

    /// Play the system beep sound.
    ///
    /// On native platforms this writes the terminal bell character (`\x07`)
    /// to stderr. On WASM this is a no-op.
    fn beep(&self);
}
