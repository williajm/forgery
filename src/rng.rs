//! Random number generation infrastructure.
//!
//! Provides a seedable RNG wrapper using ChaCha8 for deterministic generation.
//! Each `ForgeryRng` instance maintains its own state, enabling per-Faker seeding.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// A seedable random number generator for forgery.
///
/// Uses ChaCha8 for a good balance of speed and quality.
/// Each instance is independent, allowing multiple Faker instances
/// to have their own reproducible sequences.
///
/// Implements `Clone` to support async generation where RNG state
/// must be captured before entering async blocks.
#[derive(Clone)]
pub struct ForgeryRng {
    rng: ChaCha8Rng,
}

impl ForgeryRng {
    /// Create a new RNG with a random seed.
    pub fn new() -> Self {
        Self {
            rng: ChaCha8Rng::from_os_rng(),
        }
    }

    /// Seed the RNG for deterministic output.
    ///
    /// After seeding, the same sequence of calls will produce
    /// the same results.
    pub fn seed(&mut self, value: u64) {
        self.rng = ChaCha8Rng::seed_from_u64(value);
    }

    /// Generate a random value within a range (inclusive).
    #[inline]
    pub fn gen_range<T>(&mut self, min: T, max: T) -> T
    where
        T: rand::distr::uniform::SampleUniform + PartialOrd,
    {
        self.rng.random_range(min..=max)
    }

    /// Choose a random element from a slice.
    ///
    /// # Panics
    ///
    /// Panics if the slice is empty.
    #[inline]
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> &'a T {
        assert!(!slice.is_empty(), "cannot choose from an empty slice");
        let idx = self.rng.random_range(0..slice.len());
        &slice[idx]
    }

    /// Generate random bytes to fill the given buffer.
    #[inline]
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill(dest);
    }
}

impl Default for ForgeryRng {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeding_produces_deterministic_output() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(42);
        rng2.seed(42);

        let values1: Vec<i32> = (0..100).map(|_| rng1.gen_range(0, 1000)).collect();
        let values2: Vec<i32> = (0..100).map(|_| rng2.gen_range(0, 1000)).collect();

