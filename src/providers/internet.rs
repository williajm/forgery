//! Internet-related data generation provider.
//!
//! Generates email addresses, URLs, IP addresses, etc.

use crate::data::en_us::FIRST_NAMES;
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
}
