//! Text generation provider.
//!
//! Generates sentences, paragraphs, and text blocks.

use crate::data::get_locale_data;
use crate::locale::Locale;
use crate::rng::ForgeryRng;

/// Minimum number of words per sentence in paragraph generation.
const MIN_WORDS_PER_SENTENCE: usize = 5;

/// Maximum number of words per sentence in paragraph generation.
const MAX_WORDS_PER_SENTENCE: usize = 15;

/// Generate a batch of random sentences.
pub fn generate_sentences(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    word_count: usize,
) -> Vec<String> {
    let mut sentences = Vec::with_capacity(n);
    for _ in 0..n {
        sentences.push(generate_sentence(rng, locale, word_count));
    }
    sentences
}

/// Generate a single random sentence.
///
/// The sentence starts with a capital letter and ends with a period.
#[inline]
pub fn generate_sentence(rng: &mut ForgeryRng, locale: Locale, word_count: usize) -> String {
    if word_count == 0 {
        return String::new();
    }

    let data = get_locale_data(locale);
    let lorem_words = data.text_words().unwrap_or(&[]);
    if lorem_words.is_empty() {
        return "Lorem ipsum.".to_string();
    }

    let mut words = Vec::with_capacity(word_count);
    for _ in 0..word_count {
        words.push(*rng.choose(lorem_words));
    }

    // Capitalize first word
    let mut sentence = String::new();
    if let Some(first) = words.first() {
        let mut chars = first.chars();
        if let Some(c) = chars.next() {
            sentence.push(c.to_uppercase().next().unwrap_or(c));
            sentence.push_str(chars.as_str());
        }
    }

    // Add remaining words
    for word in words.iter().skip(1) {
        sentence.push(' ');
        sentence.push_str(word);
    }

    sentence.push('.');
    sentence
}

/// Generate a batch of random paragraphs.
pub fn generate_paragraphs(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    sentence_count: usize,
) -> Vec<String> {
    let mut paragraphs = Vec::with_capacity(n);
    for _ in 0..n {
        paragraphs.push(generate_paragraph(rng, locale, sentence_count));
    }
    paragraphs
}

/// Generate a single random paragraph.
///
/// Each paragraph contains the specified number of sentences.
#[inline]
pub fn generate_paragraph(rng: &mut ForgeryRng, locale: Locale, sentence_count: usize) -> String {
    if sentence_count == 0 {
        return String::new();
    }

    let mut sentences = Vec::with_capacity(sentence_count);
    for _ in 0..sentence_count {
        let word_count: usize = rng.gen_range(MIN_WORDS_PER_SENTENCE, MAX_WORDS_PER_SENTENCE);
        sentences.push(generate_sentence(rng, locale, word_count));
    }

    sentences.join(" ")
}

/// Generate a batch of random text blocks with character limits.
pub fn generate_texts(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    min_chars: usize,
    max_chars: usize,
) -> Vec<String> {
    let mut texts = Vec::with_capacity(n);
    for _ in 0..n {
        texts.push(generate_text(rng, locale, min_chars, max_chars));
    }
    texts
}

/// Capitalize the first character of a word.
#[inline]
fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(c) => {
            let upper = c.to_uppercase().next().unwrap_or(c);
            format!("{}{}", upper, chars.as_str())
        }
        None => String::new(),
    }
}

/// Truncate a string to max_chars, respecting UTF-8 character boundaries.
#[inline]
fn truncate_to_char_boundary(text: &mut String, max_chars: usize) {
    if text.len() <= max_chars {
        return;
    }
    // Find the last valid character boundary before max_chars
    let truncate_at = text
        .char_indices()
        .take_while(|(i, _)| *i <= max_chars)
        .last()
        .map(|(i, _)| i)
        .unwrap_or(0);
    text.truncate(truncate_at);
}

