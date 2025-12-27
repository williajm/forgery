//! Custom provider support for user-defined data generation.
//!
//! This module provides functionality for users to register their own data providers
//! that integrate seamlessly with forgery's generation system.

use crate::rng::ForgeryRng;

/// Error types for custom provider operations.
#[derive(Debug, Clone)]
pub enum CustomProviderError {
    /// Provider name conflicts with a built-in type.
    NameCollision(String),
    /// Provider not found.
    NotFound(String),
    /// Invalid weights (empty, zero total, overflow, etc.).
    InvalidWeights(String),
    /// Empty options list.
    EmptyOptions,
}

impl std::fmt::Display for CustomProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NameCollision(name) => {
                write!(f, "provider name '{}' conflicts with built-in type", name)
            }
            Self::NotFound(name) => write!(f, "custom provider '{}' not found", name),
            Self::InvalidWeights(msg) => write!(f, "invalid weights: {}", msg),
            Self::EmptyOptions => write!(f, "options list cannot be empty"),
        }
    }
}

impl std::error::Error for CustomProviderError {}

/// A custom data provider registered by the user.
///
/// Custom providers store data arrays and generate random values from them.
/// They support both uniform (equal probability) and weighted selection.
#[derive(Debug, Clone)]
pub enum CustomProvider {
    /// Simple uniform random choice from options.
    /// Each option has equal probability of being selected.
    Uniform(Vec<String>),

    /// Weighted random choice.
    /// Options are selected based on their relative weights.
    Weighted {
        /// The values to choose from.
        values: Vec<String>,
        /// Precomputed cumulative weights for O(log n) selection via binary search.
        cumulative_weights: Vec<u64>,
        /// Total sum of all weights.
        total_weight: u64,
    },
}

impl CustomProvider {
    /// Create a uniform (unweighted) provider.
    ///
    /// Each option has equal probability of being selected.
    ///
    /// # Arguments
    ///
    /// * `options` - The values to choose from (must not be empty)
    ///
    /// # Errors
    ///
    /// Returns `CustomProviderError::EmptyOptions` if `options` is empty.
    pub fn uniform(options: Vec<String>) -> Result<Self, CustomProviderError> {
        if options.is_empty() {
            return Err(CustomProviderError::EmptyOptions);
        }
        Ok(Self::Uniform(options))
    }

    /// Create a weighted provider from (value, weight) pairs.
    ///
    /// Options are selected based on their relative weights. A value with
    /// weight 80 is 4x more likely to be selected than one with weight 20.
    ///
    /// Zero-weight items are excluded from selection.
    ///
    /// # Arguments
    ///
    /// * `pairs` - List of (value, weight) tuples
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `pairs` is empty
    /// - All weights are zero
    /// - Total weight overflows u64
    pub fn weighted(pairs: Vec<(String, u64)>) -> Result<Self, CustomProviderError> {
        if pairs.is_empty() {
            return Err(CustomProviderError::EmptyOptions);
        }

        let mut values = Vec::with_capacity(pairs.len());
        let mut cumulative_weights = Vec::with_capacity(pairs.len());
        let mut total: u64 = 0;

        for (value, weight) in pairs {
            if weight == 0 {
                continue; // Skip zero-weight items
            }
            values.push(value);
            total = total.checked_add(weight).ok_or_else(|| {
                CustomProviderError::InvalidWeights("weight overflow".to_string())
            })?;
            cumulative_weights.push(total);
        }

        if values.is_empty() {
            return Err(CustomProviderError::InvalidWeights(
                "all weights are zero".to_string(),
            ));
        }

        Ok(Self::Weighted {
            values,
            cumulative_weights,
            total_weight: total,
        })
    }

    /// Generate a single value from this provider.
    ///
    /// # Arguments
    ///
    /// * `rng` - The random number generator to use
    ///
    /// # Returns
    ///
    /// A randomly selected string from the provider's options.
    pub fn generate(&self, rng: &mut ForgeryRng) -> String {
        match self {
            Self::Uniform(options) => rng.choose(options).clone(),
            Self::Weighted {
                values,
                cumulative_weights,
                total_weight,
            } => {
                // Generate random value in range [1, total_weight] inclusive
                let r = rng.gen_range(1u64, *total_weight);
                // Binary search for the bucket where cumulative_weight >= r
                let idx = cumulative_weights.partition_point(|&w| w < r);
                values[idx].clone()
            }
        }
    }

