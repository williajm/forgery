//! German (Germany) locale data.
//!
//! Contains name lists and other data for de_DE locale.

mod bundeslaender;
mod cities;
mod color_names;
mod companies;
mod first_names;
mod last_names;
mod streets;

pub use bundeslaender::{BUNDESLAENDER, BUNDESLAENDER_ABBRS};
pub use cities::CITIES;
pub use color_names::COLOR_NAMES;
pub use companies::{
    CATCH_PHRASE_ADJECTIVES, CATCH_PHRASE_NOUNS, COMPANY_PREFIXES, COMPANY_SUFFIXES, JOB_TITLES,
};
pub use first_names::FIRST_NAMES;
pub use last_names::LAST_NAMES;
pub use streets::{STREET_NAMES, STREET_SUFFIXES};

// Shared data
use super::en_us::{COUNTRIES, FREE_EMAIL_DOMAINS, LOREM_WORDS, SAFE_EMAIL_DOMAINS, TLDS};

use super::formats::{AddressFormat, PhoneFormat, PostalCodeFormat};
use super::traits::LocaleData;

/// Phone format patterns for Germany.
const DE_PHONE_PATTERNS: &[&str] = &[
    "+49 ### #######",
    "0### #######",
    "+49 ## ########",
    "0## ########",
];

/// Postal code patterns for Germany (5 digits).
const DE_POSTAL_PATTERNS: &[&str] = &["#####"];

/// German phone format specification.
const DE_PHONE_FORMAT: PhoneFormat = PhoneFormat::new(DE_PHONE_PATTERNS, "+49");

/// German postal code format specification.
const DE_POSTAL_FORMAT: PostalCodeFormat = PostalCodeFormat::new(DE_POSTAL_PATTERNS);

/// German address format specification.
/// In Germany, street number comes after street name, and street names are
/// compound words without spaces (e.g., "Hauptstraße" not "Haupt straße").
const DE_ADDRESS_FORMAT: AddressFormat =
    AddressFormat::with_separator("{street}\n{postal} {city}", false, "");

/// German locale data provider.
pub struct DeDEData;

/// Static instance of the German locale data.
pub static DE_DE_DATA: DeDEData = DeDEData;

impl LocaleData for DeDEData {
    fn first_names(&self) -> Option<&'static [&'static str]> {
        Some(FIRST_NAMES)
    }

    fn last_names(&self) -> Option<&'static [&'static str]> {
        Some(LAST_NAMES)
    }

    fn cities(&self) -> Option<&'static [&'static str]> {
        Some(CITIES)
    }

    fn regions(&self) -> Option<&'static [&'static str]> {
        Some(BUNDESLAENDER)
    }

    fn region_abbrs(&self) -> Option<&'static [&'static str]> {
        Some(BUNDESLAENDER_ABBRS)
    }

    fn street_names(&self) -> Option<&'static [&'static str]> {
        Some(STREET_NAMES)
    }

    fn street_suffixes(&self) -> Option<&'static [&'static str]> {
        Some(STREET_SUFFIXES)
    }

    fn countries(&self) -> Option<&'static [&'static str]> {
        Some(COUNTRIES)
    }

    fn postal_code_format(&self) -> Option<PostalCodeFormat> {
        Some(DE_POSTAL_FORMAT)
    }

    fn address_format(&self) -> Option<AddressFormat> {
        Some(DE_ADDRESS_FORMAT)
    }

    fn phone_format(&self) -> Option<PhoneFormat> {
        Some(DE_PHONE_FORMAT)
    }

    fn company_prefixes(&self) -> Option<&'static [&'static str]> {
        Some(COMPANY_PREFIXES)
    }

    fn company_suffixes(&self) -> Option<&'static [&'static str]> {
        Some(COMPANY_SUFFIXES)
    }

    fn job_titles(&self) -> Option<&'static [&'static str]> {
        Some(JOB_TITLES)
    }

    fn catch_phrase_adjectives(&self) -> Option<&'static [&'static str]> {
        Some(CATCH_PHRASE_ADJECTIVES)
    }

    fn catch_phrase_nouns(&self) -> Option<&'static [&'static str]> {
        Some(CATCH_PHRASE_NOUNS)
    }

    fn text_words(&self) -> Option<&'static [&'static str]> {
        Some(LOREM_WORDS)
    }

    fn tlds(&self) -> Option<&'static [&'static str]> {
        Some(TLDS)
    }

    fn free_email_domains(&self) -> Option<&'static [&'static str]> {
        Some(FREE_EMAIL_DOMAINS)
    }

    fn safe_email_domains(&self) -> Option<&'static [&'static str]> {
        Some(SAFE_EMAIL_DOMAINS)
    }

    fn color_names(&self) -> Option<&'static [&'static str]> {
        Some(COLOR_NAMES)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_de_data_implements_locale_data() {
        let data = &DE_DE_DATA;
        assert!(data.first_names().is_some());
        assert!(data.last_names().is_some());
        assert!(data.cities().is_some());
        assert!(data.regions().is_some());
    }

    #[test]
    fn test_de_phone_format() {
        let format = DE_PHONE_FORMAT;
        assert_eq!(format.country_code, "+49");
    }

    #[test]
    fn test_bundeslaender_count() {
        assert_eq!(BUNDESLAENDER.len(), 16);
        assert_eq!(BUNDESLAENDER_ABBRS.len(), 16);
    }
}
