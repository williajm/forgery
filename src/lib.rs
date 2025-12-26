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
pub const MAX_BATCH_SIZE: usize = 10_000_000;

/// The only currently supported locale.
pub const SUPPORTED_LOCALE: &str = "en_US";

/// Error type for batch size validation.
#[derive(Debug, Clone)]
pub struct BatchSizeError {
    /// The requested batch size.
    pub requested: usize,
    /// The maximum allowed batch size.
    pub max: usize,
}

impl std::fmt::Display for BatchSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "batch size {} exceeds maximum allowed size of {}",
            self.requested, self.max
        )
    }
}

impl std::error::Error for BatchSizeError {}

/// Error type for unsupported locale.
#[derive(Debug, Clone)]
pub struct LocaleError {
    /// The requested locale.
    pub requested: String,
    /// The supported locales.
    pub supported: Vec<String>,
}

impl std::fmt::Display for LocaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unsupported locale '{}', supported locales: {}",
            self.requested,
            self.supported.join(", ")
        )
    }
}

impl std::error::Error for LocaleError {}

/// Validate that a batch size is within acceptable limits.
///
/// # Errors
///
/// Returns `BatchSizeError` if `n` exceeds `MAX_BATCH_SIZE`.
#[inline]
pub fn validate_batch_size(n: usize) -> Result<(), BatchSizeError> {
    if n > MAX_BATCH_SIZE {
        return Err(BatchSizeError {
            requested: n,
            max: MAX_BATCH_SIZE,
        });
    }
    Ok(())
}

/// Validate that a locale is supported.
///
/// # Errors
///
/// Returns `LocaleError` if the locale is not supported.
#[inline]
pub fn validate_locale(locale: &str) -> Result<(), LocaleError> {
    if locale != SUPPORTED_LOCALE {
        return Err(LocaleError {
            requested: locale.to_string(),
            supported: vec![SUPPORTED_LOCALE.to_string()],
        });
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
    locale: String,
}

// Public Rust API - these methods are callable from Rust code (including benchmarks)
impl Faker {
    /// Create a new Faker instance with the specified locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale for generated data (currently only "en_US" is supported)
    ///
    /// # Errors
    ///
    /// Returns `LocaleError` if the locale is not supported.
    pub fn new(locale: &str) -> Result<Self, LocaleError> {
        validate_locale(locale)?;
        Ok(Self {
            rng: ForgeryRng::new(),
            locale: locale.to_string(),
        })
    }

    /// Create a new Faker instance with the default locale ("en_US").
    pub fn new_default() -> Self {
        Self {
            rng: ForgeryRng::new(),
            locale: SUPPORTED_LOCALE.to_string(),
        }
    }

    /// Get the locale for this Faker instance.
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// Seed the random number generator for deterministic output.
    ///
    /// # Arguments
    ///
    /// * `value` - The seed value
    pub fn seed(&mut self, value: u64) {
        self.rng.seed(value);
    }

    /// Generate a batch of random full names.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of names to generate
    ///
    /// # Errors
    ///
    /// Returns `BatchSizeError` if `n` exceeds the maximum batch size.
    pub fn names(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::names::generate_names(&mut self.rng, n))
    }

    /// Generate a batch of random first names.
    ///
    /// # Errors
    ///
    /// Returns `BatchSizeError` if `n` exceeds the maximum batch size.
    pub fn first_names(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::names::generate_first_names(&mut self.rng, n))
    }

    /// Generate a batch of random last names.
    ///
    /// # Errors
    ///
    /// Returns `BatchSizeError` if `n` exceeds the maximum batch size.
    pub fn last_names(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::names::generate_last_names(&mut self.rng, n))
    }

    /// Generate a single random full name.
    pub fn name(&mut self) -> String {
        providers::names::generate_name(&mut self.rng)
    }

    /// Generate a single random first name.
    pub fn first_name(&mut self) -> String {
        providers::names::generate_first_name(&mut self.rng)
    }

    /// Generate a single random last name.
    pub fn last_name(&mut self) -> String {
        providers::names::generate_last_name(&mut self.rng)
    }

    /// Generate a batch of random email addresses.
    ///
    /// # Errors
    ///
    /// Returns `BatchSizeError` if `n` exceeds the maximum batch size.
    pub fn emails(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::internet::generate_emails(&mut self.rng, n))
    }

    /// Generate a single random email address.
    pub fn email(&mut self) -> String {
        providers::internet::generate_email(&mut self.rng)
    }

    /// Generate a batch of random integers within a range.
    ///
    /// # Errors
    ///
    /// Returns an error if `min > max` or `n` exceeds the maximum batch size.
    pub fn integers(
        &mut self,
        n: usize,
        min: i64,
        max: i64,
    ) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
        validate_batch_size(n)?;
        Ok(providers::numbers::generate_integers(
            &mut self.rng,
            n,
            min,
            max,
        )?)
    }

    /// Generate a single random integer within a range.
    ///
    /// # Errors
    ///
    /// Returns an error if `min > max`.
    pub fn integer(&mut self, min: i64, max: i64) -> Result<i64, providers::numbers::RangeError> {
        providers::numbers::generate_integer(&mut self.rng, min, max)
    }

    /// Generate a batch of random UUIDs (version 4).
    ///
    /// # Errors
    ///
    /// Returns `BatchSizeError` if `n` exceeds the maximum batch size.
    pub fn uuids(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::identifiers::generate_uuids(&mut self.rng, n))
    }

    /// Generate a single random UUID (version 4).
    pub fn uuid(&mut self) -> String {
        providers::identifiers::generate_uuid(&mut self.rng)
    }
}

