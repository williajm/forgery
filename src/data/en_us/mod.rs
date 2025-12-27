//! English (US) locale data.
//!
//! Contains name lists and other data for en_US locale.

mod cities;
mod color_names;
mod companies;
mod countries;
mod first_names;
mod last_names;
mod lorem;
mod states;
mod streets;
mod tlds;

pub use cities::CITIES;
pub use color_names::COLOR_NAMES;
pub use companies::{
    CATCH_PHRASE_ADJECTIVES, CATCH_PHRASE_NOUNS, COMPANY_PREFIXES, COMPANY_SUFFIXES, JOB_TITLES,
};
pub use countries::COUNTRIES;
pub use first_names::FIRST_NAMES;
pub use last_names::LAST_NAMES;
pub use lorem::LOREM_WORDS;
pub use states::{STATES, STATE_ABBRS};
pub use streets::{STREET_NAMES, STREET_SUFFIXES};
pub use tlds::{FREE_EMAIL_DOMAINS, SAFE_EMAIL_DOMAINS, TLDS};

use super::formats::{AddressFormat, PhoneFormat, PostalCodeFormat};
use super::traits::LocaleData;

/// Phone format patterns for US.
const US_PHONE_PATTERNS: &[&str] = &["(###) ###-####", "###-###-####", "+1 (###) ###-####"];

/// Postal code patterns for US (ZIP codes).
const US_POSTAL_PATTERNS: &[&str] = &["#####", "#####-####"];

/// US phone format specification.
const US_PHONE_FORMAT: PhoneFormat = PhoneFormat::new(US_PHONE_PATTERNS, "+1");

/// US postal code format specification.
const US_POSTAL_FORMAT: PostalCodeFormat = PostalCodeFormat::new(US_POSTAL_PATTERNS);

/// US address format specification.
const US_ADDRESS_FORMAT: AddressFormat =
    AddressFormat::new("{street}, {city}, {region_abbr} {postal}", true);

/// English (US) locale data provider.
pub struct EnUsData;

/// Static instance of the English (US) locale data.
pub static EN_US_DATA: EnUsData = EnUsData;

impl LocaleData for EnUsData {
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
        Some(STATES)
    }

    fn region_abbrs(&self) -> Option<&'static [&'static str]> {
        Some(STATE_ABBRS)
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
        Some(US_POSTAL_FORMAT)
    }

    fn address_format(&self) -> Option<AddressFormat> {
        Some(US_ADDRESS_FORMAT)
    }

    fn phone_format(&self) -> Option<PhoneFormat> {
        Some(US_PHONE_FORMAT)
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
    fn test_en_us_data_implements_locale_data() {
        let data = &EN_US_DATA;

        // Verify all data is available
        assert!(data.first_names().is_some());
        assert!(data.last_names().is_some());
        assert!(data.cities().is_some());
        assert!(data.regions().is_some());
        assert!(data.region_abbrs().is_some());
        assert!(data.street_names().is_some());
        assert!(data.street_suffixes().is_some());
        assert!(data.countries().is_some());
        assert!(data.postal_code_format().is_some());
        assert!(data.address_format().is_some());
        assert!(data.phone_format().is_some());
        assert!(data.company_prefixes().is_some());
        assert!(data.company_suffixes().is_some());
        assert!(data.job_titles().is_some());
        assert!(data.catch_phrase_adjectives().is_some());
        assert!(data.catch_phrase_nouns().is_some());
        assert!(data.text_words().is_some());
        assert!(data.tlds().is_some());
        assert!(data.free_email_domains().is_some());
        assert!(data.safe_email_domains().is_some());
        assert!(data.color_names().is_some());
    }

    #[test]
    fn test_en_us_data_content() {
        let data = &EN_US_DATA;

        // Verify data has content
        assert!(!data.first_names().unwrap().is_empty());
        assert!(!data.last_names().unwrap().is_empty());
        assert!(!data.cities().unwrap().is_empty());
        assert_eq!(data.regions().unwrap().len(), 50); // 50 US states
        assert_eq!(data.region_abbrs().unwrap().len(), 50);
    }

    #[test]
    fn test_us_phone_format() {
        let format = US_PHONE_FORMAT;
        assert_eq!(format.country_code, "+1");
        assert!(!format.patterns.is_empty());
    }

    #[test]
    fn test_us_postal_format() {
        let format = US_POSTAL_FORMAT;
        assert!(!format.patterns.is_empty());
        assert!(format.patterns.contains(&"#####"));
    }

    #[test]
    fn test_us_address_format() {
        let format = US_ADDRESS_FORMAT;
        assert!(format.template.contains("{street}"));
        assert!(format.template.contains("{city}"));
        assert!(format.number_before_street);
    }
}
