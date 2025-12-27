//! Company-related data generation provider.
//!
//! Generates company names, job titles, and catch phrases.

use crate::data::get_locale_data;
use crate::locale::Locale;
use crate::rng::ForgeryRng;

/// Generate a batch of random company names.
pub fn generate_companies(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut companies = Vec::with_capacity(n);
    for _ in 0..n {
        companies.push(generate_company(rng, locale));
    }
    companies
}

/// Generate a single random company name.
#[inline]
pub fn generate_company(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let prefixes = data.company_prefixes().unwrap_or(&[]);
    let suffixes = data.company_suffixes().unwrap_or(&[]);

    let prefix = if prefixes.is_empty() {
        "Acme"
    } else {
        rng.choose(prefixes)
    };
    let suffix = if suffixes.is_empty() {
        "Inc"
    } else {
        rng.choose(suffixes)
    };
    format!("{} {}", prefix, suffix)
}

/// Generate a batch of random job titles.
pub fn generate_jobs(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut jobs = Vec::with_capacity(n);
    for _ in 0..n {
        jobs.push(generate_job(rng, locale));
    }
    jobs
}

/// Generate a single random job title.
#[inline]
pub fn generate_job(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let titles = data.job_titles().unwrap_or(&[]);
    if titles.is_empty() {
        "Manager".to_string()
    } else {
        rng.choose(titles).to_string()
    }
}

/// Generate a batch of random catch phrases.
pub fn generate_catch_phrases(rng: &mut ForgeryRng, locale: Locale, n: usize) -> Vec<String> {
    let mut phrases = Vec::with_capacity(n);
    for _ in 0..n {
        phrases.push(generate_catch_phrase(rng, locale));
    }
    phrases
}

/// Generate a single random catch phrase.
#[inline]
pub fn generate_catch_phrase(rng: &mut ForgeryRng, locale: Locale) -> String {
    let data = get_locale_data(locale);
    let adjectives = data.catch_phrase_adjectives().unwrap_or(&[]);
    let nouns = data.catch_phrase_nouns().unwrap_or(&[]);

    let adj = if adjectives.is_empty() {
        "Innovative"
    } else {
        rng.choose(adjectives)
    };
    let noun = if nouns.is_empty() {
        "solution"
    } else {
        rng.choose(nouns)
    };
    format!("{} {}", adj, noun)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::en_us::JOB_TITLES;

    #[test]
    fn test_generate_companies_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let companies = generate_companies(&mut rng, Locale::EnUS, 100);
        assert_eq!(companies.len(), 100);
    }

    #[test]
    fn test_company_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let companies = generate_companies(&mut rng, Locale::EnUS, 50);
        for company in &companies {
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

        let c1 = generate_companies(&mut rng1, Locale::EnUS, 100);
        let c2 = generate_companies(&mut rng2, Locale::EnUS, 100);

        assert_eq!(c1, c2);
    }

    #[test]
    fn test_generate_jobs_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let jobs = generate_jobs(&mut rng, Locale::EnUS, 100);
        assert_eq!(jobs.len(), 100);
    }

    #[test]
    fn test_job_from_data() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let jobs = generate_jobs(&mut rng, Locale::EnUS, 100);
        for job in &jobs {
            assert!(
                JOB_TITLES.contains(&job.as_str()),
                "Job '{}' not in data",
                job
            );
        }
    }

    #[test]
    fn test_job_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let j1 = generate_jobs(&mut rng1, Locale::EnUS, 100);
        let j2 = generate_jobs(&mut rng2, Locale::EnUS, 100);

        assert_eq!(j1, j2);
    }

    #[test]
    fn test_generate_catch_phrases_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let phrases = generate_catch_phrases(&mut rng, Locale::EnUS, 100);
        assert_eq!(phrases.len(), 100);
    }

    #[test]
    fn test_catch_phrase_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let phrases = generate_catch_phrases(&mut rng, Locale::EnUS, 50);
        for phrase in &phrases {
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

        let p1 = generate_catch_phrases(&mut rng1, Locale::EnUS, 100);
        let p2 = generate_catch_phrases(&mut rng2, Locale::EnUS, 100);

        assert_eq!(p1, p2);
    }

    #[test]
    fn test_empty_batches() {
        let mut rng = ForgeryRng::new();

        assert!(generate_companies(&mut rng, Locale::EnUS, 0).is_empty());
        assert!(generate_jobs(&mut rng, Locale::EnUS, 0).is_empty());
        assert!(generate_catch_phrases(&mut rng, Locale::EnUS, 0).is_empty());
    }

    #[test]
    fn test_different_seeds_different_companies() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let c1 = generate_companies(&mut rng1, Locale::EnUS, 100);
        let c2 = generate_companies(&mut rng2, Locale::EnUS, 100);

        assert_ne!(c1, c2, "Different seeds should produce different companies");
    }

    #[test]
    fn test_all_locales_generate_company() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        for locale in [
            Locale::EnUS,
            Locale::EnGB,
            Locale::DeDE,
            Locale::FrFR,
            Locale::EsES,
            Locale::ItIT,
            Locale::JaJP,
        ] {
            let company = generate_company(&mut rng, locale);
            assert!(
                !company.is_empty(),
                "Company should not be empty for {:?}",
                locale
            );
        }
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use crate::data::en_us::JOB_TITLES;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_company_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let companies = generate_companies(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(companies.len(), n);
        }

        #[test]
        fn prop_job_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let jobs = generate_jobs(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(jobs.len(), n);
        }

        #[test]
        fn prop_job_from_data(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let jobs = generate_jobs(&mut rng, Locale::EnUS, n);
            for job in jobs {
                prop_assert!(JOB_TITLES.contains(&job.as_str()));
            }
        }

        #[test]
        fn prop_catch_phrase_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let phrases = generate_catch_phrases(&mut rng, Locale::EnUS, n);
            prop_assert_eq!(phrases.len(), n);
        }

        #[test]
        fn prop_company_seed_determinism(seed_val in any::<u64>(), n in 1usize..50) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let c1 = generate_companies(&mut rng1, Locale::EnUS, n);
            let c2 = generate_companies(&mut rng2, Locale::EnUS, n);

            prop_assert_eq!(c1, c2);
        }
    }
}
