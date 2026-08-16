//! # Randomize Statement
//!
//! Re-initializes the random-number generator with a new seed value.
//!
//! ## Syntax
//!
//! ```vb
//! Randomize [number]
//! ```
//!
//! ## Arguments
//!
//! | Part | Optional / Required | Description |
//! |------|---------------------|-------------|
//! | number | Optional | A Variant or any valid numeric expression that is used as the new seed value to initialize the random number generator. |
//!
//! ## Remarks
//!
//! - The Randomize statement initializes the random-number generator, giving it a new seed value.
//! - If you omit number, the value returned by the system timer is used as the new seed value.
//! - If Randomize is not used, the Rnd function (with no arguments) uses the same number as a seed the first time it is called, and thereafter uses the last generated number as a seed value.
//! - To repeat sequences of random numbers, call Rnd with a negative argument immediately before using Randomize with a numeric argument.
//! - Using Randomize with the same value for number does not repeat the previous sequence.
//!
//! ## Implementation
//!
//! This follows the VB6 runtime internals (`rtRandomize` and `rtRandomizeValue`
//! in msvbvm60.dll), as preserved in .NET's `VBMath.Randomize` overloads, which
//! are explicitly documented as equivalent to those two functions.
//!
//! Both forms take a value, XOR its low and high 16-bit halves, shift the
//! result up 8 bits, and splice that into the middle 16 bits of the *current*
//! seed while keeping the seed's top and bottom bytes. Because the low byte of
//! the previous seed is preserved, re-randomizing with the same value does not
//! repeat a sequence.
//!
//! - **Omitted argument** — the value is `Single` seconds since midnight
//!   (`VB6 GetTimer`), matching `rtRandomize`.
//! - **Numeric argument** — the high 32 bits of the argument's `Double`
//!   representation are used, matching `rtRandomizeValue`.
//!
//! The spliced seed is stored as-is (it can exceed 24 bits); `Rnd` keeps the
//! sequence 24-bit by masking when it advances.
//!
//! ## Examples
//!
//! ```vb
//! ' Initialize random number generator
//! Randomize
//! x = Int((100 * Rnd) + 1)
//!
//! ' Initialize with specific seed
//! Randomize 42
//! x = Rnd
//!
//! ' Use timer as seed
//! Randomize Timer
//! ```
//!
//! # References
//!
//! [Microsoft VBA Language Reference - Randomize Statement](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/randomize-statement)

use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::VBResult;
use crate::state::random::{seed, set_seed};
use crate::value::VBVariant;

/// Splice a value into the middle 16 bits of the current seed.
///
/// Matches VB6's `rtRandomize`/`rtRandomizeValue`: the low and high 16-bit
/// words of `bits` are XORed together, shifted up 8 bits, and ORed into
/// `current` while preserving its top and bottom bytes. `bits >> 16` is the
/// C-style arithmetic shift, so a negative value's sign extension survives the
/// XOR and can set the seed's top byte.
fn splice(bits: u32, current: u32) -> u32 {
    let l = bits as i32;
    let mixed = ((l & 0xFFFF) ^ (l >> 16)) as u32;
    (current & 0xFF00_00FF) | (mixed << 8)
}

/// `Single` seconds since midnight, as VB6's `GetTimer` produces it.
///
/// The actual clock value only needs to vary with time; the exact instant is
/// what seeds the generator. The result is cast to `Single` so its IEEE-754
/// bit pattern matches what VB6 would have used for the same point in time.
fn timer_bits() -> u32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let seconds = (now.as_secs() % 86_400) as f32 + now.subsec_millis() as f32 / 1000.0;
    seconds.to_bits()
}

