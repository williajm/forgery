//! Name generation provider.
//!
//! Generates first names, last names, and full names using locale-specific data.

use crate::data::get_locale_data;
use crate::locale::Locale;
use crate::rng::ForgeryRng;

/// Generate a batch of full names (first + last).
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `locale` - The locale for generated names
/// * `n` - Number of names to generate
///
/// # Returns
///
/// A vector of full names
pub fn generate_names(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let data = get_locale_data(locale);
    let first_names = data.first_names().unwrap_or(&[]);
    let last_names = data.last_names().unwrap_or(&[]);
    let family_first = locale.family_name_first();

    let mut names = Vec::with_capacity(n);
    for _ in 0..n {
        let first = if first_names.is_empty() {
            "Unknown"
        } else {
            rng.choose(first_names)
        };
        let last = if last_names.is_empty() {
            "Unknown"
        } else {
            rng.choose(last_names)
        };
        if family_first {
            names.push(format!("{} {}", last, first));
        } else {
            names.push(format!("{} {}", first, last));
        }
    }
    names
}

/// Generate a batch of first names.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `locale` - The locale for generated names
/// * `n` - Number of first names to generate
pub fn generate_first_names(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let data = get_locale_data(locale);
    let first_names = data.first_names().unwrap_or(&[]);

    let mut names = Vec::with_capacity(n);
    for _ in 0..n {
        names.push(if first_names.is_empty() {
            "Unknown".to_string()
        } else {
            rng.choose(first_names).to_string()
        });
    }
    names
}

/// Generate a batch of last names.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `locale` - The locale for generated names
/// * `n` - Number of last names to generate
pub fn generate_last_names(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let data = get_locale_data(locale);
    let last_names = data.last_names().unwrap_or(&[]);

    let mut names = Vec::with_capacity(n);
    for _ in 0..n {
        names.push(if last_names.is_empty() {
            "Unknown".to_string()
        } else {
            rng.choose(last_names).to_string()
        });
    }
    names
}

/// Generate a single full name (first + last).
///
/// More efficient than `generate_names(rng, locale, 1)` as it avoids Vec allocation.
#[inline]
pub fn generate_name(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let first_names = data.first_names().unwrap_or(&[]);
    let last_names = data.last_names().unwrap_or(&[]);

    let first = if first_names.is_empty() {
        "Unknown"
    } else {
        rng.choose(first_names)
    };
    let last = if last_names.is_empty() {
        "Unknown"
    } else {
        rng.choose(last_names)
    };

    if locale.family_name_first() {
        format!("{} {}", last, first)
    } else {
        format!("{} {}", first, last)
    }
}

/// Generate a single first name.
///
/// More efficient than `generate_first_names(rng, locale, 1)` as it avoids Vec allocation.
#[inline]
pub fn generate_first_name(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let first_names = data.first_names().unwrap_or(&[]);
    if first_names.is_empty() {
        "Unknown".to_string()
    } else {
        rng.choose(first_names).to_string()
    }
}

