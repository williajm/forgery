//! English (UK) locale data.
//!
//! Contains name lists and other data for en_GB locale.
//! Shares some data with en_US but with UK-specific variations.

mod cities;
mod color_names;
mod companies;
mod counties;
mod first_names;
mod last_names;
mod streets;

pub use cities::CITIES;
pub use color_names::COLOR_NAMES;
pub use companies::{
    CATCH_PHRASE_ADJECTIVES, CATCH_PHRASE_NOUNS, COMPANY_PREFIXES, COMPANY_SUFFIXES, JOB_TITLES,
};
pub use counties::{COUNTIES, COUNTY_ABBRS};
pub use first_names::FIRST_NAMES;
pub use last_names::LAST_NAMES;
pub use streets::{STREET_NAMES, STREET_SUFFIXES};

// Shared data from en_US
use super::en_us::{COUNTRIES, FREE_EMAIL_DOMAINS, LOREM_WORDS, SAFE_EMAIL_DOMAINS, TLDS};

use super::formats::{AddressFormat, PhoneFormat, PostalCodeFormat};
use super::traits::LocaleData;

/// Phone format patterns for UK.
const UK_PHONE_PATTERNS: &[&str] = &[
    "+44 #### ######",
    "0#### ######",
    "+44 ### #### ####",
    "0### #### ####",
];

/// Postal code patterns for UK.
/// UK postcodes follow patterns like: SW1A 1AA, M1 1AE, B33 8TH
const UK_POSTAL_PATTERNS: &[&str] = &["AA## #AA", "AA# #AA", "A## #AA", "A# #AA"];

/// UK phone format specification.
const UK_PHONE_FORMAT: PhoneFormat = PhoneFormat::new(UK_PHONE_PATTERNS, "+44");

/// UK postal code format specification.
const UK_POSTAL_FORMAT: PostalCodeFormat = PostalCodeFormat::new(UK_POSTAL_PATTERNS);

/// UK address format specification.
const UK_ADDRESS_FORMAT: AddressFormat =
    AddressFormat::new("{street}\n{city}\n{region}\n{postal}", true);

/// English (UK) locale data provider.
pub struct EnGbData;

/// Static instance of the English (UK) locale data.
pub static EN_GB_DATA: EnGbData = EnGbData;

impl LocaleData for EnGbData {
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
        Some(COUNTIES)
    }

    fn region_abbrs(&self) -> Option<&'static [&'static str]> {
        Some(COUNTY_ABBRS)
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
        Some(UK_POSTAL_FORMAT)
    }

    fn address_format(&self) -> Option<AddressFormat> {
        Some(UK_ADDRESS_FORMAT)
    }

    fn phone_format(&self) -> Option<PhoneFormat> {
        Some(UK_PHONE_FORMAT)
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
    fn test_en_gb_data_implements_locale_data() {
        let data = &EN_GB_DATA;
        assert!(data.first_names().is_some());
        assert!(data.last_names().is_some());
        assert!(data.cities().is_some());
        assert!(data.regions().is_some());
    }

    #[test]
    fn test_uk_phone_format() {
        let format = UK_PHONE_FORMAT;
        assert_eq!(format.country_code, "+44");
    }
}
