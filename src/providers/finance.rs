//! Finance-related data generation provider.
//!
//! Generates credit card numbers (with valid Luhn checksum), IBANs,
//! BIC/SWIFT codes, bank account numbers, and bank names.

use crate::data::get_locale_data;
use crate::locale::Locale;
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

    // Ensure all characters are ASCII to avoid panic when slicing
    if !clean.is_ascii() {
        return false;
    }

    // Move first 4 characters to end (safe because we verified ASCII above)
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

/// Generate a batch of BIC/SWIFT codes.
///
/// BIC format: AAAABBCCXXX
/// - AAAA: Bank code (4 letters)
/// - BB: Country code (2 letters)
/// - CC: Location code (2 alphanumeric)
/// - XXX: Branch code (optional, 3 alphanumeric)
pub fn generate_bics(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut bics = Vec::with_capacity(n);
    for _ in 0..n {
        bics.push(generate_bic(rng));
    }
    bics
}

/// Generate a single BIC/SWIFT code.
#[inline]
pub fn generate_bic(rng: &mut ForgeryRng) -> String {
    // Generate 8 or 11 character BIC
    let include_branch = rng.gen_range(0, 1) == 1;
    let capacity = if include_branch { 11 } else { 8 };
    let mut bic = String::with_capacity(capacity);

    // Bank code (4 uppercase letters)
    for _ in 0..4 {
        let letter = rng.gen_range(0, 25) as u8;
        bic.push((b'A' + letter) as char);
    }

    // Country code (use a random one from IBAN_COUNTRIES)
    let (country_code, _) = rng.choose(IBAN_COUNTRIES);
    bic.push_str(country_code);

    // Location code (2 alphanumeric, but typically uppercase letters or digits)
    for _ in 0..2 {
        if rng.gen_range(0, 1) == 0 {
            let letter = rng.gen_range(0, 25) as u8;
            bic.push((b'A' + letter) as char);
        } else {
            let digit = rng.gen_range(0, 9) as u8;
            bic.push((b'0' + digit) as char);
        }
    }

    // Branch code (optional, 3 alphanumeric)
    if include_branch {
        for _ in 0..3 {
            if rng.gen_range(0, 1) == 0 {
                let letter = rng.gen_range(0, 25) as u8;
                bic.push((b'A' + letter) as char);
            } else {
                let digit = rng.gen_range(0, 9) as u8;
                bic.push((b'0' + digit) as char);
            }
        }
    }

    bic
}

/// Generate a batch of bank account numbers.
///
/// Generates numeric account numbers between 8 and 17 digits.
pub fn generate_bank_accounts(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut accounts = Vec::with_capacity(n);
    for _ in 0..n {
        accounts.push(generate_bank_account(rng));
    }
    accounts
}

/// Generate a single bank account number.
///
/// Generates a numeric account number between 8 and 17 digits.
#[inline]
pub fn generate_bank_account(rng: &mut ForgeryRng) -> String {
    // Account numbers are typically 8-17 digits
    let length = rng.gen_range(8, 17);
    let mut account = String::with_capacity(length);

    for _ in 0..length {
        let digit = rng.gen_range(0, 9) as u8;
        account.push((b'0' + digit) as char);
    }

    account
}

/// Generate a batch of bank names using locale-specific data.
pub fn generate_bank_names(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let data = get_locale_data(locale);
    let bank_names = data.bank_names().unwrap_or(&[]);

    let mut names = Vec::with_capacity(n);
    for _ in 0..n {
        let name = if bank_names.is_empty() {
            "Unknown Bank"
        } else {
            rng.choose(bank_names)
        };
        names.push(name.to_string());
    }
    names
}

/// Generate a single bank name using locale-specific data.
#[inline]
pub fn generate_bank_name(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let bank_names = data.bank_names().unwrap_or(&[]);

    if bank_names.is_empty() {
        "Unknown Bank".to_string()
    } else {
        rng.choose(bank_names).to_string()
    }
}

// === UK-Specific Banking ===

/// Generate a batch of UK sort codes.
///
/// Sort codes are 6-digit codes in format XX-XX-XX that identify UK bank branches.
pub fn generate_sort_codes(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut codes = Vec::with_capacity(n);
    for _ in 0..n {
        codes.push(generate_sort_code(rng));
    }
    codes
}

