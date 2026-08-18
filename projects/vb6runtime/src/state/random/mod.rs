//! Random-number generator state shared by the `Rnd` function and the
//! `Randomize` statement.
//!
//! The active generator is a pluggable [`RandomBackend`], defaulting to
//! [`ClassicBackend`], VB6's own LCG. Install a different backend with
//! [`set_backend`] to swap in [`PlaybackBackend`] (a fixed, looping sequence
//! of values) or [`ModernBackend`] (the `rand` crate's generator).

pub mod backend;
pub mod classic;
pub mod modern;
pub mod playback;

use std::sync::{Mutex, OnceLock};

pub use backend::RandomBackend;
pub use classic::{ClassicBackend, DEFAULT_SEED, MODULUS};
pub use modern::ModernBackend;
pub use playback::PlaybackBackend;

use crate::value::VBVariant;

/// The active random backend.
static BACKEND: OnceLock<Mutex<Box<dyn RandomBackend>>> = OnceLock::new();

/// Get the active backend, initializing with the classic default if needed.
fn backend() -> &'static Mutex<Box<dyn RandomBackend>> {
    BACKEND.get_or_init(|| Mutex::new(Box::new(ClassicBackend::new())))
}

/// Set the active random backend.
///
/// This is the primary way to switch generators at runtime.
pub fn set_backend(new_backend: Box<dyn RandomBackend>) {
    let mut backend_guard = backend().lock().unwrap_or_else(|e| e.into_inner());
    *backend_guard = new_backend;
}

/// Reset to the default classic (VB6-compatible) backend.
pub fn reset_backend() {
    set_backend(Box::new(ClassicBackend::new()));
}

/// The next value in the sequence, in `[0, 1)`. Backs `Rnd` with an omitted
/// or positive argument.
pub fn next() -> VBVariant {
    backend().lock().unwrap_or_else(|e| e.into_inner()).next()
}

/// The most recently generated value, without advancing the sequence. Backs
/// `Rnd(0)`.
pub fn current() -> VBVariant {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .current()
}

/// Reseed from a negative `Rnd` argument and return the resulting value.
/// Backs `Rnd(negative)`.
pub fn seed_from_rnd_argument(value: f32) -> VBVariant {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .seed_from_rnd_argument(value)
}

/// Reseed the active backend. Backs the `Randomize` statement.
pub fn randomize(bits: u32) {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .randomize(bits);
}

#[cfg(test)]
/// Point the active backend at a classic backend with a specific raw seed.
///
/// Test-only convenience matching the pre-refactor `set_seed` API; tests
/// exercise the classic backend directly since it's the one with VB6-exact
/// semantics to verify.
pub(crate) fn set_seed(value: u32) {
    set_backend(Box::new(ClassicBackend::with_seed(value)));
}

#[cfg(test)]
/// The active backend's raw seed.
///
/// # Panics
///
/// Panics if the active backend isn't a [`ClassicBackend`].
pub(crate) fn seed() -> u32 {
    let guard = backend().lock().unwrap_or_else(|e| e.into_inner());
    guard
        .as_any()
        .downcast_ref::<ClassicBackend>()
        .expect("expected the classic backend to be active in tests")
        .seed()
}
