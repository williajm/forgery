//! Italian (Italy) locale data.
//!
//! Contains name lists and other data for it_IT locale.

mod cities;
mod color_names;
mod companies;
mod first_names;
mod last_names;
mod regions;
mod streets;

pub use cities::CITIES;
pub use color_names::COLOR_NAMES;
pub use companies::{
    CATCH_PHRASE_ADJECTIVES, CATCH_PHRASE_NOUNS, COMPANY_PREFIXES, COMPANY_SUFFIXES, JOB_TITLES,
};
pub use first_names::{FIRST_NAMES, ROMANIZED_FIRST_NAMES};
pub use last_names::LAST_NAMES;
pub use regions::{REGIONS, REGION_ABBRS};
pub use streets::{STREET_NAMES, STREET_SUFFIXES};

// Shared data
use super::en_us::{COUNTRIES, FREE_EMAIL_DOMAINS, LOREM_WORDS, SAFE_EMAIL_DOMAINS, TLDS};

use super::formats::{AddressFormat, PhoneFormat, PostalCodeFormat};
use super::traits::LocaleData;

/// Phone format patterns for Italy.
const IT_PHONE_PATTERNS: &[&str] = &["+39 ## #### ####", "0## #### ####", "+39 ### #######"];

/// Postal code patterns for Italy (5 digits).
const IT_POSTAL_PATTERNS: &[&str] = &["#####"];

/// Italian phone format specification.
const IT_PHONE_FORMAT: PhoneFormat = PhoneFormat::new(IT_PHONE_PATTERNS, "+39");

/// Italian postal code format specification.
const IT_POSTAL_FORMAT: PostalCodeFormat = PostalCodeFormat::new(IT_POSTAL_PATTERNS);

/// Italian address format specification.
const IT_ADDRESS_FORMAT: AddressFormat = AddressFormat::new("{street}\n{postal} {city}", false);

/// Italian locale data provider.
pub struct ItITData;

/// Static instance of the Italian locale data.
pub static IT_IT_DATA: ItITData = ItITData;

impl LocaleData for ItITData {
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
        Some(REGIONS)
    }

    fn region_abbrs(&self) -> Option<&'static [&'static str]> {
        Some(REGION_ABBRS)
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
        Some(IT_POSTAL_FORMAT)
    }

    fn address_format(&self) -> Option<AddressFormat> {
        Some(IT_ADDRESS_FORMAT)
    }

    fn phone_format(&self) -> Option<PhoneFormat> {
        Some(IT_PHONE_FORMAT)
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

    fn romanized_first_names(&self) -> Option<&'static [&'static str]> {
        Some(ROMANIZED_FIRST_NAMES)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_it_data_implements_locale_data() {
        let data = &IT_IT_DATA;
        assert!(data.first_names().is_some());
        assert!(data.last_names().is_some());
        assert!(data.cities().is_some());
        assert!(data.regions().is_some());
    }
}
