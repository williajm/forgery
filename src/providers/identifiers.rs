//! Identifier generation provider.
//!
//! Generates UUIDs, hashes, and other identifier types.

use crate::rng::ForgeryRng;

/// Generate a batch of UUIDv4 strings.
///
/// Note: These are pseudo-random UUIDs generated from our seeded RNG,
/// not cryptographically secure. Use the `uuid` crate directly for
/// cryptographic purposes.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of UUIDs to generate
pub fn generate_uuids(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut uuids = Vec::with_capacity(n);
    for _ in 0..n {
        let mut bytes = [0u8; 16];
        rng.fill_bytes(&mut bytes);

        // Set version (4) and variant (RFC 4122)
        bytes[6] = (bytes[6] & 0x0f) | 0x40; // Version 4
        bytes[8] = (bytes[8] & 0x3f) | 0x80; // Variant RFC 4122

        uuids.push(format_uuid(&bytes));
    }
    uuids
}

/// Format 16 bytes as a UUID string.
fn format_uuid(bytes: &[u8; 16]) -> String {
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_uuids_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let uuids = generate_uuids(&mut rng, 100);
        assert_eq!(uuids.len(), 100);
    }

    #[test]
    fn test_uuid_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let uuids = generate_uuids(&mut rng, 50);
        for uuid in &uuids {
            // UUID format: 8-4-4-4-12
            assert_eq!(uuid.len(), 36);

            let parts: Vec<&str> = uuid.split('-').collect();
            assert_eq!(parts.len(), 5);
            assert_eq!(parts[0].len(), 8);
            assert_eq!(parts[1].len(), 4);
            assert_eq!(parts[2].len(), 4);
            assert_eq!(parts[3].len(), 4);
            assert_eq!(parts[4].len(), 12);

            // All characters should be hex digits or dashes
            for c in uuid.chars() {
                assert!(c.is_ascii_hexdigit() || c == '-');
            }
        }
    }

    #[test]
    fn test_uuid_version_4() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let uuids = generate_uuids(&mut rng, 100);
        for uuid in &uuids {
            // Version 4 UUIDs have '4' as the 13th character (after the second dash)
            let chars: Vec<char> = uuid.chars().collect();
            assert_eq!(chars[14], '4', "UUID version should be 4: {}", uuid);

            // Variant should be 8, 9, a, or b (19th character)
            let variant = chars[19];
            assert!(
                variant == '8' || variant == '9' || variant == 'a' || variant == 'b',
                "UUID variant should be RFC 4122: {}",
                uuid
            );
        }
    }

    #[test]
    fn test_uuids_are_unique() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let uuids = generate_uuids(&mut rng, 1000);
        let unique: std::collections::HashSet<_> = uuids.iter().collect();

        assert_eq!(unique.len(), uuids.len(), "All UUIDs should be unique");
    }

    #[test]
    fn test_deterministic_generation() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let uuids1 = generate_uuids(&mut rng1, 100);
        let uuids2 = generate_uuids(&mut rng2, 100);

        assert_eq!(uuids1, uuids2);
    }
}
