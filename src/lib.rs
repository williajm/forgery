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
pub mod error;
/// Data generation providers.
pub mod providers;
mod rng;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString, PyTuple};
use pyo3::IntoPyObjectExt;
use rng::ForgeryRng;
use std::collections::BTreeMap;

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

    // === Float Generation ===

    /// Generate a batch of random floats within a range.
    ///
    /// # Errors
    ///
    /// Returns an error if `min > max` or `n` exceeds the maximum batch size.
    pub fn floats(
        &mut self,
        n: usize,
        min: f64,
        max: f64,
    ) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        validate_batch_size(n)?;
        Ok(providers::numbers::generate_floats(
            &mut self.rng,
            n,
            min,
            max,
        )?)
    }

    /// Generate a single random float within a range.
    ///
    /// # Errors
    ///
    /// Returns an error if `min > max`.
    pub fn float(
        &mut self,
        min: f64,
        max: f64,
    ) -> Result<f64, providers::numbers::FloatRangeError> {
        providers::numbers::generate_float(&mut self.rng, min, max)
    }

    // === Hash Generation ===

    /// Generate a batch of random MD5 hashes.
    pub fn md5s(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::identifiers::generate_md5s(&mut self.rng, n))
    }

    /// Generate a single random MD5 hash.
    pub fn md5(&mut self) -> String {
        providers::identifiers::generate_md5(&mut self.rng)
    }

    /// Generate a batch of random SHA256 hashes.
    pub fn sha256s(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::identifiers::generate_sha256s(&mut self.rng, n))
    }

    /// Generate a single random SHA256 hash.
    pub fn sha256(&mut self) -> String {
        providers::identifiers::generate_sha256(&mut self.rng)
    }

    // === Color Generation ===

    /// Generate a batch of random color names.
    pub fn colors(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::colors::generate_colors(&mut self.rng, n))
    }

    /// Generate a single random color name.
    pub fn color(&mut self) -> String {
        providers::colors::generate_color(&mut self.rng)
    }

    /// Generate a batch of random hex colors.
    pub fn hex_colors(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::colors::generate_hex_colors(&mut self.rng, n))
    }

    /// Generate a single random hex color.
    pub fn hex_color(&mut self) -> String {
        providers::colors::generate_hex_color(&mut self.rng)
    }

    /// Generate a batch of random RGB color tuples.
    pub fn rgb_colors(&mut self, n: usize) -> Result<Vec<(u8, u8, u8)>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::colors::generate_rgb_colors(&mut self.rng, n))
    }

    /// Generate a single random RGB color tuple.
    pub fn rgb_color(&mut self) -> (u8, u8, u8) {
        providers::colors::generate_rgb_color(&mut self.rng)
    }

    // === DateTime Generation ===

    /// Generate a batch of random dates within a range.
    pub fn dates(
        &mut self,
        n: usize,
        start: &str,
        end: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        validate_batch_size(n)?;
        Ok(providers::datetime::generate_dates(
            &mut self.rng,
            n,
            start,
            end,
        )?)
    }

    /// Generate a single random date within a range.
    pub fn date(
        &mut self,
        start: &str,
        end: &str,
    ) -> Result<String, providers::datetime::DateRangeError> {
        providers::datetime::generate_date(&mut self.rng, start, end)
    }

    /// Generate a batch of random dates of birth.
    pub fn dates_of_birth(
        &mut self,
        n: usize,
        min_age: u32,
        max_age: u32,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        validate_batch_size(n)?;
        Ok(providers::datetime::generate_dates_of_birth(
            &mut self.rng,
            n,
            min_age,
            max_age,
        )?)
    }

    /// Generate a single random date of birth.
    pub fn date_of_birth(
        &mut self,
        min_age: u32,
        max_age: u32,
    ) -> Result<String, providers::datetime::DateRangeError> {
        providers::datetime::generate_date_of_birth(&mut self.rng, min_age, max_age)
    }

    /// Generate a batch of random datetimes within a range.
    pub fn datetimes(
        &mut self,
        n: usize,
        start: &str,
        end: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        validate_batch_size(n)?;
        Ok(providers::datetime::generate_datetimes(
            &mut self.rng,
            n,
            start,
            end,
        )?)
    }

    /// Generate a single random datetime within a range.
    pub fn datetime(
        &mut self,
        start: &str,
        end: &str,
    ) -> Result<String, providers::datetime::DateRangeError> {
        providers::datetime::generate_datetime(&mut self.rng, start, end)
    }

    // === Text Generation ===

    /// Generate a batch of random sentences.
    pub fn sentences(
        &mut self,
        n: usize,
        word_count: usize,
    ) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::text::generate_sentences(
            &mut self.rng,
            n,
            word_count,
        ))
    }

    /// Generate a single random sentence.
    pub fn sentence(&mut self, word_count: usize) -> String {
        providers::text::generate_sentence(&mut self.rng, word_count)
    }

    /// Generate a batch of random paragraphs.
    pub fn paragraphs(
        &mut self,
        n: usize,
        sentence_count: usize,
    ) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::text::generate_paragraphs(
            &mut self.rng,
            n,
            sentence_count,
        ))
    }

    /// Generate a single random paragraph.
    pub fn paragraph(&mut self, sentence_count: usize) -> String {
        providers::text::generate_paragraph(&mut self.rng, sentence_count)
    }

    /// Generate a batch of random text blocks.
    pub fn texts(
        &mut self,
        n: usize,
        min_chars: usize,
        max_chars: usize,
    ) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::text::generate_texts(
            &mut self.rng,
            n,
            min_chars,
            max_chars,
        ))
    }

    /// Generate a single random text block.
    pub fn text(&mut self, min_chars: usize, max_chars: usize) -> String {
        providers::text::generate_text(&mut self.rng, min_chars, max_chars)
    }

    // === Address Generation ===

    /// Generate a batch of random street addresses.
    pub fn street_addresses(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::address::generate_street_addresses(
            &mut self.rng,
            n,
        ))
    }

    /// Generate a single random street address.
    pub fn street_address(&mut self) -> String {
        providers::address::generate_street_address(&mut self.rng)
    }

    /// Generate a batch of random cities.
    pub fn cities(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::address::generate_cities(&mut self.rng, n))
    }

    /// Generate a single random city.
    pub fn city(&mut self) -> String {
        providers::address::generate_city(&mut self.rng)
    }

    /// Generate a batch of random states.
    pub fn states(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::address::generate_states(&mut self.rng, n))
    }

    /// Generate a single random state.
    pub fn state(&mut self) -> String {
        providers::address::generate_state(&mut self.rng)
    }

    /// Generate a batch of random countries.
    pub fn countries(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::address::generate_countries(&mut self.rng, n))
    }

    /// Generate a single random country.
    pub fn country(&mut self) -> String {
        providers::address::generate_country(&mut self.rng)
    }

    /// Generate a batch of random zip codes.
    pub fn zip_codes(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::address::generate_zip_codes(&mut self.rng, n))
    }

    /// Generate a single random zip code.
    pub fn zip_code(&mut self) -> String {
        providers::address::generate_zip_code(&mut self.rng)
    }

    /// Generate a batch of random full addresses.
    pub fn addresses(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::address::generate_addresses(&mut self.rng, n))
    }

    /// Generate a single random full address.
    pub fn address(&mut self) -> String {
        providers::address::generate_address(&mut self.rng)
    }

    // === Phone Generation ===

    /// Generate a batch of random phone numbers.
    pub fn phone_numbers(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::phone::generate_phone_numbers(&mut self.rng, n))
    }

    /// Generate a single random phone number.
    pub fn phone_number(&mut self) -> String {
        providers::phone::generate_phone_number(&mut self.rng)
    }

    // === Company Generation ===

    /// Generate a batch of random company names.
    pub fn companies(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::company::generate_companies(&mut self.rng, n))
    }

    /// Generate a single random company name.
    pub fn company(&mut self) -> String {
        providers::company::generate_company(&mut self.rng)
    }

    /// Generate a batch of random job titles.
    pub fn jobs(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::company::generate_jobs(&mut self.rng, n))
    }

    /// Generate a single random job title.
    pub fn job(&mut self) -> String {
        providers::company::generate_job(&mut self.rng)
    }

    /// Generate a batch of random catch phrases.
    pub fn catch_phrases(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::company::generate_catch_phrases(&mut self.rng, n))
    }

    /// Generate a single random catch phrase.
    pub fn catch_phrase(&mut self) -> String {
        providers::company::generate_catch_phrase(&mut self.rng)
    }

    // === Network Generation ===

    /// Generate a batch of random URLs.
    pub fn urls(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::network::generate_urls(&mut self.rng, n))
    }

    /// Generate a single random URL.
    pub fn url(&mut self) -> String {
        providers::network::generate_url(&mut self.rng)
    }

    /// Generate a batch of random domain names.
    pub fn domain_names(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::network::generate_domain_names(&mut self.rng, n))
    }

    /// Generate a single random domain name.
    pub fn domain_name(&mut self) -> String {
        providers::network::generate_domain_name(&mut self.rng)
    }

    /// Generate a batch of random IPv4 addresses.
    pub fn ipv4s(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::network::generate_ipv4s(&mut self.rng, n))
    }

    /// Generate a single random IPv4 address.
    pub fn ipv4(&mut self) -> String {
        providers::network::generate_ipv4(&mut self.rng)
    }

    /// Generate a batch of random IPv6 addresses.
    pub fn ipv6s(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::network::generate_ipv6s(&mut self.rng, n))
    }

    /// Generate a single random IPv6 address.
    pub fn ipv6(&mut self) -> String {
        providers::network::generate_ipv6(&mut self.rng)
    }

    /// Generate a batch of random MAC addresses.
    pub fn mac_addresses(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::network::generate_mac_addresses(&mut self.rng, n))
    }

    /// Generate a single random MAC address.
    pub fn mac_address(&mut self) -> String {
        providers::network::generate_mac_address(&mut self.rng)
    }

    // === Email Variants ===

    /// Generate a batch of random safe email addresses (example.com/org/net).
    pub fn safe_emails(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::internet::generate_safe_emails(&mut self.rng, n))
    }

    /// Generate a single random safe email address.
    pub fn safe_email(&mut self) -> String {
        providers::internet::generate_safe_email(&mut self.rng)
    }

    /// Generate a batch of random free email addresses (gmail.com, etc.).
    pub fn free_emails(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::internet::generate_free_emails(&mut self.rng, n))
    }

    /// Generate a single random free email address.
    pub fn free_email(&mut self) -> String {
        providers::internet::generate_free_email(&mut self.rng)
    }

    // === Finance Generation ===

    /// Generate a batch of random credit card numbers with valid Luhn checksums.
    pub fn credit_cards(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::finance::generate_credit_cards(&mut self.rng, n))
    }

    /// Generate a single random credit card number with valid Luhn checksum.
    pub fn credit_card(&mut self) -> String {
        providers::finance::generate_credit_card(&mut self.rng)
    }

    /// Generate a batch of random IBANs with valid checksums.
    pub fn ibans(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::finance::generate_ibans(&mut self.rng, n))
    }

    /// Generate a single random IBAN with valid checksum.
    pub fn iban(&mut self) -> String {
        providers::finance::generate_iban(&mut self.rng)
    }

    // === Records Generation ===

    /// Generate records based on a schema.
    ///
    /// Returns a vector of BTreeMaps, where each BTreeMap represents a record
    /// with field names as keys and generated values.
    ///
    /// # Errors
    ///
    /// Returns an error if the batch size exceeds the maximum or the schema is invalid.
    pub fn records(
        &mut self,
        n: usize,
        schema: &BTreeMap<String, providers::records::FieldSpec>,
    ) -> Result<Vec<BTreeMap<String, providers::records::Value>>, Box<dyn std::error::Error>> {
        validate_batch_size(n)?;
        Ok(providers::records::generate_records(
            &mut self.rng,
            n,
            schema,
        )?)
    }

    /// Generate records as tuples based on a schema.
    ///
    /// Returns a vector of vectors, where each inner vector contains values
    /// in the same order as the provided field order.
    ///
    /// # Errors
    ///
    /// Returns an error if the batch size exceeds the maximum or the schema is invalid.
    pub fn records_tuples(
        &mut self,
        n: usize,
        schema: &BTreeMap<String, providers::records::FieldSpec>,
        field_order: &[String],
    ) -> Result<Vec<Vec<providers::records::Value>>, Box<dyn std::error::Error>> {
        validate_batch_size(n)?;
        Ok(providers::records::generate_records_tuples(
            &mut self.rng,
            n,
            schema,
            field_order,
        )?)
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

    // === Float Generation ===

    /// Generate a batch of random floats within a range.
    #[pyo3(name = "floats", signature = (n, min = 0.0, max = 1.0))]
    fn py_floats(&mut self, n: usize, min: f64, max: f64) -> PyResult<Vec<f64>> {
        self.floats(n, min, max)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random float within a range.
    #[pyo3(name = "float", signature = (min = 0.0, max = 1.0))]
    fn py_float(&mut self, min: f64, max: f64) -> PyResult<f64> {
        self.float(min, max)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    // === Hash Generation ===

    /// Generate a batch of random MD5 hashes.
    #[pyo3(name = "md5s")]
    fn py_md5s(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.md5s(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random MD5 hash.
    #[pyo3(name = "md5")]
    fn py_md5(&mut self) -> String {
        self.md5()
    }

    /// Generate a batch of random SHA256 hashes.
    #[pyo3(name = "sha256s")]
    fn py_sha256s(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.sha256s(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random SHA256 hash.
    #[pyo3(name = "sha256")]
    fn py_sha256(&mut self) -> String {
        self.sha256()
    }

    // === Color Generation ===

    /// Generate a batch of random color names.
    #[pyo3(name = "colors")]
    fn py_colors(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.colors(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random color name.
    #[pyo3(name = "color")]
    fn py_color(&mut self) -> String {
        self.color()
    }

    /// Generate a batch of random hex colors.
    #[pyo3(name = "hex_colors")]
    fn py_hex_colors(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.hex_colors(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random hex color.
    #[pyo3(name = "hex_color")]
    fn py_hex_color(&mut self) -> String {
        self.hex_color()
    }

    /// Generate a batch of random RGB color tuples.
    #[pyo3(name = "rgb_colors")]
    fn py_rgb_colors(&mut self, n: usize) -> PyResult<Vec<(u8, u8, u8)>> {
        self.rgb_colors(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random RGB color tuple.
    #[pyo3(name = "rgb_color")]
    fn py_rgb_color(&mut self) -> (u8, u8, u8) {
        self.rgb_color()
    }

    // === DateTime Generation ===

    /// Generate a batch of random dates within a range.
    #[pyo3(name = "dates", signature = (n, start = "2000-01-01", end = "2030-12-31"))]
    fn py_dates(&mut self, n: usize, start: &str, end: &str) -> PyResult<Vec<String>> {
        self.dates(n, start, end)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random date within a range.
    #[pyo3(name = "date", signature = (start = "2000-01-01", end = "2030-12-31"))]
    fn py_date(&mut self, start: &str, end: &str) -> PyResult<String> {
        self.date(start, end)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a batch of random dates of birth.
    #[pyo3(name = "dates_of_birth", signature = (n, min_age = 18, max_age = 80))]
    fn py_dates_of_birth(&mut self, n: usize, min_age: u32, max_age: u32) -> PyResult<Vec<String>> {
        self.dates_of_birth(n, min_age, max_age)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random date of birth.
    #[pyo3(name = "date_of_birth", signature = (min_age = 18, max_age = 80))]
    fn py_date_of_birth(&mut self, min_age: u32, max_age: u32) -> PyResult<String> {
        self.date_of_birth(min_age, max_age)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a batch of random datetimes within a range.
    #[pyo3(name = "datetimes", signature = (n, start = "2000-01-01", end = "2030-12-31"))]
    fn py_datetimes(&mut self, n: usize, start: &str, end: &str) -> PyResult<Vec<String>> {
        self.datetimes(n, start, end)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random datetime within a range.
    #[pyo3(name = "datetime", signature = (start = "2000-01-01", end = "2030-12-31"))]
    fn py_datetime(&mut self, start: &str, end: &str) -> PyResult<String> {
        self.datetime(start, end)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    // === Text Generation ===

    /// Generate a batch of random sentences.
    #[pyo3(name = "sentences", signature = (n, word_count = 10))]
    fn py_sentences(&mut self, n: usize, word_count: usize) -> PyResult<Vec<String>> {
        self.sentences(n, word_count)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random sentence.
    #[pyo3(name = "sentence", signature = (word_count = 10))]
    fn py_sentence(&mut self, word_count: usize) -> String {
        self.sentence(word_count)
    }

    /// Generate a batch of random paragraphs.
    #[pyo3(name = "paragraphs", signature = (n, sentence_count = 5))]
    fn py_paragraphs(&mut self, n: usize, sentence_count: usize) -> PyResult<Vec<String>> {
        self.paragraphs(n, sentence_count)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random paragraph.
    #[pyo3(name = "paragraph", signature = (sentence_count = 5))]
    fn py_paragraph(&mut self, sentence_count: usize) -> String {
        self.paragraph(sentence_count)
    }

    /// Generate a batch of random text blocks.
    #[pyo3(name = "texts", signature = (n, min_chars = 50, max_chars = 200))]
    fn py_texts(&mut self, n: usize, min_chars: usize, max_chars: usize) -> PyResult<Vec<String>> {
        self.texts(n, min_chars, max_chars)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random text block.
    #[pyo3(name = "text", signature = (min_chars = 50, max_chars = 200))]
    fn py_text(&mut self, min_chars: usize, max_chars: usize) -> String {
        self.text(min_chars, max_chars)
    }

    // === Address Generation ===

    /// Generate a batch of random street addresses.
    #[pyo3(name = "street_addresses")]
    fn py_street_addresses(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.street_addresses(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random street address.
    #[pyo3(name = "street_address")]
    fn py_street_address(&mut self) -> String {
        self.street_address()
    }

    /// Generate a batch of random cities.
    #[pyo3(name = "cities")]
    fn py_cities(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.cities(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random city.
    #[pyo3(name = "city")]
    fn py_city(&mut self) -> String {
        self.city()
    }

    /// Generate a batch of random states.
    #[pyo3(name = "states")]
    fn py_states(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.states(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random state.
    #[pyo3(name = "state")]
    fn py_state(&mut self) -> String {
        self.state()
    }

    /// Generate a batch of random countries.
    #[pyo3(name = "countries")]
    fn py_countries(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.countries(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random country.
    #[pyo3(name = "country")]
    fn py_country(&mut self) -> String {
        self.country()
    }

    /// Generate a batch of random zip codes.
    #[pyo3(name = "zip_codes")]
    fn py_zip_codes(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.zip_codes(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random zip code.
    #[pyo3(name = "zip_code")]
    fn py_zip_code(&mut self) -> String {
        self.zip_code()
    }

    /// Generate a batch of random full addresses.
    #[pyo3(name = "addresses")]
    fn py_addresses(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.addresses(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random full address.
    #[pyo3(name = "address")]
    fn py_address(&mut self) -> String {
        self.address()
    }

    // === Phone Generation ===

    /// Generate a batch of random phone numbers.
    #[pyo3(name = "phone_numbers")]
    fn py_phone_numbers(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.phone_numbers(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random phone number.
    #[pyo3(name = "phone_number")]
    fn py_phone_number(&mut self) -> String {
        self.phone_number()
    }

    // === Company Generation ===

    /// Generate a batch of random company names.
    #[pyo3(name = "companies")]
    fn py_companies(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.companies(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random company name.
    #[pyo3(name = "company")]
    fn py_company(&mut self) -> String {
        self.company()
    }

    /// Generate a batch of random job titles.
    #[pyo3(name = "jobs")]
    fn py_jobs(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.jobs(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random job title.
    #[pyo3(name = "job")]
    fn py_job(&mut self) -> String {
        self.job()
    }

    /// Generate a batch of random catch phrases.
    #[pyo3(name = "catch_phrases")]
    fn py_catch_phrases(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.catch_phrases(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random catch phrase.
    #[pyo3(name = "catch_phrase")]
    fn py_catch_phrase(&mut self) -> String {
        self.catch_phrase()
    }

    // === Network Generation ===

    /// Generate a batch of random URLs.
    #[pyo3(name = "urls")]
    fn py_urls(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.urls(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random URL.
    #[pyo3(name = "url")]
    fn py_url(&mut self) -> String {
        self.url()
    }

    /// Generate a batch of random domain names.
    #[pyo3(name = "domain_names")]
    fn py_domain_names(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.domain_names(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random domain name.
    #[pyo3(name = "domain_name")]
    fn py_domain_name(&mut self) -> String {
        self.domain_name()
    }

    /// Generate a batch of random IPv4 addresses.
    #[pyo3(name = "ipv4s")]
    fn py_ipv4s(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.ipv4s(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random IPv4 address.
    #[pyo3(name = "ipv4")]
    fn py_ipv4(&mut self) -> String {
        self.ipv4()
    }

    /// Generate a batch of random IPv6 addresses.
    #[pyo3(name = "ipv6s")]
    fn py_ipv6s(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.ipv6s(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random IPv6 address.
    #[pyo3(name = "ipv6")]
    fn py_ipv6(&mut self) -> String {
        self.ipv6()
    }

    /// Generate a batch of random MAC addresses.
    #[pyo3(name = "mac_addresses")]
    fn py_mac_addresses(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.mac_addresses(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random MAC address.
    #[pyo3(name = "mac_address")]
    fn py_mac_address(&mut self) -> String {
        self.mac_address()
    }

    // === Email Variants ===

    /// Generate a batch of random safe email addresses (example.com/org/net).
    #[pyo3(name = "safe_emails")]
    fn py_safe_emails(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.safe_emails(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random safe email address.
    #[pyo3(name = "safe_email")]
    fn py_safe_email(&mut self) -> String {
        self.safe_email()
    }

    /// Generate a batch of random free email addresses (gmail.com, etc.).
    #[pyo3(name = "free_emails")]
    fn py_free_emails(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.free_emails(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random free email address.
    #[pyo3(name = "free_email")]
    fn py_free_email(&mut self) -> String {
        self.free_email()
    }

    // === Finance Generation ===

    /// Generate a batch of random credit card numbers with valid Luhn checksums.
    #[pyo3(name = "credit_cards")]
    fn py_credit_cards(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.credit_cards(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random credit card number with valid Luhn checksum.
    #[pyo3(name = "credit_card")]
    fn py_credit_card(&mut self) -> String {
        self.credit_card()
    }

    /// Generate a batch of random IBANs with valid checksums.
    #[pyo3(name = "ibans")]
    fn py_ibans(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.ibans(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random IBAN with valid checksum.
    #[pyo3(name = "iban")]
    fn py_iban(&mut self) -> String {
        self.iban()
    }

    // === Records Generation ===

    /// Generate records based on a schema.
    ///
    /// The schema is a dictionary mapping field names to type specifications:
    /// - Simple types: "name", "email", "uuid", "int", "float", etc.
    /// - Integer range: ("int", min, max)
    /// - Float range: ("float", min, max)
    /// - Text with limits: ("text", min_chars, max_chars)
    /// - Date range: ("date", start, end)
    /// - Choice: ("choice", ["option1", "option2", ...])
    #[pyo3(name = "records")]
    fn py_records(&mut self, n: usize, schema: &Bound<'_, PyDict>) -> PyResult<Vec<Py<PyAny>>> {
        let py = schema.py();
        let rust_schema = parse_py_schema(schema)?;
        validate_batch_size(n).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let records = providers::records::generate_records(&mut self.rng, n, &rust_schema)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        records
            .into_iter()
            .map(|record| {
                let dict = PyDict::new(py);
                for (key, value) in record {
                    dict.set_item(key, value_to_pyobject(py, value)?)?;
                }
                dict.into_py_any(py)
            })
            .collect()
    }

    /// Generate records as tuples based on a schema.
    ///
    /// Returns a list of tuples with values in alphabetical order of the schema keys.
    /// This is faster than records() since it avoids creating dictionaries.
    #[pyo3(name = "records_tuples")]
    fn py_records_tuples(
        &mut self,
        n: usize,
        schema: &Bound<'_, PyDict>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let py = schema.py();
        let rust_schema = parse_py_schema(schema)?;
        validate_batch_size(n).map_err(|e| PyValueError::new_err(e.to_string()))?;

        // Get field order from BTreeMap (sorted alphabetically)
        let field_order: Vec<String> = rust_schema.keys().cloned().collect();

        let records = providers::records::generate_records_tuples(
            &mut self.rng,
            n,
            &rust_schema,
            &field_order,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

        records
            .into_iter()
            .map(|record| {
                let values: Vec<Py<PyAny>> = record
                    .into_iter()
                    .map(|v| value_to_pyobject(py, v))
                    .collect::<PyResult<_>>()?;
                PyTuple::new(py, values)?.into_py_any(py)
            })
            .collect()
    }
}

/// Parse a Python schema dictionary into a Rust BTreeMap.
fn parse_py_schema(
    schema: &Bound<'_, PyDict>,
) -> PyResult<BTreeMap<String, providers::records::FieldSpec>> {
    let mut rust_schema = BTreeMap::new();

    for (key, value) in schema.iter() {
        let field_name: String = key.extract()?;
        let field_spec = parse_field_spec(&value)?;
        rust_schema.insert(field_name, field_spec);
    }

    Ok(rust_schema)
}

/// Parse a Python field specification into a Rust FieldSpec.
fn parse_field_spec(value: &Bound<'_, PyAny>) -> PyResult<providers::records::FieldSpec> {
    if value.is_instance_of::<PyString>() {
        return parse_string_field_spec(value);
    }
    if value.is_instance_of::<PyTuple>() {
        return parse_tuple_field_spec(value);
    }
    Err(PyValueError::new_err(
        "Field specification must be a string or tuple",
    ))
}

/// Parse a simple string type specification.
fn parse_string_field_spec(value: &Bound<'_, PyAny>) -> PyResult<providers::records::FieldSpec> {
    let type_str: String = value.extract()?;
    providers::records::parse_simple_type(&type_str)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Parse a tuple type specification like ("int", min, max).
fn parse_tuple_field_spec(value: &Bound<'_, PyAny>) -> PyResult<providers::records::FieldSpec> {
    let tuple: Vec<Bound<'_, PyAny>> = value.extract()?;
    if tuple.len() < 2 {
        return Err(PyValueError::new_err(
            "Tuple specification must have at least 2 elements",
        ));
    }

    let type_name: String = tuple[0].extract()?;
    match type_name.as_str() {
        "int" => parse_int_range(&tuple),
        "float" => parse_float_range(&tuple),
        "text" => parse_text_spec(&tuple),
        "date" => parse_date_range(&tuple),
        "choice" => parse_choice_spec(&tuple),
        _ => Err(PyValueError::new_err(format!(
            "Unknown parameterized type: {}",
            type_name
        ))),
    }
}

/// Parse an integer range specification: ("int", min, max).
fn parse_int_range(tuple: &[Bound<'_, PyAny>]) -> PyResult<providers::records::FieldSpec> {
    if tuple.len() != 3 {
        return Err(PyValueError::new_err(
            "int specification must be (\"int\", min, max)",
        ));
    }
    let min: i64 = tuple[1].extract()?;
    let max: i64 = tuple[2].extract()?;
    Ok(providers::records::FieldSpec::IntRange { min, max })
}

/// Parse a float range specification: ("float", min, max).
fn parse_float_range(tuple: &[Bound<'_, PyAny>]) -> PyResult<providers::records::FieldSpec> {
    if tuple.len() != 3 {
        return Err(PyValueError::new_err(
            "float specification must be (\"float\", min, max)",
        ));
    }
    let min: f64 = tuple[1].extract()?;
    let max: f64 = tuple[2].extract()?;
    Ok(providers::records::FieldSpec::FloatRange { min, max })
}

/// Parse a text specification: ("text", min_chars, max_chars).
fn parse_text_spec(tuple: &[Bound<'_, PyAny>]) -> PyResult<providers::records::FieldSpec> {
    if tuple.len() != 3 {
        return Err(PyValueError::new_err(
            "text specification must be (\"text\", min_chars, max_chars)",
        ));
    }
    let min_chars: usize = tuple[1].extract()?;
    let max_chars: usize = tuple[2].extract()?;
    if min_chars > max_chars {
        return Err(PyValueError::new_err(format!(
            "Invalid text range: min_chars ({}) > max_chars ({})",
            min_chars, max_chars
        )));
    }
    Ok(providers::records::FieldSpec::Text {
        min_chars,
        max_chars,
    })
}

/// Parse a date range specification: ("date", start, end).
fn parse_date_range(tuple: &[Bound<'_, PyAny>]) -> PyResult<providers::records::FieldSpec> {
    if tuple.len() != 3 {
        return Err(PyValueError::new_err(
            "date specification must be (\"date\", start, end)",
        ));
    }
    let start: String = tuple[1].extract()?;
    let end: String = tuple[2].extract()?;
    Ok(providers::records::FieldSpec::DateRange { start, end })
}

/// Parse a choice specification: ("choice", [options]).
fn parse_choice_spec(tuple: &[Bound<'_, PyAny>]) -> PyResult<providers::records::FieldSpec> {
    if tuple.len() != 2 {
        return Err(PyValueError::new_err(
            "choice specification must be (\"choice\", [options])",
        ));
    }
    if !tuple[1].is_instance_of::<PyList>() {
        return Err(PyValueError::new_err("choice options must be a list"));
    }
    let options: Vec<String> = tuple[1].extract()?;
    Ok(providers::records::FieldSpec::Choice(options))
}

/// Convert a Rust Value to a Python object.
fn value_to_pyobject(py: Python<'_>, value: providers::records::Value) -> PyResult<Py<PyAny>> {
    match value {
        providers::records::Value::String(s) => Ok(s.into_pyobject(py)?.into_any().unbind()),
        providers::records::Value::Int(i) => Ok(i.into_pyobject(py)?.into_any().unbind()),
        providers::records::Value::Float(f) => Ok(f.into_pyobject(py)?.into_any().unbind()),
        providers::records::Value::Tuple3U8(r, g, b) => {
            Ok(PyTuple::new(py, [r, g, b])?.into_any().unbind())
        }
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
