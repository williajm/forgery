//! Address generation provider.
//!
//! Generates addresses, street names, cities, states, countries, and zip codes.

use crate::data::en_us::COUNTRIES;
use crate::data::get_locale_data;
use crate::locale::Locale;
use crate::rng::ForgeryRng;

/// Generate a batch of random street addresses.
///
/// Format: "123 Main Street"
pub fn generate_street_addresses(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut addresses = Vec::with_capacity(n);
    for _ in 0..n {
        addresses.push(generate_street_address(rng, locale));
    }
    addresses
}

/// Generate a single random street address.
#[inline]
pub fn generate_street_address(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let street_names = data.street_names().unwrap_or(&[]);
    let street_suffixes = data.street_suffixes().unwrap_or(&[]);
    let format = data.address_format();

    let number: u32 = rng.gen_range(1, 9999);
    let name = if street_names.is_empty() {
        "Main"
    } else {
        rng.choose(street_names)
    };
    let street_type = if street_suffixes.is_empty() {
        "Street"
    } else {
        rng.choose(street_suffixes)
    };

    // Get format options from address format
    let (number_before_street, separator, type_prefix) = match format {
        Some(fmt) => (
            fmt.number_before_street,
            fmt.street_name_separator,
            fmt.street_type_prefix,
        ),
        None => (true, " ", false), // Default to US-style (suffix)
    };

    // Build the street name: either "type name" (prefix) or "name type" (suffix)
    let street_name = if type_prefix {
        // Prefix: "Calle Mayor", "Via Roma", "rue de la République"
        format!("{}{}{}", street_type, separator, name)
    } else {
        // Suffix: "Main Street", "Hauptstraße"
        format!("{}{}{}", name, separator, street_type)
    };

    if number_before_street {
        format!("{} {}", number, street_name)
    } else {
        format!("{} {}", street_name, number)
    }
}

/// Generate a batch of random city names.
pub fn generate_cities(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut cities = Vec::with_capacity(n);
    for _ in 0..n {
        cities.push(generate_city(rng, locale));
    }
    cities
}

/// Generate a single random city name.
#[inline]
pub fn generate_city(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let cities = data.cities().unwrap_or(&[]);
    if cities.is_empty() {
        "City".to_string()
    } else {
        rng.choose(cities).to_string()
    }
}

/// Generate a batch of random state/region names.
pub fn generate_states(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut states = Vec::with_capacity(n);
    for _ in 0..n {
        states.push(generate_state(rng, locale));
    }
    states
}

/// Generate a single random state/region name.
#[inline]
pub fn generate_state(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let regions = data.regions().unwrap_or(&[]);
    if regions.is_empty() {
        "State".to_string()
    } else {
        rng.choose(regions).to_string()
    }
}

/// Generate a batch of random state/region abbreviations.
#[allow(dead_code)]
pub fn generate_state_abbrs(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut abbrs = Vec::with_capacity(n);
    for _ in 0..n {
        abbrs.push(generate_state_abbr(rng, locale));
    }
    abbrs
}

/// Generate a single random state/region abbreviation.
#[inline]
pub fn generate_state_abbr(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let abbrs = data.region_abbrs().unwrap_or(&[]);
    if abbrs.is_empty() {
        "ST".to_string()
    } else {
        rng.choose(abbrs).to_string()
    }
}

/// Generate a batch of random country names.
pub fn generate_countries(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut countries = Vec::with_capacity(n);
    for _ in 0..n {
        countries.push(generate_country(rng));
    }
    countries
}

/// Generate a single random country name.
#[inline]
pub fn generate_country(rng: &mut ForgeryRng) -> String {
    // Countries are universal, not locale-specific
    rng.choose(COUNTRIES).to_string()
}

/// Generate a batch of random postal/zip codes.
pub fn generate_zip_codes(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut zips = Vec::with_capacity(n);
    for _ in 0..n {
        zips.push(generate_zip_code(rng, locale));
    }
    zips
}

/// Generate a single random postal/zip code.
#[inline]
pub fn generate_zip_code(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    if let Some(format) = data.postal_code_format() {
        if !format.patterns.is_empty() {
            let pattern = rng.choose(format.patterns);
            return expand_pattern(rng, pattern);
        }
    }
    // Default US format
    let zip5: u32 = rng.gen_range(10000, 99999);
    format!("{:05}", zip5)
}

