//! Finance-related data generation provider.
//!
//! Generates credit card numbers (with valid Luhn checksum) and IBANs.

use crate::rng::ForgeryRng;

/// Credit card prefixes (IIN ranges) for major card networks.
const CARD_PREFIXES: &[(&str, usize)] = &[
    ("4", 16),    // Visa
    ("51", 16),   // Mastercard
    ("52", 16),   // Mastercard
    ("53", 16),   // Mastercard
    ("54", 16),   // Mastercard
    ("55", 16),   // Mastercard
    ("34", 15),   // American Express
    ("37", 15),   // American Express
    ("6011", 16), // Discover
    ("65", 16),   // Discover
];

/// Country codes and BBAN lengths for IBAN generation.
const IBAN_COUNTRIES: &[(&str, usize)] = &[
    ("DE", 18), // Germany
    ("FR", 23), // France
    ("GB", 18), // United Kingdom
    ("ES", 20), // Spain
    ("IT", 23), // Italy
    ("NL", 14), // Netherlands
    ("BE", 12), // Belgium
    ("AT", 16), // Austria
    ("CH", 17), // Switzerland
    ("PL", 24), // Poland
];

/// Calculate Luhn checksum digit for a partial number string.
/// The returned digit should be appended to make a valid Luhn number.
fn luhn_checksum(number: &str) -> u8 {
    let mut sum: u32 = 0;
    // When calculating check digit, the rightmost digit of the partial number
    // will be doubled (because the check digit we append won't be doubled).
    let mut double = true;

    // Process digits from right to left
    for c in number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            let mut d = digit;
            if double {
                d *= 2;
                if d > 9 {
                    d -= 9;
                }
            }
            sum += d;
            double = !double;
        }
    }

    // Return the digit that makes the sum a multiple of 10
    ((10 - (sum % 10)) % 10) as u8
}

/// Validate a credit card number using the Luhn algorithm.
#[allow(dead_code)]
pub fn validate_luhn(number: &str) -> bool {
    let digits: String = number.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return false;
    }

    let mut sum: u32 = 0;
    let mut double = false;

    for c in digits.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            let mut d = digit;
            if double {
                d *= 2;
                if d > 9 {
                    d -= 9;
                }
            }
            sum += d;
            double = !double;
        }
    }

    sum.is_multiple_of(10)
}

/// Generate a batch of credit card numbers with valid Luhn checksums.
pub fn generate_credit_cards(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut cards = Vec::with_capacity(n);
    for _ in 0..n {
        cards.push(generate_credit_card(rng));
    }
    cards
}

/// Generate a single credit card number with a valid Luhn checksum.
#[inline]
pub fn generate_credit_card(rng: &mut ForgeryRng) -> String {
    let (prefix, total_length) = rng.choose(CARD_PREFIXES);

    // Generate random digits (excluding the check digit)
    let random_length = total_length - prefix.len() - 1;
    let mut number = prefix.to_string();

    for _ in 0..random_length {
        let digit: u8 = rng.gen_range(0, 9);
        number.push((b'0' + digit) as char);
    }

    // Calculate and append check digit
    let check_digit = luhn_checksum(&number);
    number.push((b'0' + check_digit) as char);

    number
}

/// Calculate IBAN check digits (ISO 7064 Mod 97-10).
fn iban_check_digits(country_code: &str, bban: &str) -> String {
    // Move country code to end and append "00"
    let mut check_string = String::with_capacity(bban.len() + 6);
    check_string.push_str(bban);

    // Convert country code letters to numbers (A=10, B=11, etc.)
    for c in country_code.chars() {
        let val = (c as u32) - ('A' as u32) + 10;
        check_string.push_str(&val.to_string());
    }
    check_string.push_str("00");

    // Calculate mod 97
    let mut remainder: u64 = 0;
    for c in check_string.chars() {
        if let Some(digit) = c.to_digit(10) {
            remainder = (remainder * 10 + digit as u64) % 97;
        }
    }

    // Check digits = 98 - remainder
    let check = 98 - remainder;
    format!("{:02}", check)
}

/// Validate an IBAN using ISO 7064 Mod 97-10.
#[allow(dead_code)]
pub fn validate_iban(iban: &str) -> bool {
    // Remove spaces and convert to uppercase
    let clean: String = iban.chars().filter(|c| !c.is_whitespace()).collect();
    let clean = clean.to_uppercase();

    if clean.len() < 5 {
        return false;
    }

    // Move first 4 characters to end
    let rearranged = format!("{}{}", &clean[4..], &clean[..4]);

    // Convert letters to numbers
    let mut numeric = String::new();
    for c in rearranged.chars() {
        if c.is_ascii_digit() {
            numeric.push(c);
        } else if c.is_ascii_alphabetic() {
            let val = (c as u32) - ('A' as u32) + 10;
            numeric.push_str(&val.to_string());
        } else {
            return false;
        }
    }

    // Calculate mod 97
    let mut remainder: u64 = 0;
    for c in numeric.chars() {
        if let Some(digit) = c.to_digit(10) {
            remainder = (remainder * 10 + digit as u64) % 97;
        }
    }

    remainder == 1
}

/// Generate a batch of IBANs with valid checksums.
pub fn generate_ibans(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut ibans = Vec::with_capacity(n);
    for _ in 0..n {
        ibans.push(generate_iban(rng));
    }
    ibans
}

