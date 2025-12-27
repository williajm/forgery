//! Phone number generation provider.
//!
//! Generates locale-specific phone numbers.

use crate::data::get_locale_data;
use crate::locale::Locale;
use crate::rng::ForgeryRng;

/// Generate a batch of random phone numbers.
pub fn generate_phone_numbers(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut numbers = Vec::with_capacity(n);
    for _ in 0..n {
        numbers.push(generate_phone_number(rng, locale));
    }
    numbers
}

/// Generate a single random phone number.
#[inline]
pub fn generate_phone_number(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    if let Some(format) = data.phone_format() {
        if !format.patterns.is_empty() {
            let pattern = rng.choose(format.patterns);
            return expand_pattern(rng, pattern);
        }
    }
    // Default US format
    generate_us_phone_number(rng)
}

/// Generate a US-format phone number.
fn generate_us_phone_number(rng: &mut ForgeryRng) -> String {
    // Area code: first digit 2-9, next two digits 0-9
    let area1: u8 = rng.gen_range(2, 9);
    let area2: u8 = rng.gen_range(0, 9);
    let area3: u8 = rng.gen_range(0, 9);

    // Exchange code: first digit 2-9, next two digits 0-9
    let ex1: u8 = rng.gen_range(2, 9);
    let ex2: u8 = rng.gen_range(0, 9);
    let ex3: u8 = rng.gen_range(0, 9);

    // Subscriber number: 4 digits
    let sub: u16 = rng.gen_range(0, 9999);

    format!(
        "({}{}{}) {}{}{}-{:04}",
        area1, area2, area3, ex1, ex2, ex3, sub
    )
}

/// Expand a format pattern where # is a digit.
fn expand_pattern(rng: &mut ForgeryRng, pattern: &str) -> String {
    pattern
        .chars()
        .map(|c| match c {
            '#' => {
                let digit = rng.gen_range(0u8, 9);
                char::from_digit(digit as u32, 10).unwrap()
            }
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_phone_numbers_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let numbers = generate_phone_numbers(&mut rng, Locale::EnUS, 100);
        assert_eq!(numbers.len(), 100);
    }

    #[test]
    fn test_phone_number_format_us() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let numbers = generate_phone_numbers(&mut rng, Locale::EnUS, 100);
        for number in &numbers {
            // US phone numbers should:
            // - Not be empty
            // - Contain digits
            // - Contain formatting characters (dashes, spaces, or parens)
            assert!(!number.is_empty(), "Phone should not be empty");
            assert!(
                number.chars().any(|c| c.is_ascii_digit()),
                "Phone should have digits: {}",
                number
            );
            // Should have some formatting character
            assert!(
                number.contains('-') || number.contains(' ') || number.contains('('),
                "Phone should have formatting: {}",
                number
            );
        }
    }

    #[test]
    fn test_phone_number_has_digits() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let numbers = generate_phone_numbers(&mut rng, Locale::EnUS, 100);
        for number in &numbers {
            // Count the digits
            let digit_count: usize = number.chars().filter(|c| c.is_ascii_digit()).count();
            assert!(
                digit_count >= 10,
                "Phone should have at least 10 digits: {} (has {})",
                number,
                digit_count
            );
        }
    }

    #[test]
    fn test_phone_number_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let n1 = generate_phone_numbers(&mut rng1, Locale::EnUS, 100);
        let n2 = generate_phone_numbers(&mut rng2, Locale::EnUS, 100);

        assert_eq!(n1, n2);
    }

    #[test]
    fn test_phone_number_empty_batch() {
        let mut rng = ForgeryRng::new();
        let numbers = generate_phone_numbers(&mut rng, Locale::EnUS, 0);
        assert!(numbers.is_empty());
    }

    #[test]
    fn test_phone_number_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let number = generate_phone_number(&mut rng, Locale::EnUS);
        assert!(!number.is_empty());
    }

    #[test]
    fn test_different_seeds_different_numbers() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let n1 = generate_phone_numbers(&mut rng1, Locale::EnUS, 100);
        let n2 = generate_phone_numbers(&mut rng2, Locale::EnUS, 100);

        assert_ne!(n1, n2, "Different seeds should produce different numbers");
    }

    #[test]
    fn test_all_locales_generate_phones() {
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
            let number = generate_phone_number(&mut rng, locale);
            assert!(
                !number.is_empty(),
                "Phone should not be empty for {:?}",
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
        fn prop_batch_size_respected(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let numbers = generate_phone_numbers(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(numbers.len(), n);
        }

        #[test]
        fn prop_seed_determinism(seed_val in any::<u64>(), n in 1usize..50) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let n1 = generate_phone_numbers(&mut rng1, Locale::EnUS, n);
            let n2 = generate_phone_numbers(&mut rng2, Locale::EnUS, n);

            prop_assert_eq!(n1, n2);
        }
    }
}