/// Expand a format pattern where # is a digit and A/@ is a letter.
///
/// Placeholders:
/// - `#` = random digit (0-9)
/// - `A` or `@` = random uppercase letter (A-Z)
/// - Other characters pass through unchanged
fn expand_pattern(rng: &mut ForgeryRng, pattern: &str) -> String {
    pattern
        .chars()
        .map(|c| match c {
            '#' => {
                let digit = rng.gen_range(0u8, 9);
                char::from_digit(digit as u32, 10).unwrap()
            }
            'A' | '@' => {
                let letter_idx = rng.gen_range(0u8, 25);
                (b'A' + letter_idx) as char
            }
            _ => c,
        })
        .collect()
}

/// Generate a batch of random full addresses.
pub fn generate_addresses(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut addresses = Vec::with_capacity(n);
    for _ in 0..n {
        addresses.push(generate_address(rng, locale));
    }
    addresses
}

/// Generate a single random full address.
///
/// Uses the locale's address format template to assemble components.
/// Supported placeholders: `{street}`, `{city}`, `{region}`, `{region_abbr}`, `{postal}`
#[inline]
pub fn generate_address(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let street = generate_street_address(rng, locale);
    let city = generate_city(rng, locale);
    let region = generate_state(rng, locale);
    let region_abbr = generate_state_abbr(rng, locale);
    let postal = generate_zip_code(rng, locale);

    // Get the template from locale's address format, or use US default
    let template = data
        .address_format()
        .map(|f| f.template)
        .unwrap_or("{street}, {city}, {region_abbr} {postal}");

    // Replace placeholders with actual values
    template
        .replace("{street}", &street)
        .replace("{city}", &city)
        .replace("{region}", &region)
        .replace("{region_abbr}", &region_abbr)
        .replace("{postal}", &postal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::en_us::{CITIES, STATES};

    #[test]
    fn test_generate_street_addresses_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let addresses = generate_street_addresses(&mut rng, Locale::EnUS, 100);
        assert_eq!(addresses.len(), 100);
    }

    #[test]
    fn test_street_address_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let addresses = generate_street_addresses(&mut rng, Locale::EnUS, 50);
        for addr in &addresses {
            let parts: Vec<&str> = addr.split_whitespace().collect();
            assert!(
                parts.len() >= 3,
                "Address should have at least 3 parts: {}",
                addr
            );
            // First part should be numeric for en_US
            assert!(
                parts[0].parse::<u32>().is_ok(),
                "First part should be numeric: {}",
                addr
            );
        }
    }

    #[test]
    fn test_street_address_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let a1 = generate_street_addresses(&mut rng1, Locale::EnUS, 100);
        let a2 = generate_street_addresses(&mut rng2, Locale::EnUS, 100);

        assert_eq!(a1, a2);
    }

    #[test]
    fn test_generate_cities_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let cities = generate_cities(&mut rng, Locale::EnUS, 100);
        assert_eq!(cities.len(), 100);
    }

    #[test]
    fn test_city_from_data() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let cities = generate_cities(&mut rng, Locale::EnUS, 100);
        for city in &cities {
            assert!(
                CITIES.contains(&city.as_str()),
                "City '{}' not in data",
                city
            );
        }
    }

    #[test]
    fn test_generate_states_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let states = generate_states(&mut rng, Locale::EnUS, 100);
        assert_eq!(states.len(), 100);
    }

    #[test]
    fn test_state_from_data() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let states = generate_states(&mut rng, Locale::EnUS, 100);
        for state in &states {
            assert!(
                STATES.contains(&state.as_str()),
                "State '{}' not in data",
                state
            );
        }
    }

    #[test]
    fn test_state_abbr_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let abbrs = generate_state_abbrs(&mut rng, Locale::EnUS, 100);
        for abbr in &abbrs {
            assert_eq!(abbr.len(), 2, "State abbr should be 2 chars: {}", abbr);
            assert!(
                abbr.chars().all(|c| c.is_uppercase()),
                "Should be uppercase: {}",
                abbr
            );
        }
    }

    #[test]
    fn test_generate_countries_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let countries = generate_countries(&mut rng, 100);
        assert_eq!(countries.len(), 100);
    }

    #[test]
    fn test_country_from_data() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let countries = generate_countries(&mut rng, 100);
        for country in &countries {
            assert!(
                COUNTRIES.contains(&country.as_str()),
                "Country '{}' not in data",
                country
            );
        }
    }

    #[test]
    fn test_generate_zip_codes_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let zips = generate_zip_codes(&mut rng, Locale::EnUS, 100);
        assert_eq!(zips.len(), 100);
    }

    #[test]
    fn test_zip_code_format_us() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let zips = generate_zip_codes(&mut rng, Locale::EnUS, 100);
        for zip in &zips {
            // US format is 5 digits or 5-4 format
            assert!(
                zip.len() == 5 || zip.len() == 10,
                "US Zip should be 5 or 10 chars: {}",
                zip
            );
        }
    }

    #[test]
    fn test_zip_code_format_uk() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let zips = generate_zip_codes(&mut rng, Locale::EnGB, 100);
        for zip in &zips {
            // UK format should have letters and numbers
            assert!(
                zip.chars().any(|c| c.is_alphabetic()),
                "UK postcode should have letters: {}",
                zip
            );
        }
    }

    #[test]
    fn test_zip_code_uk_letters_are_randomized() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let zips = generate_zip_codes(&mut rng, Locale::EnGB, 50);
        // Collect all letters from all postcodes
        let letters: Vec<char> = zips
            .iter()
            .flat_map(|z| z.chars().filter(|c| c.is_alphabetic()))
            .collect();

        // Should have multiple different letters (not all 'A')
        let unique_letters: std::collections::HashSet<_> = letters.iter().collect();
        assert!(
            unique_letters.len() > 5,
            "UK postcodes should have varied letters, got: {:?}",
            unique_letters
        );
    }

    #[test]
    fn test_generate_addresses_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let addresses = generate_addresses(&mut rng, Locale::EnUS, 100);
        assert_eq!(addresses.len(), 100);
    }

    #[test]
    fn test_address_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let addresses = generate_addresses(&mut rng, Locale::EnUS, 50);
        for addr in &addresses {
            assert!(addr.contains(','), "Address should have commas: {}", addr);
        }
    }

    #[test]
    fn test_address_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let a1 = generate_addresses(&mut rng1, Locale::EnUS, 100);
        let a2 = generate_addresses(&mut rng2, Locale::EnUS, 100);

        assert_eq!(a1, a2);
    }

    #[test]
    fn test_different_seeds_different_addresses() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let a1 = generate_addresses(&mut rng1, Locale::EnUS, 100);
        let a2 = generate_addresses(&mut rng2, Locale::EnUS, 100);

        assert_ne!(a1, a2, "Different seeds should produce different addresses");
    }

    #[test]
    fn test_empty_batches() {
        let mut rng = ForgeryRng::new();

        assert!(generate_street_addresses(&mut rng, Locale::EnUS, 0).is_empty());
        assert!(generate_cities(&mut rng, Locale::EnUS, 0).is_empty());
        assert!(generate_states(&mut rng, Locale::EnUS, 0).is_empty());
        assert!(generate_countries(&mut rng, 0).is_empty());
        assert!(generate_zip_codes(&mut rng, Locale::EnUS, 0).is_empty());
        assert!(generate_addresses(&mut rng, Locale::EnUS, 0).is_empty());
    }

    #[test]
    fn test_all_locales_generate_addresses() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        for locale in [
            Locale::EnUS,
            Locale::EnGB,
            Locale::DeDE,
            Locale::FrFR,
            Locale::EsES,
            Locale::ItIT,
            Locale::JaJP,
        ] {
            let addr = generate_address(&mut rng, locale);
            assert!(
                !addr.is_empty(),
                "Address should not be empty for {:?}",
                locale
            );
        }
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_street_address_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let addresses = generate_street_addresses(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(addresses.len(), n);
        }

        #[test]
        fn prop_city_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let cities = generate_cities(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(cities.len(), n);
        }

        #[test]
        fn prop_address_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let addresses = generate_addresses(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(addresses.len(), n);
        }

        #[test]
        fn prop_address_seed_determinism(seed_val in any::<u64>(), n in 1usize..50) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let a1 = generate_addresses(&mut rng1, Locale::EnUS, n);
            let a2 = generate_addresses(&mut rng2, Locale::EnUS, n);

            prop_assert_eq!(a1, a2);
        }
    }
}
