//! # forgery
//!
//! Fake data at the speed of Rust.
//!
//! A high-performance fake data generation library for Python, powered by Rust.
//! Designed to be 50-100x faster than Faker for batch operations.
//!
//! ## Architecture
//!
//! - **Batch-first**: All generators return `Vec<T>` efficiently
//! - **Per-instance RNG**: Each `Faker` has its own seeded random state
//! - **Deterministic**: Same seed produces same output (single-threaded)

#![deny(missing_docs)]

mod data;
mod providers;
mod rng;

use pyo3::prelude::*;
use rng::ForgeryRng;

/// A fake data generator with its own random state.
///
/// Each instance maintains independent RNG state, allowing for deterministic
/// generation when seeded. The default locale is "en_US".
#[pyclass]
pub struct Faker {
    rng: ForgeryRng,
    #[allow(dead_code)]
    locale: String,
}

#[pymethods]
impl Faker {
    /// Create a new Faker instance with the specified locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale for generated data (default: "en_US")
    #[new]
    #[pyo3(signature = (locale = "en_US"))]
    fn new(locale: &str) -> Self {
        Self {
            rng: ForgeryRng::new(),
            locale: locale.to_string(),
        }
    }

    /// Seed the random number generator for deterministic output.
    ///
    /// # Arguments
    ///
    /// * `value` - The seed value
    ///
    /// # Example
    ///
    /// ```python
    /// from forgery import Faker
    /// fake = Faker()
    /// fake.seed(42)
    /// names1 = fake.names(10)
    /// fake.seed(42)
    /// names2 = fake.names(10)
    /// assert names1 == names2
    /// ```
    fn seed(&mut self, value: u64) {
        self.rng.seed(value);
    }

    /// Generate a batch of random full names.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of names to generate
    ///
    /// # Returns
    ///
    /// A list of full names (first + last)
    fn names(&mut self, n: usize) -> Vec<String> {
        providers::names::generate_names(&mut self.rng, n)
    }

    /// Generate a batch of random first names.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of first names to generate
    fn first_names(&mut self, n: usize) -> Vec<String> {
        providers::names::generate_first_names(&mut self.rng, n)
    }

    /// Generate a batch of random last names.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of last names to generate
    fn last_names(&mut self, n: usize) -> Vec<String> {
        providers::names::generate_last_names(&mut self.rng, n)
    }

    /// Generate a single random full name.
    fn name(&mut self) -> String {
        providers::names::generate_names(&mut self.rng, 1)
            .pop()
            .unwrap_or_default()
    }

    /// Generate a single random first name.
    fn first_name(&mut self) -> String {
        providers::names::generate_first_names(&mut self.rng, 1)
            .pop()
            .unwrap_or_default()
    }

    /// Generate a single random last name.
    fn last_name(&mut self) -> String {
        providers::names::generate_last_names(&mut self.rng, 1)
            .pop()
            .unwrap_or_default()
    }

    /// Generate a batch of random email addresses.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of emails to generate
    fn emails(&mut self, n: usize) -> Vec<String> {
        providers::internet::generate_emails(&mut self.rng, n)
    }

    /// Generate a single random email address.
    fn email(&mut self) -> String {
        providers::internet::generate_emails(&mut self.rng, 1)
            .pop()
            .unwrap_or_default()
    }

    /// Generate a batch of random integers within a range.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of integers to generate
    /// * `min` - Minimum value (inclusive)
    /// * `max` - Maximum value (inclusive)
    #[pyo3(signature = (n, min = 0, max = 100))]
    fn integers(&mut self, n: usize, min: i64, max: i64) -> Vec<i64> {
        providers::numbers::generate_integers(&mut self.rng, n, min, max)
    }

    /// Generate a single random integer within a range.
    #[pyo3(signature = (min = 0, max = 100))]
    fn integer(&mut self, min: i64, max: i64) -> i64 {
        providers::numbers::generate_integers(&mut self.rng, 1, min, max)
            .pop()
            .unwrap_or(min)
    }

    /// Generate a batch of random UUIDs (version 4).
    ///
    /// # Arguments
    ///
    /// * `n` - Number of UUIDs to generate
    fn uuids(&mut self, n: usize) -> Vec<String> {
        providers::identifiers::generate_uuids(&mut self.rng, n)
    }

    /// Generate a single random UUID (version 4).
    fn uuid(&mut self) -> String {
        providers::identifiers::generate_uuids(&mut self.rng, 1)
            .pop()
            .unwrap_or_default()
    }
}

/// The forgery Python module.
#[pymodule]
fn _forgery(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Faker>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faker_creation() {
        let faker = Faker::new("en_US");
        assert_eq!(faker.locale, "en_US");
    }

    #[test]
    fn test_seeding_determinism() {
        let mut faker1 = Faker::new("en_US");
        let mut faker2 = Faker::new("en_US");

        faker1.seed(42);
        faker2.seed(42);

        let names1 = faker1.names(10);
        let names2 = faker2.names(10);

        assert_eq!(names1, names2);
    }

    #[test]
    fn test_batch_generation() {
        let mut faker = Faker::new("en_US");
        faker.seed(123);

        let names = faker.names(100);
        assert_eq!(names.len(), 100);

        let emails = faker.emails(50);
        assert_eq!(emails.len(), 50);

        let ints = faker.integers(200, 0, 1000);
        assert_eq!(ints.len(), 200);
        for i in &ints {
            assert!(*i >= 0 && *i <= 1000);
        }
    }
}
