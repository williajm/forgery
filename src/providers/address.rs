//! Address generation provider.
//!
//! Generates addresses, street names, cities, states, countries, and zip codes.

use crate::data::en_us::{CITIES, COUNTRIES, STATES, STATE_ABBRS, STREET_NAMES, STREET_SUFFIXES};
use crate::rng::ForgeryRng;

/// Generate a batch of random street addresses.
///
/// Format: "123 Main Street"
pub fn generate_street_addresses(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut addresses = Vec::with_capacity(n);
    for _ in 0..n {
        addresses.push(generate_street_address(rng));
    }
    addresses
}

/// Generate a single random street address.
#[inline]
pub fn generate_street_address(rng: &mut ForgeryRng) -> String {
    let number: u32 = rng.gen_range(1, 9999);
    let street = rng.choose(STREET_NAMES);
    let suffix = rng.choose(STREET_SUFFIXES);
    format!("{} {} {}", number, street, suffix)
}

/// Generate a batch of random city names.
pub fn generate_cities(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut cities = Vec::with_capacity(n);
    for _ in 0..n {
        cities.push(generate_city(rng));
    }
    cities
}

/// Generate a single random city name.
#[inline]
pub fn generate_city(rng: &mut ForgeryRng) -> String {
    rng.choose(CITIES).to_string()
}

/// Generate a batch of random state names.
pub fn generate_states(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut states = Vec::with_capacity(n);
    for _ in 0..n {
        states.push(generate_state(rng));
    }
    states
}

/// Generate a single random state name.
#[inline]
pub fn generate_state(rng: &mut ForgeryRng) -> String {
    rng.choose(STATES).to_string()
}

/// Generate a batch of random state abbreviations.
#[allow(dead_code)]
pub fn generate_state_abbrs(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut abbrs = Vec::with_capacity(n);
    for _ in 0..n {
        abbrs.push(generate_state_abbr(rng));
    }
    abbrs
}

/// Generate a single random state abbreviation.
#[inline]
pub fn generate_state_abbr(rng: &mut ForgeryRng) -> String {
    rng.choose(STATE_ABBRS).to_string()
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
    rng.choose(COUNTRIES).to_string()
}

/// Generate a batch of random zip codes.
///
/// Format: "12345" or "12345-6789"
pub fn generate_zip_codes(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut zips = Vec::with_capacity(n);
    for _ in 0..n {
        zips.push(generate_zip_code(rng));
    }
    zips
}

/// Generate a single random zip code.
#[inline]
pub fn generate_zip_code(rng: &mut ForgeryRng) -> String {
    let zip5: u32 = rng.gen_range(10000, 99999);
    // 50% chance of extended zip code
    if rng.gen_range(0u8, 1) == 1 {
        let zip4: u32 = rng.gen_range(1000, 9999);
        format!("{:05}-{:04}", zip5, zip4)
    } else {
        format!("{:05}", zip5)
    }
}

/// Generate a batch of random full addresses.
///
/// Format: "123 Main Street, City, ST 12345"
pub fn generate_addresses(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut addresses = Vec::with_capacity(n);
    for _ in 0..n {
        addresses.push(generate_address(rng));
    }
    addresses
}

