//! Internet-related data generation provider.
//!
//! Generates email addresses, URLs, IP addresses, etc.

use crate::data::en_us::{FREE_EMAIL_DOMAINS, SAFE_EMAIL_DOMAINS};
use crate::data::get_locale_data;
use crate::locale::Locale;
use crate::rng::ForgeryRng;

/// Common email domains for generation.
const EMAIL_DOMAINS: &[&str] = &[
    "gmail.com",
    "yahoo.com",
    "hotmail.com",
    "outlook.com",
    "icloud.com",
    "protonmail.com",
    "mail.com",
    "aol.com",
];

/// Generate a batch of email addresses.
pub fn generate_emails(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut emails = Vec::with_capacity(n);
    for _ in 0..n {
        emails.push(generate_email(rng, locale));
    }
    emails
}

/// Generate a single email address.
///
/// Uses romanized first names for non-Latin locales.
#[inline]
pub fn generate_email(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    // Use romanized names for email (important for non-Latin scripts like Japanese)
    let names = data.romanized_first_names().unwrap_or(&[]);
    let name = if names.is_empty() {
        "user"
    } else {
        rng.choose(names)
    };
    let num: u16 = rng.gen_range(1, 999);
    let domain = rng.choose(EMAIL_DOMAINS);
    format!("{}{:03}@{}", name.to_lowercase(), num, domain)
}

/// Generate a batch of safe email addresses.
///
/// Safe emails use example.com/org/net domains that are reserved for testing
/// and documentation (RFC 2606).
pub fn generate_safe_emails(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut emails = Vec::with_capacity(n);
    for _ in 0..n {
        emails.push(generate_safe_email(rng, locale));
    }
    emails
}

/// Generate a single safe email address.
///
/// Uses example.com, example.org, or example.net (RFC 2606 reserved domains).
#[inline]
pub fn generate_safe_email(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let names = data.romanized_first_names().unwrap_or(&[]);
    let name = if names.is_empty() {
        "user"
    } else {
        rng.choose(names)
    };
    let num: u16 = rng.gen_range(1, 999);
    let domain = rng.choose(SAFE_EMAIL_DOMAINS);
    format!("{}{:03}@{}", name.to_lowercase(), num, domain)
}

/// Generate a batch of free email addresses.
///
/// Free emails use common free email provider domains (gmail.com, yahoo.com, etc.).
pub fn generate_free_emails(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut emails = Vec::with_capacity(n);
    for _ in 0..n {
        emails.push(generate_free_email(rng, locale));
    }
    emails
}

