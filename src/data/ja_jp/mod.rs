//! Japanese (Japan) locale data.
//!
//! Contains name lists and other data for ja_JP locale.
//!
//! Note: Japanese names are provided in both kanji and romanized forms.
//! For emails, romanized forms are used to ensure ASCII compatibility.
//! Japanese names traditionally have family name first (e.g., 田中 太郎 = Tanaka Taro).

mod cities;
mod color_names;
mod companies;
mod first_names;
mod last_names;
mod prefectures;
mod streets;

pub use cities::CITIES;
pub use color_names::COLOR_NAMES;
pub use companies::{
    CATCH_PHRASE_ADJECTIVES, CATCH_PHRASE_NOUNS, COMPANY_PREFIXES, COMPANY_SUFFIXES, JOB_TITLES,
};
pub use first_names::{FIRST_NAMES, FIRST_NAMES_ROMANIZED};
pub use last_names::{LAST_NAMES, LAST_NAMES_ROMANIZED};
pub use prefectures::{PREFECTURES, PREFECTURE_ABBRS};
pub use streets::{STREET_NAMES, STREET_SUFFIXES};

// Shared data
use super::en_us::{COUNTRIES, FREE_EMAIL_DOMAINS, LOREM_WORDS, SAFE_EMAIL_DOMAINS, TLDS};

use super::formats::{AddressFormat, PhoneFormat, PostalCodeFormat};

/// Phone format patterns for Japan.
const JP_PHONE_PATTERNS: &[&str] = &["+81 ##-####-####", "0##-####-####", "+81 #-####-####"];

/// Postal code patterns for Japan (###-####).
const JP_POSTAL_PATTERNS: &[&str] = &["###-####"];

/// Japanese phone format specification.
const JP_PHONE_FORMAT: PhoneFormat = PhoneFormat::new(JP_PHONE_PATTERNS, "+81");

/// Japanese postal code format specification.
const JP_POSTAL_FORMAT: PostalCodeFormat = PostalCodeFormat::new(JP_POSTAL_PATTERNS);

/// Japanese address format specification.
/// Japanese addresses go from largest to smallest: postal, prefecture, city, street.
/// Street names are compound words without spaces (e.g., "中央通り" not "中央 通り").
const JP_ADDRESS_FORMAT: AddressFormat =
    AddressFormat::with_separator("〒{postal} {region}{city}{street}", false, "");

/// Japanese locale data provider.
pub struct JaJPData;

/// Static instance of the Japanese locale data.
pub static JA_JP_DATA: JaJPData = JaJPData;

crate::impl_locale_data! {
    JaJPData,
    first_names: FIRST_NAMES,
    last_names: LAST_NAMES,
    cities: CITIES,
    regions: PREFECTURES,
    region_abbrs: PREFECTURE_ABBRS,
    street_names: STREET_NAMES,
    street_suffixes: STREET_SUFFIXES,
    countries: COUNTRIES,
    postal_format: JP_POSTAL_FORMAT,
    address_format: JP_ADDRESS_FORMAT,
    phone_format: JP_PHONE_FORMAT,
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
    romanized_first_names: FIRST_NAMES_ROMANIZED,
    romanized_last_names: LAST_NAMES_ROMANIZED,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::traits::LocaleData;

    #[test]
    fn test_ja_jp_data_implements_locale_data() {
        let data = &JA_JP_DATA;
        assert!(data.first_names().is_some());
        assert!(data.last_names().is_some());
        assert!(data.romanized_first_names().is_some());
        assert!(data.romanized_last_names().is_some());
        assert!(data.cities().is_some());
        assert!(data.regions().is_some());
    }

    #[test]
    fn test_prefectures_count() {
        assert_eq!(PREFECTURES.len(), 47);
    }

    #[test]
    fn test_romanized_names_match() {
        // Verify romanized arrays match size with kanji arrays
        assert_eq!(FIRST_NAMES.len(), FIRST_NAMES_ROMANIZED.len());
        assert_eq!(LAST_NAMES.len(), LAST_NAMES_ROMANIZED.len());
    }
}
