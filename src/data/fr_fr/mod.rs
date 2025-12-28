//! French (France) locale data.
//!
//! Contains name lists and other data for fr_FR locale.

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

/// Phone format patterns for France.
const FR_PHONE_PATTERNS: &[&str] = &["+33 # ## ## ## ##", "0# ## ## ## ##", "+33 ### ### ###"];

/// Postal code patterns for France (5 digits).
const FR_POSTAL_PATTERNS: &[&str] = &["#####"];

/// French phone format specification.
const FR_PHONE_FORMAT: PhoneFormat = PhoneFormat::new(FR_PHONE_PATTERNS, "+33");

/// French postal code format specification.
const FR_POSTAL_FORMAT: PostalCodeFormat = PostalCodeFormat::new(FR_POSTAL_PATTERNS);

/// French address format specification.
/// French uses street type as prefix: "rue de la République", "avenue Victor Hugo"
/// Street number comes before the street: "15 rue de la République"
const FR_ADDRESS_FORMAT: AddressFormat =
    AddressFormat::with_prefix_type("{street}\n{postal} {city}", true);

/// French locale data provider.
pub struct FrFRData;

/// Static instance of the French locale data.
pub static FR_FR_DATA: FrFRData = FrFRData;

crate::impl_locale_data! {
    FrFRData,
    first_names: FIRST_NAMES,
    last_names: LAST_NAMES,
    cities: CITIES,
    regions: REGIONS,
    region_abbrs: REGION_ABBRS,
    street_names: STREET_NAMES,
    street_suffixes: STREET_SUFFIXES,
    countries: COUNTRIES,
    postal_format: FR_POSTAL_FORMAT,
    address_format: FR_ADDRESS_FORMAT,
    phone_format: FR_PHONE_FORMAT,
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
    fn test_fr_fr_data_implements_locale_data() {
        let data = &FR_FR_DATA;

        // Test all LocaleData trait methods for full macro coverage
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
        assert!(data.romanized_first_names().is_some());
    }
}
