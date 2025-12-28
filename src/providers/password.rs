//! Password generation provider.
//!
//! Generates random passwords with configurable character sets and length.

use crate::rng::ForgeryRng;

/// Lowercase letters.
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

/// Uppercase letters.
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Digits.
const DIGITS: &[u8] = b"0123456789";

/// Common symbols.
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{}|;:,.<>?";

/// Generate a batch of passwords.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of passwords to generate
/// * `length` - Length of each password
/// * `uppercase` - Include uppercase letters
/// * `lowercase` - Include lowercase letters
/// * `digits` - Include digits
/// * `symbols` - Include symbols
///
/// # Returns
///
/// A vector of random passwords, or an error if no character sets are enabled.
pub fn generate_passwords(
    rng: &mut ForgeryRng,
    n: usize,
    length: usize,
    uppercase: bool,
    lowercase: bool,
    digits: bool,
    symbols: bool,
) -> Result<Vec<String>, PasswordError> {
    // Build the character pool
    let pool = build_char_pool(uppercase, lowercase, digits, symbols)?;

    let mut passwords = Vec::with_capacity(n);
    for _ in 0..n {
        passwords.push(generate_password_from_pool(rng, length, &pool));
    }
    Ok(passwords)
}

/// Generate a single password.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `length` - Length of the password
/// * `uppercase` - Include uppercase letters
/// * `lowercase` - Include lowercase letters
/// * `digits` - Include digits
/// * `symbols` - Include symbols
///
/// # Returns
///
/// A random password, or an error if no character sets are enabled.
#[inline]
pub fn generate_password(
    rng: &mut ForgeryRng,
    length: usize,
    uppercase: bool,
    lowercase: bool,
    digits: bool,
    symbols: bool,
) -> Result<String, PasswordError> {
    let pool = build_char_pool(uppercase, lowercase, digits, symbols)?;
    Ok(generate_password_from_pool(rng, length, &pool))
}

/// Build the character pool from the enabled character sets.
fn build_char_pool(
    uppercase: bool,
    lowercase: bool,
    digits: bool,
    symbols: bool,
) -> Result<Vec<u8>, PasswordError> {
    let mut pool = Vec::with_capacity(
        if lowercase { LOWERCASE.len() } else { 0 }
            + if uppercase { UPPERCASE.len() } else { 0 }
            + if digits { DIGITS.len() } else { 0 }
            + if symbols { SYMBOLS.len() } else { 0 },
    );

    if lowercase {
        pool.extend_from_slice(LOWERCASE);
    }
    if uppercase {
        pool.extend_from_slice(UPPERCASE);
    }
    if digits {
        pool.extend_from_slice(DIGITS);
    }
    if symbols {
        pool.extend_from_slice(SYMBOLS);
    }

    if pool.is_empty() {
        return Err(PasswordError::NoCharacterSetsEnabled);
    }

    Ok(pool)
}

/// Generate a password from a pre-built character pool.
#[inline]
fn generate_password_from_pool(rng: &mut ForgeryRng, length: usize, pool: &[u8]) -> String {
    let mut password = String::with_capacity(length);
    for _ in 0..length {
        let idx = rng.gen_range(0, pool.len() - 1);
        password.push(pool[idx] as char);
    }
    password
}

/// Errors that can occur during password generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordError {
    /// No character sets were enabled.
    NoCharacterSetsEnabled,
}

impl std::fmt::Display for PasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCharacterSetsEnabled => {
                write!(f, "at least one character set must be enabled")
            }
        }
    }
}