    /// Generate a batch of values from this provider.
    ///
    /// # Arguments
    ///
    /// * `rng` - The random number generator to use
    /// * `n` - Number of values to generate
    ///
    /// # Returns
    ///
    /// A vector of `n` randomly selected strings.
    pub fn generate_batch(&self, rng: &mut ForgeryRng, n: usize) -> Vec<String> {
        let mut results = Vec::with_capacity(n);
        for _ in 0..n {
            results.push(self.generate(rng));
        }
        results
    }
}

/// Reserved provider names that cannot be used for custom providers.
///
/// This list includes both schema type names (used in `records()`) and
/// API method names to prevent confusion. Name matching is case-sensitive,
/// so "Name" is allowed even though "name" is reserved.
pub const RESERVED_PROVIDER_NAMES: &[&str] = &[
    // Names
    "name",
    "first_name",
    "last_name",
    // Internet
    "email",
    "safe_email",
    "free_email",
    "url",
    "domain_name",
    // Identifiers
    "uuid",
    "md5",
    "sha256",
    // Numbers
    "int",
    "float",
    // Phone
    "phone",
    "phone_number",
    // Address
    "address",
    "street_address",
    "city",
    "state",
    "country",
    "zip_code",
    // Company
    "company",
    "job",
    "catch_phrase",
    // Network
    "ipv4",
    "ipv6",
    "mac_address",
    // Colors
    "color",
    "hex_color",
    "rgb_color",
    // Finance
    "credit_card",
    "iban",
    // DateTime
    "date",
    "datetime",
    "date_of_birth",
    // Text
    "sentence",
    "paragraph",
    "text",
];

