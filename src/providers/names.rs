//! Name generation provider.
//!
//! Generates first names, last names, and full names using embedded data.

use crate::data::en_us::{FIRST_NAMES, LAST_NAMES};
use crate::rng::ForgeryRng;

/// Generate a batch of full names (first + last).
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of names to generate
///
/// # Returns
///
/// A vector of full names
pub fn generate_names(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut names = Vec::with_capacity(n);
    for _ in 0..n {
        let first = rng.choose(FIRST_NAMES);
        let last = rng.choose(LAST_NAMES);
        names.push(format!("{} {}", first, last));
    }
    names
}

/// Generate a batch of first names.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of first names to generate
pub fn generate_first_names(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut names = Vec::with_capacity(n);
    for _ in 0..n {
        names.push(rng.choose(FIRST_NAMES).to_string());
    }
    names
}

/// Generate a batch of last names.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of last names to generate
pub fn generate_last_names(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut names = Vec::with_capacity(n);
    for _ in 0..n {
        names.push(rng.choose(LAST_NAMES).to_string());
    }
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_names_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let names = generate_names(&mut rng, 100);
        assert_eq!(names.len(), 100);
    }

    #[test]
    fn test_generate_names_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let names = generate_names(&mut rng, 10);
        for name in &names {
            // Each name should have exactly one space (first + last)
            assert_eq!(name.matches(' ').count(), 1);
            assert!(!name.is_empty());
        }
    }

    #[test]
    fn test_generate_first_names() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let names = generate_first_names(&mut rng, 50);
        assert_eq!(names.len(), 50);
        for name in &names {
            assert!(!name.contains(' '));
            assert!(FIRST_NAMES.contains(&name.as_str()));
        }
    }

    #[test]
    fn test_generate_last_names() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let names = generate_last_names(&mut rng, 50);
        assert_eq!(names.len(), 50);
        for name in &names {
            assert!(LAST_NAMES.contains(&name.as_str()));
        }
    }

    #[test]
    fn test_deterministic_generation() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let names1 = generate_names(&mut rng1, 100);
        let names2 = generate_names(&mut rng2, 100);

        assert_eq!(names1, names2);
    }

    #[test]
    fn test_empty_batch() {
        let mut rng = ForgeryRng::new();
        let names = generate_names(&mut rng, 0);
        assert!(names.is_empty());
    }
}
