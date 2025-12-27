//! Date and time generation provider.
//!
//! Generates dates, times, and datetime values.

use crate::rng::ForgeryRng;
use chrono::{Datelike, NaiveDate};

/// Error type for date range generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateRangeError {
    /// The start date string.
    pub start: String,
    /// The end date string.
    pub end: String,
    /// The reason for the error.
    pub reason: String,
}

impl std::fmt::Display for DateRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid date range '{}' to '{}': {}",
            self.start, self.end, self.reason
        )
    }
}

impl std::error::Error for DateRangeError {}

/// Default start date for date generation.
#[allow(dead_code)]
pub const DEFAULT_START_DATE: &str = "1970-01-01";

/// Default end date for date generation.
#[allow(dead_code)]
pub const DEFAULT_END_DATE: &str = "2030-12-31";

/// Parse a date string in YYYY-MM-DD format.
fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| e.to_string())
}

/// Generate a batch of random dates within a range.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of dates to generate
/// * `start` - Start date in YYYY-MM-DD format (inclusive)
/// * `end` - End date in YYYY-MM-DD format (inclusive)
///
/// # Errors
///
/// Returns `DateRangeError` if dates cannot be parsed or start > end.
pub fn generate_dates(
    rng: &mut ForgeryRng,
    n: usize,
    start: &str,
    end: &str,
) -> Result<Vec<String>, DateRangeError> {
    let start_date = parse_date(start).map_err(|e| DateRangeError {
        start: start.to_string(),
        end: end.to_string(),
        reason: format!("invalid start date: {}", e),
    })?;

    let end_date = parse_date(end).map_err(|e| DateRangeError {
        start: start.to_string(),
        end: end.to_string(),
        reason: format!("invalid end date: {}", e),
    })?;

    if start_date > end_date {
        return Err(DateRangeError {
            start: start.to_string(),
            end: end.to_string(),
            reason: "start date must be before or equal to end date".to_string(),
        });
    }

    let start_days = start_date.num_days_from_ce();
    let end_days = end_date.num_days_from_ce();

    let mut dates = Vec::with_capacity(n);
    for _ in 0..n {
        let days = rng.gen_range(start_days, end_days);
        let date = NaiveDate::from_num_days_from_ce_opt(days).ok_or_else(|| DateRangeError {
            start: start.to_string(),
            end: end.to_string(),
            reason: format!("internal error: invalid days from CE value {}", days),
        })?;
        dates.push(date.format("%Y-%m-%d").to_string());
    }
    Ok(dates)
}

/// Generate a single random date within a range.
///
/// More efficient than `generate_dates(rng, 1, start, end)` as it avoids Vec allocation.
///
/// # Errors
///
/// Returns `DateRangeError` if dates cannot be parsed or start > end.
#[inline]
pub fn generate_date(
    rng: &mut ForgeryRng,
    start: &str,
    end: &str,
) -> Result<String, DateRangeError> {
    let start_date = parse_date(start).map_err(|e| DateRangeError {
        start: start.to_string(),
        end: end.to_string(),
        reason: format!("invalid start date: {}", e),
    })?;

    let end_date = parse_date(end).map_err(|e| DateRangeError {
        start: start.to_string(),
        end: end.to_string(),
        reason: format!("invalid end date: {}", e),
    })?;

    if start_date > end_date {
        return Err(DateRangeError {
            start: start.to_string(),
            end: end.to_string(),
            reason: "start date must be before or equal to end date".to_string(),
        });
    }

    let start_days = start_date.num_days_from_ce();
    let end_days = end_date.num_days_from_ce();
    let days = rng.gen_range(start_days, end_days);
    let date = NaiveDate::from_num_days_from_ce_opt(days).ok_or_else(|| DateRangeError {
        start: start.to_string(),
        end: end.to_string(),
        reason: format!("internal error: invalid days from CE value {}", days),
    })?;
    Ok(date.format("%Y-%m-%d").to_string())
}

