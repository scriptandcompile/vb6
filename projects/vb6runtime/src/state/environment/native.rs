//! Native environment backend: seeds from the real process environment.

use super::backend::EnvironmentBackend;

/// Environment backend that seeds the snapshot from the host process's real
/// environment variables.
#[derive(Debug, Default)]
pub struct NativeBackend;

impl NativeBackend {
    /// Create a new native backend.
    pub fn new() -> Self {
        Self
    }
}

impl EnvironmentBackend for NativeBackend {
    fn load(&self) -> Vec<(String, String)> {
        // `std::env::vars` panics on targets that carry no environment
        // (notably the browser's wasm32-unknown-unknown), so we seed empty
        // there and hosts install variables with `set_env` instead.
        //
        // TODO(wasm): once this backend is compiled for wasm32, read the
        // persisted variables directly from browser `localStorage` here
        // (behind a `web-sys` dependency) instead of returning an empty
        // snapshot. That would let this backend persist across page
        // reloads on its own and let the playground drop its JS-side
        // env-sync bridge (see `vb6interpret`'s `wasm.rs`).
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        {
            std::env::vars().collect()
        }
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        {
            Vec::new()
        }
    }
}
