//! Spanish (Spain) locale data.
//!
//! Contains name lists and other data for es_ES locale.

mod banks;
mod cities;
mod color_names;
mod companies;
mod first_names;
mod last_names;
mod provinces;
mod streets;

pub use banks::BANK_NAMES;
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

crate::impl_locale_data! {
    EsESData,
    first_names: FIRST_NAMES,
    last_names: LAST_NAMES,
    cities: CITIES,
    regions: PROVINCES,
    region_abbrs: PROVINCE_ABBRS,
    street_names: STREET_NAMES,
    street_suffixes: STREET_SUFFIXES,
    countries: COUNTRIES,
    postal_format: ES_POSTAL_FORMAT,
    address_format: ES_ADDRESS_FORMAT,
    phone_format: ES_PHONE_FORMAT,
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
    fn test_es_es_data_implements_locale_data() {
        let data = &ES_ES_DATA;
        assert!(data.first_names().is_some());
        assert!(data.last_names().is_some());
        assert!(data.cities().is_some());
        assert!(data.regions().is_some());
    }
}
