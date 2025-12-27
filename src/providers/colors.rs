//! Color generation provider.
//!
//! Generates color names, hex colors, and RGB tuples.

use crate::data::en_us::COLOR_NAMES;
use crate::rng::ForgeryRng;

/// Generate a batch of random color names.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of colors to generate
pub fn generate_colors(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut colors = Vec::with_capacity(n);
    for _ in 0..n {
        colors.push(generate_color(rng));
    }
    colors
}

/// Generate a single random color name.
///
/// More efficient than `generate_colors(rng, 1)` as it avoids Vec allocation.
#[inline]
pub fn generate_color(rng: &mut ForgeryRng) -> String {
    rng.choose(COLOR_NAMES).to_string()
}

/// Generate a batch of random hex color codes.
///
/// Returns colors in the format `#RRGGBB` (lowercase).
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of hex colors to generate
pub fn generate_hex_colors(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut colors = Vec::with_capacity(n);
    for _ in 0..n {
        colors.push(generate_hex_color(rng));
    }
    colors
}

/// Generate a single random hex color code.
///
/// Returns a color in the format `#RRGGBB` (lowercase).
#[inline]
pub fn generate_hex_color(rng: &mut ForgeryRng) -> String {
    let r: u8 = rng.gen_range(0, 255);
    let g: u8 = rng.gen_range(0, 255);
    let b: u8 = rng.gen_range(0, 255);
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

/// Generate a batch of random RGB color tuples.
///
/// Returns colors as `(r, g, b)` tuples where each component is 0-255.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of RGB colors to generate
pub fn generate_rgb_colors(rng: &mut ForgeryRng, n: usize) -> Vec<(u8, u8, u8)> {
    let mut colors = Vec::with_capacity(n);
    for _ in 0..n {
        colors.push(generate_rgb_color(rng));
    }
    colors
}

/// Generate a single random RGB color tuple.
///
/// Returns a color as `(r, g, b)` where each component is 0-255.
#[inline]
pub fn generate_rgb_color(rng: &mut ForgeryRng) -> (u8, u8, u8) {
    let r: u8 = rng.gen_range(0, 255);
    let g: u8 = rng.gen_range(0, 255);
    let b: u8 = rng.gen_range(0, 255);
    (r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Color name tests
    #[test]
    fn test_generate_colors_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let colors = generate_colors(&mut rng, 100);
        assert_eq!(colors.len(), 100);
    }

    #[test]
    fn test_generate_color_from_data() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let colors = generate_colors(&mut rng, 100);
        for color in &colors {
            assert!(
                COLOR_NAMES.contains(&color.as_str()),
                "Color '{}' not in data",
                color
            );
        }
    }

    #[test]
    fn test_color_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let colors1 = generate_colors(&mut rng1, 100);
        let colors2 = generate_colors(&mut rng2, 100);

        assert_eq!(colors1, colors2);
    }

    #[test]
    fn test_color_empty_batch() {
        let mut rng = ForgeryRng::new();
        let colors = generate_colors(&mut rng, 0);
        assert!(colors.is_empty());
    }

    #[test]
    fn test_color_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let color = generate_color(&mut rng);
        assert!(!color.is_empty());
        assert!(COLOR_NAMES.contains(&color.as_str()));
    }

    // Hex color tests
    #[test]
    fn test_generate_hex_colors_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let colors = generate_hex_colors(&mut rng, 100);
        assert_eq!(colors.len(), 100);
    }

    #[test]
    fn test_hex_color_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let colors = generate_hex_colors(&mut rng, 100);
        for color in &colors {
            // Format: #RRGGBB
            assert_eq!(
                color.len(),
                7,
                "Hex color should be 7 characters: {}",
                color
            );
            assert!(
                color.starts_with('#'),
                "Hex color should start with #: {}",
                color
            );

            // All characters after # should be hex
            for c in color[1..].chars() {
                assert!(c.is_ascii_hexdigit(), "Should be hex digit: {}", color);
                if c.is_alphabetic() {
                    assert!(c.is_lowercase(), "Should be lowercase: {}", color);
                }
            }
        }
    }

    #[test]
    fn test_hex_color_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let colors1 = generate_hex_colors(&mut rng1, 100);
        let colors2 = generate_hex_colors(&mut rng2, 100);

        assert_eq!(colors1, colors2);
    }

    #[test]
    fn test_hex_color_empty_batch() {
        let mut rng = ForgeryRng::new();
        let colors = generate_hex_colors(&mut rng, 0);
        assert!(colors.is_empty());
    }

    #[test]
    fn test_hex_color_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let color = generate_hex_color(&mut rng);
        assert_eq!(color.len(), 7);
        assert!(color.starts_with('#'));
    }

    // RGB color tests
    #[test]
    fn test_generate_rgb_colors_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let colors = generate_rgb_colors(&mut rng, 100);
        assert_eq!(colors.len(), 100);
    }

    #[test]
    fn test_rgb_color_batch_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let colors = generate_rgb_colors(&mut rng, 1000);
        // Verify we get the requested number of colors
        assert_eq!(colors.len(), 1000);
    }

    #[test]
    fn test_rgb_color_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let colors1 = generate_rgb_colors(&mut rng1, 100);
        let colors2 = generate_rgb_colors(&mut rng2, 100);

        assert_eq!(colors1, colors2);
    }

    #[test]
    fn test_rgb_color_empty_batch() {
        let mut rng = ForgeryRng::new();
        let colors = generate_rgb_colors(&mut rng, 0);
        assert!(colors.is_empty());
    }

    #[test]
    fn test_rgb_color_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        // Just verify the function returns a valid tuple (u8 values are always 0-255)
        let (_r, _g, _b) = generate_rgb_color(&mut rng);
    }

    #[test]
    fn test_rgb_variety() {
        // Test that we get a variety of values, not just 0 or 255
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let colors = generate_rgb_colors(&mut rng, 1000);
        let mut has_middle_values = false;
        for (r, g, b) in colors {
            if (r > 50 && r < 200) || (g > 50 && g < 200) || (b > 50 && b < 200) {
                has_middle_values = true;
                break;
            }
        }
        assert!(has_middle_values, "Should have some middle-range values");
    }

    #[test]
    fn test_different_seeds_different_colors() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let colors1 = generate_colors(&mut rng1, 100);
        let colors2 = generate_colors(&mut rng2, 100);

        assert_ne!(
            colors1, colors2,
            "Different seeds should produce different colors"
        );
    }

    #[test]
    fn test_different_seeds_different_hex_colors() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let colors1 = generate_hex_colors(&mut rng1, 100);
        let colors2 = generate_hex_colors(&mut rng2, 100);

        assert_ne!(
            colors1, colors2,
            "Different seeds should produce different hex colors"
        );
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: color batch size is always respected
        #[test]
        fn prop_color_batch_size_respected(n in 0usize..1000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let colors = generate_colors(&mut rng, n);
            prop_assert_eq!(colors.len(), n);
        }

        /// Property: all colors come from the data array
        #[test]
        fn prop_color_from_data(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let colors = generate_colors(&mut rng, n);
            for color in colors {
                prop_assert!(COLOR_NAMES.contains(&color.as_str()));
            }
        }

        /// Property: hex color batch size is always respected
        #[test]
        fn prop_hex_color_batch_size_respected(n in 0usize..1000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let colors = generate_hex_colors(&mut rng, n);
            prop_assert_eq!(colors.len(), n);
        }

        /// Property: all hex colors have correct format
        #[test]
        fn prop_hex_color_format(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let colors = generate_hex_colors(&mut rng, n);
            for color in colors {
                prop_assert_eq!(color.len(), 7);
                prop_assert!(color.starts_with('#'));
                for c in color[1..].chars() {
                    prop_assert!(c.is_ascii_hexdigit());
                }
            }
        }

        /// Property: RGB color batch size is always respected
        #[test]
        fn prop_rgb_color_batch_size_respected(n in 0usize..1000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let colors = generate_rgb_colors(&mut rng, n);
            prop_assert_eq!(colors.len(), n);
        }

        /// Property: color same seed produces same output
        #[test]
        fn prop_color_seed_determinism(seed_val in any::<u64>(), n in 1usize..100) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let colors1 = generate_colors(&mut rng1, n);
            let colors2 = generate_colors(&mut rng2, n);

            prop_assert_eq!(colors1, colors2);
        }

        /// Property: hex color same seed produces same output
        #[test]
        fn prop_hex_color_seed_determinism(seed_val in any::<u64>(), n in 1usize..100) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let colors1 = generate_hex_colors(&mut rng1, n);
            let colors2 = generate_hex_colors(&mut rng2, n);

            prop_assert_eq!(colors1, colors2);
        }

        /// Property: RGB color same seed produces same output
        #[test]
        fn prop_rgb_color_seed_determinism(seed_val in any::<u64>(), n in 1usize..100) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let colors1 = generate_rgb_colors(&mut rng1, n);
            let colors2 = generate_rgb_colors(&mut rng2, n);

            prop_assert_eq!(colors1, colors2);
        }
    }
}
