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

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rng::ForgeryRng;

/// Maximum batch size to prevent memory exhaustion.
///
/// This limit is set to 10 million items, which should handle most
/// reasonable use cases while preventing accidental memory exhaustion.
const MAX_BATCH_SIZE: usize = 10_000_000;

/// Validate that a batch size is within acceptable limits.
///
/// # Errors
///
/// Returns `PyValueError` if `n` exceeds `MAX_BATCH_SIZE`.
#[inline]
fn validate_batch_size(n: usize) -> PyResult<()> {
    if n > MAX_BATCH_SIZE {
        return Err(PyValueError::new_err(format!(
            "batch size {} exceeds maximum allowed size of {}",
            n, MAX_BATCH_SIZE
        )));
    }
    Ok(())
}

/// A fake data generator with its own random state.
///
/// Each instance maintains independent RNG state, allowing for deterministic
/// generation when seeded. The default locale is "en_US".
#[pyclass]
pub struct Faker {
    rng: ForgeryRng,
    /// The locale for this Faker instance.
    ///
    /// Currently only "en_US" is supported. This field is stored for future
    /// locale support but does not affect generation behavior yet.
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
    ///
    /// # Errors
    ///
    /// Returns `ValueError` if `n` exceeds the maximum batch size.
    fn names(&mut self, n: usize) -> PyResult<Vec<String>> {
        validate_batch_size(n)?;
        Ok(providers::names::generate_names(&mut self.rng, n))
    }

    /// Generate a batch of random first names.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of first names to generate
    ///
    /// # Errors
    ///
    /// Returns `ValueError` if `n` exceeds the maximum batch size.
    fn first_names(&mut self, n: usize) -> PyResult<Vec<String>> {
        validate_batch_size(n)?;
        Ok(providers::names::generate_first_names(&mut self.rng, n))
    }

    /// Generate a batch of random last names.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of last names to generate
    ///
    /// # Errors
    ///
    /// Returns `ValueError` if `n` exceeds the maximum batch size.
    fn last_names(&mut self, n: usize) -> PyResult<Vec<String>> {
        validate_batch_size(n)?;
        Ok(providers::names::generate_last_names(&mut self.rng, n))
    }

    /// Generate a single random full name.
    fn name(&mut self) -> String {
        providers::names::generate_name(&mut self.rng)
    }

    /// Generate a single random first name.
    fn first_name(&mut self) -> String {
        providers::names::generate_first_name(&mut self.rng)
    }

    /// Generate a single random last name.
    fn last_name(&mut self) -> String {
        providers::names::generate_last_name(&mut self.rng)
    }

    /// Generate a batch of random email addresses.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of emails to generate
    ///
    /// # Errors
    ///
    /// Returns `ValueError` if `n` exceeds the maximum batch size.
    fn emails(&mut self, n: usize) -> PyResult<Vec<String>> {
        validate_batch_size(n)?;
        Ok(providers::internet::generate_emails(&mut self.rng, n))
    }

    /// Generate a single random email address.
    fn email(&mut self) -> String {
        providers::internet::generate_email(&mut self.rng)
    }

    /// Generate a batch of random integers within a range.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of integers to generate
    /// * `min` - Minimum value (inclusive)
    /// * `max` - Maximum value (inclusive)
    ///
    /// # Errors
    ///
    /// Returns `ValueError` if `min > max` or `n` exceeds the maximum batch size.
    #[pyo3(signature = (n, min = 0, max = 100))]
    fn integers(&mut self, n: usize, min: i64, max: i64) -> PyResult<Vec<i64>> {
        validate_batch_size(n)?;
        providers::numbers::generate_integers(&mut self.rng, n, min, max)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random integer within a range.
    ///
    /// # Errors
    ///
    /// Returns `ValueError` if `min > max`.
    #[pyo3(signature = (min = 0, max = 100))]
    fn integer(&mut self, min: i64, max: i64) -> PyResult<i64> {
        providers::numbers::generate_integer(&mut self.rng, min, max)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a batch of random UUIDs (version 4).
    ///
    /// # Arguments
    ///
    /// * `n` - Number of UUIDs to generate
    ///
    /// # Errors
    ///
    /// Returns `ValueError` if `n` exceeds the maximum batch size.
    fn uuids(&mut self, n: usize) -> PyResult<Vec<String>> {
        validate_batch_size(n)?;
        Ok(providers::identifiers::generate_uuids(&mut self.rng, n))
    }

    /// Generate a single random UUID (version 4).
    fn uuid(&mut self) -> String {
        providers::identifiers::generate_uuid(&mut self.rng)
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

        let names1 = faker1.names(10).unwrap();
        let names2 = faker2.names(10).unwrap();

        assert_eq!(names1, names2);
    }

    #[test]
    fn test_batch_generation() {
        let mut faker = Faker::new("en_US");
        faker.seed(123);

        let names = faker.names(100).unwrap();
        assert_eq!(names.len(), 100);

        let emails = faker.emails(50).unwrap();
        assert_eq!(emails.len(), 50);

        let ints = faker.integers(200, 0, 1000).unwrap();
        assert_eq!(ints.len(), 200);
        for i in &ints {
            assert!(*i >= 0 && *i <= 1000);
        }
    }
}
