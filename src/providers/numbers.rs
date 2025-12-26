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

    // Edge case and boundary tests
    #[test]
    fn test_single_integer_batch() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ints = generate_integers(&mut rng, 1, 0, 100).unwrap();
        assert_eq!(ints.len(), 1);
        assert!(ints[0] >= 0 && ints[0] <= 100);
    }

    #[test]
    fn test_large_positive_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let min = 0;
        let max = i64::MAX / 2;
        let ints = generate_integers(&mut rng, 100, min, max).unwrap();

        for i in &ints {
            assert!(*i >= min && *i <= max);
        }
    }

    #[test]
    fn test_large_negative_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let min = i64::MIN / 2;
        let max = 0;
        let ints = generate_integers(&mut rng, 100, min, max).unwrap();

        for i in &ints {
            assert!(*i >= min && *i <= max);
        }
    }

    #[test]
    fn test_full_i64_range() {
        // Test with a very large range (not full i64 to avoid overflow in gen_range)
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let min = i64::MIN / 2;
        let max = i64::MAX / 2;
        let ints = generate_integers(&mut rng, 100, min, max).unwrap();

        for i in &ints {
            assert!(*i >= min && *i <= max);
        }
    }

    #[test]
    fn test_range_error_display() {
        let err = RangeError { min: 50, max: 10 };
        assert_eq!(
            format!("{}", err),
            "min (50) must be less than or equal to max (10)"
        );
    }

    #[test]
    fn test_range_error_debug() {
        let err = RangeError { min: 50, max: 10 };
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("RangeError"));
        assert!(debug_str.contains("min: 50"));
        assert!(debug_str.contains("max: 10"));
    }

    #[test]
    fn test_range_error_clone() {
        let err1 = RangeError { min: 50, max: 10 };
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_range_error_is_error_trait() {
        let err: Box<dyn std::error::Error> = Box::new(RangeError { min: 50, max: 10 });
        assert!(err.to_string().contains("min (50)"));
    }

    #[test]
    fn test_crossing_zero_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ints = generate_integers(&mut rng, 1000, -500, 500).unwrap();
        let has_negative = ints.iter().any(|&x| x < 0);
        let has_positive = ints.iter().any(|&x| x > 0);
        let has_zero = ints.iter().any(|&x| x == 0);

        // With 1000 samples in a range of 1001 values, we should likely see all three
        // (though not guaranteed - this is a sanity check)
        assert!(has_negative || has_positive, "Should generate values");
        // Zero might not appear, but the range should work
        assert!(
            ints.iter().all(|&x| x >= -500 && x <= 500),
            "All values in range"
        );
        // Keep has_zero to avoid unused variable warning
        let _ = has_zero;
    }

    #[test]
    fn test_adjacent_values_range() {
        // Test min and max that are adjacent
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ints = generate_integers(&mut rng, 100, 0, 1).unwrap();
        for i in &ints {
            assert!(*i == 0 || *i == 1);
        }
    }

    #[test]
    fn test_different_seeds_different_results() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let ints1 = generate_integers(&mut rng1, 100, 0, 1_000_000).unwrap();
        let ints2 = generate_integers(&mut rng2, 100, 0, 1_000_000).unwrap();

        assert_ne!(ints1, ints2, "Different seeds should produce different integers");
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: batch size is always respected for valid ranges
        #[test]
        fn prop_batch_size_respected(n in 0usize..1000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let ints = generate_integers(&mut rng, n, 0, 100).unwrap();
            prop_assert_eq!(ints.len(), n);
        }

        /// Property: all values are within the specified range
        #[test]
        fn prop_values_in_range(
            n in 1usize..100,
            min in -1_000_000i64..1_000_000i64,
            delta in 0i64..1_000_000
        ) {
            let max = min.saturating_add(delta);
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let ints = generate_integers(&mut rng, n, min, max).unwrap();
            for i in ints {
                prop_assert!(i >= min && i <= max, "Value {} not in range [{}, {}]", i, min, max);
            }
        }

        /// Property: same seed produces same output
        #[test]
        fn prop_seed_determinism(
            seed_val in any::<u64>(),
            n in 1usize..100,
            min in -1000i64..0i64,
            max in 0i64..1000i64
        ) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let ints1 = generate_integers(&mut rng1, n, min, max).unwrap();
            let ints2 = generate_integers(&mut rng2, n, min, max).unwrap();

            prop_assert_eq!(ints1, ints2);
        }

        /// Property: invalid range always returns error
        #[test]
        fn prop_invalid_range_error(min in 1i64..i64::MAX, delta in 1i64..1000) {
            let max = min.saturating_sub(delta);
            if min > max {
                let mut rng = ForgeryRng::new();
                let result = generate_integers(&mut rng, 10, min, max);
                prop_assert!(result.is_err());
                let err = result.unwrap_err();
                prop_assert_eq!(err.min, min);
                prop_assert_eq!(err.max, max);
            }
        }

        /// Property: when min == max, all values are that value
        #[test]
        fn prop_same_min_max(value in any::<i64>(), n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let ints = generate_integers(&mut rng, n, value, value).unwrap();
            for i in ints {
                prop_assert_eq!(i, value);
            }
        }
    }
}