/// Generate a single random full address.
#[inline]
pub fn generate_address(rng: &mut ForgeryRng) -> String {
    let street = generate_street_address(rng);
    let city = generate_city(rng);
    let state = generate_state_abbr(rng);
    let zip = generate_zip_code(rng);
    format!("{}, {}, {} {}", street, city, state, zip)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Street address tests
    #[test]
    fn test_generate_street_addresses_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let addresses = generate_street_addresses(&mut rng, 100);
        assert_eq!(addresses.len(), 100);
    }

    #[test]
    fn test_street_address_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let addresses = generate_street_addresses(&mut rng, 50);
        for addr in &addresses {
            // Should have at least 3 parts: number, street, suffix
            let parts: Vec<&str> = addr.split_whitespace().collect();
            assert!(
                parts.len() >= 3,
                "Address should have at least 3 parts: {}",
                addr
            );
            // First part should be numeric
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

        let a1 = generate_street_addresses(&mut rng1, 100);
        let a2 = generate_street_addresses(&mut rng2, 100);

        assert_eq!(a1, a2);
    }

    // City tests
    #[test]
    fn test_generate_cities_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let cities = generate_cities(&mut rng, 100);
        assert_eq!(cities.len(), 100);
    }

    #[test]
    fn test_city_from_data() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let cities = generate_cities(&mut rng, 100);
        for city in &cities {
            assert!(
                CITIES.contains(&city.as_str()),
                "City '{}' not in data",
                city
            );
        }
    }

    // State tests
    #[test]
    fn test_generate_states_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let states = generate_states(&mut rng, 100);
        assert_eq!(states.len(), 100);
    }

    #[test]
    fn test_state_from_data() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let states = generate_states(&mut rng, 100);
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

        let abbrs = generate_state_abbrs(&mut rng, 100);
        for abbr in &abbrs {
            assert_eq!(abbr.len(), 2, "State abbr should be 2 chars: {}", abbr);
            assert!(
                abbr.chars().all(|c| c.is_uppercase()),
                "Should be uppercase: {}",
                abbr
            );
        }
    }

    // Country tests
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

    // Zip code tests
    #[test]
    fn test_generate_zip_codes_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let zips = generate_zip_codes(&mut rng, 100);
        assert_eq!(zips.len(), 100);
    }

    #[test]
    fn test_zip_code_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let zips = generate_zip_codes(&mut rng, 100);
        for zip in &zips {
            // Either 5 digits or 5-4 format
            assert!(
                zip.len() == 5 || zip.len() == 10,
                "Zip should be 5 or 10 chars: {}",
                zip
            );
            if zip.len() == 10 {
                assert!(zip.contains('-'), "Extended zip should have dash: {}", zip);
            }
        }
    }

    // Full address tests
    #[test]
    fn test_generate_addresses_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let addresses = generate_addresses(&mut rng, 100);
        assert_eq!(addresses.len(), 100);
    }

    #[test]
    fn test_address_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let addresses = generate_addresses(&mut rng, 50);
        for addr in &addresses {
            // Should contain commas
            assert!(addr.contains(','), "Address should have commas: {}", addr);
        }
    }

    #[test]
    fn test_address_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let a1 = generate_addresses(&mut rng1, 100);
        let a2 = generate_addresses(&mut rng2, 100);

        assert_eq!(a1, a2);
    }

    #[test]
    fn test_different_seeds_different_addresses() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let a1 = generate_addresses(&mut rng1, 100);
        let a2 = generate_addresses(&mut rng2, 100);

        assert_ne!(a1, a2, "Different seeds should produce different addresses");
    }

    #[test]
    fn test_empty_batches() {
        let mut rng = ForgeryRng::new();

        assert!(generate_street_addresses(&mut rng, 0).is_empty());
        assert!(generate_cities(&mut rng, 0).is_empty());
        assert!(generate_states(&mut rng, 0).is_empty());
        assert!(generate_countries(&mut rng, 0).is_empty());
        assert!(generate_zip_codes(&mut rng, 0).is_empty());
        assert!(generate_addresses(&mut rng, 0).is_empty());
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: street address batch size is always respected
        #[test]
        fn prop_street_address_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let addresses = generate_street_addresses(&mut rng, n);
            prop_assert_eq!(addresses.len(), n);
        }

        /// Property: city batch size is always respected
        #[test]
        fn prop_city_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let cities = generate_cities(&mut rng, n);
            prop_assert_eq!(cities.len(), n);
        }

        /// Property: all cities come from data
        #[test]
        fn prop_city_from_data(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let cities = generate_cities(&mut rng, n);
            for city in cities {
                prop_assert!(CITIES.contains(&city.as_str()));
            }
        }

        /// Property: address batch size is always respected
        #[test]
        fn prop_address_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let addresses = generate_addresses(&mut rng, n);
            prop_assert_eq!(addresses.len(), n);
        }

        /// Property: same seed produces same output
        #[test]
        fn prop_address_seed_determinism(seed_val in any::<u64>(), n in 1usize..50) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let a1 = generate_addresses(&mut rng1, n);
            let a2 = generate_addresses(&mut rng2, n);

            prop_assert_eq!(a1, a2);
        }
    }
}
