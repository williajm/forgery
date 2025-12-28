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

/// Embedded locale data for generation.
pub mod data;
pub mod error;
/// Locale definitions and errors.
pub mod locale;
/// Data generation providers.
pub mod providers;
mod rng;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString, PyTuple};
use pyo3::IntoPyObjectExt;
use pyo3_arrow::PyRecordBatch;
use rng::ForgeryRng;
use std::collections::{BTreeMap, HashMap, HashSet};

use error::{ForgeryError, UniqueExhaustedError};
use locale::{Locale, LocaleError};
use providers::custom::{is_reserved_name, CustomProvider, CustomProviderError};
use std::str::FromStr;

/// Maximum batch size to prevent memory exhaustion.
///
/// This limit is set to 10 million items, which should handle most
/// reasonable use cases while preventing accidental memory exhaustion.
pub const MAX_BATCH_SIZE: usize = 10_000_000;

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

/// Maximum schema size (number of fields) to prevent resource exhaustion.
///
/// This limit prevents DoS attacks via schemas with millions of columns.
pub const MAX_SCHEMA_SIZE: usize = 10_000;

/// Error type for schema size validation.
#[derive(Debug, Clone)]
pub struct SchemaSizeError {
    /// The requested schema size.
    pub requested: usize,
    /// The maximum allowed schema size.
    pub max: usize,
}

impl std::fmt::Display for SchemaSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "schema size {} exceeds maximum allowed size of {}",
            self.requested, self.max
        )
    }
}

impl std::error::Error for SchemaSizeError {}

/// Validate that a schema size is within acceptable limits.
///
/// # Errors
///
/// Returns `SchemaSizeError` if schema size exceeds `MAX_SCHEMA_SIZE`.
#[inline]
pub fn validate_schema_size(size: usize) -> Result<(), SchemaSizeError> {
    if size > MAX_SCHEMA_SIZE {
        return Err(SchemaSizeError {
            requested: size,
            max: MAX_SCHEMA_SIZE,
        });
    }
    Ok(())
}

/// Validate that a locale is supported and parse it.
///
/// # Errors
///
/// Returns `LocaleError` if the locale is not supported.
#[inline]
pub fn validate_locale(locale: &str) -> Result<Locale, LocaleError> {
    Locale::from_str(locale)
}

/// A fake data generator with its own random state.
///
/// Each instance maintains independent RNG state, allowing for deterministic
/// generation when seeded. The default locale is "en_US".
///
/// # Supported Locales
///
/// - `en_US` - English (United States)
/// - `de_DE` - German (Germany)
/// - `fr_FR` - French (France)
/// - `es_ES` - Spanish (Spain)
/// - `it_IT` - Italian (Italy)
/// - `ja_JP` - Japanese (Japan)
/// - `en_GB` - English (United Kingdom)
#[pyclass]
pub struct Faker {
    rng: ForgeryRng,
    locale: Locale,
    custom_providers: HashMap<String, CustomProvider>,
}

// Public Rust API - these methods are callable from Rust code (including benchmarks)
impl Faker {
    /// Create a new Faker instance with the specified locale.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale for generated data
    ///
    /// # Errors
    ///
    /// Returns `LocaleError` if the locale is not supported.
    ///
    /// # Supported Locales
    ///
    /// - `en_US` - English (United States)
    /// - `de_DE` - German (Germany)
    /// - `fr_FR` - French (France)
    /// - `es_ES` - Spanish (Spain)
    /// - `it_IT` - Italian (Italy)
    /// - `ja_JP` - Japanese (Japan)
    /// - `en_GB` - English (United Kingdom)
    pub fn new(locale: &str) -> Result<Self, LocaleError> {
        let parsed_locale = validate_locale(locale)?;
        Ok(Self {
            rng: ForgeryRng::new(),
            locale: parsed_locale,
            custom_providers: HashMap::new(),
        })
    }

    /// Create a new Faker instance with the default locale ("en_US").
    pub fn new_default() -> Self {
        Self {
            rng: ForgeryRng::new(),
            locale: Locale::default(),
            custom_providers: HashMap::new(),
        }
    }

    /// Get the locale for this Faker instance.
    pub fn locale(&self) -> &str {
        self.locale.as_str()
    }

    /// Get the locale enum for this Faker instance.
    pub fn locale_enum(&self) -> Locale {
        self.locale
    }

    /// Maximum attempts multiplier for unique generation.
    ///
    /// We try up to n * 100 attempts before giving up.
    const UNIQUE_ATTEMPTS_MULTIPLIER: usize = 100;