/// Generate a batch of random date-of-birth values.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of dates to generate
/// * `min_age` - Minimum age in years
/// * `max_age` - Maximum age in years
///
/// # Determinism Note
///
/// This function uses a fixed reference date of 2024-01-01 for age calculations
/// to ensure reproducible output with the same seed. This means ages are calculated
/// relative to January 1, 2024, not the actual current date.
///
/// # Errors
///
/// Returns `DateRangeError` if min_age > max_age.
pub fn generate_dates_of_birth(
    rng: &mut ForgeryRng,
    n: usize,
    min_age: u32,
    max_age: u32,
) -> Result<Vec<String>, DateRangeError> {
    if min_age > max_age {
        return Err(DateRangeError {
            start: format!("min_age={}", min_age),
            end: format!("max_age={}", max_age),
            reason: "min_age must be less than or equal to max_age".to_string(),
        });
    }

    // Calculate date range based on ages
    // Use a fixed "today" for determinism (2024-01-01)
    let today = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end_date = today
        .with_year(today.year() - min_age as i32)
        .unwrap_or(today);
    let start_date = today
        .with_year(today.year() - max_age as i32 - 1)
        .unwrap_or(today);

    let start_str = start_date.format("%Y-%m-%d").to_string();
    let end_str = end_date.format("%Y-%m-%d").to_string();

    generate_dates(rng, n, &start_str, &end_str)
}

/// Generate a single random date-of-birth value.
///
/// # Determinism Note
///
/// This function uses a fixed reference date of 2024-01-01 for age calculations
/// to ensure reproducible output with the same seed. This means ages are calculated
/// relative to January 1, 2024, not the actual current date.
///
/// # Errors
///
/// Returns `DateRangeError` if min_age > max_age.
#[inline]
pub fn generate_date_of_birth(
    rng: &mut ForgeryRng,
    min_age: u32,
    max_age: u32,
) -> Result<String, DateRangeError> {
    if min_age > max_age {
        return Err(DateRangeError {
            start: format!("min_age={}", min_age),
            end: format!("max_age={}", max_age),
            reason: "min_age must be less than or equal to max_age".to_string(),
        });
    }

    let today = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end_date = today
        .with_year(today.year() - min_age as i32)
        .unwrap_or(today);
    let start_date = today
        .with_year(today.year() - max_age as i32 - 1)
        .unwrap_or(today);

    let start_str = start_date.format("%Y-%m-%d").to_string();
    let end_str = end_date.format("%Y-%m-%d").to_string();

    generate_date(rng, &start_str, &end_str)
}

/// Generate a batch of random datetime strings in ISO 8601 format.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of datetimes to generate
/// * `start` - Start date in YYYY-MM-DD format (inclusive)
/// * `end` - End date in YYYY-MM-DD format (inclusive)
///
/// # Errors
///
/// Returns `DateRangeError` if dates cannot be parsed or start > end.
pub fn generate_datetimes(
    rng: &mut ForgeryRng,
    n: usize,
    start: &str,
    end: &str,
) -> Result<Vec<String>, DateRangeError> {
    let start_date = parse_date(start).map_err(|e| DateRangeError {
        start: start.to_string(),
        end: end.to_string(),
        reason: format!("invalid start date: {}", e),
    })?;

    let end_date = parse_date(end).map_err(|e| DateRangeError {
        start: start.to_string(),
        end: end.to_string(),
        reason: format!("invalid end date: {}", e),
    })?;

    if start_date > end_date {
        return Err(DateRangeError {
            start: start.to_string(),
            end: end.to_string(),
            reason: "start date must be before or equal to end date".to_string(),
        });
    }

    let start_days = start_date.num_days_from_ce();
    let end_days = end_date.num_days_from_ce();

    let mut datetimes = Vec::with_capacity(n);
    for _ in 0..n {
        let days = rng.gen_range(start_days, end_days);
        let date = NaiveDate::from_num_days_from_ce_opt(days).ok_or_else(|| DateRangeError {
            start: start.to_string(),
            end: end.to_string(),
            reason: format!("internal error: invalid days from CE value {}", days),
        })?;
        let hour: u32 = rng.gen_range(0, 23);
        let minute: u32 = rng.gen_range(0, 59);
        let second: u32 = rng.gen_range(0, 59);
        datetimes.push(format!(
            "{}T{:02}:{:02}:{:02}",
            date.format("%Y-%m-%d"),
            hour,
            minute,
            second
        ));
    }
    Ok(datetimes)
}

