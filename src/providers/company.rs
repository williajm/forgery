//! Company-related data generation provider.
//!
//! Generates company names, job titles, and catch phrases.

use crate::data::en_us::{
    CATCH_PHRASE_ADJECTIVES, CATCH_PHRASE_NOUNS, COMPANY_PREFIXES, COMPANY_SUFFIXES, JOB_TITLES,
};
use crate::rng::ForgeryRng;

/// Generate a batch of random company names.
///
/// Format: "Prefix Suffix" (e.g., "Alpha Inc")
pub fn generate_companies(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut companies = Vec::with_capacity(n);
    for _ in 0..n {
        companies.push(generate_company(rng));
    }
    companies
}

/// Generate a single random company name.
#[inline]
pub fn generate_company(rng: &mut ForgeryRng) -> String {
    let prefix = rng.choose(COMPANY_PREFIXES);
    let suffix = rng.choose(COMPANY_SUFFIXES);
    format!("{} {}", prefix, suffix)
}

/// Generate a batch of random job titles.
pub fn generate_jobs(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut jobs = Vec::with_capacity(n);
    for _ in 0..n {
        jobs.push(generate_job(rng));
    }
    jobs
}

/// Generate a single random job title.
#[inline]
pub fn generate_job(rng: &mut ForgeryRng) -> String {
    rng.choose(JOB_TITLES).to_string()
}

/// Generate a batch of random catch phrases.
///
/// Format: "Adjective Noun" (e.g., "Innovative solution")
pub fn generate_catch_phrases(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut phrases = Vec::with_capacity(n);
    for _ in 0..n {
        phrases.push(generate_catch_phrase(rng));
    }
    phrases
}

/// Generate a single random catch phrase.
#[inline]
pub fn generate_catch_phrase(rng: &mut ForgeryRng) -> String {
    let adj = rng.choose(CATCH_PHRASE_ADJECTIVES);
    let noun = rng.choose(CATCH_PHRASE_NOUNS);
    format!("{} {}", adj, noun)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Company tests
    #[test]
    fn test_generate_companies_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let companies = generate_companies(&mut rng, 100);
        assert_eq!(companies.len(), 100);
    }

    #[test]
    fn test_company_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let companies = generate_companies(&mut rng, 50);
        for company in &companies {
            // Should have at least 2 parts: prefix and suffix
            let parts: Vec<&str> = company.split_whitespace().collect();
            assert!(
                parts.len() >= 2,
                "Company should have at least 2 parts: {}",
                company
            );
        }
    }

    #[test]
    fn test_company_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let c1 = generate_companies(&mut rng1, 100);
        let c2 = generate_companies(&mut rng2, 100);

        assert_eq!(c1, c2);
    }

    // Job tests
    #[test]
    fn test_generate_jobs_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let jobs = generate_jobs(&mut rng, 100);
        assert_eq!(jobs.len(), 100);
    }

    #[test]
    fn test_job_from_data() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let jobs = generate_jobs(&mut rng, 100);
        for job in &jobs {
            assert!(JOB_TITLES.contains(&job.as_str()), "Job '{}' not in data", job);
        }
    }

    #[test]
    fn test_job_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let j1 = generate_jobs(&mut rng1, 100);
        let j2 = generate_jobs(&mut rng2, 100);

        assert_eq!(j1, j2);
    }

    // Catch phrase tests
    #[test]
    fn test_generate_catch_phrases_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let phrases = generate_catch_phrases(&mut rng, 100);
        assert_eq!(phrases.len(), 100);
    }

    #[test]
    fn test_catch_phrase_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let phrases = generate_catch_phrases(&mut rng, 50);
        for phrase in &phrases {
            // Should have at least 2 parts: adjective and noun
            let parts: Vec<&str> = phrase.split_whitespace().collect();
            assert!(
                parts.len() >= 2,
                "Catch phrase should have at least 2 parts: {}",
                phrase
            );
        }
    }

    #[test]
    fn test_catch_phrase_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let p1 = generate_catch_phrases(&mut rng1, 100);
        let p2 = generate_catch_phrases(&mut rng2, 100);

        assert_eq!(p1, p2);
    }

    #[test]
    fn test_empty_batches() {
        let mut rng = ForgeryRng::new();

        assert!(generate_companies(&mut rng, 0).is_empty());
        assert!(generate_jobs(&mut rng, 0).is_empty());
        assert!(generate_catch_phrases(&mut rng, 0).is_empty());
    }

    #[test]
    fn test_different_seeds_different_companies() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let c1 = generate_companies(&mut rng1, 100);
        let c2 = generate_companies(&mut rng2, 100);

        assert_ne!(c1, c2, "Different seeds should produce different companies");
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: company batch size is always respected
        #[test]
        fn prop_company_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let companies = generate_companies(&mut rng, n);
            prop_assert_eq!(companies.len(), n);
        }

        /// Property: job batch size is always respected
        #[test]
        fn prop_job_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let jobs = generate_jobs(&mut rng, n);
            prop_assert_eq!(jobs.len(), n);
        }

        /// Property: all jobs come from data
        #[test]
        fn prop_job_from_data(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let jobs = generate_jobs(&mut rng, n);
            for job in jobs {
                prop_assert!(JOB_TITLES.contains(&job.as_str()));
            }
        }

        /// Property: catch phrase batch size is always respected
        #[test]
        fn prop_catch_phrase_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let phrases = generate_catch_phrases(&mut rng, n);
            prop_assert_eq!(phrases.len(), n);
        }

        /// Property: same seed produces same output
        #[test]
        fn prop_company_seed_determinism(seed_val in any::<u64>(), n in 1usize..50) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let c1 = generate_companies(&mut rng1, n);
            let c2 = generate_companies(&mut rng2, n);

            prop_assert_eq!(c1, c2);
        }
    }
}