// Python API - these methods are exposed to Python via PyO3
#[pymethods]
impl Faker {
    /// Create a new Faker instance with the specified locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale for generated data (default: "en_US")
    ///
    /// # Errors
    ///
    /// Returns `ValueError` if the locale is not supported.
    #[new]
    #[pyo3(signature = (locale = "en_US"))]
    fn py_new(locale: &str) -> PyResult<Self> {
        Self::new(locale).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Seed the random number generator for deterministic output.
    #[pyo3(name = "seed")]
    fn py_seed(&mut self, value: u64) {
        self.seed(value);
    }

    /// Generate a batch of random full names.
    #[pyo3(name = "names")]
    fn py_names(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.names(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a batch of random first names.
    #[pyo3(name = "first_names")]
    fn py_first_names(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.first_names(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a batch of random last names.
    #[pyo3(name = "last_names")]
    fn py_last_names(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.last_names(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random full name.
    #[pyo3(name = "name")]
    fn py_name(&mut self) -> String {
        self.name()
    }

    /// Generate a single random first name.
    #[pyo3(name = "first_name")]
    fn py_first_name(&mut self) -> String {
        self.first_name()
    }

    /// Generate a single random last name.
    #[pyo3(name = "last_name")]
    fn py_last_name(&mut self) -> String {
        self.last_name()
    }

    /// Generate a batch of random email addresses.
    #[pyo3(name = "emails")]
    fn py_emails(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.emails(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random email address.
    #[pyo3(name = "email")]
    fn py_email(&mut self) -> String {
        self.email()
    }

    /// Generate a batch of random integers within a range.
    #[pyo3(name = "integers", signature = (n, min = 0, max = 100))]
    fn py_integers(&mut self, n: usize, min: i64, max: i64) -> PyResult<Vec<i64>> {
        self.integers(n, min, max)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random integer within a range.
    #[pyo3(name = "integer", signature = (min = 0, max = 100))]
    fn py_integer(&mut self, min: i64, max: i64) -> PyResult<i64> {
        self.integer(min, max)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a batch of random UUIDs (version 4).
    #[pyo3(name = "uuids")]
    fn py_uuids(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.uuids(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random UUID (version 4).
    #[pyo3(name = "uuid")]
    fn py_uuid(&mut self) -> String {
        self.uuid()
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
        let faker = Faker::new("en_US").unwrap();
        assert_eq!(faker.locale(), "en_US");
    }

    #[test]
    fn test_faker_creation_invalid_locale() {
        let result = Faker::new("fr_FR");
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("unsupported locale"));
            assert!(err.to_string().contains("fr_FR"));
        }
    }

    #[test]
    fn test_faker_default() {
        let faker = Faker::new_default();
        assert_eq!(faker.locale(), "en_US");
    }

    #[test]
    fn test_seeding_determinism() {
        let mut faker1 = Faker::new("en_US").unwrap();
        let mut faker2 = Faker::new("en_US").unwrap();

        faker1.seed(42);
        faker2.seed(42);

        let names1 = faker1.names(10).unwrap();
        let names2 = faker2.names(10).unwrap();

        assert_eq!(names1, names2);
    }

    #[test]
    fn test_batch_generation() {
        let mut faker = Faker::new("en_US").unwrap();
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

    #[test]
    fn test_batch_size_error() {
        let mut faker = Faker::new_default();
        let result = faker.names(MAX_BATCH_SIZE + 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_locale() {
        assert!(validate_locale("en_US").is_ok());
        assert!(validate_locale("fr_FR").is_err());
        assert!(validate_locale("").is_err());
    }
}
