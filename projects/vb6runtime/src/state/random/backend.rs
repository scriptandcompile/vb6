//! Trait abstracting over different random-number generator backends.
//!
//! `Rnd` and `Randomize` are implemented against this trait rather than a
//! single hardcoded generator, so a host can swap in a different backend:
//!
//! - **Classic** ([`ClassicBackend`](super::classic::ClassicBackend), the
//!   default): reproduces VB6's own 24-bit linear congruential generator bit
//!   for bit.
//! - **Playback** ([`PlaybackBackend`](super::playback::PlaybackBackend)):
//!   replays a fixed, caller-supplied list of values, looping back to the
//!   start once exhausted. Useful for deterministic tests and demos.
//! - **Modern** ([`ModernBackend`](super::modern::ModernBackend)): draws from
//!   the `rand` crate's generator instead of VB6's LCG, for callers that want
//!   better statistical quality and don't need bit-for-bit VB6 compatibility.

use crate::value::VBVariant;

/// Abstraction over a random-number generator backing `Rnd` and `Randomize`.
pub trait RandomBackend: Send {
    /// Get a reference to self as `Any` for downcasting.
    fn as_any(&self) -> &dyn std::any::Any;

    /// The next value in the sequence, in `[0, 1)`. Backs `Rnd` with an
    /// omitted argument or a positive argument.
    fn next(&mut self) -> VBVariant;

    /// The most recently generated value, without advancing the sequence.
    /// Backs `Rnd(0)`.
    fn current(&self) -> VBVariant;

    /// Reseed from a negative `Rnd` argument and return the resulting value.
    /// Backs `Rnd(negative)`; the same argument must always produce the same
    /// result.
    fn seed_from_rnd_argument(&mut self, value: f32) -> VBVariant;

    /// Reseed the generator for the `Randomize` statement.
    ///
    /// `bits` is the VB6-style seed material: the IEEE-754 bits of the
    /// system timer if the argument was omitted, or the high 32 bits of the
    /// argument's `Double` representation otherwise. A backend that doesn't
    /// need VB6-specific seeding may treat this as opaque entropy.
    fn randomize(&mut self, bits: u32);
}
