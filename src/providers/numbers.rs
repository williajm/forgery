//! Numeric data generation provider.
//!
//! Generates integers, floats, and other numeric values.

use crate::rng::ForgeryRng;

/// Generate a batch of random integers within a range.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of integers to generate
/// * `min` - Minimum value (inclusive)
/// * `max` - Maximum value (inclusive)
///
/// # Panics
///
/// Panics if `min > max`.
pub fn generate_integers(rng: &mut ForgeryRng, n: usize, min: i64, max: i64) -> Vec<i64> {
    assert!(min <= max, "min must be <= max");

    let mut integers = Vec::with_capacity(n);
    for _ in 0..n {
        integers.push(rng.gen_range(min, max));
    }
    integers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_integers_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ints = generate_integers(&mut rng, 100, 0, 1000);
        assert_eq!(ints.len(), 100);
    }

    #[test]
    fn test_generate_integers_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let min = -50;
        let max = 50;
        let ints = generate_integers(&mut rng, 1000, min, max);

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

        let ints = generate_integers(&mut rng, 10, 42, 42);
        for i in &ints {
            assert_eq!(*i, 42);
        }
    }

    #[test]
    fn test_generate_integers_negative_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ints = generate_integers(&mut rng, 100, -1000, -1);
        for i in &ints {
            assert!(*i >= -1000 && *i <= -1);
        }
    }

    #[test]
    #[should_panic(expected = "min must be <= max")]
    fn test_generate_integers_invalid_range() {
        let mut rng = ForgeryRng::new();
        generate_integers(&mut rng, 10, 100, 0);
    }

    #[test]
    fn test_deterministic_generation() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let ints1 = generate_integers(&mut rng1, 100, 0, 1_000_000);
        let ints2 = generate_integers(&mut rng2, 100, 0, 1_000_000);

        assert_eq!(ints1, ints2);
    }

    #[test]
    fn test_empty_batch() {
        let mut rng = ForgeryRng::new();
        let ints = generate_integers(&mut rng, 0, 0, 100);
        assert!(ints.is_empty());
    }
}
