//! Identifier generation provider.
//!
//! Generates UUIDs, and hash-like hex strings.
//!
//! # Note on MD5/SHA256
//!
//! The `generate_md5` and `generate_sha256` functions produce random hex strings
//! that match the format of MD5 (32 chars) and SHA256 (64 chars) hashes.
//! They are NOT cryptographic hashes of any input data - they are simply
//! random hex strings useful for generating fake data.

use crate::rng::ForgeryRng;

/// Lookup table for fast hex encoding.
/// Each index maps to a two-character lowercase hex string.
const HEX_TABLE: &[u8; 512] = b"\
000102030405060708090a0b0c0d0e0f\
101112131415161718191a1b1c1d1e1f\
202122232425262728292a2b2c2d2e2f\
303132333435363738393a3b3c3d3e3f\
404142434445464748494a4b4c4d4e4f\
505152535455565758595a5b5c5d5e5f\
606162636465666768696a6b6c6d6e6f\
707172737475767778797a7b7c7d7e7f\
808182838485868788898a8b8c8d8e8f\
909192939495969798999a9b9c9d9e9f\
a0a1a2a3a4a5a6a7a8a9aaabacadaeaf\
b0b1b2b3b4b5b6b7b8b9babbbcbdbebf\
c0c1c2c3c4c5c6c7c8c9cacbcccdcecf\
d0d1d2d3d4d5d6d7d8d9dadbdcdddedf\
e0e1e2e3e4e5e6e7e8e9eaebecedeeef\
f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff";

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

/// Generate a single UUIDv4 string.
///
/// More efficient than `generate_uuids(rng, 1)` as it avoids Vec allocation.
#[inline]
pub fn generate_uuid(rng: &mut ForgeryRng) -> String {
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);

    // Set version (4) and variant (RFC 4122)
    bytes[6] = (bytes[6] & 0x0f) | 0x40; // Version 4
    bytes[8] = (bytes[8] & 0x3f) | 0x80; // Variant RFC 4122

    format_uuid(&bytes)
}

/// Format 16 bytes as a UUID string using lookup table for performance.
fn format_uuid(bytes: &[u8; 16]) -> String {
    // Pre-allocate exact size: 32 hex chars + 4 dashes = 36
    let mut result = String::with_capacity(36);

    // Helper to push a byte as two hex chars using the lookup table
    #[inline(always)]
    fn push_hex(s: &mut String, byte: u8) {
        let idx = (byte as usize) * 2;
        s.push(HEX_TABLE[idx] as char);
        s.push(HEX_TABLE[idx + 1] as char);
    }

    // xxxxxxxx-
    push_hex(&mut result, bytes[0]);
    push_hex(&mut result, bytes[1]);
    push_hex(&mut result, bytes[2]);
    push_hex(&mut result, bytes[3]);
    result.push('-');

    // xxxx-
    push_hex(&mut result, bytes[4]);
    push_hex(&mut result, bytes[5]);
    result.push('-');

    // xxxx-
    push_hex(&mut result, bytes[6]);
    push_hex(&mut result, bytes[7]);
    result.push('-');

    // xxxx-
    push_hex(&mut result, bytes[8]);
    push_hex(&mut result, bytes[9]);
    result.push('-');

    // xxxxxxxxxxxx
    push_hex(&mut result, bytes[10]);
    push_hex(&mut result, bytes[11]);
    push_hex(&mut result, bytes[12]);
    push_hex(&mut result, bytes[13]);
    push_hex(&mut result, bytes[14]);
    push_hex(&mut result, bytes[15]);

    result
}

/// Generate a batch of MD5-like hash strings (32 lowercase hex characters).
///
/// Note: These are pseudo-random hashes generated from our seeded RNG,
/// not actual MD5 hashes of any input data.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of hashes to generate
pub fn generate_md5s(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut hashes = Vec::with_capacity(n);
    for _ in 0..n {
        hashes.push(generate_md5(rng));
    }
    hashes
}

/// Generate a single MD5-like hash string (32 lowercase hex characters).
///
/// More efficient than `generate_md5s(rng, 1)` as it avoids Vec allocation.
#[inline]
pub fn generate_md5(rng: &mut ForgeryRng) -> String {
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);
    format_hex(&bytes)
}

