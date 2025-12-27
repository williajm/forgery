//! Internet-related data generation provider.
//!
//! Generates email addresses, URLs, IP addresses, etc.

use crate::data::en_us::{FIRST_NAMES, FREE_EMAIL_DOMAINS, SAFE_EMAIL_DOMAINS};
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
///
/// Emails are generated in the format: firstname123@domain.com
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of emails to generate
pub fn generate_emails(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut emails = Vec::with_capacity(n);
    for _ in 0..n {
        let name = rng.choose(FIRST_NAMES).to_lowercase();
        let num: u16 = rng.gen_range(1, 999);
        let domain = rng.choose(EMAIL_DOMAINS);
        emails.push(format!("{}{:03}@{}", name, num, domain));
    }
    emails
}

/// Generate a single email address.
///
/// More efficient than `generate_emails(rng, 1)` as it avoids Vec allocation.
#[inline]
pub fn generate_email(rng: &mut ForgeryRng) -> String {
    let name = rng.choose(FIRST_NAMES).to_lowercase();
    let num: u16 = rng.gen_range(1, 999);
    let domain = rng.choose(EMAIL_DOMAINS);
    format!("{}{:03}@{}", name, num, domain)
}

/// Generate a batch of safe email addresses.
///
/// Safe emails use example.com/org/net domains that are reserved for testing
/// and documentation (RFC 2606).
pub fn generate_safe_emails(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut emails = Vec::with_capacity(n);
    for _ in 0..n {
        emails.push(generate_safe_email(rng));
    }
    emails
}

/// Generate a single safe email address.
///
/// Uses example.com, example.org, or example.net (RFC 2606 reserved domains).
#[inline]
pub fn generate_safe_email(rng: &mut ForgeryRng) -> String {
    let name = rng.choose(FIRST_NAMES).to_lowercase();
    let num: u16 = rng.gen_range(1, 999);
    let domain = rng.choose(SAFE_EMAIL_DOMAINS);
    format!("{}{:03}@{}", name, num, domain)
}

/// Generate a batch of free email addresses.
///
/// Free emails use common free email provider domains (gmail.com, yahoo.com, etc.).
pub fn generate_free_emails(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut emails = Vec::with_capacity(n);
    for _ in 0..n {
        emails.push(generate_free_email(rng));
    }
    emails
}