/// Check if a name is reserved and cannot be used for custom providers.
///
/// Name matching is case-sensitive: "Name" is allowed, "name" is not.
///
/// # Arguments
///
/// * `name` - The name to check
///
/// # Returns
///
/// `true` if the name is reserved, `false` otherwise.
pub fn is_reserved_name(name: &str) -> bool {
    RESERVED_PROVIDER_NAMES.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_provider_creation() {
        let provider =
            CustomProvider::uniform(vec!["a".to_string(), "b".to_string(), "c".to_string()])
                .unwrap();
        assert!(matches!(provider, CustomProvider::Uniform(_)));
    }

    #[test]
    fn test_uniform_provider_empty_fails() {
        let result = CustomProvider::uniform(vec![]);
        assert!(matches!(result, Err(CustomProviderError::EmptyOptions)));
    }

    #[test]
    fn test_weighted_provider_creation() {
        let pairs = vec![("a".to_string(), 80), ("b".to_string(), 20)];
        let provider = CustomProvider::weighted(pairs).unwrap();
        assert!(matches!(provider, CustomProvider::Weighted { .. }));
    }

    #[test]
    fn test_weighted_provider_empty_fails() {
        let result = CustomProvider::weighted(vec![]);
        assert!(matches!(result, Err(CustomProviderError::EmptyOptions)));
    }

    #[test]
    fn test_weighted_provider_all_zero_fails() {
        let pairs = vec![("a".to_string(), 0), ("b".to_string(), 0)];
        let result = CustomProvider::weighted(pairs);
        assert!(matches!(
            result,
            Err(CustomProviderError::InvalidWeights(_))
        ));
    }

    #[test]
    fn test_weighted_provider_skips_zero_weights() {
        let pairs = vec![
            ("a".to_string(), 50),
            ("b".to_string(), 0), // This should be skipped
            ("c".to_string(), 50),
        ];
        let provider = CustomProvider::weighted(pairs).unwrap();
        if let CustomProvider::Weighted { values, .. } = provider {
            assert_eq!(values.len(), 2);
            assert!(values.contains(&"a".to_string()));
            assert!(values.contains(&"c".to_string()));
            assert!(!values.contains(&"b".to_string()));
        } else {
            panic!("Expected Weighted variant");
        }
    }

    #[test]
    fn test_uniform_generation_returns_valid_option() {
        let options = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let provider = CustomProvider::uniform(options.clone()).unwrap();

        let mut rng = ForgeryRng::new();
        rng.seed(42);

        for _ in 0..100 {
            let value = provider.generate(&mut rng);
            assert!(options.contains(&value));
        }
    }

    #[test]
    fn test_uniform_generation_deterministic() {
        let provider =
            CustomProvider::uniform(vec!["a".to_string(), "b".to_string(), "c".to_string()])
                .unwrap();

        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();
        rng1.seed(42);
        rng2.seed(42);

        let v1: Vec<_> = (0..100).map(|_| provider.generate(&mut rng1)).collect();
        let v2: Vec<_> = (0..100).map(|_| provider.generate(&mut rng2)).collect();

        assert_eq!(v1, v2);
    }

    #[test]
    fn test_weighted_generation_returns_valid_option() {
        let pairs = vec![("a".to_string(), 80), ("b".to_string(), 20)];
        let provider = CustomProvider::weighted(pairs).unwrap();

        let mut rng = ForgeryRng::new();
        rng.seed(42);

        for _ in 0..100 {
            let value = provider.generate(&mut rng);
            assert!(value == "a" || value == "b");
        }
    }

    #[test]
    fn test_weighted_distribution() {
        let pairs = vec![("a".to_string(), 90), ("b".to_string(), 10)];
        let provider = CustomProvider::weighted(pairs).unwrap();

        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let results = provider.generate_batch(&mut rng, 10000);
        let a_count = results.iter().filter(|s| *s == "a").count();

        // Should be roughly 90% (9000), allow some variance
        assert!(
            a_count > 8500 && a_count < 9500,
            "a_count was {} (expected ~9000)",
            a_count
        );
    }

    #[test]
    fn test_batch_generation() {
        let provider =
            CustomProvider::uniform(vec!["x".to_string(), "y".to_string(), "z".to_string()])
                .unwrap();

        let mut rng = ForgeryRng::new();
        let results = provider.generate_batch(&mut rng, 50);

        assert_eq!(results.len(), 50);
    }

    #[test]
    fn test_single_option_always_returns_it() {
        let provider = CustomProvider::uniform(vec!["only".to_string()]).unwrap();

        let mut rng = ForgeryRng::new();
        for _ in 0..100 {
            assert_eq!(provider.generate(&mut rng), "only");
        }
    }

    #[test]
    fn test_reserved_name_check() {
        // Reserved names
        assert!(is_reserved_name("name"));
        assert!(is_reserved_name("email"));
        assert!(is_reserved_name("uuid"));
        assert!(is_reserved_name("int"));
        assert!(is_reserved_name("float"));
        assert!(is_reserved_name("phone"));
        assert!(is_reserved_name("address"));
        assert!(is_reserved_name("credit_card"));

        // Custom names (not reserved)
        assert!(!is_reserved_name("department"));
        assert!(!is_reserved_name("custom_field"));
        assert!(!is_reserved_name("my_provider"));
        assert!(!is_reserved_name(""));

        // Case sensitivity: "Name" is allowed even though "name" is reserved
        assert!(!is_reserved_name("Name"));
        assert!(!is_reserved_name("EMAIL"));
        assert!(!is_reserved_name("Int"));
    }

    #[test]
    fn test_error_display() {
        let err = CustomProviderError::NameCollision("name".to_string());
        assert!(err.to_string().contains("conflicts with built-in"));

        let err = CustomProviderError::NotFound("foo".to_string());
        assert!(err.to_string().contains("not found"));

        let err = CustomProviderError::InvalidWeights("overflow".to_string());
        assert!(err.to_string().contains("invalid weights"));

        let err = CustomProviderError::EmptyOptions;
        assert!(err.to_string().contains("empty"));
    }
}
