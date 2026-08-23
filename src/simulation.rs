//! Utilities used to simulate a Magic game.

use std::fmt;

use rand::Rng;

/// The result of a coin flip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoinFlip {
    /// The coin landed heads up.
    Heads,
    /// The coin landed tails up.
    Tails,
}

/// Error returned when a random range has no valid result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandomRangeError {
    upper_bound: u32,
}

impl RandomRangeError {
    /// Returns the invalid inclusive upper bound.
    #[must_use]
    pub const fn upper_bound(&self) -> u32 {
        self.upper_bound
    }
}

impl fmt::Display for RandomRangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "a random number range from 1 requires a positive upper bound; received {}",
            self.upper_bound
        )
    }
}

impl std::error::Error for RandomRangeError {}

/// Returns a random number in the inclusive range from 1 through `upper_bound`.
///
/// # Errors
///
/// Returns [`RandomRangeError`] when `upper_bound` is zero.
///
/// # Rules
///
/// 100.3. Some cards require coins or traditional dice. Some casual variants
/// require additional items, such as specially designated cards,
/// nontraditional Magic cards, and specialized dice.
///
/// # Future work
///
/// The game simulator must replace this non-deterministic RNG with a
/// seed-based deterministic RNG before simulation is implemented.
pub fn random_number_from_one_to(upper_bound: u32) -> Result<u32, RandomRangeError> {
    if upper_bound <= 0 {
        return Err(RandomRangeError { upper_bound });
    }

    Ok(rand::rng().random_range(1..=upper_bound))
}

/// Simulates a coin flip.
///
/// # Rules
///
/// 100.3. Some cards require coins or traditional dice. Some casual variants
/// require additional items, such as specially designated cards,
/// nontraditional Magic cards, and specialized dice.
///
/// # Future work
///
/// The game simulator must replace this non-deterministic RNG with a
/// seed-based deterministic RNG before simulation is implemented.
#[must_use]
pub fn flip_coin() -> CoinFlip {
    match random_number_from_one_to(2).expect("two is a valid random upper bound") {
        1 => CoinFlip::Heads,
        2 => CoinFlip::Tails,
        _ => unreachable!("the random number is constrained to the requested range"),
    }
}

#[cfg(test)]
mod tests {
    use super::{CoinFlip, RandomRangeError, flip_coin, random_number_from_one_to};

    #[test]
    fn generates_a_number_within_the_requested_range() {
        let result = random_number_from_one_to(20).expect("20 is a valid upper bound");

        assert!((1..=20).contains(&result));
    }

    #[test]
    fn rejects_zero_as_a_random_number_upper_bound() {
        assert_eq!(
            random_number_from_one_to(0),
            Err(RandomRangeError { upper_bound: 0 })
        );
    }

    #[test]
    fn flips_a_coin_to_heads_or_tails() {
        assert!(matches!(flip_coin(), CoinFlip::Heads | CoinFlip::Tails));
    }
}
