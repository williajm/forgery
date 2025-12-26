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
pub struct ForgeryRng {
    rng: ChaCha8Rng,
}

impl ForgeryRng {
    /// Create a new RNG with a random seed.
    pub fn new() -> Self {
        Self {
            rng: ChaCha8Rng::from_entropy(),
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
        T: rand::distributions::uniform::SampleUniform + PartialOrd,
    {
        self.rng.gen_range(min..=max)
    }

    /// Choose a random element from a slice.
    #[inline]
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> &'a T {
        let idx = self.rng.gen_range(0..slice.len());
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
}
