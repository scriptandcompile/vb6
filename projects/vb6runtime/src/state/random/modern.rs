//! A random backend using the `rand` crate's generator.
//!
//! Draws from `rand`'s `StdRng` rather than reproducing VB6's own LCG. Use
//! this when a caller wants better statistical quality than VB6's classic
//! generator and doesn't need bit-for-bit VB6 compatibility.

use std::any::Any;

use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

use crate::value::VBVariant;

use super::backend::RandomBackend;

/// A random backend backed by `rand`'s `StdRng`.
pub struct ModernBackend {
    rng: StdRng,
    last: f32,
}

impl ModernBackend {
    /// Create a backend seeded from OS entropy.
    pub fn new() -> Self {
        let mut rng = StdRng::from_rng(&mut rand::rng());
        let last = rng.random_range(0.0f32..1.0f32);
        Self { rng, last }
    }

    /// Create a backend seeded deterministically from a fixed value.
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let last = rng.random_range(0.0f32..1.0f32);
        Self { rng, last }
    }
}

impl Default for ModernBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomBackend for ModernBackend {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn next(&mut self) -> VBVariant {
        self.last = self.rng.random_range(0.0f32..1.0f32);
        VBVariant::from_single(self.last)
    }

    fn current(&self) -> VBVariant {
        VBVariant::from_single(self.last)
    }

    fn seed_from_rnd_argument(&mut self, value: f32) -> VBVariant {
        self.rng = StdRng::seed_from_u64(value.to_bits() as u64);
        self.next()
    }

    fn randomize(&mut self, bits: u32) {
        self.rng = StdRng::seed_from_u64(bits as u64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_produces_the_same_sequence() {
        let mut a = ModernBackend::with_seed(42);
        let mut b = ModernBackend::with_seed(42);
        for _ in 0..100 {
            assert_eq!(a.next(), b.next());
        }
    }

    #[test]
    fn values_stay_within_range() {
        let mut backend = ModernBackend::with_seed(1);
        for _ in 0..10_000 {
            match backend.next() {
                VBVariant::Single(v) => assert!((0.0..1.0).contains(&v)),
                other => panic!("expected Single, got {other:?}"),
            }
        }
    }

    #[test]
    fn current_does_not_advance() {
        let mut backend = ModernBackend::with_seed(7);
        let value = backend.next();
        assert_eq!(backend.current(), value);
        assert_eq!(backend.current(), value);
    }

    #[test]
    fn seed_from_rnd_argument_is_deterministic() {
        let mut a = ModernBackend::with_seed(7);
        let mut b = ModernBackend::with_seed(9);
        let x = a.seed_from_rnd_argument(-1.0);
        let y = b.seed_from_rnd_argument(-1.0);
        assert_eq!(x, y);
    }
}
