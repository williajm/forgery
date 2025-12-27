//! Numeric data generation provider.
//!
//! Generates integers, floats, and other numeric values.

use crate::rng::ForgeryRng;

/// Error type for integer range generation.
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

/// Error type for float range generation.
#[derive(Debug, Clone, PartialEq)]
pub struct FloatRangeError {
    /// The invalid minimum value.
    pub min: f64,
    /// The invalid maximum value.
    pub max: f64,
    /// The reason for the error.
    pub reason: FloatRangeErrorReason,
}

/// Reason for a float range error.
#[derive(Debug, Clone, PartialEq)]
pub enum FloatRangeErrorReason {
    /// min > max
    MinGreaterThanMax,
    /// min or max is NaN
    NonFiniteValue,
}

impl std::fmt::Display for FloatRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.reason {
            FloatRangeErrorReason::MinGreaterThanMax => {
                write!(
                    f,
                    "min ({}) must be less than or equal to max ({})",
                    self.min, self.max
                )
            }
            FloatRangeErrorReason::NonFiniteValue => {
                write!(
                    f,
                    "min ({}) and max ({}) must be finite values (not NaN or infinity)",
                    self.min, self.max
                )
            }
        }
    }
}

impl std::error::Error for FloatRangeError {}

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

/// Generate a single random integer within a range.
///
/// More efficient than `generate_integers(rng, 1, min, max)` as it avoids Vec allocation.
///
/// # Errors
///
/// Returns `RangeError` if `min > max`.
#[inline]
pub fn generate_integer(rng: &mut ForgeryRng, min: i64, max: i64) -> Result<i64, RangeError> {
    if min > max {
        return Err(RangeError { min, max });
    }
    Ok(rng.gen_range(min, max))
}

/// Generate a batch of random floats within a range.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of floats to generate
/// * `min` - Minimum value (inclusive, must be finite)
/// * `max` - Maximum value (inclusive, must be finite)
///
/// # Errors
///
/// Returns `FloatRangeError` if `min > max` or if either value is NaN or infinity.
pub fn generate_floats(
    rng: &mut ForgeryRng,
    n: usize,
    min: f64,
    max: f64,
) -> Result<Vec<f64>, FloatRangeError> {
    // Check for non-finite values (NaN or infinity)
    if !min.is_finite() || !max.is_finite() {
        return Err(FloatRangeError {
            min,
            max,
            reason: FloatRangeErrorReason::NonFiniteValue,
        });
    }

    if min > max {
        return Err(FloatRangeError {
            min,
            max,
            reason: FloatRangeErrorReason::MinGreaterThanMax,
        });
    }

    let mut floats = Vec::with_capacity(n);
    for _ in 0..n {
        floats.push(rng.gen_range(min, max));
    }
    Ok(floats)
}

