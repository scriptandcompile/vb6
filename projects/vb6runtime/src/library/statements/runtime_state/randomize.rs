//! Parse a VB6 Randomize statement.
//!
//! # Syntax
//!
//! ```vb
//! Randomize [number]
//! ```
//!
//! # Arguments
//!
//! | Part | Optional / Required | Description |
//! |------|---------------------|-------------|
//! | number | Optional | A Variant or any valid numeric expression that is used as the new seed value to initialize the random number generator. |
//!
//! # Remarks
//!
//! - The Randomize statement initializes the random-number generator, giving it a new seed value.
//! - If you omit number, the value returned by the system timer is used as the new seed value.
//! - If Randomize is not used, the Rnd function (with no arguments) uses the same number as a seed the first time it is called, and thereafter uses the last generated number as a seed value.
//! - To repeat sequences of random numbers, call Rnd with a negative argument immediately before using Randomize with a numeric argument.
//! - Using Randomize with the same value for number does not repeat the previous sequence.
//!
//! # Examples
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
