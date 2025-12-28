//! Unified error types for the forgery crate.
//!
//! This module provides a unified error enum that wraps all error types
//! used throughout the crate, enabling consistent error handling.

use crate::providers::datetime::DateRangeError;
use crate::providers::numbers::{FloatRangeError, RangeError};
use crate::{BatchSizeError, LocaleError};
use std::fmt;

/// Error when unique value generation cannot produce enough unique values.
#[derive(Debug, Clone)]
pub struct UniqueExhaustedError {
    /// Number of unique values requested.
    pub requested: usize,
    /// Number of unique values actually generated before exhaustion.
    pub generated: usize,
}

impl fmt::Display for UniqueExhaustedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unique value generation exhausted: requested {} but could only generate {} unique values",
            self.requested, self.generated
        )
    }
}

impl std::error::Error for UniqueExhaustedError {}

/// Unified error type for all forgery operations.
///
/// This enum wraps all specific error types used throughout the crate,
/// allowing callers to handle errors consistently using pattern matching.
#[derive(Debug, Clone)]
pub enum ForgeryError {
    /// Batch size exceeds maximum allowed.
    BatchSize(BatchSizeError),
    /// Unsupported locale.
    Locale(LocaleError),
    /// Invalid integer range (min > max).
    IntegerRange(RangeError),
    /// Invalid float range (min > max or non-finite values).
    FloatRange(FloatRangeError),
    /// Invalid date range.
    DateRange(DateRangeError),
    /// Unique value generation exhausted.
    UniqueExhausted(UniqueExhaustedError),
}

impl fmt::Display for ForgeryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForgeryError::BatchSize(e) => write!(f, "{}", e),
            ForgeryError::Locale(e) => write!(f, "{}", e),
            ForgeryError::IntegerRange(e) => write!(f, "{}", e),
            ForgeryError::FloatRange(e) => write!(f, "{}", e),
            ForgeryError::DateRange(e) => write!(f, "{}", e),
            ForgeryError::UniqueExhausted(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for ForgeryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ForgeryError::BatchSize(e) => Some(e),
            ForgeryError::Locale(e) => Some(e),
            ForgeryError::IntegerRange(e) => Some(e),
            ForgeryError::FloatRange(e) => Some(e),
            ForgeryError::DateRange(e) => Some(e),
            ForgeryError::UniqueExhausted(e) => Some(e),
        }
    }
}

// Implement From for automatic conversion from specific error types

impl From<BatchSizeError> for ForgeryError {
    fn from(err: BatchSizeError) -> Self {
        ForgeryError::BatchSize(err)
    }
}

impl From<LocaleError> for ForgeryError {
    fn from(err: LocaleError) -> Self {
        ForgeryError::Locale(err)
    }
}

impl From<RangeError> for ForgeryError {
    fn from(err: RangeError) -> Self {
        ForgeryError::IntegerRange(err)
    }
}

impl From<FloatRangeError> for ForgeryError {
    fn from(err: FloatRangeError) -> Self {
        ForgeryError::FloatRange(err)
    }
}

impl From<DateRangeError> for ForgeryError {
    fn from(err: DateRangeError) -> Self {
        ForgeryError::DateRange(err)
    }
}

impl From<UniqueExhaustedError> for ForgeryError {
    fn from(err: UniqueExhaustedError) -> Self {
        ForgeryError::UniqueExhausted(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::numbers::FloatRangeErrorReason;
    use std::error::Error;

    #[test]
    fn test_forgery_error_from_batch_size() {
        let err = BatchSizeError {
            requested: 20_000_000,
            max: 10_000_000,
        };
        let forgery_err: ForgeryError = err.into();
        assert!(matches!(forgery_err, ForgeryError::BatchSize(_)));
        assert!(forgery_err.to_string().contains("20000000"));
    }

    #[test]
    fn test_forgery_error_from_range() {
        let err = RangeError { min: 100, max: 0 };
        let forgery_err: ForgeryError = err.into();
        assert!(matches!(forgery_err, ForgeryError::IntegerRange(_)));
    }

    #[test]
    fn test_forgery_error_from_float_range() {
        let err = FloatRangeError {
            min: 100.0,
            max: 0.0,
            reason: FloatRangeErrorReason::MinGreaterThanMax,
        };
        let forgery_err: ForgeryError = err.into();
        assert!(matches!(forgery_err, ForgeryError::FloatRange(_)));
    }

    #[test]
    fn test_forgery_error_from_date_range() {
        let err = DateRangeError {
            start: "2024-01-01".to_string(),
            end: "2023-01-01".to_string(),
            reason: "start after end".to_string(),
        };
        let forgery_err: ForgeryError = err.into();
        assert!(matches!(forgery_err, ForgeryError::DateRange(_)));
    }

    #[test]
    fn test_error_source() {
        let err = RangeError { min: 100, max: 0 };
        let forgery_err: ForgeryError = err.into();
        assert!(forgery_err.source().is_some());
    }
}
