//! Embedded data for fake data generation.
//!
//! Contains locale-specific data used by providers.

pub mod formats;
pub mod traits;

pub mod de_de;
pub mod en_gb;
pub mod en_us;
pub mod es_es;
pub mod fr_fr;
pub mod it_it;
pub mod ja_jp;

pub use formats::{AddressFormat, PhoneFormat, PostalCodeFormat};
pub use traits::LocaleData;

use crate::locale::Locale;

/// Get the locale data provider for a specific locale.
///
/// Returns a reference to the static locale data instance for the given locale.
///
/// # Examples
///
/// ```ignore
/// use forgery::locale::Locale;
/// use forgery::data::get_locale_data;
///
/// let data = get_locale_data(Locale::EnUS);
/// assert!(data.first_names().is_some());
/// ```
pub fn get_locale_data(locale: Locale) -> &'static dyn LocaleData {
    match locale {
        Locale::EnUS => &en_us::EN_US_DATA,
        Locale::DeDE => &de_de::DE_DE_DATA,
        Locale::FrFR => &fr_fr::FR_FR_DATA,
        Locale::EsES => &es_es::ES_ES_DATA,
        Locale::ItIT => &it_it::IT_IT_DATA,
        Locale::JaJP => &ja_jp::JA_JP_DATA,
        Locale::EnGB => &en_gb::EN_GB_DATA,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_locale_data_en_us() {
        let data = get_locale_data(Locale::EnUS);
        assert!(data.first_names().is_some());
        assert!(data.last_names().is_some());
    }

    #[test]
    fn test_all_locales_have_data() {
        for locale in Locale::ALL {
            let data = get_locale_data(*locale);
            // All locales should have at least first names
            assert!(
                data.first_names().is_some(),
                "Locale {} should have first names",
                locale
            );
        }
    }
}
