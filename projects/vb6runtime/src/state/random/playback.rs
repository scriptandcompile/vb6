//! A random backend that replays a fixed, caller-supplied list of values.
//!
//! Useful for deterministic tests and demos where the exact `Rnd` sequence
//! needs to be controlled rather than pseudo-random. Each call to `Rnd`
//! returns the next value in the list, looping back to the start once the
//! list is exhausted.

use std::any::Any;

use crate::value::VBVariant;

use super::backend::RandomBackend;

/// A random backend that cycles through a fixed list of values.
///
/// `Rnd(0)` returns the most recently produced value without advancing.
/// `Rnd(negative)` and `Randomize` both restart the cycle from the beginning,
/// mirroring "reset the sequence" rather than VB6's seed-splicing semantics,
/// since there is no seed to splice.
///
/// Values are returned as-is; the caller is responsible for supplying values
/// in `[0, 1)` if VB6-compatible `Rnd` output is required.
pub struct PlaybackBackend {
    values: Vec<f32>,
    index: usize,
}

impl PlaybackBackend {
    /// Create a backend that cycles through `values`.
    ///
    /// # Panics
    ///
    /// Panics if `values` is empty.
    pub fn new(values: Vec<f32>) -> Self {
        assert!(
            !values.is_empty(),
            "PlaybackBackend requires at least one value"
        );
        Self { values, index: 0 }
    }

    fn current_value(&self) -> f32 {
        self.values[self.index]
    }
}

impl RandomBackend for PlaybackBackend {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn next(&mut self) -> VBVariant {
        let value = self.current_value();
        self.index = (self.index + 1) % self.values.len();
        VBVariant::from_single(value)
    }

    fn current(&self) -> VBVariant {
        VBVariant::from_single(self.current_value())
    }

    fn seed_from_rnd_argument(&mut self, _value: f32) -> VBVariant {
        self.index = 0;
        self.current()
    }

    fn randomize(&mut self, _bits: u32) {
        self.index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_single(value: VBVariant) -> f32 {
        match value {
            VBVariant::Single(v) => v,
            other => panic!("expected Single, got {other:?}"),
        }
    }

    #[test]
    fn cycles_through_the_list() {
        let mut backend = PlaybackBackend::new(vec![0.1, 0.2, 0.3]);
        assert_eq!(as_single(backend.next()), 0.1);
        assert_eq!(as_single(backend.next()), 0.2);
        assert_eq!(as_single(backend.next()), 0.3);
        assert_eq!(as_single(backend.next()), 0.1);
    }

    #[test]
    fn current_does_not_advance() {
        let mut backend = PlaybackBackend::new(vec![0.1, 0.2]);
        assert_eq!(as_single(backend.next()), 0.1);
        assert_eq!(as_single(backend.current()), 0.2);
        assert_eq!(as_single(backend.current()), 0.2);
    }

    #[test]
    fn randomize_restarts_the_cycle() {
        let mut backend = PlaybackBackend::new(vec![0.1, 0.2, 0.3]);
        backend.next();
        backend.next();
        backend.randomize(0);
        assert_eq!(as_single(backend.next()), 0.1);
    }

    #[test]
    fn seed_from_rnd_argument_restarts_the_cycle() {
        let mut backend = PlaybackBackend::new(vec![0.1, 0.2, 0.3]);
        backend.next();
        let value = backend.seed_from_rnd_argument(-1.0);
        assert_eq!(as_single(value), 0.1);
    }

    #[test]
    #[should_panic(expected = "at least one value")]
    fn rejects_an_empty_list() {
        PlaybackBackend::new(vec![]);
    }
}
