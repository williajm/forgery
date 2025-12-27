//! Spanish (Spain) locale data.
//!
//! Contains name lists and other data for es_ES locale.

mod cities;
mod color_names;
mod companies;
mod first_names;
mod last_names;
mod provinces;
mod streets;

pub use cities::CITIES;
pub use color_names::COLOR_NAMES;
pub use companies::{
    CATCH_PHRASE_ADJECTIVES, CATCH_PHRASE_NOUNS, COMPANY_PREFIXES, COMPANY_SUFFIXES, JOB_TITLES,
};
pub use first_names::{FIRST_NAMES, ROMANIZED_FIRST_NAMES};
pub use last_names::LAST_NAMES;
pub use provinces::{PROVINCES, PROVINCE_ABBRS};
pub use streets::{STREET_NAMES, STREET_SUFFIXES};

// Shared data
use super::en_us::{COUNTRIES, FREE_EMAIL_DOMAINS, LOREM_WORDS, SAFE_EMAIL_DOMAINS, TLDS};

use super::formats::{AddressFormat, PhoneFormat, PostalCodeFormat};
use super::traits::LocaleData;

/// Phone format patterns for Spain.
const ES_PHONE_PATTERNS: &[&str] = &["+34 ### ### ###", "### ### ###", "+34 ## ### ## ##"];

/// Postal code patterns for Spain (5 digits).
const ES_POSTAL_PATTERNS: &[&str] = &["#####"];

/// Spanish phone format specification.
const ES_PHONE_FORMAT: PhoneFormat = PhoneFormat::new(ES_PHONE_PATTERNS, "+34");

/// Spanish postal code format specification.
const ES_POSTAL_FORMAT: PostalCodeFormat = PostalCodeFormat::new(ES_POSTAL_PATTERNS);

/// Spanish address format specification.
/// Spanish uses street type as prefix: "Calle Mayor", "Avenida de España"
/// Street number comes after the street: "Calle Mayor 15"
const ES_ADDRESS_FORMAT: AddressFormat =
    AddressFormat::with_prefix_type("{street}\n{postal} {city}", false);

/// Spanish locale data provider.
pub struct EsESData;

/// Static instance of the Spanish locale data.
pub static ES_ES_DATA: EsESData = EsESData;

impl LocaleData for EsESData {
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
        Some(PROVINCES)
    }

    fn region_abbrs(&self) -> Option<&'static [&'static str]> {
        Some(PROVINCE_ABBRS)
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
        Some(ES_POSTAL_FORMAT)
    }

    fn address_format(&self) -> Option<AddressFormat> {
        Some(ES_ADDRESS_FORMAT)
    }

    fn phone_format(&self) -> Option<PhoneFormat> {
        Some(ES_PHONE_FORMAT)
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
    fn test_es_es_data_implements_locale_data() {
        let data = &ES_ES_DATA;
        assert!(data.first_names().is_some());
        assert!(data.last_names().is_some());
        assert!(data.cities().is_some());
        assert!(data.regions().is_some());
    }
}
