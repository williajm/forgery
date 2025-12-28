//! German (Germany) locale data.
//!
//! Contains name lists and other data for de_DE locale.

mod banks;
mod bundeslaender;
mod cities;
mod color_names;
mod companies;
mod first_names;
mod last_names;
mod streets;

pub use banks::BANK_NAMES;
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

crate::impl_locale_data! {
    DeDEData,
    first_names: FIRST_NAMES,
    last_names: LAST_NAMES,
    cities: CITIES,
    regions: BUNDESLAENDER,
    region_abbrs: BUNDESLAENDER_ABBRS,
    street_names: STREET_NAMES,
    street_suffixes: STREET_SUFFIXES,
    countries: COUNTRIES,
    postal_format: DE_POSTAL_FORMAT,
    address_format: DE_ADDRESS_FORMAT,
    phone_format: DE_PHONE_FORMAT,
    company_prefixes: COMPANY_PREFIXES,
    company_suffixes: COMPANY_SUFFIXES,
    job_titles: JOB_TITLES,
    catch_phrase_adjectives: CATCH_PHRASE_ADJECTIVES,
    catch_phrase_nouns: CATCH_PHRASE_NOUNS,
    text_words: LOREM_WORDS,
    tlds: TLDS,
    free_email_domains: FREE_EMAIL_DOMAINS,
    safe_email_domains: SAFE_EMAIL_DOMAINS,
    color_names: COLOR_NAMES,
    bank_names: BANK_NAMES,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::traits::LocaleData;

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