/// Generate a single UK sort code.
///
/// Format: XX-XX-XX (e.g., 12-34-56)
#[inline]
pub fn generate_sort_code(rng: &mut ForgeryRng) -> String {
    let mut code = String::with_capacity(8);

    // Generate 3 pairs of digits separated by dashes
    for pair in 0..3 {
        if pair > 0 {
            code.push('-');
        }
        let d1 = rng.gen_range(0, 9) as u8;
        let d2 = rng.gen_range(0, 9) as u8;
        code.push((b'0' + d1) as char);
        code.push((b'0' + d2) as char);
    }

    code
}

/// Generate a batch of UK bank account numbers.
///
/// UK account numbers are exactly 8 digits.
pub fn generate_uk_account_numbers(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut accounts = Vec::with_capacity(n);
    for _ in 0..n {
        accounts.push(generate_uk_account_number(rng));
    }
    accounts
}

/// Generate a single UK bank account number.
///
/// UK account numbers are exactly 8 digits.
#[inline]
pub fn generate_uk_account_number(rng: &mut ForgeryRng) -> String {
    let mut account = String::with_capacity(8);
    for _ in 0..8 {
        let digit = rng.gen_range(0, 9) as u8;
        account.push((b'0' + digit) as char);
    }
    account
}

// === Financial Transaction Data ===

/// Transaction types for realistic banking data.
pub const TRANSACTION_TYPES: &[&str] = &[
    "Direct Debit",
    "Standing Order",
    "Faster Payment",
    "Card Payment",
    "Cash Withdrawal",
    "Bank Transfer",
    "BACS Payment",
    "CHAPS Payment",
    "Cheque",
    "Interest Payment",
    "Refund",
    "Salary",
    "Dividend",
];

/// Common merchant/payee names for transactions.
const MERCHANTS: &[&str] = &[
    "Tesco",
    "Sainsbury's",
    "Amazon UK",
    "British Gas",
    "EDF Energy",
    "Sky UK",
    "Netflix",
    "Spotify",
    "Apple",
    "Google",
    "TfL",
    "National Rail",
    "Costa Coffee",
    "Greggs",
    "McDonald's",
    "BP",
    "Shell",
    "Vodafone",
    "EE",
    "Three",
    "HMRC",
    "Council Tax",
    "Thames Water",
    "Virgin Media",
    "BT",
];

/// A financial transaction record.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Unique transaction reference
    pub reference: String,
    /// Transaction date (YYYY-MM-DD)
    pub date: String,
    /// Transaction amount (negative for debits, positive for credits)
    pub amount: f64,
    /// Transaction type (e.g., "Card Payment", "Direct Debit")
    pub transaction_type: String,
    /// Merchant or payee description
    pub description: String,
    /// Running balance after transaction
    pub balance: f64,
}