/// Generate a single free email address.
///
/// Uses common free email providers like gmail.com, yahoo.com, etc.
#[inline]
pub fn generate_free_email(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let names = data.romanized_first_names().unwrap_or(&[]);
    let name = if names.is_empty() {
        "user"
    } else {
        rng.choose(names)
    };
    let num: u16 = rng.gen_range(1, 999);
    let domain = rng.choose(FREE_EMAIL_DOMAINS);
    format!("{}{:03}@{}", name.to_lowercase(), num, domain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_emails_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, Locale::EnUS, 100);
        assert_eq!(emails.len(), 100);
    }

    #[test]
    fn test_generate_emails_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, Locale::EnUS, 50);
        for email in &emails {
            // Basic email validation
            assert!(email.contains('@'));
            assert!(email.contains('.'));

            let parts: Vec<&str> = email.split('@').collect();
            assert_eq!(parts.len(), 2);

            // Local part should not be empty
            assert!(!parts[0].is_empty());

            // Domain should be one of our known domains
            assert!(EMAIL_DOMAINS.contains(&parts[1]));
        }
    }

    #[test]
    fn test_emails_are_lowercase() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, Locale::EnUS, 100);
        for email in &emails {
            let local_part = email.split('@').next().unwrap();
            let name_part: String = local_part.chars().filter(|c| c.is_alphabetic()).collect();
            assert_eq!(name_part, name_part.to_lowercase());
        }
    }

    #[test]
    fn test_deterministic_generation() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let emails1 = generate_emails(&mut rng1, Locale::EnUS, 100);
        let emails2 = generate_emails(&mut rng2, Locale::EnUS, 100);

        assert_eq!(emails1, emails2);
    }

    #[test]
    fn test_empty_batch() {
        let mut rng = ForgeryRng::new();
        let emails = generate_emails(&mut rng, Locale::EnUS, 0);
        assert!(emails.is_empty());
    }

    #[test]
    fn test_single_email_batch() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, Locale::EnUS, 1);
        assert_eq!(emails.len(), 1);
        assert!(emails[0].contains('@'));
    }

    #[test]
    fn test_email_number_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, Locale::EnUS, 100);
        for email in &emails {
            let local = email.split('@').next().unwrap();
            let digits: String = local.chars().filter(|c| c.is_ascii_digit()).collect();
            assert_eq!(
                digits.len(),
                3,
                "Email should have 3-digit number: {}",
                email
            );
        }
    }

    #[test]
    fn test_all_domains_used() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, Locale::EnUS, 1000);
        let domains: std::collections::HashSet<_> = emails
            .iter()
            .map(|e| e.split('@').nth(1).unwrap())
            .collect();

        assert!(domains.len() >= 2, "Should use multiple domains");
    }

    #[test]
    fn test_different_seeds_different_results() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let emails1 = generate_emails(&mut rng1, Locale::EnUS, 100);
        let emails2 = generate_emails(&mut rng2, Locale::EnUS, 100);

        assert_ne!(
            emails1, emails2,
            "Different seeds should produce different emails"
        );
    }

    #[test]
    fn test_large_batch() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, Locale::EnUS, 10000);
        assert_eq!(emails.len(), 10000);

        assert!(emails[0].contains('@'));
        assert!(emails[9999].contains('@'));
    }

    #[test]
    fn test_email_local_part_structure() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, Locale::EnUS, 100);
        for email in &emails {
            let parts: Vec<&str> = email.split('@').collect();
            let local = parts[0];

            let has_letters = local.chars().any(|c| c.is_alphabetic());
            let has_digits = local.chars().any(|c| c.is_ascii_digit());

            assert!(has_letters, "Local part should have letters");
            assert!(has_digits, "Local part should have digits");
        }
    }

    #[test]
    fn test_generate_safe_emails_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_safe_emails(&mut rng, Locale::EnUS, 100);
        assert_eq!(emails.len(), 100);
    }

    #[test]
    fn test_safe_email_uses_safe_domains() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_safe_emails(&mut rng, Locale::EnUS, 100);
        for email in &emails {
            let domain = email.split('@').nth(1).unwrap();
            assert!(
                SAFE_EMAIL_DOMAINS.contains(&domain),
                "Safe email should use safe domain: {}",
                email
            );
        }
    }

    #[test]
    fn test_safe_email_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let email = generate_safe_email(&mut rng, Locale::EnUS);
        let domain = email.split('@').nth(1).unwrap();
        assert!(SAFE_EMAIL_DOMAINS.contains(&domain));
    }

    #[test]
    fn test_generate_free_emails_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_free_emails(&mut rng, Locale::EnUS, 100);
        assert_eq!(emails.len(), 100);
    }

    #[test]
    fn test_free_email_uses_free_domains() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_free_emails(&mut rng, Locale::EnUS, 100);
        for email in &emails {
            let domain = email.split('@').nth(1).unwrap();
            assert!(
                FREE_EMAIL_DOMAINS.contains(&domain),
                "Free email should use free domain: {}",
                email
            );
        }
    }

    #[test]
    fn test_free_email_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let email = generate_free_email(&mut rng, Locale::EnUS);
        let domain = email.split('@').nth(1).unwrap();
        assert!(FREE_EMAIL_DOMAINS.contains(&domain));
    }

    #[test]
    fn test_safe_email_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let e1 = generate_safe_emails(&mut rng1, Locale::EnUS, 50);
        let e2 = generate_safe_emails(&mut rng2, Locale::EnUS, 50);

        assert_eq!(e1, e2);
    }

    #[test]
    fn test_free_email_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let e1 = generate_free_emails(&mut rng1, Locale::EnUS, 50);
        let e2 = generate_free_emails(&mut rng2, Locale::EnUS, 50);

        assert_eq!(e1, e2);
    }

    #[test]
    fn test_empty_safe_batch() {
        let mut rng = ForgeryRng::new();
        assert!(generate_safe_emails(&mut rng, Locale::EnUS, 0).is_empty());
    }

    #[test]
    fn test_empty_free_batch() {
        let mut rng = ForgeryRng::new();
        assert!(generate_free_emails(&mut rng, Locale::EnUS, 0).is_empty());
    }

    #[test]
    fn test_all_locales_generate_email() {
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
            let email = generate_email(&mut rng, locale);
            assert!(email.contains('@'), "Email should have @ for {:?}", locale);
            // Verify email is ASCII (important for ja_JP)
            assert!(
                email.is_ascii(),
                "Email should be ASCII for {:?}: {}",
                locale,
                email
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
        fn prop_batch_size_respected(n in 0usize..1000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_emails(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(emails.len(), n);
        }

        #[test]
        fn prop_email_has_one_at(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_emails(&mut rng, Locale::EnUS, n);
            for email in emails {
                prop_assert_eq!(email.matches('@').count(), 1);
            }
        }

        #[test]
        fn prop_valid_domains(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_emails(&mut rng, Locale::EnUS, n);
            for email in emails {
                let domain = email.split('@').nth(1).unwrap();
                prop_assert!(EMAIL_DOMAINS.contains(&domain));
            }
        }

        #[test]
        fn prop_seed_determinism(seed_val in any::<u64>(), n in 1usize..100) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let emails1 = generate_emails(&mut rng1, Locale::EnUS, n);
            let emails2 = generate_emails(&mut rng2, Locale::EnUS, n);

            prop_assert_eq!(emails1, emails2);
        }

        #[test]
        fn prop_lowercase_emails(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_emails(&mut rng, Locale::EnUS, n);
            for email in &emails {
                prop_assert_eq!(email, &email.to_lowercase());
            }
        }

        #[test]
        fn prop_local_part_format(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_emails(&mut rng, Locale::EnUS, n);
            for email in emails {
                let local = email.split('@').next().unwrap();
                let digits: String = local.chars().filter(|c| c.is_ascii_digit()).collect();
                prop_assert_eq!(digits.len(), 3);
            }
        }

        #[test]
        fn prop_safe_email_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_safe_emails(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(emails.len(), n);
        }

        #[test]
        fn prop_safe_email_domains(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_safe_emails(&mut rng, Locale::EnUS, n);
            for email in emails {
                let domain = email.split('@').nth(1).unwrap();
                prop_assert!(SAFE_EMAIL_DOMAINS.contains(&domain));
            }
        }

        #[test]
        fn prop_free_email_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_free_emails(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(emails.len(), n);
        }

        #[test]
        fn prop_free_email_domains(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_free_emails(&mut rng, Locale::EnUS, n);
            for email in emails {
                let domain = email.split('@').nth(1).unwrap();
                prop_assert!(FREE_EMAIL_DOMAINS.contains(&domain));
            }
        }
    }
}
