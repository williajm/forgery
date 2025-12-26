//! Numeric data generation provider.
//!
//! Generates integers, floats, and other numeric values.

use crate::rng::ForgeryRng;

/// Error type for number generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeError {
    /// The invalid minimum value.
    pub min: i64,
    /// The invalid maximum value.
    pub max: i64,
}

impl std::fmt::Display for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "min ({}) must be less than or equal to max ({})",
            self.min, self.max
        )
    }
}

impl std::error::Error for RangeError {}

/// Generate a batch of random integers within a range.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of integers to generate
/// * `min` - Minimum value (inclusive)
/// * `max` - Maximum value (inclusive)
///
/// # Errors
///
/// Returns `RangeError` if `min > max`.
pub fn generate_integers(
    rng: &mut ForgeryRng,
    n: usize,
    min: i64,
    max: i64,
) -> Result<Vec<i64>, RangeError> {
    if min > max {
        return Err(RangeError { min, max });
    }

    let mut integers = Vec::with_capacity(n);
    for _ in 0..n {
        integers.push(rng.gen_range(min, max));
    }
    Ok(integers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_integers_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ints = generate_integers(&mut rng, 100, 0, 1000).unwrap();
        assert_eq!(ints.len(), 100);
    }

    #[test]
    fn test_generate_integers_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let min = -50;
        let max = 50;
        let ints = generate_integers(&mut rng, 1000, min, max).unwrap();

        for i in &ints {
            assert!(
                *i >= min && *i <= max,
                "Value {} out of range [{}, {}]",
                i,
                min,
                max
            );
        }
    }

    #[test]
    fn test_generate_integers_same_min_max() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ints = generate_integers(&mut rng, 10, 42, 42).unwrap();
        for i in &ints {
            assert_eq!(*i, 42);
        }
    }

    #[test]
    fn test_generate_integers_negative_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ints = generate_integers(&mut rng, 100, -1000, -1).unwrap();
        for i in &ints {
            assert!(*i >= -1000 && *i <= -1);
        }
    }

    #[test]
    fn test_generate_integers_invalid_range_returns_error() {
        let mut rng = ForgeryRng::new();
        let result = generate_integers(&mut rng, 10, 100, 0);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.min, 100);
        assert_eq!(err.max, 0);
        assert_eq!(
            err.to_string(),
            "min (100) must be less than or equal to max (0)"
        );
    }

    #[test]
    fn test_deterministic_generation() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let ints1 = generate_integers(&mut rng1, 100, 0, 1_000_000).unwrap();
        let ints2 = generate_integers(&mut rng2, 100, 0, 1_000_000).unwrap();

        assert_eq!(ints1, ints2);
    }

    #[test]
    fn test_empty_batch() {
        let mut rng = ForgeryRng::new();
        let ints = generate_integers(&mut rng, 0, 0, 100).unwrap();
        assert!(ints.is_empty());
    }
}