/// Generate a single random text block with character limits.
///
/// The text will be between min_chars and max_chars in length.
#[inline]
pub fn generate_text(
    rng: &mut ForgeryRng,
    locale: Locale,
    min_chars: usize,
    max_chars: usize,
) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let data = get_locale_data(locale);
    let lorem_words = data.text_words().unwrap_or(&[]);
    if lorem_words.is_empty() {
        return "Lorem".to_string();
    }

    let target_len = if min_chars >= max_chars {
        max_chars
    } else {
        rng.gen_range(min_chars, max_chars)
    };

    let mut text = String::new();

    // First word - capitalize it
    let first_word = *rng.choose(lorem_words);
    text.push_str(&capitalize_word(first_word));

    // Remaining words
    while text.len() < target_len {
        text.push(' ');
        let word = rng.choose(lorem_words);
        text.push_str(word);
    }

    truncate_to_char_boundary(&mut text, max_chars);
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sentences_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let sentences = generate_sentences(&mut rng, Locale::EnUS, 100, 6);
        assert_eq!(sentences.len(), 100);
    }

    #[test]
    fn test_sentence_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let sentences = generate_sentences(&mut rng, Locale::EnUS, 50, 6);
        for sentence in &sentences {
            // Should start with uppercase
            assert!(
                sentence.chars().next().unwrap().is_uppercase(),
                "Should start with uppercase: {}",
                sentence
            );
            // Should end with period
            assert!(
                sentence.ends_with('.'),
                "Should end with period: {}",
                sentence
            );
        }
    }

    #[test]
    fn test_sentence_word_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let word_count = 8;
        let sentences = generate_sentences(&mut rng, Locale::EnUS, 50, word_count);
        for sentence in &sentences {
            // Remove period and count words
            let without_period = &sentence[..sentence.len() - 1];
            let words: Vec<&str> = without_period.split_whitespace().collect();
            assert_eq!(
                words.len(),
                word_count,
                "Should have {} words: {}",
                word_count,
                sentence
            );
        }
    }

    #[test]
    fn test_sentence_empty() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let sentence = generate_sentence(&mut rng, Locale::EnUS, 0);
        assert!(sentence.is_empty());
    }

    #[test]
    fn test_sentence_single_word() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let sentence = generate_sentence(&mut rng, Locale::EnUS, 1);
        assert!(sentence.ends_with('.'));
        assert!(sentence.chars().next().unwrap().is_uppercase());
    }

    #[test]
    fn test_sentence_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let s1 = generate_sentences(&mut rng1, Locale::EnUS, 100, 6);
        let s2 = generate_sentences(&mut rng2, Locale::EnUS, 100, 6);

        assert_eq!(s1, s2);
    }

    #[test]
    fn test_generate_paragraphs_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let paragraphs = generate_paragraphs(&mut rng, Locale::EnUS, 50, 3);
        assert_eq!(paragraphs.len(), 50);
    }

    #[test]
    fn test_paragraph_sentence_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let sentence_count = 4;
        let paragraphs = generate_paragraphs(&mut rng, Locale::EnUS, 20, sentence_count);
        for paragraph in &paragraphs {
            // Count periods (each sentence ends with one)
            let period_count = paragraph.matches('.').count();
            assert_eq!(
                period_count, sentence_count,
                "Should have {} sentences: {}",
                sentence_count, paragraph
            );
        }
    }

    #[test]
    fn test_paragraph_empty() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let paragraph = generate_paragraph(&mut rng, Locale::EnUS, 0);
        assert!(paragraph.is_empty());
    }

    #[test]
    fn test_paragraph_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let p1 = generate_paragraphs(&mut rng1, Locale::EnUS, 50, 3);
        let p2 = generate_paragraphs(&mut rng2, Locale::EnUS, 50, 3);

        assert_eq!(p1, p2);
    }

    #[test]
    fn test_generate_texts_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let texts = generate_texts(&mut rng, Locale::EnUS, 50, 50, 100);
        assert_eq!(texts.len(), 50);
    }

    #[test]
    fn test_text_length_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let min_chars = 50;
        let max_chars = 100;
        let texts = generate_texts(&mut rng, Locale::EnUS, 100, min_chars, max_chars);
        for text in &texts {
            assert!(
                text.len() >= min_chars && text.len() <= max_chars,
                "Text length {} should be between {} and {}: {}",
                text.len(),
                min_chars,
                max_chars,
                text
            );
        }
    }

    #[test]
    fn test_text_starts_uppercase() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let texts = generate_texts(&mut rng, Locale::EnUS, 50, 10, 50);
        for text in &texts {
            if !text.is_empty() {
                assert!(
                    text.chars().next().unwrap().is_uppercase(),
                    "Should start with uppercase: {}",
                    text
                );
            }
        }
    }

    #[test]
    fn test_text_empty_max() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let text = generate_text(&mut rng, Locale::EnUS, 0, 0);
        assert!(text.is_empty());
    }

    #[test]
    fn test_text_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let t1 = generate_texts(&mut rng1, Locale::EnUS, 50, 50, 100);
        let t2 = generate_texts(&mut rng2, Locale::EnUS, 50, 50, 100);

        assert_eq!(t1, t2);
    }

    #[test]
    fn test_different_seeds_different_sentences() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let s1 = generate_sentences(&mut rng1, Locale::EnUS, 100, 6);
        let s2 = generate_sentences(&mut rng2, Locale::EnUS, 100, 6);

        assert_ne!(s1, s2, "Different seeds should produce different sentences");
    }

    #[test]
    fn test_sentences_empty_batch() {
        let mut rng = ForgeryRng::new();
        let sentences = generate_sentences(&mut rng, Locale::EnUS, 0, 6);
        assert!(sentences.is_empty());
    }

    #[test]
    fn test_paragraphs_empty_batch() {
        let mut rng = ForgeryRng::new();
        let paragraphs = generate_paragraphs(&mut rng, Locale::EnUS, 0, 3);
        assert!(paragraphs.is_empty());
    }

    #[test]
    fn test_texts_empty_batch() {
        let mut rng = ForgeryRng::new();
        let texts = generate_texts(&mut rng, Locale::EnUS, 0, 50, 100);
        assert!(texts.is_empty());
    }

    #[test]
    fn test_all_locales_generate_text() {
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
            let sentence = generate_sentence(&mut rng, locale, 5);
            assert!(
                !sentence.is_empty(),
                "Sentence should not be empty for {:?}",
                locale
            );
        }
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_sentence_batch_size_respected(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let sentences = generate_sentences(&mut rng, Locale::EnUS, n, 6);
            prop_assert_eq!(sentences.len(), n);
        }

        #[test]
        fn prop_sentence_ends_with_period(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let sentences = generate_sentences(&mut rng, Locale::EnUS, n, 6);
            for sentence in sentences {
                prop_assert!(sentence.ends_with('.'));
            }
        }

        #[test]
        fn prop_sentence_starts_uppercase(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let sentences = generate_sentences(&mut rng, Locale::EnUS, n, 6);
            for sentence in sentences {
                prop_assert!(sentence.chars().next().unwrap().is_uppercase());
            }
        }

        #[test]
        fn prop_paragraph_batch_size_respected(n in 0usize..200) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let paragraphs = generate_paragraphs(&mut rng, Locale::EnUS, n, 3);
            prop_assert_eq!(paragraphs.len(), n);
        }

        #[test]
        fn prop_text_batch_size_respected(n in 0usize..200) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let texts = generate_texts(&mut rng, Locale::EnUS, n, 50, 100);
            prop_assert_eq!(texts.len(), n);
        }

        #[test]
        fn prop_text_length_in_range(
            n in 1usize..50,
            min in 10usize..50,
            delta in 10usize..100
        ) {
            let max = min + delta;
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let texts = generate_texts(&mut rng, Locale::EnUS, n, min, max);
            for text in texts {
                prop_assert!(text.len() >= min && text.len() <= max);
            }
        }

        #[test]
        fn prop_sentence_seed_determinism(seed_val in any::<u64>(), n in 1usize..50) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let s1 = generate_sentences(&mut rng1, Locale::EnUS, n, 6);
            let s2 = generate_sentences(&mut rng2, Locale::EnUS, n, 6);

            prop_assert_eq!(s1, s2);
        }
    }
}