/// Implementation of the `Randomize` statement.
///
/// - an omitted argument (passed as `Empty`) reseeds from the system timer
/// - `Randomize(number)` reseeds from `number` (coerced to `Double`)
///
/// The generator's seed is replaced in place; the shared state is the same
/// `SEED` used by the `Rnd` function, so `Rnd(0)` immediately after a
/// `Randomize` reports the value VB6 would derive from the new seed.
pub fn randomize(value: Option<VBVariant>) -> VBResult<()> {
    let bits = match value {
        None | Some(VBVariant::Empty) => timer_bits(),
        Some(v) => {
            let number = v.as_f64()?;
            (number.to_bits() >> 32) as u32
        }
    };

    set_seed(splice(bits, seed()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::randomize;
    use crate::error::err_number;
    use crate::library::math::rnd::rnd;
    use crate::state::random::{seed, set_seed, MODULUS};
    use crate::state::test_support::TEST_LOCK;
    use crate::value::VBVariant;

    /// The exact `Single` value produced by a given seed, `seed / 2^24`.
    fn expected(seed: u32) -> f32 {
        seed as f32 / MODULUS as f32
    }

    /// Extract the `Single` payload of an `Rnd` result.
    fn as_single(value: VBVariant) -> f32 {
        match value {
            VBVariant::Single(v) => v,
            other => panic!("expected Single, got {other:?}"),
        }
    }

    #[test]
    fn randomize_with_value_is_deterministic() {
        let _guard = TEST_LOCK.lock().unwrap();
        set_seed(0);
        randomize(Some(VBVariant::from_double(42.0))).unwrap();
        assert_eq!(seed(), 0x0040_4500);

        let next = as_single(rnd(&VBVariant::Empty).unwrap());
        assert_eq!(next, f32::from_bits(0x3EAD_9F86));
    }

    #[test]
    fn randomize_preserves_the_low_byte_of_the_seed() {
        let _guard = TEST_LOCK.lock().unwrap();
        // Randomize only replaces the middle two bytes; the low byte (0x23)
        // survives, which is why the same number never repeats a sequence.
        set_seed(0x0000_0123);
        randomize(Some(VBVariant::from_double(1.0))).unwrap();
        assert_eq!(seed(), 0x003F_F023);
    }

    #[test]
    fn randomize_with_negative_value_can_set_a_full_32_bit_seed() {
        let _guard = TEST_LOCK.lock().unwrap();
        set_seed(0);
        randomize(Some(VBVariant::from_double(-1.0))).unwrap();
        assert_eq!(seed(), 0xFFBF_F000);

        // Rnd(0) normalizes the raw stored seed (VB6-faithful; it can be >= 1).
        assert_eq!(
            as_single(rnd(&VBVariant::from_single(0.0)).unwrap()),
            expected(0xFFBF_F000)
        );

        // Advancing still keeps the sequence 24-bit and in [0, 1).
        let next = as_single(rnd(&VBVariant::Empty).unwrap());
        assert_eq!(next, f32::from_bits(0x3E87_9D86));
        assert!((0.0..1.0).contains(&next));
    }

    #[test]
    fn omitted_argument_reseeds_from_the_timer() {
        let _guard = TEST_LOCK.lock().unwrap();
        set_seed(0x1234_5678);
        randomize(None).unwrap();

        let reseeded = seed();
        assert_ne!(reseeded, 0x1234_5678);
        // The low byte of the previous seed survives the splice.
        assert_eq!(reseeded & 0xFF, 0x78);

        let value = as_single(rnd(&VBVariant::Empty).unwrap());
        assert!((0.0..1.0).contains(&value));
    }

    #[test]
    fn accepts_numeric_strings() {
        let _guard = TEST_LOCK.lock().unwrap();
        set_seed(0);
        randomize(Some(VBVariant::from_string("42"))).unwrap();
        assert_eq!(seed(), 0x0040_4500);
    }

    #[test]
    fn rejects_non_numeric_values() {
        let err = randomize(Some(VBVariant::from_string("not-a-number"))).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn rejects_null() {
        let err = randomize(Some(VBVariant::Null)).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }
}
