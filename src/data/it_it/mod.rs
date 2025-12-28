//! Italian (Italy) locale data.
//!
//! Contains name lists and other data for it_IT locale.

mod banks;
mod cities;
mod color_names;
mod companies;
mod first_names;
mod last_names;
mod regions;
mod streets;

pub use banks::BANK_NAMES;
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

/// Phone format patterns for Italy.
const IT_PHONE_PATTERNS: &[&str] = &["+39 ## #### ####", "0## #### ####", "+39 ### #######"];

/// Postal code patterns for Italy (5 digits).
const IT_POSTAL_PATTERNS: &[&str] = &["#####"];

/// Italian phone format specification.
const IT_PHONE_FORMAT: PhoneFormat = PhoneFormat::new(IT_PHONE_PATTERNS, "+39");

/// Italian postal code format specification.
const IT_POSTAL_FORMAT: PostalCodeFormat = PostalCodeFormat::new(IT_POSTAL_PATTERNS);

/// Italian address format specification.
/// Italian uses street type as prefix: "Via Roma", "Piazza Garibaldi"
/// Street number comes after the street: "Via Roma 15"
const IT_ADDRESS_FORMAT: AddressFormat =
    AddressFormat::with_prefix_type("{street}\n{postal} {city}", false);

/// Italian locale data provider.
pub struct ItITData;

/// Static instance of the Italian locale data.
pub static IT_IT_DATA: ItITData = ItITData;

crate::impl_locale_data! {
    ItITData,
    first_names: FIRST_NAMES,
    last_names: LAST_NAMES,
    cities: CITIES,
    regions: REGIONS,
    region_abbrs: REGION_ABBRS,
    street_names: STREET_NAMES,
    street_suffixes: STREET_SUFFIXES,
    countries: COUNTRIES,
    postal_format: IT_POSTAL_FORMAT,
    address_format: IT_ADDRESS_FORMAT,
    phone_format: IT_PHONE_FORMAT,
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
    romanized_first_names: ROMANIZED_FIRST_NAMES,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::traits::LocaleData;

    #[test]
    fn test_it_it_data_implements_locale_data() {
        let data = &IT_IT_DATA;
        assert!(data.first_names().is_some());
        assert!(data.last_names().is_some());
        assert!(data.cities().is_some());
        assert!(data.regions().is_some());
    }
}
