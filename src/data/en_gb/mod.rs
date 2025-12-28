//! English (UK) locale data.
//!
//! Contains name lists and other data for en_GB locale.
//! Shares some data with en_US but with UK-specific variations.

mod banks;
mod cities;
mod color_names;
mod companies;
mod counties;
mod first_names;
mod last_names;
mod streets;

pub use banks::BANK_NAMES;
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

crate::impl_locale_data! {
    EnGbData,
    first_names: FIRST_NAMES,
    last_names: LAST_NAMES,
    cities: CITIES,
    regions: COUNTIES,
    region_abbrs: COUNTY_ABBRS,
    street_names: STREET_NAMES,
    street_suffixes: STREET_SUFFIXES,
    countries: COUNTRIES,
    postal_format: UK_POSTAL_FORMAT,
    address_format: UK_ADDRESS_FORMAT,
    phone_format: UK_PHONE_FORMAT,
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