/// Generate a batch of SHA256-like hash strings (64 lowercase hex characters).
///
/// Note: These are pseudo-random hashes generated from our seeded RNG,
/// not actual SHA256 hashes of any input data.
///
/// # Arguments
///
/// * `rng` - The random number generator to use
/// * `n` - Number of hashes to generate
pub fn generate_sha256s(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut hashes = Vec::with_capacity(n);
    for _ in 0..n {
        hashes.push(generate_sha256(rng));
    }
    hashes
}

/// Generate a single SHA256-like hash string (64 lowercase hex characters).
///
/// More efficient than `generate_sha256s(rng, 1)` as it avoids Vec allocation.
#[inline]
pub fn generate_sha256(rng: &mut ForgeryRng) -> String {
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    format_hex(&bytes)
}

/// Format bytes as a lowercase hex string using a lookup table for performance.
fn format_hex(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        let idx = (byte as usize) * 2;
        // SAFETY: idx is always in range 0..512 since byte is u8 (0..256)
        // and HEX_TABLE has exactly 512 bytes (256 entries * 2 chars each)
        result.push(HEX_TABLE[idx] as char);
        result.push(HEX_TABLE[idx + 1] as char);
    }
    result
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

    // Edge case tests
    #[test]
    fn test_empty_batch() {
        let mut rng = ForgeryRng::new();
        let uuids = generate_uuids(&mut rng, 0);
        assert!(uuids.is_empty());
    }

    #[test]
    fn test_single_uuid_batch() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let uuids = generate_uuids(&mut rng, 1);
        assert_eq!(uuids.len(), 1);
        assert_eq!(uuids[0].len(), 36);
    }

    #[test]
    fn test_format_uuid_function() {
        let bytes: [u8; 16] = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0x4c, 0xde, 0x8f, 0x01, 0x23, 0x45, 0x67, 0x89,
            0xab, 0xcd,
        ];
        let uuid = format_uuid(&bytes);
        assert_eq!(uuid, "01234567-89ab-4cde-8f01-23456789abcd");
    }

    #[test]
    fn test_uuid_all_zeros_bytes() {
        // Test format_uuid with known bytes
        let bytes: [u8; 16] = [0x00; 16];
        let uuid = format_uuid(&bytes);
        assert_eq!(uuid, "00000000-0000-0000-0000-000000000000");
    }

    #[test]
    fn test_uuid_all_ones_bytes() {
        let bytes: [u8; 16] = [0xff; 16];
        let uuid = format_uuid(&bytes);
        assert_eq!(uuid, "ffffffff-ffff-ffff-ffff-ffffffffffff");
    }

    #[test]
    fn test_different_seeds_different_results() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let uuids1 = generate_uuids(&mut rng1, 100);
        let uuids2 = generate_uuids(&mut rng2, 100);

        assert_ne!(
            uuids1, uuids2,
            "Different seeds should produce different UUIDs"
        );
    }

    #[test]
    fn test_large_batch() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let uuids = generate_uuids(&mut rng, 10000);
        assert_eq!(uuids.len(), 10000);

        // All should still be unique
        let unique: std::collections::HashSet<_> = uuids.iter().collect();
        assert_eq!(unique.len(), uuids.len());
    }

    #[test]
    fn test_uuid_lowercase() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let uuids = generate_uuids(&mut rng, 100);
        for uuid in &uuids {
            assert_eq!(*uuid, uuid.to_lowercase(), "UUID should be lowercase");
        }
    }

    #[test]
    fn test_uuid_version_nibble_is_always_4() {
        // The version nibble is at position bytes[6] >> 4
        // After our masking, it should always be 4
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let uuids = generate_uuids(&mut rng, 1000);
        for uuid in &uuids {
            let parts: Vec<&str> = uuid.split('-').collect();
            let version_part = parts[2];
            assert!(
                version_part.starts_with('4'),
                "Version nibble should be 4: {}",
                uuid
            );
        }
    }

    #[test]
    fn test_uuid_variant_nibble_is_rfc4122() {
        // The variant nibble is at position bytes[8] >> 6
        // For RFC 4122, it should be 10xxxxxx, giving chars 8, 9, a, b
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let uuids = generate_uuids(&mut rng, 1000);
        for uuid in &uuids {
            let parts: Vec<&str> = uuid.split('-').collect();
            let variant_part = parts[3];
            let first_char = variant_part.chars().next().unwrap();
            assert!(
                ['8', '9', 'a', 'b'].contains(&first_char),
                "Variant nibble should be 8, 9, a, or b: {}",
                uuid
            );
        }
    }

    // MD5 tests
    #[test]
    fn test_generate_md5s_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let hashes = generate_md5s(&mut rng, 100);
        assert_eq!(hashes.len(), 100);
    }

    #[test]
    fn test_md5_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let hashes = generate_md5s(&mut rng, 50);
        for hash in &hashes {
            // MD5 format: 32 hex characters
            assert_eq!(hash.len(), 32, "MD5 hash should be 32 characters");

            // All characters should be lowercase hex digits
            for c in hash.chars() {
                assert!(
                    c.is_ascii_hexdigit(),
                    "All characters should be hex: {}",
                    hash
                );
                if c.is_alphabetic() {
                    assert!(c.is_lowercase(), "Should be lowercase: {}", hash);
                }
            }
        }
    }

    #[test]
    fn test_md5_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let hashes1 = generate_md5s(&mut rng1, 100);
        let hashes2 = generate_md5s(&mut rng2, 100);

        assert_eq!(hashes1, hashes2);
    }

    #[test]
    fn test_md5_empty_batch() {
        let mut rng = ForgeryRng::new();
        let hashes = generate_md5s(&mut rng, 0);
        assert!(hashes.is_empty());
    }

    #[test]
    fn test_md5_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let hash = generate_md5(&mut rng);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_md5_unique() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let hashes = generate_md5s(&mut rng, 1000);
        let unique: std::collections::HashSet<_> = hashes.iter().collect();
        assert_eq!(
            unique.len(),
            hashes.len(),
            "All MD5 hashes should be unique"
        );
    }

    // SHA256 tests
    #[test]
    fn test_generate_sha256s_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let hashes = generate_sha256s(&mut rng, 100);
        assert_eq!(hashes.len(), 100);
    }

    #[test]
    fn test_sha256_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let hashes = generate_sha256s(&mut rng, 50);
        for hash in &hashes {
            // SHA256 format: 64 hex characters
            assert_eq!(hash.len(), 64, "SHA256 hash should be 64 characters");

            // All characters should be lowercase hex digits
            for c in hash.chars() {
                assert!(
                    c.is_ascii_hexdigit(),
                    "All characters should be hex: {}",
                    hash
                );
                if c.is_alphabetic() {
                    assert!(c.is_lowercase(), "Should be lowercase: {}", hash);
                }
            }
        }
    }

    #[test]
    fn test_sha256_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let hashes1 = generate_sha256s(&mut rng1, 100);
        let hashes2 = generate_sha256s(&mut rng2, 100);

        assert_eq!(hashes1, hashes2);
    }

    #[test]
    fn test_sha256_empty_batch() {
        let mut rng = ForgeryRng::new();
        let hashes = generate_sha256s(&mut rng, 0);
        assert!(hashes.is_empty());
    }

    #[test]
    fn test_sha256_single() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let hash = generate_sha256(&mut rng);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_sha256_unique() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let hashes = generate_sha256s(&mut rng, 1000);
        let unique: std::collections::HashSet<_> = hashes.iter().collect();
        assert_eq!(
            unique.len(),
            hashes.len(),
            "All SHA256 hashes should be unique"
        );
    }

    #[test]
    fn test_format_hex() {
        let bytes = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let hex = format_hex(&bytes);
        assert_eq!(hex, "0123456789abcdef");
    }

    #[test]
    fn test_format_hex_zeros() {
        let bytes = [0x00; 8];
        let hex = format_hex(&bytes);
        assert_eq!(hex, "0000000000000000");
    }

    #[test]
    fn test_format_hex_ones() {
        let bytes = [0xff; 4];
        let hex = format_hex(&bytes);
        assert_eq!(hex, "ffffffff");
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: batch size is always respected
        #[test]
        fn prop_batch_size_respected(n in 0usize..1000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let uuids = generate_uuids(&mut rng, n);
            prop_assert_eq!(uuids.len(), n);
        }

        /// Property: all UUIDs have correct length
        #[test]
        fn prop_uuid_length(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let uuids = generate_uuids(&mut rng, n);
            for uuid in uuids {
                prop_assert_eq!(uuid.len(), 36);
            }
        }

        /// Property: all UUIDs have correct structure (8-4-4-4-12)
        #[test]
        fn prop_uuid_structure(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let uuids = generate_uuids(&mut rng, n);
            for uuid in uuids {
                let parts: Vec<&str> = uuid.split('-').collect();
                prop_assert_eq!(parts.len(), 5);
                prop_assert_eq!(parts[0].len(), 8);
                prop_assert_eq!(parts[1].len(), 4);
                prop_assert_eq!(parts[2].len(), 4);
                prop_assert_eq!(parts[3].len(), 4);
                prop_assert_eq!(parts[4].len(), 12);
            }
        }

        /// Property: all UUIDs are version 4
        #[test]
        fn prop_uuid_version_4(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let uuids = generate_uuids(&mut rng, n);
            for uuid in uuids {
                let chars: Vec<char> = uuid.chars().collect();
                prop_assert_eq!(chars[14], '4');
            }
        }

        /// Property: all UUIDs have RFC 4122 variant
        #[test]
        fn prop_uuid_variant_rfc4122(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let uuids = generate_uuids(&mut rng, n);
            for uuid in uuids {
                let chars: Vec<char> = uuid.chars().collect();
                let variant = chars[19];
                prop_assert!(
                    variant == '8' || variant == '9' || variant == 'a' || variant == 'b'
                );
            }
        }

        /// Property: same seed produces same output
        #[test]
        fn prop_seed_determinism(seed_val in any::<u64>(), n in 1usize..100) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let uuids1 = generate_uuids(&mut rng1, n);
            let uuids2 = generate_uuids(&mut rng2, n);

            prop_assert_eq!(uuids1, uuids2);
        }

        /// Property: all characters are lowercase hex or dash
        #[test]
        fn prop_uuid_chars_valid(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let uuids = generate_uuids(&mut rng, n);
            for uuid in uuids {
                for c in uuid.chars() {
                    prop_assert!(c.is_ascii_hexdigit() || c == '-');
                    if c.is_alphabetic() {
                        prop_assert!(c.is_lowercase());
                    }
                }
            }
        }

        /// Property: UUIDs in a batch are unique (within reasonable batch sizes)
        #[test]
        fn prop_uuids_unique_in_batch(n in 1usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let uuids = generate_uuids(&mut rng, n);
            let unique: std::collections::HashSet<_> = uuids.iter().collect();
            prop_assert_eq!(unique.len(), n);
        }

        // MD5 property tests

        /// Property: MD5 batch size is always respected
        #[test]
        fn prop_md5_batch_size_respected(n in 0usize..1000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let hashes = generate_md5s(&mut rng, n);
            prop_assert_eq!(hashes.len(), n);
        }

        /// Property: all MD5 hashes have correct length (32 chars)
        #[test]
        fn prop_md5_length(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let hashes = generate_md5s(&mut rng, n);
            for hash in hashes {
                prop_assert_eq!(hash.len(), 32);
            }
        }

        /// Property: all MD5 characters are lowercase hex
        #[test]
        fn prop_md5_chars_valid(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let hashes = generate_md5s(&mut rng, n);
            for hash in hashes {
                for c in hash.chars() {
                    prop_assert!(c.is_ascii_hexdigit());
                    if c.is_alphabetic() {
                        prop_assert!(c.is_lowercase());
                    }
                }
            }
        }

        /// Property: MD5 same seed produces same output
        #[test]
        fn prop_md5_seed_determinism(seed_val in any::<u64>(), n in 1usize..100) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let hashes1 = generate_md5s(&mut rng1, n);
            let hashes2 = generate_md5s(&mut rng2, n);

            prop_assert_eq!(hashes1, hashes2);
        }

        // SHA256 property tests

        /// Property: SHA256 batch size is always respected
        #[test]
        fn prop_sha256_batch_size_respected(n in 0usize..1000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let hashes = generate_sha256s(&mut rng, n);
            prop_assert_eq!(hashes.len(), n);
        }

        /// Property: all SHA256 hashes have correct length (64 chars)
        #[test]
        fn prop_sha256_length(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let hashes = generate_sha256s(&mut rng, n);
            for hash in hashes {
                prop_assert_eq!(hash.len(), 64);
            }
        }

        /// Property: all SHA256 characters are lowercase hex
        #[test]
        fn prop_sha256_chars_valid(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let hashes = generate_sha256s(&mut rng, n);
            for hash in hashes {
                for c in hash.chars() {
                    prop_assert!(c.is_ascii_hexdigit());
                    if c.is_alphabetic() {
                        prop_assert!(c.is_lowercase());
                    }
                }
            }
        }

        /// Property: SHA256 same seed produces same output
        #[test]
        fn prop_sha256_seed_determinism(seed_val in any::<u64>(), n in 1usize..100) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let hashes1 = generate_sha256s(&mut rng1, n);
            let hashes2 = generate_sha256s(&mut rng2, n);

            prop_assert_eq!(hashes1, hashes2);
        }
    }
}
