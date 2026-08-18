//! Trait abstracting over different environment-snapshot backends.
//!
//! The snapshot itself (ordering, case-insensitive lookup, `set`/`remove`)
//! lives in [`super`]; a backend's only job is providing the entries the
//! snapshot is seeded with:
//!
//! - **Native**: [`NativeBackend`](super::native::NativeBackend) seeds from the real process environment
//! - **WASM/tests**: [`MemoryBackend`](super::memory::MemoryBackend) seeds empty

/// Abstraction over where the environment snapshot's initial entries come from.
///
/// Implementations must be `Send` so the backend can be shared across
/// threads via a `Mutex<Box<dyn EnvironmentBackend>>`.
pub trait EnvironmentBackend: Send {
    /// The `NAME`/value pairs to seed the snapshot with, in environment-table
    /// order. Called once when the snapshot is first accessed, and again
    /// whenever the backend is switched or [`reset`](super::reset) is called.
    fn load(&self) -> Vec<(String, String)>;
}