impl std::error::Error for PasswordError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_passwords_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let passwords = generate_passwords(&mut rng, 100, 12, true, true, true, true).unwrap();
        assert_eq!(passwords.len(), 100);
    }

    #[test]
    fn test_password_length() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        for length in [8, 12, 16, 24, 32, 64] {
            let passwords =
                generate_passwords(&mut rng, 10, length, true, true, true, true).unwrap();
            for password in &passwords {
                assert_eq!(
                    password.len(),
                    length,
                    "Password should have length {}",
                    length
                );
            }
        }
    }

    #[test]
    fn test_password_lowercase_only() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let passwords = generate_passwords(&mut rng, 100, 20, false, true, false, false).unwrap();
        for password in &passwords {
            for c in password.chars() {
                assert!(
                    c.is_ascii_lowercase(),
                    "Should only contain lowercase: {}",
                    password
                );
            }
        }
    }

    #[test]
    fn test_password_uppercase_only() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let passwords = generate_passwords(&mut rng, 100, 20, true, false, false, false).unwrap();
        for password in &passwords {
            for c in password.chars() {
                assert!(
                    c.is_ascii_uppercase(),
                    "Should only contain uppercase: {}",
                    password
                );
            }
        }
    }

    #[test]
    fn test_password_digits_only() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let passwords = generate_passwords(&mut rng, 100, 20, false, false, true, false).unwrap();
        for password in &passwords {
            for c in password.chars() {
                assert!(
                    c.is_ascii_digit(),
                    "Should only contain digits: {}",
                    password
                );
            }
        }
    }

    #[test]
    fn test_password_symbols_only() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let passwords = generate_passwords(&mut rng, 100, 20, false, false, false, true).unwrap();
        for password in &passwords {
            for c in password.chars() {
                assert!(
                    SYMBOLS.contains(&(c as u8)),
                    "Should only contain symbols: {}",
                    password
                );
            }
        }
    }

    #[test]
    fn test_password_no_charsets_error() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let result = generate_passwords(&mut rng, 10, 12, false, false, false, false);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PasswordError::NoCharacterSetsEnabled);
    }

    #[test]
    fn test_password_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let passwords1 = generate_passwords(&mut rng1, 50, 16, true, true, true, true).unwrap();
        let passwords2 = generate_passwords(&mut rng2, 50, 16, true, true, true, true).unwrap();

        assert_eq!(passwords1, passwords2);
    }

    #[test]
    fn test_password_empty_batch() {
        let mut rng = ForgeryRng::new();
        let passwords = generate_passwords(&mut rng, 0, 12, true, true, true, true).unwrap();
        assert!(passwords.is_empty());
    }

    #[test]
    fn test_password_zero_length() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let passwords = generate_passwords(&mut rng, 10, 0, true, true, true, true).unwrap();
        for password in &passwords {
            assert!(password.is_empty());
        }
    }

    #[test]
    fn test_single_password() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let password = generate_password(&mut rng, 16, true, true, true, true).unwrap();
        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_single_password_error() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let result = generate_password(&mut rng, 16, false, false, false, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_different_seeds_different_results() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let passwords1 = generate_passwords(&mut rng1, 100, 16, true, true, true, true).unwrap();
        let passwords2 = generate_passwords(&mut rng2, 100, 16, true, true, true, true).unwrap();

        assert_ne!(
            passwords1, passwords2,
            "Different seeds should produce different passwords"
        );
    }

    #[test]
    fn test_password_mixed_charsets() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        // Generate enough passwords to statistically hit all character sets
        let passwords = generate_passwords(&mut rng, 100, 100, true, true, true, true).unwrap();

        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_digit = false;
        let mut has_symbol = false;

        for password in &passwords {
            for c in password.chars() {
                if c.is_ascii_uppercase() {
                    has_upper = true;
                } else if c.is_ascii_lowercase() {
                    has_lower = true;
                } else if c.is_ascii_digit() {
                    has_digit = true;
                } else if SYMBOLS.contains(&(c as u8)) {
                    has_symbol = true;
                }
            }
        }

        assert!(has_upper, "Should contain uppercase letters");
        assert!(has_lower, "Should contain lowercase letters");
        assert!(has_digit, "Should contain digits");
        assert!(has_symbol, "Should contain symbols");
    }

    #[test]
    fn test_password_error_display() {
        let err = PasswordError::NoCharacterSetsEnabled;
        let msg = format!("{}", err);
        assert!(msg.contains("character set"));
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

            let passwords = generate_passwords(&mut rng, n, 12, true, true, true, true).unwrap();
            prop_assert_eq!(passwords.len(), n);
        }

        /// Property: all passwords have the specified length
        #[test]
        fn prop_password_length(length in 0usize..100, n in 1usize..50) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let passwords = generate_passwords(&mut rng, n, length, true, true, true, true).unwrap();
            for password in passwords {
                prop_assert_eq!(password.len(), length);
            }
        }

        /// Property: same seed produces same output
        #[test]
        fn prop_seed_determinism(seed_val in any::<u64>(), n in 1usize..50) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let passwords1 = generate_passwords(&mut rng1, n, 16, true, true, true, true).unwrap();
            let passwords2 = generate_passwords(&mut rng2, n, 16, true, true, true, true).unwrap();

            prop_assert_eq!(passwords1, passwords2);
        }

        /// Property: lowercase-only passwords contain only lowercase
        #[test]
        fn prop_lowercase_only(n in 1usize..50) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let passwords = generate_passwords(&mut rng, n, 20, false, true, false, false).unwrap();
            for password in passwords {
                for c in password.chars() {
                    prop_assert!(c.is_ascii_lowercase());
                }
            }
        }

        /// Property: digits-only passwords contain only digits
        #[test]
        fn prop_digits_only(n in 1usize..50) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let passwords = generate_passwords(&mut rng, n, 20, false, false, true, false).unwrap();
            for password in passwords {
                for c in password.chars() {
                    prop_assert!(c.is_ascii_digit());
                }
            }
        }
    }
}
