//! Phone number generation provider.
//!
//! Generates US-format phone numbers.

use crate::rng::ForgeryRng;

/// Generate a batch of random phone numbers.
///
/// Format: "(XXX) XXX-XXXX"
pub fn generate_phone_numbers(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut numbers = Vec::with_capacity(n);
    for _ in 0..n {
        numbers.push(generate_phone_number(rng));
    }
    numbers
}

/// Generate a single random phone number.
///
/// Format: "(XXX) XXX-XXXX"
#[inline]
pub fn generate_phone_number(rng: &mut ForgeryRng) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_phone_numbers_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let numbers = generate_phone_numbers(&mut rng, 100);
        assert_eq!(numbers.len(), 100);
    }

    #[test]
    fn test_phone_number_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let numbers = generate_phone_numbers(&mut rng, 100);
        for number in &numbers {
            // Format: (XXX) XXX-XXXX = 14 chars
            assert_eq!(number.len(), 14, "Phone should be 14 chars: {}", number);
            assert!(number.starts_with('('), "Should start with (: {}", number);
            assert!(number.contains(')'), "Should have ): {}", number);
            assert!(number.contains('-'), "Should have -: {}", number);
        }
    }

    #[test]
    fn test_phone_number_valid_area_code() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let numbers = generate_phone_numbers(&mut rng, 100);
        for number in &numbers {
            // Extract area code first digit (position 1)
            let first_digit = number.chars().nth(1).unwrap();
            assert!(
                ('2'..='9').contains(&first_digit),
                "Area code first digit should be 2-9: {}",
                number
            );
        }
    }

    #[test]
    fn test_phone_number_valid_exchange() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let numbers = generate_phone_numbers(&mut rng, 100);
        for number in &numbers {
            // Extract exchange first digit (position 6)
            let first_digit = number.chars().nth(6).unwrap();
            assert!(
                ('2'..='9').contains(&first_digit),
                "Exchange first digit should be 2-9: {}",
                number
            );
        }
    }

    #[test]
    fn test_phone_number_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let n1 = generate_phone_numbers(&mut rng1, 100);
        let n2 = generate_phone_numbers(&mut rng2, 100);

        assert_eq!(n1, n2);
    }

    #[test]
    fn test_phone_number_empty_batch() {
        let mut rng = ForgeryRng::new();
        let numbers = generate_phone_numbers(&mut rng, 0);
        assert!(numbers.is_empty());
    }

    #[test]
    fn test_phone_number_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let number = generate_phone_number(&mut rng);
        assert_eq!(number.len(), 14);
    }

    #[test]
    fn test_different_seeds_different_numbers() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let n1 = generate_phone_numbers(&mut rng1, 100);
        let n2 = generate_phone_numbers(&mut rng2, 100);

        assert_ne!(n1, n2, "Different seeds should produce different numbers");
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: batch size is always respected
        #[test]
        fn prop_batch_size_respected(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let numbers = generate_phone_numbers(&mut rng, n);
            prop_assert_eq!(numbers.len(), n);
        }

        /// Property: all phone numbers have correct length
        #[test]
        fn prop_phone_length(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let numbers = generate_phone_numbers(&mut rng, n);
            for number in numbers {
                prop_assert_eq!(number.len(), 14);
            }
        }

        /// Property: same seed produces same output
        #[test]
        fn prop_seed_determinism(seed_val in any::<u64>(), n in 1usize..50) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let n1 = generate_phone_numbers(&mut rng1, n);
            let n2 = generate_phone_numbers(&mut rng2, n);

            prop_assert_eq!(n1, n2);
        }
    }
}