/// Generate a single free email address.
///
/// Uses common free email providers like gmail.com, yahoo.com, etc.
#[inline]
pub fn generate_free_email(rng: &mut ForgeryRng) -> String {
    let name = rng.choose(FIRST_NAMES).to_lowercase();
    let num: u16 = rng.gen_range(1, 999);
    let domain = rng.choose(FREE_EMAIL_DOMAINS);
    format!("{}{:03}@{}", name, num, domain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_emails_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, 100);
        assert_eq!(emails.len(), 100);
    }

    #[test]
    fn test_generate_emails_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, 50);
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

        let emails = generate_emails(&mut rng, 100);
        for email in &emails {
            let local_part = email.split('@').next().unwrap();
            // The name part should be lowercase (numbers don't have case)
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

        let emails1 = generate_emails(&mut rng1, 100);
        let emails2 = generate_emails(&mut rng2, 100);

        assert_eq!(emails1, emails2);
    }

    // Edge case tests
    #[test]
    fn test_empty_batch() {
        let mut rng = ForgeryRng::new();
        let emails = generate_emails(&mut rng, 0);
        assert!(emails.is_empty());
    }

    #[test]
    fn test_single_email_batch() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, 1);
        assert_eq!(emails.len(), 1);
        assert!(emails[0].contains('@'));
    }

    #[test]
    fn test_email_number_format() {
        // Verify that emails have a 3-digit number
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, 100);
        for email in &emails {
            let local = email.split('@').next().unwrap();
            // Extract digits
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
        // With enough samples, we should see multiple domains used
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, 1000);
        let domains: std::collections::HashSet<_> = emails
            .iter()
            .map(|e| e.split('@').nth(1).unwrap())
            .collect();

        // Should see at least 2 different domains with 1000 samples
        assert!(domains.len() >= 2, "Should use multiple domains");
    }

    #[test]
    fn test_different_seeds_different_results() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let emails1 = generate_emails(&mut rng1, 100);
        let emails2 = generate_emails(&mut rng2, 100);

        assert_ne!(
            emails1, emails2,
            "Different seeds should produce different emails"
        );
    }

    #[test]
    fn test_large_batch() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, 10000);
        assert_eq!(emails.len(), 10000);

        // Verify first and last are valid
        assert!(emails[0].contains('@'));
        assert!(emails[9999].contains('@'));
    }

    #[test]
    fn test_email_local_part_structure() {
        // Email format is: name + 3-digit number + @ + domain
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_emails(&mut rng, 100);
        for email in &emails {
            let parts: Vec<&str> = email.split('@').collect();
            let local = parts[0];

            // Local should have letters followed by digits
            let has_letters = local.chars().any(|c| c.is_alphabetic());
            let has_digits = local.chars().any(|c| c.is_ascii_digit());

            assert!(has_letters, "Local part should have letters");
            assert!(has_digits, "Local part should have digits");
        }
    }

    // Safe email tests
    #[test]
    fn test_generate_safe_emails_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_safe_emails(&mut rng, 100);
        assert_eq!(emails.len(), 100);
    }

    #[test]
    fn test_safe_email_uses_safe_domains() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_safe_emails(&mut rng, 100);
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

        let email = generate_safe_email(&mut rng);
        let domain = email.split('@').nth(1).unwrap();
        assert!(SAFE_EMAIL_DOMAINS.contains(&domain));
    }

    // Free email tests
    #[test]
    fn test_generate_free_emails_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_free_emails(&mut rng, 100);
        assert_eq!(emails.len(), 100);
    }

    #[test]
    fn test_free_email_uses_free_domains() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let emails = generate_free_emails(&mut rng, 100);
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

        let email = generate_free_email(&mut rng);
        let domain = email.split('@').nth(1).unwrap();
        assert!(FREE_EMAIL_DOMAINS.contains(&domain));
    }

    #[test]
    fn test_safe_email_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let e1 = generate_safe_emails(&mut rng1, 50);
        let e2 = generate_safe_emails(&mut rng2, 50);

        assert_eq!(e1, e2);
    }

    #[test]
    fn test_free_email_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let e1 = generate_free_emails(&mut rng1, 50);
        let e2 = generate_free_emails(&mut rng2, 50);

        assert_eq!(e1, e2);
    }

    #[test]
    fn test_empty_safe_batch() {
        let mut rng = ForgeryRng::new();
        assert!(generate_safe_emails(&mut rng, 0).is_empty());
    }

    #[test]
    fn test_empty_free_batch() {
        let mut rng = ForgeryRng::new();
        assert!(generate_free_emails(&mut rng, 0).is_empty());
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: batch size is always respected
        #[test]
        fn prop_batch_size_respected(n in 0usize..1000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_emails(&mut rng, n);
            prop_assert_eq!(emails.len(), n);
        }

        /// Property: all emails contain exactly one @
        #[test]
        fn prop_email_has_one_at(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_emails(&mut rng, n);
            for email in emails {
                prop_assert_eq!(email.matches('@').count(), 1);
            }
        }

        /// Property: all emails have valid domains
        #[test]
        fn prop_valid_domains(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_emails(&mut rng, n);
            for email in emails {
                let domain = email.split('@').nth(1).unwrap();
                prop_assert!(EMAIL_DOMAINS.contains(&domain));
            }
        }

        /// Property: same seed produces same output
        #[test]
        fn prop_seed_determinism(seed_val in any::<u64>(), n in 1usize..100) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let emails1 = generate_emails(&mut rng1, n);
            let emails2 = generate_emails(&mut rng2, n);

            prop_assert_eq!(emails1, emails2);
        }

        /// Property: all emails are lowercase
        #[test]
        fn prop_lowercase_emails(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_emails(&mut rng, n);
            for email in &emails {
                prop_assert_eq!(email, &email.to_lowercase());
            }
        }

        /// Property: local part has expected format (letters + 3 digits)
        #[test]
        fn prop_local_part_format(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_emails(&mut rng, n);
            for email in emails {
                let local = email.split('@').next().unwrap();
                let digits: String = local.chars().filter(|c| c.is_ascii_digit()).collect();
                prop_assert_eq!(digits.len(), 3);
            }
        }

        /// Property: safe email batch size is respected
        #[test]
        fn prop_safe_email_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_safe_emails(&mut rng, n);
            prop_assert_eq!(emails.len(), n);
        }

        /// Property: safe emails use valid domains
        #[test]
        fn prop_safe_email_domains(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_safe_emails(&mut rng, n);
            for email in emails {
                let domain = email.split('@').nth(1).unwrap();
                prop_assert!(SAFE_EMAIL_DOMAINS.contains(&domain));
            }
        }

        /// Property: free email batch size is respected
        #[test]
        fn prop_free_email_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_free_emails(&mut rng, n);
            prop_assert_eq!(emails.len(), n);
        }

        /// Property: free emails use valid domains
        #[test]
        fn prop_free_email_domains(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let emails = generate_free_emails(&mut rng, n);
            for email in emails {
                let domain = email.split('@').nth(1).unwrap();
                prop_assert!(FREE_EMAIL_DOMAINS.contains(&domain));
            }
        }
    }
}
