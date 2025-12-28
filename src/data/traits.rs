//! Trait definition for locale-specific data providers.
//!
//! The `LocaleData` trait defines the interface that all locale modules
//! must implement to provide locale-specific data for generation.

use super::formats::{AddressFormat, PhoneFormat, PostalCodeFormat};

/// Trait for locale-specific data access.
///
/// Each locale module implements this trait to provide data for
/// various generators. Methods return `Option` to indicate whether
/// the locale supports that particular data type.
///
/// # Implementation Notes
///
/// - Return `Some(&DATA)` for supported data types
/// - Return `None` for unsupported data types (will cause error at runtime)
/// - Data should be `&'static` to avoid allocation
pub trait LocaleData: Send + Sync {
    // === Names ===

    /// First names for the locale.
    fn first_names(&self) -> Option<&'static [&'static str]>;

    /// Last names/surnames for the locale.
    fn last_names(&self) -> Option<&'static [&'static str]>;

    // === Address Components ===

    /// Cities for the locale.
    fn cities(&self) -> Option<&'static [&'static str]>;

    /// Regions (states, provinces, prefectures, etc.) for the locale.
    fn regions(&self) -> Option<&'static [&'static str]>;

    /// Region abbreviations (e.g., "CA" for California).
    fn region_abbrs(&self) -> Option<&'static [&'static str]>;

    /// Street names for the locale.
    fn street_names(&self) -> Option<&'static [&'static str]>;

    /// Street suffixes (e.g., "Street", "Avenue", "Straße").
    fn street_suffixes(&self) -> Option<&'static [&'static str]>;

    /// Countries list (typically in the locale's language).
    fn countries(&self) -> Option<&'static [&'static str]>;

    /// Postal code format specification.
    fn postal_code_format(&self) -> Option<PostalCodeFormat>;

    /// Address format specification.
    fn address_format(&self) -> Option<AddressFormat>;

    // === Phone ===

    /// Phone number format specification.
    fn phone_format(&self) -> Option<PhoneFormat>;

    // === Company ===

    /// Company name prefixes (e.g., last names used in company names).
    fn company_prefixes(&self) -> Option<&'static [&'static str]>;

    /// Company name suffixes (e.g., "Inc.", "GmbH", "Ltd.").
    fn company_suffixes(&self) -> Option<&'static [&'static str]>;

    /// Job titles for the locale.
    fn job_titles(&self) -> Option<&'static [&'static str]>;

    /// Catch phrase adjectives.
    fn catch_phrase_adjectives(&self) -> Option<&'static [&'static str]>;

    /// Catch phrase nouns.
    fn catch_phrase_nouns(&self) -> Option<&'static [&'static str]>;

    // === Text ===

    /// Words for generating lorem-ipsum-style text.
    fn text_words(&self) -> Option<&'static [&'static str]>;

    // === Internet ===

    /// Top-level domains.
    fn tlds(&self) -> Option<&'static [&'static str]>;

    /// Free email domains (e.g., gmail.com, yahoo.com).
    fn free_email_domains(&self) -> Option<&'static [&'static str]>;

    /// Safe email domains (e.g., example.com, example.org).
    fn safe_email_domains(&self) -> Option<&'static [&'static str]>;

    // === Colors ===

    /// Color names in the locale's language.
    fn color_names(&self) -> Option<&'static [&'static str]>;

    // === Finance ===

    /// Bank names for the locale.
    fn bank_names(&self) -> Option<&'static [&'static str]>;

    // === Romanization (for non-Latin scripts) ===

    /// Romanized first names for email generation.
    /// Only needed for locales with non-Latin scripts (e.g., ja_JP).
    /// Defaults to returning `first_names()`.
    fn romanized_first_names(&self) -> Option<&'static [&'static str]> {
        self.first_names()
    }

    /// Romanized last names for email generation.
    /// Only needed for locales with non-Latin scripts.
    /// Defaults to returning `last_names()`.
    fn romanized_last_names(&self) -> Option<&'static [&'static str]> {
        self.last_names()
    }
}