    /// Generate unique values using a generator function.
    ///
    /// This helper method generates unique values by tracking seen values
    /// in a HashSet and retrying until enough unique values are generated.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of unique values to generate
    /// * `generator` - A closure that generates a single value
    ///
    /// # Errors
    ///
    /// Returns `UniqueExhaustedError` if we cannot generate enough unique values
    /// within the maximum number of attempts.
    fn generate_unique<F>(&mut self, n: usize, generator: F) -> Result<Vec<String>, ForgeryError>
    where
        F: Fn(&mut ForgeryRng, Locale) -> String,
    {
        let mut seen = HashSet::with_capacity(n);
        let mut results = Vec::with_capacity(n);
        let max_attempts = n.saturating_mul(Self::UNIQUE_ATTEMPTS_MULTIPLIER);
        let mut attempts = 0;

        while results.len() < n {
            if attempts >= max_attempts {
                return Err(UniqueExhaustedError {
                    requested: n,
                    generated: results.len(),
                }
                .into());
            }
            let value = generator(&mut self.rng, self.locale);
            if seen.insert(value.clone()) {
                results.push(value);
            }
            attempts += 1;
        }
        Ok(results)
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
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn names(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::names::generate_name)
        } else {
            Ok(providers::names::generate_names(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a batch of random first names.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of names to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn first_names(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::names::generate_first_name)
        } else {
            Ok(providers::names::generate_first_names(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a batch of random last names.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of names to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn last_names(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::names::generate_last_name)
        } else {
            Ok(providers::names::generate_last_names(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random full name.
    pub fn name(&mut self) -> String {
        providers::names::generate_name(&mut self.rng, self.locale)
    }

    /// Generate a single random first name.
    pub fn first_name(&mut self) -> String {
        providers::names::generate_first_name(&mut self.rng, self.locale)
    }

    /// Generate a single random last name.
    pub fn last_name(&mut self) -> String {
        providers::names::generate_last_name(&mut self.rng, self.locale)
    }

    /// Generate a batch of random email addresses.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of emails to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn emails(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::internet::generate_email)
        } else {
            Ok(providers::internet::generate_emails(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random email address.
    pub fn email(&mut self) -> String {
        providers::internet::generate_email(&mut self.rng, self.locale)
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
    ///
    /// # Arguments
    ///
    /// * `n` - Number of colors to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn colors(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::colors::generate_color)
        } else {
            Ok(providers::colors::generate_colors(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random color name.
    pub fn color(&mut self) -> String {
        providers::colors::generate_color(&mut self.rng, self.locale)
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
            self.locale,
            n,
            word_count,
        ))
    }

    /// Generate a single random sentence.
    pub fn sentence(&mut self, word_count: usize) -> String {
        providers::text::generate_sentence(&mut self.rng, self.locale, word_count)
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
            self.locale,
            n,
            sentence_count,
        ))
    }

    /// Generate a single random paragraph.
    pub fn paragraph(&mut self, sentence_count: usize) -> String {
        providers::text::generate_paragraph(&mut self.rng, self.locale, sentence_count)
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
            self.locale,
            n,
            min_chars,
            max_chars,
        ))
    }

    /// Generate a single random text block.
    pub fn text(&mut self, min_chars: usize, max_chars: usize) -> String {
        providers::text::generate_text(&mut self.rng, self.locale, min_chars, max_chars)
    }

    // === Address Generation ===

    /// Generate a batch of random street addresses.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of addresses to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn street_addresses(
        &mut self,
        n: usize,
        unique: bool,
    ) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::address::generate_street_address)
        } else {
            Ok(providers::address::generate_street_addresses(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random street address.
    pub fn street_address(&mut self) -> String {
        providers::address::generate_street_address(&mut self.rng, self.locale)
    }

    /// Generate a batch of random cities.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of cities to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn cities(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::address::generate_city)
        } else {
            Ok(providers::address::generate_cities(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random city.
    pub fn city(&mut self) -> String {
        providers::address::generate_city(&mut self.rng, self.locale)
    }

    /// Generate a batch of random states.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of states to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn states(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::address::generate_state)
        } else {
            Ok(providers::address::generate_states(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random state.
    pub fn state(&mut self) -> String {
        providers::address::generate_state(&mut self.rng, self.locale)
    }

    /// Generate a batch of random countries.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of countries to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn countries(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            // countries generator doesn't take locale, need a wrapper
            self.generate_unique(n, |rng, _locale| providers::address::generate_country(rng))
        } else {
            Ok(providers::address::generate_countries(&mut self.rng, n))
        }
    }

    /// Generate a single random country.
    pub fn country(&mut self) -> String {
        providers::address::generate_country(&mut self.rng)
    }

    /// Generate a batch of random zip codes.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of zip codes to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn zip_codes(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::address::generate_zip_code)
        } else {
            Ok(providers::address::generate_zip_codes(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random zip code.
    pub fn zip_code(&mut self) -> String {
        providers::address::generate_zip_code(&mut self.rng, self.locale)
    }

    /// Generate a batch of random full addresses.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of addresses to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn addresses(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::address::generate_address)
        } else {
            Ok(providers::address::generate_addresses(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random full address.
    pub fn address(&mut self) -> String {
        providers::address::generate_address(&mut self.rng, self.locale)
    }

    // === Phone Generation ===

    /// Generate a batch of random phone numbers.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of phone numbers to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn phone_numbers(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::phone::generate_phone_number)
        } else {
            Ok(providers::phone::generate_phone_numbers(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random phone number.
    pub fn phone_number(&mut self) -> String {
        providers::phone::generate_phone_number(&mut self.rng, self.locale)
    }

    // === Company Generation ===

    /// Generate a batch of random company names.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of companies to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn companies(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::company::generate_company)
        } else {
            Ok(providers::company::generate_companies(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random company name.
    pub fn company(&mut self) -> String {
        providers::company::generate_company(&mut self.rng, self.locale)
    }

    /// Generate a batch of random job titles.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of job titles to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn jobs(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::company::generate_job)
        } else {
            Ok(providers::company::generate_jobs(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random job title.
    pub fn job(&mut self) -> String {
        providers::company::generate_job(&mut self.rng, self.locale)
    }

    /// Generate a batch of random catch phrases.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of catch phrases to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn catch_phrases(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::company::generate_catch_phrase)
        } else {
            Ok(providers::company::generate_catch_phrases(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random catch phrase.
    pub fn catch_phrase(&mut self) -> String {
        providers::company::generate_catch_phrase(&mut self.rng, self.locale)
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
    ///
    /// # Arguments
    ///
    /// * `n` - Number of emails to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn safe_emails(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::internet::generate_safe_email)
        } else {
            Ok(providers::internet::generate_safe_emails(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random safe email address.
    pub fn safe_email(&mut self) -> String {
        providers::internet::generate_safe_email(&mut self.rng, self.locale)
    }

    /// Generate a batch of random free email addresses (gmail.com, etc.).
    ///
    /// # Arguments
    ///
    /// * `n` - Number of emails to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn free_emails(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::internet::generate_free_email)
        } else {
            Ok(providers::internet::generate_free_emails(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random free email address.
    pub fn free_email(&mut self) -> String {
        providers::internet::generate_free_email(&mut self.rng, self.locale)
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

    /// Generate a batch of random BIC/SWIFT codes.
    pub fn bics(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::finance::generate_bics(&mut self.rng, n))
    }

    /// Generate a single random BIC/SWIFT code.
    pub fn bic(&mut self) -> String {
        providers::finance::generate_bic(&mut self.rng)
    }

    /// Generate a batch of random bank account numbers.
    pub fn bank_accounts(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::finance::generate_bank_accounts(&mut self.rng, n))
    }

    /// Generate a single random bank account number.
    pub fn bank_account(&mut self) -> String {
        providers::finance::generate_bank_account(&mut self.rng)
    }

    /// Generate a batch of random bank names.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of bank names to generate
    /// * `unique` - If true, ensure all generated values are unique
    ///
    /// # Errors
    ///
    /// Returns `ForgeryError` if `n` exceeds the maximum batch size or
    /// if unique generation cannot produce enough unique values.
    pub fn bank_names(&mut self, n: usize, unique: bool) -> Result<Vec<String>, ForgeryError> {
        validate_batch_size(n)?;
        if unique {
            self.generate_unique(n, providers::finance::generate_bank_name)
        } else {
            Ok(providers::finance::generate_bank_names(
                &mut self.rng,
                self.locale,
                n,
            ))
        }
    }

    /// Generate a single random bank name.
    pub fn bank_name(&mut self) -> String {
        providers::finance::generate_bank_name(&mut self.rng, self.locale)
    }

    /// Generate a batch of UK sort codes.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of sort codes to generate
    ///
    /// # Returns
    ///
    /// Sort codes in XX-XX-XX format (e.g., "12-34-56")
    pub fn sort_codes(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::finance::generate_sort_codes(&mut self.rng, n))
    }

    /// Generate a single UK sort code.
    ///
    /// # Returns
    ///
    /// A sort code in XX-XX-XX format (e.g., "12-34-56")
    pub fn sort_code(&mut self) -> String {
        providers::finance::generate_sort_code(&mut self.rng)
    }

    /// Generate a batch of UK bank account numbers.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of account numbers to generate
    ///
    /// # Returns
    ///
    /// Account numbers as exactly 8 digits (UK standard format)
    pub fn uk_account_numbers(&mut self, n: usize) -> Result<Vec<String>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::finance::generate_uk_account_numbers(
            &mut self.rng,
            n,
        ))
    }

    /// Generate a single UK bank account number.
    ///
    /// # Returns
    ///
    /// An account number as exactly 8 digits (UK standard format)
    pub fn uk_account_number(&mut self) -> String {
        providers::finance::generate_uk_account_number(&mut self.rng)
    }

    /// Generate a batch of financial transactions.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of transactions to generate
    /// * `starting_balance` - The opening balance before the first transaction
    /// * `start_date` - Start date in YYYY-MM-DD format
    /// * `end_date` - End date in YYYY-MM-DD format
    ///
    /// # Returns
    ///
    /// A list of transactions, each containing reference, date, amount,
    /// transaction_type, description, and running balance. Transactions
    /// are sorted chronologically.
    ///
    /// # Errors
    ///
    /// Returns an error if batch size exceeds the limit or if the date range is invalid.
    pub fn transactions(
        &mut self,
        n: usize,
        starting_balance: f64,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<providers::finance::Transaction>, error::ForgeryError> {
        validate_batch_size(n)?;
        Ok(providers::finance::generate_transactions(
            &mut self.rng,
            n,
            starting_balance,
            start_date,
            end_date,
        )?)
    }

    /// Generate a batch of transaction amounts.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of amounts to generate
    /// * `min` - Minimum amount (inclusive)
    /// * `max` - Maximum amount (inclusive)
    ///
    /// # Returns
    ///
    /// Transaction amounts rounded to 2 decimal places
    pub fn transaction_amounts(
        &mut self,
        n: usize,
        min: f64,
        max: f64,
    ) -> Result<Vec<f64>, BatchSizeError> {
        validate_batch_size(n)?;
        Ok(providers::finance::generate_transaction_amounts(
            &mut self.rng,
            n,
            min,
            max,
        ))
    }

    /// Generate a single transaction amount.
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum amount (inclusive)
    /// * `max` - Maximum amount (inclusive)
    ///
    /// # Returns
    ///
    /// A transaction amount rounded to 2 decimal places
    pub fn transaction_amount(&mut self, min: f64, max: f64) -> f64 {
        providers::finance::generate_transaction_amount(&mut self.rng, min, max)
    }

    // === Password Generation ===

    /// Generate a batch of random passwords.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of passwords to generate
    /// * `length` - Length of each password
    /// * `uppercase` - Include uppercase letters
    /// * `lowercase` - Include lowercase letters
    /// * `digits` - Include digits
    /// * `symbols` - Include symbols
    ///
    /// # Errors
    ///
    /// Returns an error if no character sets are enabled or batch size exceeds maximum.
    pub fn passwords(
        &mut self,
        n: usize,
        length: usize,
        uppercase: bool,
        lowercase: bool,
        digits: bool,
        symbols: bool,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        validate_batch_size(n)?;
        Ok(providers::password::generate_passwords(
            &mut self.rng,
            n,
            length,
            uppercase,
            lowercase,
            digits,
            symbols,
        )?)
    }

    /// Generate a single random password.
    ///
    /// # Arguments
    ///
    /// * `length` - Length of the password
    /// * `uppercase` - Include uppercase letters
    /// * `lowercase` - Include lowercase letters
    /// * `digits` - Include digits
    /// * `symbols` - Include symbols
    ///
    /// # Errors
    ///
    /// Returns an error if no character sets are enabled.
    pub fn password(
        &mut self,
        length: usize,
        uppercase: bool,
        lowercase: bool,
        digits: bool,
        symbols: bool,
    ) -> Result<String, providers::password::PasswordError> {
        providers::password::generate_password(
            &mut self.rng,
            length,
            uppercase,
            lowercase,
            digits,
            symbols,
        )
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
        Ok(providers::records::generate_records_with_custom(
            &mut self.rng,
            self.locale,
            n,
            schema,
            &self.custom_providers,
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
        Ok(providers::records::generate_records_tuples_with_custom(
            &mut self.rng,
            self.locale,
            n,
            schema,
            field_order,
            &self.custom_providers,
        )?)
    }

    /// Generate records as an Arrow RecordBatch based on a schema.
    ///
    /// # Errors
    ///
    /// Returns an error if the batch size exceeds the maximum or the schema is invalid.
    pub fn records_arrow(
        &mut self,
        n: usize,
        schema: &BTreeMap<String, providers::records::FieldSpec>,
    ) -> Result<arrow_array::RecordBatch, Box<dyn std::error::Error>> {
        validate_batch_size(n)?;
        Ok(providers::records::generate_records_arrow_with_custom(
            &mut self.rng,
            self.locale,
            n,
            schema,
            &self.custom_providers,
        )?)
    }

    // === Custom Providers ===

    /// Register a custom provider with uniform random selection.
    ///
    /// Each option has equal probability of being selected.
    ///
    /// # Arguments
    ///
    /// * `name` - The provider name (must not conflict with built-in types)
    /// * `options` - List of string options to choose from
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The name conflicts with a built-in type
    /// - The options list is empty
    pub fn add_provider(
        &mut self,
        name: &str,
        options: Vec<String>,
    ) -> Result<(), CustomProviderError> {
        if is_reserved_name(name) {
            return Err(CustomProviderError::NameCollision(name.to_string()));
        }
        let provider = CustomProvider::uniform(options)?;
        self.custom_providers.insert(name.to_string(), provider);
        Ok(())
    }

    /// Register a custom provider with weighted random selection.
    ///
    /// Options are selected based on their relative weights.
    ///
    /// # Arguments
    ///
    /// * `name` - The provider name (must not conflict with built-in types)
    /// * `pairs` - List of (value, weight) tuples
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The name conflicts with a built-in type
    /// - The options list is empty
    /// - All weights are zero
    pub fn add_weighted_provider(
        &mut self,
        name: &str,
        pairs: Vec<(String, u64)>,
    ) -> Result<(), CustomProviderError> {
        if is_reserved_name(name) {
            return Err(CustomProviderError::NameCollision(name.to_string()));
        }
        let provider = CustomProvider::weighted(pairs)?;
        self.custom_providers.insert(name.to_string(), provider);
        Ok(())
    }

    /// Remove a custom provider.
    ///
    /// # Arguments
    ///
    /// * `name` - The provider name to remove
    ///
    /// # Returns
    ///
    /// `true` if the provider was removed, `false` if it didn't exist.
    pub fn remove_provider(&mut self, name: &str) -> bool {
        self.custom_providers.remove(name).is_some()
    }

    /// Check if a custom provider exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The provider name to check
    ///
    /// # Returns
    ///
    /// `true` if the provider exists, `false` otherwise.
    pub fn has_provider(&self, name: &str) -> bool {
        self.custom_providers.contains_key(name)
    }

    /// List all registered custom provider names.
    ///
    /// # Returns
    ///
    /// A sorted vector of provider names (for deterministic output).
    pub fn list_providers(&self) -> Vec<String> {
        let mut names: Vec<String> = self.custom_providers.keys().cloned().collect();
        names.sort();
        names
    }

    /// Get a set of all custom provider names.
    ///
    /// Used internally for schema validation.
    pub fn custom_provider_names(&self) -> HashSet<String> {
        self.custom_providers.keys().cloned().collect()
    }

    /// Generate a single value from a custom provider.
    ///
    /// # Arguments
    ///
    /// * `name` - The custom provider name
    ///
    /// # Errors
    ///
    /// Returns an error if the provider doesn't exist.
    pub fn generate(&mut self, name: &str) -> Result<String, CustomProviderError> {
        let provider = self
            .custom_providers
            .get(name)
            .ok_or_else(|| CustomProviderError::NotFound(name.to_string()))?;
        Ok(provider.generate(&mut self.rng))
    }

    /// Generate a batch of values from a custom provider.
    ///
    /// # Arguments
    ///
    /// * `name` - The custom provider name
    /// * `n` - Number of values to generate
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The provider doesn't exist
    /// - `n` exceeds the maximum batch size
    pub fn generate_batch(
        &mut self,
        name: &str,
        n: usize,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        validate_batch_size(n)?;
        let provider = self
            .custom_providers
            .get(name)
            .ok_or_else(|| CustomProviderError::NotFound(name.to_string()))?;
        Ok(provider.generate_batch(&mut self.rng, n))
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
    #[pyo3(name = "names", signature = (n, unique=false))]
    fn py_names(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.names(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a batch of random first names.
    #[pyo3(name = "first_names", signature = (n, unique=false))]
    fn py_first_names(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.first_names(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a batch of random last names.
    #[pyo3(name = "last_names", signature = (n, unique=false))]
    fn py_last_names(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.last_names(n, unique)
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
    #[pyo3(name = "emails", signature = (n, unique=false))]
    fn py_emails(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.emails(n, unique)
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
    #[pyo3(name = "colors", signature = (n, unique=false))]
    fn py_colors(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.colors(n, unique)
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
    #[pyo3(name = "street_addresses", signature = (n, unique=false))]
    fn py_street_addresses(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.street_addresses(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random street address.
    #[pyo3(name = "street_address")]
    fn py_street_address(&mut self) -> String {
        self.street_address()
    }

    /// Generate a batch of random cities.
    #[pyo3(name = "cities", signature = (n, unique=false))]
    fn py_cities(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.cities(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random city.
    #[pyo3(name = "city")]
    fn py_city(&mut self) -> String {
        self.city()
    }

    /// Generate a batch of random states.
    #[pyo3(name = "states", signature = (n, unique=false))]
    fn py_states(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.states(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random state.
    #[pyo3(name = "state")]
    fn py_state(&mut self) -> String {
        self.state()
    }

    /// Generate a batch of random countries.
    #[pyo3(name = "countries", signature = (n, unique=false))]
    fn py_countries(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.countries(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random country.
    #[pyo3(name = "country")]
    fn py_country(&mut self) -> String {
        self.country()
    }

    /// Generate a batch of random zip codes.
    #[pyo3(name = "zip_codes", signature = (n, unique=false))]
    fn py_zip_codes(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.zip_codes(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random zip code.
    #[pyo3(name = "zip_code")]
    fn py_zip_code(&mut self) -> String {
        self.zip_code()
    }

    /// Generate a batch of random full addresses.
    #[pyo3(name = "addresses", signature = (n, unique=false))]
    fn py_addresses(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.addresses(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random full address.
    #[pyo3(name = "address")]
    fn py_address(&mut self) -> String {
        self.address()
    }

    // === Phone Generation ===

    /// Generate a batch of random phone numbers.
    #[pyo3(name = "phone_numbers", signature = (n, unique=false))]
    fn py_phone_numbers(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.phone_numbers(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random phone number.
    #[pyo3(name = "phone_number")]
    fn py_phone_number(&mut self) -> String {
        self.phone_number()
    }

    // === Company Generation ===

    /// Generate a batch of random company names.
    #[pyo3(name = "companies", signature = (n, unique=false))]
    fn py_companies(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.companies(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random company name.
    #[pyo3(name = "company")]
    fn py_company(&mut self) -> String {
        self.company()
    }

    /// Generate a batch of random job titles.
    #[pyo3(name = "jobs", signature = (n, unique=false))]
    fn py_jobs(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.jobs(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random job title.
    #[pyo3(name = "job")]
    fn py_job(&mut self) -> String {
        self.job()
    }

    /// Generate a batch of random catch phrases.
    #[pyo3(name = "catch_phrases", signature = (n, unique=false))]
    fn py_catch_phrases(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.catch_phrases(n, unique)
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
    #[pyo3(name = "safe_emails", signature = (n, unique=false))]
    fn py_safe_emails(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.safe_emails(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random safe email address.
    #[pyo3(name = "safe_email")]
    fn py_safe_email(&mut self) -> String {
        self.safe_email()
    }

    /// Generate a batch of random free email addresses (gmail.com, etc.).
    #[pyo3(name = "free_emails", signature = (n, unique=false))]
    fn py_free_emails(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.free_emails(n, unique)
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

    /// Generate a batch of random BIC/SWIFT codes.
    #[pyo3(name = "bics")]
    fn py_bics(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.bics(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random BIC/SWIFT code.
    #[pyo3(name = "bic")]
    fn py_bic(&mut self) -> String {
        self.bic()
    }

    /// Generate a batch of random bank account numbers.
    #[pyo3(name = "bank_accounts")]
    fn py_bank_accounts(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.bank_accounts(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random bank account number.
    #[pyo3(name = "bank_account")]
    fn py_bank_account(&mut self) -> String {
        self.bank_account()
    }

    /// Generate a batch of random bank names.
    #[pyo3(name = "bank_names", signature = (n, unique=false))]
    fn py_bank_names(&mut self, n: usize, unique: bool) -> PyResult<Vec<String>> {
        self.bank_names(n, unique)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random bank name.
    #[pyo3(name = "bank_name")]
    fn py_bank_name(&mut self) -> String {
        self.bank_name()
    }

    /// Generate a batch of UK sort codes.
    #[pyo3(name = "sort_codes")]
    fn py_sort_codes(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.sort_codes(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single UK sort code (format: XX-XX-XX).
    #[pyo3(name = "sort_code")]
    fn py_sort_code(&mut self) -> String {
        self.sort_code()
    }

    /// Generate a batch of UK bank account numbers (8 digits).
    #[pyo3(name = "uk_account_numbers")]
    fn py_uk_account_numbers(&mut self, n: usize) -> PyResult<Vec<String>> {
        self.uk_account_numbers(n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single UK bank account number (8 digits).
    #[pyo3(name = "uk_account_number")]
    fn py_uk_account_number(&mut self) -> String {
        self.uk_account_number()
    }

    /// Generate a batch of financial transactions.
    ///
    /// Args:
    ///     n: Number of transactions to generate
    ///     starting_balance: Opening balance before first transaction
    ///     start_date: Start date in YYYY-MM-DD format
    ///     end_date: End date in YYYY-MM-DD format
    ///
    /// Returns:
    ///     List of transaction dicts with keys: reference, date, amount,
    ///     transaction_type, description, balance
    #[pyo3(name = "transactions")]
    fn py_transactions(
        &mut self,
        py: Python<'_>,
        n: usize,
        starting_balance: f64,
        start_date: &str,
        end_date: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let txns = self
            .transactions(n, starting_balance, start_date, end_date)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        txns.into_iter()
            .map(|t| {
                let dict = PyDict::new(py);
                dict.set_item("reference", &t.reference)?;
                dict.set_item("date", &t.date)?;
                dict.set_item("amount", t.amount)?;
                dict.set_item("transaction_type", &t.transaction_type)?;
                dict.set_item("description", &t.description)?;
                dict.set_item("balance", t.balance)?;
                dict.into_py_any(py)
            })
            .collect()
    }

    /// Generate a batch of transaction amounts.
    #[pyo3(name = "transaction_amounts")]
    fn py_transaction_amounts(&mut self, n: usize, min: f64, max: f64) -> PyResult<Vec<f64>> {
        self.transaction_amounts(n, min, max)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single transaction amount.
    #[pyo3(name = "transaction_amount")]
    fn py_transaction_amount(&mut self, min: f64, max: f64) -> f64 {
        self.transaction_amount(min, max)
    }

    // === Password Generation ===

    /// Generate a batch of random passwords.
    ///
    /// Args:
    ///     n: Number of passwords to generate
    ///     length: Length of each password (default: 12)
    ///     uppercase: Include uppercase letters (default: True)
    ///     lowercase: Include lowercase letters (default: True)
    ///     digits: Include digits (default: True)
    ///     symbols: Include symbols (default: True)
    ///
    /// Returns:
    ///     List of random passwords
    ///
    /// Raises:
    ///     ValueError: If no character sets are enabled or batch size exceeds limit
    #[pyo3(name = "passwords", signature = (n, length=12, uppercase=true, lowercase=true, digits=true, symbols=true))]
    fn py_passwords(
        &mut self,
        n: usize,
        length: usize,
        uppercase: bool,
        lowercase: bool,
        digits: bool,
        symbols: bool,
    ) -> PyResult<Vec<String>> {
        self.passwords(n, length, uppercase, lowercase, digits, symbols)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a single random password.
    ///
    /// Args:
    ///     length: Length of the password (default: 12)
    ///     uppercase: Include uppercase letters (default: True)
    ///     lowercase: Include lowercase letters (default: True)
    ///     digits: Include digits (default: True)
    ///     symbols: Include symbols (default: True)
    ///
    /// Returns:
    ///     A random password
    ///
    /// Raises:
    ///     ValueError: If no character sets are enabled
    #[pyo3(name = "password", signature = (length=12, uppercase=true, lowercase=true, digits=true, symbols=true))]
    fn py_password(
        &mut self,
        length: usize,
        uppercase: bool,
        lowercase: bool,
        digits: bool,
        symbols: bool,
    ) -> PyResult<String> {
        self.password(length, uppercase, lowercase, digits, symbols)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    // === Custom Providers ===

    /// Register a custom provider with uniform random selection.
    ///
    /// Args:
    ///     name: The provider name (must not conflict with built-in types)
    ///     options: List of string options to choose from
    ///
    /// Raises:
    ///     ValueError: If name conflicts with built-in type or options is empty
    ///
    /// Example:
    ///     >>> fake = Faker()
    ///     >>> fake.add_provider("department", ["Engineering", "Sales", "HR"])
    ///     >>> fake.generate("department")
    ///     'Sales'
    #[pyo3(name = "add_provider")]
    fn py_add_provider(&mut self, name: &str, options: Vec<String>) -> PyResult<()> {
        self.add_provider(name, options)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Register a custom provider with weighted random selection.
    ///
    /// Args:
    ///     name: The provider name
    ///     weighted_options: List of (value, weight) tuples. Higher weights = more likely.
    ///
    /// Raises:
    ///     ValueError: If name conflicts, options empty, or weights invalid
    ///
    /// Example:
    ///     >>> fake = Faker()
    ///     >>> fake.add_weighted_provider("status", [("active", 80), ("inactive", 20)])
    ///     >>> fake.generate("status")  # ~80% chance of "active"
    ///     'active'
    #[pyo3(name = "add_weighted_provider")]
    fn py_add_weighted_provider(
        &mut self,
        name: &str,
        weighted_options: Vec<(String, u64)>,
    ) -> PyResult<()> {
        self.add_weighted_provider(name, weighted_options)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Remove a custom provider.
    ///
    /// Args:
    ///     name: The provider name to remove
    ///
    /// Returns:
    ///     True if provider was removed, False if it didn't exist
    #[pyo3(name = "remove_provider")]
    fn py_remove_provider(&mut self, name: &str) -> bool {
        self.remove_provider(name)
    }

    /// Check if a custom provider exists.
    ///
    /// Args:
    ///     name: The provider name to check
    ///
    /// Returns:
    ///     True if provider exists, False otherwise
    #[pyo3(name = "has_provider")]
    fn py_has_provider(&self, name: &str) -> bool {
        self.has_provider(name)
    }

    /// List all registered custom provider names.
    ///
    /// Returns:
    ///     List of registered custom provider names
    #[pyo3(name = "list_providers")]
    fn py_list_providers(&self) -> Vec<String> {
        self.list_providers()
    }

    /// Generate a single value from a custom provider.
    ///
    /// Args:
    ///     name: The custom provider name
    ///
    /// Returns:
    ///     A randomly selected string from the provider's options
    ///
    /// Raises:
    ///     ValueError: If provider doesn't exist
    #[pyo3(name = "generate")]
    fn py_generate(&mut self, name: &str) -> PyResult<String> {
        self.generate(name)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Generate a batch of values from a custom provider.
    ///
    /// Args:
    ///     name: The custom provider name
    ///     n: Number of values to generate
    ///
    /// Returns:
    ///     A list of randomly selected strings
    ///
    /// Raises:
    ///     ValueError: If provider doesn't exist or n exceeds batch limit
    #[pyo3(name = "generate_batch")]
    fn py_generate_batch(&mut self, name: &str, n: usize) -> PyResult<Vec<String>> {
        self.generate_batch(name, n)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    // === Records Generation ===

    /// Generate records based on a schema.
    ///
    /// The schema is a dictionary mapping field names to type specifications:
    /// - Simple types: "name", "email", "uuid", "int", "float", etc.
    /// - Custom providers: Any registered custom provider name
    /// - Integer range: ("int", min, max)
    /// - Float range: ("float", min, max)
    /// - Text with limits: ("text", min_chars, max_chars)
    /// - Date range: ("date", start, end)
    /// - Choice: ("choice", ["option1", "option2", ...])
    #[pyo3(name = "records")]
    fn py_records(&mut self, n: usize, schema: &Bound<'_, PyDict>) -> PyResult<Vec<Py<PyAny>>> {
        let py = schema.py();
        let custom_names = self.custom_provider_names();
        let rust_schema = parse_py_schema_with_custom(schema, &custom_names)?;
        validate_batch_size(n).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let records = providers::records::generate_records_with_custom(
            &mut self.rng,
            self.locale,
            n,
            &rust_schema,
            &self.custom_providers,
        )
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
        let custom_names = self.custom_provider_names();
        let rust_schema = parse_py_schema_with_custom(schema, &custom_names)?;
        validate_batch_size(n).map_err(|e| PyValueError::new_err(e.to_string()))?;

        // Get field order from BTreeMap (sorted alphabetically)
        let field_order: Vec<String> = rust_schema.keys().cloned().collect();

        let records = providers::records::generate_records_tuples_with_custom(
            &mut self.rng,
            self.locale,
            n,
            &rust_schema,
            &field_order,
            &self.custom_providers,
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

    /// Generate records as a PyArrow RecordBatch.
    ///
    /// This is the high-performance path for generating structured data,
    /// suitable for use with PyArrow, Polars, and other Arrow-compatible tools.
    ///
    /// The data is generated in columnar format and returned as a PyArrow RecordBatch,
    /// which can be used directly with pandas, polars, or other data processing tools.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of records to generate
    /// * `schema` - Schema dictionary mapping field names to type specifications
    ///
    /// # Returns
    ///
    /// A PyArrow RecordBatch with the generated data.
    ///
    /// # Example
    ///
    /// ```python
    /// import pyarrow as pa
    /// from forgery import Faker
    ///
    /// fake = Faker()
    /// batch = fake.records_arrow(1000, {
    ///     "id": "uuid",
    ///     "name": "name",
    ///     "age": ("int", 18, 65),
    ///     "salary": ("float", 30000.0, 150000.0),
    /// })
    /// # batch is a pyarrow.RecordBatch
    /// df = batch.to_pandas()  # Convert to pandas DataFrame
    /// ```
    #[pyo3(name = "records_arrow")]
    fn py_records_arrow(
        &mut self,
        py: Python<'_>,
        n: usize,
        schema: &Bound<'_, PyDict>,
    ) -> PyResult<Py<PyAny>> {
        let custom_names = self.custom_provider_names();
        let rust_schema = parse_py_schema_with_custom(schema, &custom_names)?;
        validate_batch_size(n).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let record_batch = providers::records::generate_records_arrow_with_custom(
            &mut self.rng,
            self.locale,
            n,
            &rust_schema,
            &self.custom_providers,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

        // Convert to PyArrow RecordBatch via pyo3-arrow
        let py_batch = PyRecordBatch::new(record_batch);
        py_batch.into_pyarrow(py).map(|bound| bound.unbind())
    }

    // ============================================================================
    // Async Methods
    // ============================================================================

    /// Generate records asynchronously for non-blocking batch generation.
    ///
    /// This method generates records in chunks, yielding control between chunks
    /// to allow other async tasks to run. Ideal for generating millions of records
    /// without blocking the event loop.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of records to generate
    /// * `schema` - Schema dictionary mapping field names to type specifications
    /// * `chunk_size` - Optional number of records per chunk (default: 10,000)
    ///
    /// # Returns
    ///
    /// A coroutine that resolves to a list of dictionaries.
    ///
    /// # Note on RNG State
    ///
    /// The async methods use a snapshot of the RNG state at call time. The main
    /// Faker instance's RNG is not advanced. For different results on each call,
    /// create separate Faker instances or re-seed between calls.
    ///
    /// # Example
    ///
    /// ```python
    /// import asyncio
    /// from forgery import Faker
    ///
    /// async def main():
    ///     fake = Faker()
    ///     records = await fake.records_async(1_000_000, {
    ///         "name": "name",
    ///         "email": "email",
    ///     })
    ///     print(f"Generated {len(records)} records")
    ///
    /// asyncio.run(main())
    /// ```
    #[pyo3(name = "records_async", signature = (n, schema, chunk_size = None))]
    fn py_records_async<'py>(
        &self,
        py: Python<'py>,
        n: usize,
        schema: &Bound<'py, PyDict>,
        chunk_size: Option<usize>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use pyo3_async_runtimes::tokio::future_into_py;

        let mut state = self.prepare_async_state(n, schema, chunk_size)?;

        future_into_py(py, async move {
            let records = providers::async_records::generate_records_async(
                &mut state.rng,
                state.locale,
                n,
                &state.schema,
                state.chunk_size,
                &state.custom_providers,
            )
            .await
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

            // Convert to Python objects
            Python::attach(|py| {
                let py_records: PyResult<Vec<Py<PyAny>>> = records
                    .into_iter()
                    .map(|record| {
                        let dict = PyDict::new(py);
                        for (key, value) in record {
                            dict.set_item(key, value_to_pyobject(py, value)?)?;
                        }
                        dict.into_py_any(py)
                    })
                    .collect();
                py_records
            })
        })
    }

    /// Generate records as tuples asynchronously for non-blocking batch generation.
    ///
    /// Similar to records_async() but returns tuples instead of dictionaries,
    /// which is faster for large datasets.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of records to generate
    /// * `schema` - Schema dictionary mapping field names to type specifications
    /// * `chunk_size` - Optional number of records per chunk (default: 10,000)
    ///
    /// # Returns
    ///
    /// A coroutine that resolves to a list of tuples (values in alphabetical field order).
    #[pyo3(name = "records_tuples_async", signature = (n, schema, chunk_size = None))]
    fn py_records_tuples_async<'py>(
        &self,
        py: Python<'py>,
        n: usize,
        schema: &Bound<'py, PyDict>,
        chunk_size: Option<usize>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use pyo3_async_runtimes::tokio::future_into_py;

        let mut state = self.prepare_async_state(n, schema, chunk_size)?;
        let field_order: Vec<String> = state.schema.keys().cloned().collect();

        future_into_py(py, async move {
            let records = providers::async_records::generate_records_tuples_async(
                &mut state.rng,
                state.locale,
                n,
                &state.schema,
                &field_order,
                state.chunk_size,
                &state.custom_providers,
            )
            .await
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

            // Convert to Python objects
            Python::attach(|py| {
                let py_records: PyResult<Vec<Py<PyAny>>> = records
                    .into_iter()
                    .map(|record| {
                        let values: Vec<Py<PyAny>> = record
                            .into_iter()
                            .map(|v| value_to_pyobject(py, v))
                            .collect::<PyResult<_>>()?;
                        PyTuple::new(py, values)?.into_py_any(py)
                    })
                    .collect();
                py_records
            })
        })
    }

    /// Generate records as an Arrow RecordBatch asynchronously.
    ///
    /// This is the high-performance async path for generating structured data.
    /// Generates data in chunks and concatenates them into a single RecordBatch.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of records to generate
    /// * `schema` - Schema dictionary mapping field names to type specifications
    /// * `chunk_size` - Optional number of records per chunk (default: 10,000)
    ///
    /// # Returns
    ///
    /// A coroutine that resolves to a PyArrow RecordBatch.
    #[pyo3(name = "records_arrow_async", signature = (n, schema, chunk_size = None))]
    fn py_records_arrow_async<'py>(
        &self,
        py: Python<'py>,
        n: usize,
        schema: &Bound<'py, PyDict>,
        chunk_size: Option<usize>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use pyo3_async_runtimes::tokio::future_into_py;

        let mut state = self.prepare_async_state(n, schema, chunk_size)?;

        future_into_py(py, async move {
            let record_batch = providers::async_records::generate_records_arrow_async(
                &mut state.rng,
                state.locale,
                n,
                &state.schema,
                state.chunk_size,
                &state.custom_providers,
            )
            .await
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

            // Convert to PyArrow RecordBatch
            // Preserve original error type (e.g., ImportError if pyarrow missing)
            Python::attach(|py| {
                let py_batch = PyRecordBatch::new(record_batch);
                py_batch.into_pyarrow(py).map(|bound| bound.unbind())
            })
        })
    }
}

/// Prepared state for async record generation operations.
///
/// This struct bundles all the validated and cloned state needed for async operations,
/// reducing code duplication across the three async methods.
struct AsyncRecordState {
    rng: ForgeryRng,
    locale: Locale,
    schema: BTreeMap<String, providers::records::FieldSpec>,
    chunk_size: usize,
    custom_providers: HashMap<String, CustomProvider>,
}

impl Faker {
    /// Prepare state for async record generation.
    ///
    /// Validates inputs and clones necessary state for use in async blocks.
    fn prepare_async_state(
        &self,
        n: usize,
        schema: &Bound<'_, PyDict>,
        chunk_size: Option<usize>,
    ) -> PyResult<AsyncRecordState> {
        validate_batch_size(n).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let custom_names = self.custom_provider_names();
        let rust_schema = parse_py_schema_with_custom(schema, &custom_names)?;

        Ok(AsyncRecordState {
            rng: self.rng.clone(),
            locale: self.locale,
            schema: rust_schema,
            chunk_size: chunk_size.unwrap_or(providers::async_records::DEFAULT_CHUNK_SIZE),
            custom_providers: self.custom_providers.clone(),
        })
    }
}

/// Parse a Python schema dictionary into a Rust BTreeMap, with custom provider support.
fn parse_py_schema_with_custom(
    schema: &Bound<'_, PyDict>,
    custom_provider_names: &HashSet<String>,
) -> PyResult<BTreeMap<String, providers::records::FieldSpec>> {
    // Validate schema size to prevent DoS attacks via huge schemas
    validate_schema_size(schema.len()).map_err(|e| PyValueError::new_err(e.to_string()))?;

    let mut rust_schema = BTreeMap::new();

    for (key, value) in schema.iter() {
        let field_name: String = key.extract()?;
        let field_spec = parse_field_spec_with_custom(&value, custom_provider_names)?;
        rust_schema.insert(field_name, field_spec);
    }

    Ok(rust_schema)
}

/// Parse a Python field specification into a Rust FieldSpec, with custom provider support.
fn parse_field_spec_with_custom(
    value: &Bound<'_, PyAny>,
    custom_provider_names: &HashSet<String>,
) -> PyResult<providers::records::FieldSpec> {
    if value.is_instance_of::<PyString>() {
        return parse_string_field_spec_with_custom(value, custom_provider_names);
    }
    if value.is_instance_of::<PyTuple>() {
        return parse_tuple_field_spec(value);
    }
    Err(PyValueError::new_err(
        "Field specification must be a string or tuple",
    ))
}

/// Parse a simple string type specification, with custom provider support.
fn parse_string_field_spec_with_custom(
    value: &Bound<'_, PyAny>,
    custom_provider_names: &HashSet<String>,
) -> PyResult<providers::records::FieldSpec> {
    let type_str: String = value.extract()?;
    providers::records::parse_simple_type_with_custom(&type_str, custom_provider_names)
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
        let result = Faker::new("xx_YY");
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("unsupported locale"));
            assert!(err.to_string().contains("xx_YY"));
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

        let names1 = faker1.names(10, false).unwrap();
        let names2 = faker2.names(10, false).unwrap();

        assert_eq!(names1, names2);
    }

    #[test]
    fn test_batch_generation() {
        let mut faker = Faker::new("en_US").unwrap();
        faker.seed(123);

        let names = faker.names(100, false).unwrap();
        assert_eq!(names.len(), 100);

        let emails = faker.emails(50, false).unwrap();
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
        let result = faker.names(MAX_BATCH_SIZE + 1, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_unique_generation() {
        let mut faker = Faker::new_default();
        faker.seed(42);

        // Generate unique names
        let names = faker.names(50, true).unwrap();
        assert_eq!(names.len(), 50);

        // Verify all names are unique
        let mut seen = std::collections::HashSet::new();
        for name in &names {
            assert!(seen.insert(name.clone()), "Duplicate name found: {}", name);
        }
    }

    #[test]
    fn test_unique_exhaustion() {
        let mut faker = Faker::new_default();
        faker.seed(42);

        // Try to generate more unique cities than exist in the data
        // This should fail with UniqueExhaustedError
        let result = faker.cities(10000, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_locale() {
        assert!(validate_locale("en_US").is_ok());
        assert!(validate_locale("de_DE").is_ok());
        assert!(validate_locale("fr_FR").is_ok());
        assert!(validate_locale("es_ES").is_ok());
        assert!(validate_locale("it_IT").is_ok());
        assert!(validate_locale("ja_JP").is_ok());
        assert!(validate_locale("en_GB").is_ok());
        assert!(validate_locale("xx_YY").is_err());
        assert!(validate_locale("").is_err());
    }
}