/// Generate a batch of financial transactions.
///
/// Generates realistic-looking transaction data with running balance.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `n` - Number of transactions to generate
/// * `starting_balance` - Opening balance for the account
/// * `start_date` - Start date for transactions (YYYY-MM-DD)
/// * `end_date` - End date for transactions (YYYY-MM-DD)
///
/// # Errors
///
/// Returns a `DateRangeError` if the date range is invalid.
pub fn generate_transactions(
    rng: &mut ForgeryRng,
    n: usize,
    starting_balance: f64,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<Transaction>, crate::providers::datetime::DateRangeError> {
    use crate::providers::datetime::generate_dates;

    let mut transactions = Vec::with_capacity(n);
    let mut balance = starting_balance;

    // Generate sorted dates for transactions
    let mut dates = generate_dates(rng, n, start_date, end_date)?;
    dates.sort();

    for date in dates {
        // Generate transaction reference (8 alphanumeric chars)
        let reference = generate_transaction_reference(rng);

        // Decide if this is a credit or debit (80% debits, 20% credits)
        let is_credit = rng.gen_range(0, 9) < 2;

        // Generate amount
        let amount = if is_credit {
            // Credits: typically larger amounts (salary, refunds, transfers)
            let base = rng.gen_range(50, 5000) as f64;
            base + (rng.gen_range(0, 99) as f64 / 100.0)
        } else {
            // Debits: more varied, typically smaller
            let base = rng.gen_range(1, 500) as f64;
            -(base + (rng.gen_range(0, 99) as f64 / 100.0))
        };

        balance += amount;

        // Select transaction type based on credit/debit
        let transaction_type = if is_credit {
            match rng.gen_range(0, 3) {
                0 => "Faster Payment",
                1 => "Bank Transfer",
                2 => "Salary",
                _ => "Refund",
            }
        } else {
            rng.choose(TRANSACTION_TYPES)
        }
        .to_string();

        // Generate description
        let description = if is_credit {
            match transaction_type.as_str() {
                "Salary" => "SALARY PAYMENT".to_string(),
                "Refund" => format!("{} REFUND", rng.choose(MERCHANTS)),
                _ => format!("TRANSFER FROM {}", generate_payee_name(rng)),
            }
        } else {
            match transaction_type.as_str() {
                "Card Payment" | "Cash Withdrawal" => rng.choose(MERCHANTS).to_string(),
                "Direct Debit" | "Standing Order" => {
                    format!("{} DD", rng.choose(MERCHANTS))
                }
                _ => rng.choose(MERCHANTS).to_string(),
            }
        };

        transactions.push(Transaction {
            reference,
            date,
            amount: (amount * 100.0).round() / 100.0, // Round to 2 decimal places
            transaction_type,
            description,
            balance: (balance * 100.0).round() / 100.0,
        });
    }

    Ok(transactions)
}

/// Reference characters: uppercase letters and digits.
const REFERENCE_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Generate a transaction reference (8 alphanumeric characters).
fn generate_transaction_reference(rng: &mut ForgeryRng) -> String {
    let mut reference = String::with_capacity(8);

    for _ in 0..8 {
        let c = *rng.choose(REFERENCE_CHARS);
        reference.push(c as char);
    }

    reference
}

/// Generate a random payee name for transfers.
fn generate_payee_name(rng: &mut ForgeryRng) -> String {
    const FIRST_NAMES: &[&str] = &[
        "J", "M", "S", "A", "R", "T", "D", "C", "L", "E", "P", "K", "N", "B", "G",
    ];
    const LAST_NAMES: &[&str] = &[
        "SMITH", "JONES", "TAYLOR", "BROWN", "WILLIAMS", "WILSON", "JOHNSON", "DAVIES", "ROBINSON",
        "WRIGHT", "THOMPSON", "EVANS", "WALKER", "WHITE", "ROBERTS",
    ];

    format!("{} {}", rng.choose(FIRST_NAMES), rng.choose(LAST_NAMES))
}

/// Generate a single transaction amount.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `min` - Minimum amount (can be negative for debits)
/// * `max` - Maximum amount
#[inline]
pub fn generate_transaction_amount(rng: &mut ForgeryRng, min: f64, max: f64) -> f64 {
    let amount = min + (rng.gen_range(0, 1_000_000) as f64 / 1_000_000.0) * (max - min);
    (amount * 100.0).round() / 100.0 // Round to 2 decimal places
}

/// Generate a batch of transaction amounts.
pub fn generate_transaction_amounts(
    rng: &mut ForgeryRng,
    n: usize,
    min: f64,
    max: f64,
) -> Vec<f64> {
    let mut amounts = Vec::with_capacity(n);
    for _ in 0..n {
        amounts.push(generate_transaction_amount(rng, min, max));
    }
    amounts
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
    fn test_iban_non_ascii_does_not_panic() {
        // Non-ASCII characters should return false, not panic
        assert!(!validate_iban("DE89370400440532013000日本"));
        assert!(!validate_iban("日本89370400440532013000"));
        assert!(!validate_iban("ÜÖ89370400440532013000"));
        assert!(!validate_iban("🎉🎉89370400440532013000"));
    }

    // BIC/SWIFT tests
    #[test]
    fn test_generate_bics_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let bics = generate_bics(&mut rng, 100);
        assert_eq!(bics.len(), 100);
    }

    #[test]
    fn test_bic_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let bics = generate_bics(&mut rng, 100);
        for bic in &bics {
            // BIC should be 8 or 11 characters
            let len = bic.len();
            assert!(
                len == 8 || len == 11,
                "BIC should be 8 or 11 characters: {} (len {})",
                bic,
                len
            );

            // First 4 characters should be uppercase letters (bank code)
            assert!(
                bic.chars().take(4).all(|c| c.is_ascii_uppercase()),
                "BIC bank code should be uppercase letters: {}",
                bic
            );

            // Next 2 characters should be uppercase letters (country code)
            assert!(
                bic.chars().skip(4).take(2).all(|c| c.is_ascii_uppercase()),
                "BIC country code should be uppercase letters: {}",
                bic
            );

            // Location code (2 chars) should be alphanumeric
            assert!(
                bic.chars()
                    .skip(6)
                    .take(2)
                    .all(|c| c.is_ascii_alphanumeric()),
                "BIC location code should be alphanumeric: {}",
                bic
            );

            // Branch code (if present, 3 chars) should be alphanumeric
            if len == 11 {
                assert!(
                    bic.chars().skip(8).all(|c| c.is_ascii_alphanumeric()),
                    "BIC branch code should be alphanumeric: {}",
                    bic
                );
            }
        }
    }

    #[test]
    fn test_bic_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let b1 = generate_bics(&mut rng1, 50);
        let b2 = generate_bics(&mut rng2, 50);

        assert_eq!(b1, b2);
    }

    // Bank account tests
    #[test]
    fn test_generate_bank_accounts_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let accounts = generate_bank_accounts(&mut rng, 100);
        assert_eq!(accounts.len(), 100);
    }

    #[test]
    fn test_bank_account_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let accounts = generate_bank_accounts(&mut rng, 100);
        for account in &accounts {
            // Account should be 8-17 digits
            let len = account.len();
            assert!(
                (8..=17).contains(&len),
                "Bank account should be 8-17 digits: {} (len {})",
                account,
                len
            );

            // Should be all digits
            assert!(
                account.chars().all(|c| c.is_ascii_digit()),
                "Bank account should only contain digits: {}",
                account
            );
        }
    }

    #[test]
    fn test_bank_account_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let a1 = generate_bank_accounts(&mut rng1, 50);
        let a2 = generate_bank_accounts(&mut rng2, 50);

        assert_eq!(a1, a2);
    }

    // UK Sort Code tests
    #[test]
    fn test_generate_sort_codes_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let codes = generate_sort_codes(&mut rng, 100);
        assert_eq!(codes.len(), 100);
    }

    #[test]
    fn test_sort_code_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let codes = generate_sort_codes(&mut rng, 100);
        for code in &codes {
            // Should be XX-XX-XX format (8 chars)
            assert_eq!(code.len(), 8, "Sort code should be 8 chars: {}", code);

            // Check format
            let chars: Vec<char> = code.chars().collect();
            assert!(
                chars[0].is_ascii_digit(),
                "Expected digit at pos 0: {}",
                code
            );
            assert!(
                chars[1].is_ascii_digit(),
                "Expected digit at pos 1: {}",
                code
            );
            assert_eq!(chars[2], '-', "Expected dash at pos 2: {}", code);
            assert!(
                chars[3].is_ascii_digit(),
                "Expected digit at pos 3: {}",
                code
            );
            assert!(
                chars[4].is_ascii_digit(),
                "Expected digit at pos 4: {}",
                code
            );
            assert_eq!(chars[5], '-', "Expected dash at pos 5: {}", code);
            assert!(
                chars[6].is_ascii_digit(),
                "Expected digit at pos 6: {}",
                code
            );
            assert!(
                chars[7].is_ascii_digit(),
                "Expected digit at pos 7: {}",
                code
            );
        }
    }

    #[test]
    fn test_sort_code_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let s1 = generate_sort_codes(&mut rng1, 50);
        let s2 = generate_sort_codes(&mut rng2, 50);

        assert_eq!(s1, s2);
    }

    // UK Account Number tests
    #[test]
    fn test_generate_uk_account_numbers_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let accounts = generate_uk_account_numbers(&mut rng, 100);
        assert_eq!(accounts.len(), 100);
    }

    #[test]
    fn test_uk_account_number_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let accounts = generate_uk_account_numbers(&mut rng, 100);
        for account in &accounts {
            // UK account should be exactly 8 digits
            assert_eq!(
                account.len(),
                8,
                "UK account should be exactly 8 digits: {} (len {})",
                account,
                account.len()
            );

            // Should be all digits
            assert!(
                account.chars().all(|c| c.is_ascii_digit()),
                "UK account should only contain digits: {}",
                account
            );
        }
    }

    #[test]
    fn test_uk_account_number_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let a1 = generate_uk_account_numbers(&mut rng1, 50);
        let a2 = generate_uk_account_numbers(&mut rng2, 50);

        assert_eq!(a1, a2);
    }

    // Transaction tests
    #[test]
    fn test_generate_transactions_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let transactions =
            generate_transactions(&mut rng, 100, 1000.0, "2024-01-01", "2024-12-31").unwrap();
        assert_eq!(transactions.len(), 100);
    }

    #[test]
    fn test_transaction_data() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let transactions =
            generate_transactions(&mut rng, 10, 5000.0, "2024-01-01", "2024-03-31").unwrap();

        for tx in &transactions {
            // Reference should be 8 alphanumeric chars
            assert_eq!(tx.reference.len(), 8, "Reference should be 8 chars");
            assert!(
                tx.reference.chars().all(|c| c.is_ascii_alphanumeric()),
                "Reference should be alphanumeric: {}",
                tx.reference
            );

            // Date should be valid format
            assert_eq!(tx.date.len(), 10, "Date should be YYYY-MM-DD");
            assert!(
                tx.date.starts_with("2024-"),
                "Date should be in 2024: {}",
                tx.date
            );

            // Transaction type should not be empty
            assert!(
                !tx.transaction_type.is_empty(),
                "Transaction type should not be empty"
            );

            // Description should not be empty
            assert!(
                !tx.description.is_empty(),
                "Description should not be empty"
            );
        }
    }

    #[test]
    fn test_transaction_running_balance() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let starting_balance = 1000.0;
        let transactions =
            generate_transactions(&mut rng, 50, starting_balance, "2024-01-01", "2024-06-30")
                .unwrap();

        // Verify running balance is calculated correctly
        let mut expected_balance = starting_balance;
        for tx in &transactions {
            expected_balance += tx.amount;
            // Allow for small floating point differences
            assert!(
                (tx.balance - (expected_balance * 100.0).round() / 100.0).abs() < 0.01,
                "Balance mismatch: expected {}, got {}",
                expected_balance,
                tx.balance
            );
        }
    }

    #[test]
    fn test_transaction_dates_sorted() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let transactions =
            generate_transactions(&mut rng, 50, 1000.0, "2024-01-01", "2024-12-31").unwrap();

        // Dates should be sorted
        for i in 1..transactions.len() {
            assert!(
                transactions[i].date >= transactions[i - 1].date,
                "Dates should be sorted: {} comes after {}",
                transactions[i].date,
                transactions[i - 1].date
            );
        }
    }

    #[test]
    fn test_transaction_amount_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let amounts = generate_transaction_amounts(&mut rng, 100, -500.0, 500.0);

        for amount in &amounts {
            // Check amount is in range
            assert!(
                *amount >= -500.0 && *amount <= 500.0,
                "Amount should be in range: {}",
                amount
            );

            // Check amount is rounded to 2 decimal places
            let rounded = (*amount * 100.0).round() / 100.0;
            assert!(
                (*amount - rounded).abs() < 0.001,
                "Amount should be rounded to 2 decimals: {}",
                amount
            );
        }
    }

    #[test]
    fn test_empty_batches() {
        let mut rng = ForgeryRng::new();

        assert!(generate_credit_cards(&mut rng, 0).is_empty());
        assert!(generate_ibans(&mut rng, 0).is_empty());
        assert!(generate_bics(&mut rng, 0).is_empty());
        assert!(generate_bank_accounts(&mut rng, 0).is_empty());
        assert!(generate_sort_codes(&mut rng, 0).is_empty());
        assert!(generate_uk_account_numbers(&mut rng, 0).is_empty());
        assert!(
            generate_transactions(&mut rng, 0, 1000.0, "2024-01-01", "2024-12-31")
                .unwrap()
                .is_empty()
        );
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

    #[test]
    fn test_all_reference_chars_reachable() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        // Generate enough transactions to statistically hit all 36 chars (26 letters + 10 digits)
        let transactions =
            generate_transactions(&mut rng, 500, 1000.0, "2024-01-01", "2024-12-31").unwrap();
        let mut seen = std::collections::HashSet::new();

        for tx in &transactions {
            for c in tx.reference.chars() {
                seen.insert(c);
            }
        }

        // Should have seen all 36 characters (26 letters + 10 digits)
        assert_eq!(
            seen.len(),
            36,
            "Should see all 36 reference chars, but only saw {} chars: {:?}",
            seen.len(),
            seen
        );
    }

    #[test]
    fn test_transaction_amounts_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let amounts = generate_transaction_amounts(&mut rng, 100, 0.0, 1000.0);
        assert_eq!(amounts.len(), 100);
    }

    #[test]
    fn test_single_transaction_amount() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        for _ in 0..100 {
            let amount = generate_transaction_amount(&mut rng, 10.0, 100.0);
            assert!((10.0..=100.0).contains(&amount));
            // Check 2 decimal place precision
            assert_eq!(amount, (amount * 100.0).round() / 100.0);
        }
    }

    #[test]
    fn test_single_uk_account_number() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let account = generate_uk_account_number(&mut rng);
        assert_eq!(account.len(), 8);
        assert!(account.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_single_sort_code() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let code = generate_sort_code(&mut rng);
        assert_eq!(code.len(), 8);
        assert_eq!(&code[2..3], "-");
        assert_eq!(&code[5..6], "-");
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