/// Generate a single random datetime string in ISO 8601 format.
///
/// # Errors
///
/// Returns `DateRangeError` if dates cannot be parsed or start > end.
#[inline]
pub fn generate_datetime(
    rng: &mut ForgeryRng,
    start: &str,
    end: &str,
) -> Result<String, DateRangeError> {
    let start_date = parse_date(start).map_err(|e| DateRangeError {
        start: start.to_string(),
        end: end.to_string(),
        reason: format!("invalid start date: {}", e),
    })?;

    let end_date = parse_date(end).map_err(|e| DateRangeError {
        start: start.to_string(),
        end: end.to_string(),
        reason: format!("invalid end date: {}", e),
    })?;

    if start_date > end_date {
        return Err(DateRangeError {
            start: start.to_string(),
            end: end.to_string(),
            reason: "start date must be before or equal to end date".to_string(),
        });
    }

    let start_days = start_date.num_days_from_ce();
    let end_days = end_date.num_days_from_ce();
    let days = rng.gen_range(start_days, end_days);
    let date = NaiveDate::from_num_days_from_ce_opt(days).ok_or_else(|| DateRangeError {
        start: start.to_string(),
        end: end.to_string(),
        reason: format!("internal error: invalid days from CE value {}", days),
    })?;
    let hour: u32 = rng.gen_range(0, 23);
    let minute: u32 = rng.gen_range(0, 59);
    let second: u32 = rng.gen_range(0, 59);
    Ok(format!(
        "{}T{:02}:{:02}:{:02}",
        date.format("%Y-%m-%d"),
        hour,
        minute,
        second
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Date tests
    #[test]
    fn test_generate_dates_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let dates = generate_dates(&mut rng, 100, "2020-01-01", "2023-12-31").unwrap();
        assert_eq!(dates.len(), 100);
    }

    #[test]
    fn test_date_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let dates = generate_dates(&mut rng, 100, "2020-01-01", "2023-12-31").unwrap();
        for date in &dates {
            // Format: YYYY-MM-DD
            assert_eq!(date.len(), 10, "Date should be 10 characters: {}", date);
            assert!(parse_date(date).is_ok(), "Should be valid date: {}", date);
        }
    }

    #[test]
    fn test_date_in_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let start = "2020-01-01";
        let end = "2020-12-31";
        let dates = generate_dates(&mut rng, 100, start, end).unwrap();

        let start_date = parse_date(start).unwrap();
        let end_date = parse_date(end).unwrap();

        for date in &dates {
            let d = parse_date(date).unwrap();
            assert!(
                d >= start_date && d <= end_date,
                "Date {} not in range",
                date
            );
        }
    }

    #[test]
    fn test_date_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let dates1 = generate_dates(&mut rng1, 100, "2020-01-01", "2023-12-31").unwrap();
        let dates2 = generate_dates(&mut rng2, 100, "2020-01-01", "2023-12-31").unwrap();

        assert_eq!(dates1, dates2);
    }

    #[test]
    fn test_date_empty_batch() {
        let mut rng = ForgeryRng::new();
        let dates = generate_dates(&mut rng, 0, "2020-01-01", "2023-12-31").unwrap();
        assert!(dates.is_empty());
    }

    #[test]
    fn test_date_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let date = generate_date(&mut rng, "2020-01-01", "2023-12-31").unwrap();
        assert_eq!(date.len(), 10);
        assert!(parse_date(&date).is_ok());
    }

    #[test]
    fn test_date_same_start_end() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let dates = generate_dates(&mut rng, 10, "2020-06-15", "2020-06-15").unwrap();
        for date in &dates {
            assert_eq!(date, "2020-06-15");
        }
    }

    #[test]
    fn test_date_invalid_start() {
        let mut rng = ForgeryRng::new();
        let result = generate_dates(&mut rng, 10, "invalid", "2023-12-31");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("invalid start date"));
    }

    #[test]
    fn test_date_invalid_end() {
        let mut rng = ForgeryRng::new();
        let result = generate_dates(&mut rng, 10, "2020-01-01", "invalid");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("invalid end date"));
    }

    #[test]
    fn test_date_start_after_end() {
        let mut rng = ForgeryRng::new();
        let result = generate_dates(&mut rng, 10, "2023-12-31", "2020-01-01");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("start date must be before"));
    }

    // Date of birth tests
    #[test]
    fn test_generate_dates_of_birth_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let dates = generate_dates_of_birth(&mut rng, 100, 18, 65).unwrap();
        assert_eq!(dates.len(), 100);
    }

    #[test]
    fn test_date_of_birth_ages() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let dates = generate_dates_of_birth(&mut rng, 100, 18, 65).unwrap();
        let today = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        for date in &dates {
            let dob = parse_date(date).unwrap();
            let age = today.year() - dob.year();
            // Allow for birthday not yet passed
            assert!(
                (18..=66).contains(&age),
                "Age {} not in range for DOB {}",
                age,
                date
            );
        }
    }

    #[test]
    fn test_date_of_birth_invalid_range() {
        let mut rng = ForgeryRng::new();
        let result = generate_dates_of_birth(&mut rng, 10, 65, 18);
        assert!(result.is_err());
    }

    #[test]
    fn test_date_of_birth_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let date = generate_date_of_birth(&mut rng, 18, 65).unwrap();
        assert_eq!(date.len(), 10);
    }

    // Datetime tests
    #[test]
    fn test_generate_datetimes_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let datetimes = generate_datetimes(&mut rng, 100, "2020-01-01", "2023-12-31").unwrap();
        assert_eq!(datetimes.len(), 100);
    }

    #[test]
    fn test_datetime_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let datetimes = generate_datetimes(&mut rng, 100, "2020-01-01", "2023-12-31").unwrap();
        for dt in &datetimes {
            // Format: YYYY-MM-DDTHH:MM:SS
            assert_eq!(dt.len(), 19, "Datetime should be 19 characters: {}", dt);
            assert!(dt.contains('T'), "Should contain T separator: {}", dt);

            let parts: Vec<&str> = dt.split('T').collect();
            assert_eq!(parts.len(), 2);
            assert!(parse_date(parts[0]).is_ok(), "Date part invalid: {}", dt);

            let time_parts: Vec<&str> = parts[1].split(':').collect();
            assert_eq!(time_parts.len(), 3);
        }
    }

    #[test]
    fn test_datetime_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let dts1 = generate_datetimes(&mut rng1, 100, "2020-01-01", "2023-12-31").unwrap();
        let dts2 = generate_datetimes(&mut rng2, 100, "2020-01-01", "2023-12-31").unwrap();

        assert_eq!(dts1, dts2);
    }

    #[test]
    fn test_datetime_empty_batch() {
        let mut rng = ForgeryRng::new();
        let dts = generate_datetimes(&mut rng, 0, "2020-01-01", "2023-12-31").unwrap();
        assert!(dts.is_empty());
    }

    #[test]
    fn test_datetime_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let dt = generate_datetime(&mut rng, "2020-01-01", "2023-12-31").unwrap();
        assert_eq!(dt.len(), 19);
        assert!(dt.contains('T'));
    }

    #[test]
    fn test_date_range_error_display() {
        let err = DateRangeError {
            start: "2023-01-01".to_string(),
            end: "2020-01-01".to_string(),
            reason: "start after end".to_string(),
        };
        let display = err.to_string();
        assert!(display.contains("2023-01-01"));
        assert!(display.contains("2020-01-01"));
        assert!(display.contains("start after end"));
    }

    #[test]
    fn test_different_seeds_different_dates() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let dates1 = generate_dates(&mut rng1, 100, "2020-01-01", "2023-12-31").unwrap();
        let dates2 = generate_dates(&mut rng2, 100, "2020-01-01", "2023-12-31").unwrap();

        assert_ne!(
            dates1, dates2,
            "Different seeds should produce different dates"
        );
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: date batch size is always respected
        #[test]
        fn prop_date_batch_size_respected(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let dates = generate_dates(&mut rng, n, "2020-01-01", "2023-12-31").unwrap();
            prop_assert_eq!(dates.len(), n);
        }

        /// Property: all dates have correct format (10 chars)
        #[test]
        fn prop_date_format(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let dates = generate_dates(&mut rng, n, "2020-01-01", "2023-12-31").unwrap();
            for date in dates {
                prop_assert_eq!(date.len(), 10);
                prop_assert!(parse_date(&date).is_ok());
            }
        }

        /// Property: same seed produces same output
        #[test]
        fn prop_date_seed_determinism(seed_val in any::<u64>(), n in 1usize..100) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let dates1 = generate_dates(&mut rng1, n, "2020-01-01", "2023-12-31").unwrap();
            let dates2 = generate_dates(&mut rng2, n, "2020-01-01", "2023-12-31").unwrap();

            prop_assert_eq!(dates1, dates2);
        }

        /// Property: datetime batch size is always respected
        #[test]
        fn prop_datetime_batch_size_respected(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let dts = generate_datetimes(&mut rng, n, "2020-01-01", "2023-12-31").unwrap();
            prop_assert_eq!(dts.len(), n);
        }

        /// Property: all datetimes have correct format (19 chars)
        #[test]
        fn prop_datetime_format(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let dts = generate_datetimes(&mut rng, n, "2020-01-01", "2023-12-31").unwrap();
            for dt in dts {
                prop_assert_eq!(dt.len(), 19);
                prop_assert!(dt.contains('T'));
            }
        }

        /// Property: date of birth batch size is always respected
        #[test]
        fn prop_dob_batch_size_respected(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let dates = generate_dates_of_birth(&mut rng, n, 18, 65).unwrap();
            prop_assert_eq!(dates.len(), n);
        }
    }
}