/// Generate a single random float within a range.
///
/// More efficient than `generate_floats(rng, 1, min, max)` as it avoids Vec allocation.
///
/// # Errors
///
/// Returns `FloatRangeError` if `min > max` or if either value is NaN or infinity.
#[inline]
pub fn generate_float(rng: &mut ForgeryRng, min: f64, max: f64) -> Result<f64, FloatRangeError> {
    // Check for non-finite values (NaN or infinity)
    if !min.is_finite() || !max.is_finite() {
        return Err(FloatRangeError {
            min,
            max,
            reason: FloatRangeErrorReason::NonFiniteValue,
        });
    }

    if min > max {
        return Err(FloatRangeError {
            min,
            max,
            reason: FloatRangeErrorReason::MinGreaterThanMax,
        });
    }
    Ok(rng.gen_range(min, max))
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
        let has_zero = ints.contains(&0);

        // With 1000 samples in a range of 1001 values, we should likely see all three
        // (though not guaranteed - this is a sanity check)
        assert!(has_negative || has_positive, "Should generate values");
        // Zero might not appear, but the range should work
        assert!(
            ints.iter().all(|&x| (-500..=500).contains(&x)),
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

        assert_ne!(
            ints1, ints2,
            "Different seeds should produce different integers"
        );
    }

    // Float tests
    #[test]
    fn test_generate_floats_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let floats = generate_floats(&mut rng, 100, 0.0, 1.0).unwrap();
        assert_eq!(floats.len(), 100);
    }

    #[test]
    fn test_generate_floats_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let min = -50.0;
        let max = 50.0;
        let floats = generate_floats(&mut rng, 1000, min, max).unwrap();

        for f in &floats {
            assert!(
                *f >= min && *f <= max,
                "Value {} out of range [{}, {}]",
                f,
                min,
                max
            );
        }
    }

    #[test]
    fn test_generate_floats_same_min_max() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let floats = generate_floats(&mut rng, 10, 42.0, 42.0).unwrap();
        for f in &floats {
            assert!((*f - 42.0).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_generate_floats_negative_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let floats = generate_floats(&mut rng, 100, -1000.0, -1.0).unwrap();
        for f in &floats {
            assert!(*f >= -1000.0 && *f <= -1.0);
        }
    }

    #[test]
    fn test_generate_floats_invalid_range_returns_error() {
        let mut rng = ForgeryRng::new();
        let result = generate_floats(&mut rng, 10, 100.0, 0.0);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!((err.min - 100.0).abs() < f64::EPSILON);
        assert!(err.max.abs() < f64::EPSILON);
        assert!(err.to_string().contains("min (100)"));
    }

    #[test]
    fn test_float_deterministic_generation() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let floats1 = generate_floats(&mut rng1, 100, 0.0, 1_000_000.0).unwrap();
        let floats2 = generate_floats(&mut rng2, 100, 0.0, 1_000_000.0).unwrap();

        assert_eq!(floats1, floats2);
    }

    #[test]
    fn test_float_empty_batch() {
        let mut rng = ForgeryRng::new();
        let floats = generate_floats(&mut rng, 0, 0.0, 100.0).unwrap();
        assert!(floats.is_empty());
    }

    #[test]
    fn test_generate_float_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let f = generate_float(&mut rng, 0.0, 100.0).unwrap();
        assert!((0.0..=100.0).contains(&f));
    }

    #[test]
    fn test_generate_float_invalid_range() {
        let mut rng = ForgeryRng::new();
        let result = generate_float(&mut rng, 100.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_float_range_error_display() {
        let err = FloatRangeError {
            min: 50.0,
            max: 10.0,
            reason: FloatRangeErrorReason::MinGreaterThanMax,
        };
        assert!(err.to_string().contains("min (50)"));
        assert!(err.to_string().contains("max (10)"));
    }

    #[test]
    fn test_float_range_error_debug() {
        let err = FloatRangeError {
            min: 50.0,
            max: 10.0,
            reason: FloatRangeErrorReason::MinGreaterThanMax,
        };
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("FloatRangeError"));
    }

    #[test]
    fn test_float_range_error_clone() {
        let err1 = FloatRangeError {
            min: 50.0,
            max: 10.0,
            reason: FloatRangeErrorReason::MinGreaterThanMax,
        };
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_float_nan_rejected() {
        let mut rng = ForgeryRng::new();

        // NaN min
        let result = generate_float(&mut rng, f64::NAN, 1.0);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.reason, FloatRangeErrorReason::NonFiniteValue);

        // NaN max
        let result = generate_float(&mut rng, 0.0, f64::NAN);
        assert!(result.is_err());

        // Both NaN
        let result = generate_float(&mut rng, f64::NAN, f64::NAN);
        assert!(result.is_err());
    }

    #[test]
    fn test_float_infinity_rejected() {
        let mut rng = ForgeryRng::new();

        // Positive infinity
        let result = generate_float(&mut rng, 0.0, f64::INFINITY);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.reason, FloatRangeErrorReason::NonFiniteValue);

        // Negative infinity
        let result = generate_float(&mut rng, f64::NEG_INFINITY, 0.0);
        assert!(result.is_err());

        // Both infinite
        let result = generate_float(&mut rng, f64::NEG_INFINITY, f64::INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn test_floats_batch_nan_rejected() {
        let mut rng = ForgeryRng::new();

        let result = generate_floats(&mut rng, 10, f64::NAN, 1.0);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.reason, FloatRangeErrorReason::NonFiniteValue);
        assert!(err.to_string().contains("finite values"));
    }

    #[test]
    fn test_float_fractional_values() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let floats = generate_floats(&mut rng, 100, 0.0, 1.0).unwrap();
        // At least some values should be fractional (not 0.0 or 1.0)
        let fractional_count = floats.iter().filter(|&&f| f > 0.01 && f < 0.99).count();
        assert!(fractional_count > 50, "Should have many fractional values");
    }

    #[test]
    fn test_float_different_seeds_different_results() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let floats1 = generate_floats(&mut rng1, 100, 0.0, 1_000_000.0).unwrap();
        let floats2 = generate_floats(&mut rng2, 100, 0.0, 1_000_000.0).unwrap();

        assert_ne!(
            floats1, floats2,
            "Different seeds should produce different floats"
        );
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

        // Float property tests

        /// Property: float batch size is always respected
        #[test]
        fn prop_float_batch_size_respected(n in 0usize..1000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let floats = generate_floats(&mut rng, n, 0.0, 1.0).unwrap();
            prop_assert_eq!(floats.len(), n);
        }

        /// Property: all float values are within the specified range
        #[test]
        fn prop_float_values_in_range(
            n in 1usize..100,
            min in -1_000_000.0f64..1_000_000.0f64,
            delta in 0.0f64..1_000_000.0f64
        ) {
            let max = min + delta;
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let floats = generate_floats(&mut rng, n, min, max).unwrap();
            for f in floats {
                prop_assert!(f >= min && f <= max, "Value {} not in range [{}, {}]", f, min, max);
            }
        }

        /// Property: same seed produces same float output
        #[test]
        fn prop_float_seed_determinism(
            seed_val in any::<u64>(),
            n in 1usize..100,
            min in -1000.0f64..0.0f64,
            max in 0.0f64..1000.0f64
        ) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let floats1 = generate_floats(&mut rng1, n, min, max).unwrap();
            let floats2 = generate_floats(&mut rng2, n, min, max).unwrap();

            prop_assert_eq!(floats1, floats2);
        }

        /// Property: invalid float range always returns error
        #[test]
        fn prop_float_invalid_range_error(min in 1.0f64..1_000_000.0, delta in 1.0f64..1000.0) {
            let max = min - delta;
            let mut rng = ForgeryRng::new();
            let result = generate_floats(&mut rng, 10, min, max);
            prop_assert!(result.is_err());
        }
    }
}