        assert_eq!(values1, values2);
    }

    #[test]
    fn test_different_seeds_produce_different_output() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(42);
        rng2.seed(43);

        let values1: Vec<i32> = (0..100).map(|_| rng1.gen_range(0, 1000)).collect();
        let values2: Vec<i32> = (0..100).map(|_| rng2.gen_range(0, 1000)).collect();

        assert_ne!(values1, values2);
    }

    #[test]
    fn test_choose_from_slice() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let items = ["a", "b", "c", "d", "e"];
        let chosen = rng.choose(&items);

        assert!(items.contains(chosen));
    }

    #[test]
    #[should_panic(expected = "cannot choose from an empty slice")]
    fn test_choose_from_empty_slice_panics() {
        let mut rng = ForgeryRng::new();
        let empty: &[&str] = &[];
        rng.choose(empty);
    }

    // Edge case tests
    #[test]
    fn test_default_trait() {
        let rng1 = ForgeryRng::default();
        let rng2 = ForgeryRng::new();

        // Both should work (though their initial state is random)
        assert!(std::mem::size_of_val(&rng1) == std::mem::size_of_val(&rng2));
    }

    #[test]
    fn test_seed_zero() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(0);
        rng2.seed(0);

        let values1: Vec<i32> = (0..100).map(|_| rng1.gen_range(0, 1000)).collect();
        let values2: Vec<i32> = (0..100).map(|_| rng2.gen_range(0, 1000)).collect();

        assert_eq!(values1, values2);
    }

    #[test]
    fn test_seed_max_u64() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(u64::MAX);
        rng2.seed(u64::MAX);

        let values1: Vec<i32> = (0..100).map(|_| rng1.gen_range(0, 1000)).collect();
        let values2: Vec<i32> = (0..100).map(|_| rng2.gen_range(0, 1000)).collect();

        assert_eq!(values1, values2);
    }

    #[test]
    fn test_reseed_resets_state() {
        let mut rng = ForgeryRng::new();

        rng.seed(42);
        let values1: Vec<i32> = (0..50).map(|_| rng.gen_range(0, 1000)).collect();

        // Advance state further
        for _ in 0..1000 {
            rng.gen_range(0i32, 1000);
        }

        // Reseed with same value
        rng.seed(42);
        let values2: Vec<i32> = (0..50).map(|_| rng.gen_range(0, 1000)).collect();

        assert_eq!(values1, values2);
    }

    #[test]
    fn test_gen_range_single_value() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        // When min == max, result should always be that value
        for _ in 0..100 {
            assert_eq!(rng.gen_range(42i32, 42), 42);
        }
    }

    #[test]
    fn test_gen_range_adjacent_values() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        // With min and max adjacent, should only produce 0 or 1
        let values: Vec<i32> = (0..1000).map(|_| rng.gen_range(0, 1)).collect();
        for v in &values {
            assert!(*v == 0 || *v == 1);
        }
        // Should have both values in 1000 samples
        assert!(values.contains(&0));
        assert!(values.contains(&1));
    }

    #[test]
    fn test_gen_range_negative_values() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let values: Vec<i32> = (0..100).map(|_| rng.gen_range(-1000, -500)).collect();
        for v in &values {
            assert!(*v >= -1000 && *v <= -500);
        }
    }

    #[test]
    fn test_gen_range_crossing_zero() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let values: Vec<i32> = (0..1000).map(|_| rng.gen_range(-500, 500)).collect();

        // Should have both negative and positive values
        assert!(values.iter().any(|&x| x < 0));
        assert!(values.iter().any(|&x| x > 0));
    }

    #[test]
    fn test_choose_from_single_element_slice() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let items = ["only"];
        for _ in 0..100 {
            assert_eq!(*rng.choose(&items), "only");
        }
    }

    #[test]
    fn test_choose_distribution() {
        // Test that choose produces a reasonable distribution
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let items = [0, 1, 2, 3, 4];
        let mut counts = [0usize; 5];

        for _ in 0..10000 {
            let chosen = *rng.choose(&items);
            counts[chosen] += 1;
        }

        // Each item should be chosen roughly 2000 times (10000/5)
        // Allow 40% deviation for randomness
        for count in counts.iter() {
            assert!(*count > 1200, "Count {} too low", count);
            assert!(*count < 2800, "Count {} too high", count);
        }
    }

    #[test]
    fn test_fill_bytes_various_sizes() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        // Test various buffer sizes
        let mut buf1 = [0u8; 0];
        rng.fill_bytes(&mut buf1);
        assert!(buf1.is_empty());

        let mut buf2 = [0u8; 1];
        rng.fill_bytes(&mut buf2);
        // Just verify it doesn't panic

        let mut buf3 = [0u8; 16];
        rng.fill_bytes(&mut buf3);
        // Verify at least some bytes are non-zero
        assert!(buf3.iter().any(|&b| b != 0));

        let mut buf4 = [0u8; 1024];
        rng.fill_bytes(&mut buf4);
        assert!(buf4.iter().any(|&b| b != 0));
    }

    #[test]
    fn test_fill_bytes_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(42);
        rng2.seed(42);

        let mut buf1 = [0u8; 64];
        let mut buf2 = [0u8; 64];

        rng1.fill_bytes(&mut buf1);
        rng2.fill_bytes(&mut buf2);

        assert_eq!(buf1, buf2);
    }

    #[test]
    fn test_gen_range_with_floats() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let values: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0, 1.0)).collect();
        for v in &values {
            assert!(*v >= 0.0 && *v <= 1.0);
        }
    }

    #[test]
    fn test_gen_range_with_usize() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let values: Vec<usize> = (0..100).map(|_| rng.gen_range(0usize, 1000)).collect();
        for v in &values {
            assert!(*v <= 1000);
        }
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: same seed always produces same sequence
        #[test]
        fn prop_seed_determinism(seed_val in any::<u64>()) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let values1: Vec<i32> = (0..50).map(|_| rng1.gen_range(0, 10000)).collect();
            let values2: Vec<i32> = (0..50).map(|_| rng2.gen_range(0, 10000)).collect();

            prop_assert_eq!(values1, values2);
        }

        /// Property: gen_range always returns values in range
        #[test]
        fn prop_gen_range_in_bounds(
            seed_val in any::<u64>(),
            min in -10000i32..0,
            delta in 1i32..10000
        ) {
            let max = min + delta;
            let mut rng = ForgeryRng::new();
            rng.seed(seed_val);

            for _ in 0..100 {
                let value = rng.gen_range(min, max);
                prop_assert!(value >= min && value <= max);
            }
        }

        /// Property: choose always returns an element from the slice
        #[test]
        fn prop_choose_valid_element(seed_val in any::<u64>(), slice_size in 1usize..100) {
            let items: Vec<usize> = (0..slice_size).collect();
            let mut rng = ForgeryRng::new();
            rng.seed(seed_val);

            for _ in 0..100 {
                let chosen = rng.choose(&items);
                prop_assert!(items.contains(chosen));
            }
        }

        /// Property: fill_bytes is deterministic with same seed
        #[test]
        fn prop_fill_bytes_deterministic(seed_val in any::<u64>(), buf_size in 1usize..256) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let mut buf1 = vec![0u8; buf_size];
            let mut buf2 = vec![0u8; buf_size];

            rng1.fill_bytes(&mut buf1);
            rng2.fill_bytes(&mut buf2);

            prop_assert_eq!(buf1, buf2);
        }

        /// Property: different seeds produce different sequences
        #[test]
        fn prop_different_seeds_different_results(seed1 in any::<u64>(), seed2 in any::<u64>()) {
            prop_assume!(seed1 != seed2);

            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed1);
            rng2.seed(seed2);

            let values1: Vec<i32> = (0..50).map(|_| rng1.gen_range(0, 1_000_000)).collect();
            let values2: Vec<i32> = (0..50).map(|_| rng2.gen_range(0, 1_000_000)).collect();

            // Different seeds should produce different sequences
            prop_assert_ne!(values1, values2);
        }
    }
}