/// Generate a single IBAN with a valid checksum.
#[inline]
pub fn generate_iban(rng: &mut ForgeryRng) -> String {
    let (country_code, bban_length) = rng.choose(IBAN_COUNTRIES);

    // Generate random BBAN (Basic Bank Account Number)
    let mut bban = String::with_capacity(*bban_length);
    for _ in 0..*bban_length {
        let digit: u8 = rng.gen_range(0, 9);
        bban.push((b'0' + digit) as char);
    }

    // Calculate check digits
    let check_digits = iban_check_digits(country_code, &bban);

    format!("{}{}{}", country_code, check_digits, bban)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Credit card tests
    #[test]
    fn test_generate_credit_cards_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let cards = generate_credit_cards(&mut rng, 100);
        assert_eq!(cards.len(), 100);
    }

    #[test]
    fn test_credit_card_valid_luhn() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let cards = generate_credit_cards(&mut rng, 100);
        for card in &cards {
            assert!(
                validate_luhn(card),
                "Credit card should have valid Luhn checksum: {}",
                card
            );
        }
    }

    #[test]
    fn test_credit_card_length() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let cards = generate_credit_cards(&mut rng, 100);
        for card in &cards {
            let len = card.len();
            assert!(
                len == 15 || len == 16,
                "Credit card should be 15 or 16 digits: {} (len {})",
                card,
                len
            );
        }
    }

    #[test]
    fn test_credit_card_all_digits() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let cards = generate_credit_cards(&mut rng, 100);
        for card in &cards {
            assert!(
                card.chars().all(|c| c.is_ascii_digit()),
                "Credit card should only contain digits: {}",
                card
            );
        }
    }

    #[test]
    fn test_credit_card_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let c1 = generate_credit_cards(&mut rng1, 50);
        let c2 = generate_credit_cards(&mut rng2, 50);

        assert_eq!(c1, c2);
    }

    // IBAN tests
    #[test]
    fn test_generate_ibans_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ibans = generate_ibans(&mut rng, 100);
        assert_eq!(ibans.len(), 100);
    }

    #[test]
    fn test_iban_valid_checksum() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ibans = generate_ibans(&mut rng, 100);
        for iban in &ibans {
            assert!(
                validate_iban(iban),
                "IBAN should have valid checksum: {}",
                iban
            );
        }
    }

    #[test]
    fn test_iban_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ibans = generate_ibans(&mut rng, 100);
        for iban in &ibans {
            // Should start with 2 uppercase letters
            assert!(
                iban.chars().take(2).all(|c| c.is_ascii_uppercase()),
                "IBAN should start with country code: {}",
                iban
            );
            // Should have 2 check digits
            assert!(
                iban.chars().skip(2).take(2).all(|c| c.is_ascii_digit()),
                "IBAN should have check digits: {}",
                iban
            );
        }
    }

    #[test]
    fn test_iban_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let i1 = generate_ibans(&mut rng1, 50);
        let i2 = generate_ibans(&mut rng2, 50);

        assert_eq!(i1, i2);
    }

    // Luhn validation tests
    #[test]
    fn test_luhn_known_valid() {
        // Known valid test numbers
        assert!(validate_luhn("4532015112830366")); // Visa test
        assert!(validate_luhn("5425233430109903")); // Mastercard test
        assert!(validate_luhn("4111111111111111")); // Visa test
    }

    #[test]
    fn test_luhn_known_invalid() {
        assert!(!validate_luhn("4532015112830367")); // Changed last digit
        assert!(!validate_luhn("1234567890123456")); // Random number
    }

    // IBAN validation tests
    #[test]
    fn test_iban_known_valid() {
        // Known valid test IBANs
        assert!(validate_iban("DE89370400440532013000")); // Germany
        assert!(validate_iban("GB82WEST12345698765432")); // UK
        assert!(validate_iban("FR1420041010050500013M02606")); // France
    }

    #[test]
    fn test_iban_known_invalid() {
        assert!(!validate_iban("DE89370400440532013001")); // Changed last digit
        assert!(!validate_iban("XX00123456789")); // Invalid format
    }

    #[test]
    fn test_empty_batches() {
        let mut rng = ForgeryRng::new();

        assert!(generate_credit_cards(&mut rng, 0).is_empty());
        assert!(generate_ibans(&mut rng, 0).is_empty());
    }

    #[test]
    fn test_different_seeds_different_cards() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let c1 = generate_credit_cards(&mut rng1, 100);
        let c2 = generate_credit_cards(&mut rng2, 100);

        assert_ne!(c1, c2, "Different seeds should produce different cards");
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: credit card batch size is always respected
        #[test]
        fn prop_credit_card_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let cards = generate_credit_cards(&mut rng, n);
            prop_assert_eq!(cards.len(), n);
        }

        /// Property: all credit cards have valid Luhn checksums
        #[test]
        fn prop_credit_card_valid_luhn(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let cards = generate_credit_cards(&mut rng, n);
            for card in cards {
                prop_assert!(validate_luhn(&card));
            }
        }

        /// Property: IBAN batch size is always respected
        #[test]
        fn prop_iban_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let ibans = generate_ibans(&mut rng, n);
            prop_assert_eq!(ibans.len(), n);
        }

        /// Property: all IBANs have valid checksums
        #[test]
        fn prop_iban_valid_checksum(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let ibans = generate_ibans(&mut rng, n);
            for iban in ibans {
                prop_assert!(validate_iban(&iban));
            }
        }

        /// Property: same seed produces same credit cards
        #[test]
        fn prop_credit_card_seed_determinism(seed_val in any::<u64>(), n in 1usize..50) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let c1 = generate_credit_cards(&mut rng1, n);
            let c2 = generate_credit_cards(&mut rng2, n);

            prop_assert_eq!(c1, c2);
        }

        /// Property: same seed produces same IBANs
        #[test]
        fn prop_iban_seed_determinism(seed_val in any::<u64>(), n in 1usize..50) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let i1 = generate_ibans(&mut rng1, n);
            let i2 = generate_ibans(&mut rng2, n);

            prop_assert_eq!(i1, i2);
        }
    }
}