/// Generate a single last name.
///
/// More efficient than `generate_last_names(rng, locale, 1)` as it avoids Vec allocation.
#[inline]
pub fn generate_last_name(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let last_names = data.last_names().unwrap_or(&[]);
    if last_names.is_empty() {
        "Unknown".to_string()
    } else {
        rng.choose(last_names).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::en_us::{FIRST_NAMES, LAST_NAMES};

    #[test]
    fn test_generate_names_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let names = generate_names(&mut rng, Locale::EnUS, 100);
        assert_eq!(names.len(), 100);
    }

    #[test]
    fn test_generate_names_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let names = generate_names(&mut rng, Locale::EnUS, 10);
        for name in &names {
            // Each name should have exactly one space (first + last)
            assert_eq!(name.matches(' ').count(), 1);
            assert!(!name.is_empty());
        }
    }

    #[test]
    fn test_generate_first_names() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let names = generate_first_names(&mut rng, Locale::EnUS, 50);
        assert_eq!(names.len(), 50);
        for name in &names {
            assert!(!name.contains(' '));
            assert!(FIRST_NAMES.contains(&name.as_str()));
        }
    }

    #[test]
    fn test_generate_last_names() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let names = generate_last_names(&mut rng, Locale::EnUS, 50);
        assert_eq!(names.len(), 50);
        for name in &names {
            assert!(LAST_NAMES.contains(&name.as_str()));
        }
    }

    #[test]
    fn test_deterministic_generation() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let names1 = generate_names(&mut rng1, Locale::EnUS, 100);
        let names2 = generate_names(&mut rng2, Locale::EnUS, 100);

        assert_eq!(names1, names2);
    }

    #[test]
    fn test_empty_batch() {
        let mut rng = ForgeryRng::new();
        let names = generate_names(&mut rng, Locale::EnUS, 0);
        assert!(names.is_empty());
    }

    #[test]
    fn test_single_item_batch() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let names = generate_names(&mut rng, Locale::EnUS, 1);
        assert_eq!(names.len(), 1);
        assert!(names[0].contains(' '));
    }

    #[test]
    fn test_all_names_from_data_sources() {
        let mut rng = ForgeryRng::new();
        rng.seed(12345);

        for _ in 0..1000 {
            let name = generate_names(&mut rng, Locale::EnUS, 1).pop().unwrap();
            let parts: Vec<&str> = name.split(' ').collect();
            assert_eq!(parts.len(), 2, "Name should have first and last");
            assert!(
                FIRST_NAMES.contains(&parts[0]),
                "First name '{}' should be in FIRST_NAMES",
                parts[0]
            );
            assert!(
                LAST_NAMES.contains(&parts[1]),
                "Last name '{}' should be in LAST_NAMES",
                parts[1]
            );
        }
    }

    #[test]
    fn test_large_batch_allocation() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let names = generate_names(&mut rng, Locale::EnUS, 10000);
        assert_eq!(names.len(), 10000);

        assert!(names[0].contains(' '));
        assert!(names[9999].contains(' '));
    }

    #[test]
    fn test_different_seeds_produce_different_sequences() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let names1 = generate_names(&mut rng1, Locale::EnUS, 100);
        let names2 = generate_names(&mut rng2, Locale::EnUS, 100);

        assert_ne!(
            names1, names2,
            "Different seeds should produce different names"
        );
    }

    #[test]
    fn test_name_components_not_empty() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let names = generate_names(&mut rng, Locale::EnUS, 100);
        for name in names {
            let parts: Vec<&str> = name.split(' ').collect();
            assert!(!parts[0].is_empty(), "First name should not be empty");
            assert!(!parts[1].is_empty(), "Last name should not be empty");
        }
    }

    #[test]
    fn test_japanese_name_order() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        // Japanese names should be family name first
        let name = generate_name(&mut rng, Locale::JaJP);
        assert!(name.contains(' '), "Name should have a space");
    }

    #[test]
    fn test_german_names() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let name = generate_name(&mut rng, Locale::DeDE);
        assert!(name.contains(' '), "Name should have a space");
    }

    #[test]
    fn test_all_locales_generate_names() {
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
            let name = generate_name(&mut rng, locale);
            assert!(
                !name.is_empty(),
                "Name should not be empty for {:?}",
                locale
            );
            assert!(
                name.contains(' '),
                "Name should have space for {:?}",
                locale
            );
        }
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use crate::data::en_us::{FIRST_NAMES, LAST_NAMES};
    use proptest::prelude::*;

    proptest! {
        /// Property: batch size is always respected
        #[test]
        fn prop_batch_size_respected(n in 0usize..5000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let names = generate_names(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(names.len(), n);

            let first_names = generate_first_names(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(first_names.len(), n);

            let last_names = generate_last_names(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(last_names.len(), n);
        }

        /// Property: all generated names have exactly one space
        #[test]
        fn prop_name_format_valid(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let names = generate_names(&mut rng, Locale::EnUS, n);
            for name in names {
                prop_assert_eq!(name.matches(' ').count(), 1);
            }
        }

        /// Property: same seed always produces same output
        #[test]
        fn prop_seed_determinism(seed_val in any::<u64>(), n in 1usize..100) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let names1 = generate_names(&mut rng1, Locale::EnUS, n);
            let names2 = generate_names(&mut rng2, Locale::EnUS, n);

            prop_assert_eq!(names1, names2);
        }

        /// Property: first names contain no spaces
        #[test]
        fn prop_first_names_no_spaces(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let names = generate_first_names(&mut rng, Locale::EnUS, n);
            for name in names {
                prop_assert!(!name.contains(' '));
            }
        }

        /// Property: all first names come from data source
        #[test]
        fn prop_first_names_from_data(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let names = generate_first_names(&mut rng, Locale::EnUS, n);
            for name in names {
                prop_assert!(FIRST_NAMES.contains(&name.as_str()));
            }
        }

        /// Property: all last names come from data source
        #[test]
        fn prop_last_names_from_data(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let names = generate_last_names(&mut rng, Locale::EnUS, n);
            for name in names {
                prop_assert!(LAST_NAMES.contains(&name.as_str()));
            }
        }
    }
}
