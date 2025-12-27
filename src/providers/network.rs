//! Network-related data generation provider.
//!
//! Generates URLs, domain names, IP addresses, and MAC addresses.

use crate::data::en_us::TLDS;
use crate::rng::ForgeryRng;

/// Generate a batch of random domain names.
pub fn generate_domain_names(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut domains = Vec::with_capacity(n);
    for _ in 0..n {
        domains.push(generate_domain_name(rng));
    }
    domains
}

/// Generate a single random domain name.
#[inline]
pub fn generate_domain_name(rng: &mut ForgeryRng) -> String {
    let words = [
        "example", "test", "sample", "demo", "data", "info", "site", "web", "app", "api",
    ];
    let word = rng.choose(&words);
    let tld = rng.choose(TLDS);
    format!("{}.{}", word, tld)
}

/// Generate a batch of random URLs.
pub fn generate_urls(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut urls = Vec::with_capacity(n);
    for _ in 0..n {
        urls.push(generate_url(rng));
    }
    urls
}

/// Generate a single random URL.
#[inline]
pub fn generate_url(rng: &mut ForgeryRng) -> String {
    let domain = generate_domain_name(rng);
    let paths = [
        "",
        "/about",
        "/contact",
        "/products",
        "/services",
        "/blog",
        "/api",
        "/docs",
    ];
    let path = rng.choose(&paths);
    format!("https://{}{}", domain, path)
}

/// Generate a batch of random IPv4 addresses.
pub fn generate_ipv4s(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut ips = Vec::with_capacity(n);
    for _ in 0..n {
        ips.push(generate_ipv4(rng));
    }
    ips
}

/// Generate a single random IPv4 address.
#[inline]
pub fn generate_ipv4(rng: &mut ForgeryRng) -> String {
    let a: u8 = rng.gen_range(1, 255);
    let b: u8 = rng.gen_range(0, 255);
    let c: u8 = rng.gen_range(0, 255);
    let d: u8 = rng.gen_range(1, 254);
    format!("{}.{}.{}.{}", a, b, c, d)
}

/// Generate a batch of random IPv6 addresses.
pub fn generate_ipv6s(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut ips = Vec::with_capacity(n);
    for _ in 0..n {
        ips.push(generate_ipv6(rng));
    }
    ips
}

/// Generate a single random IPv6 address.
#[inline]
pub fn generate_ipv6(rng: &mut ForgeryRng) -> String {
    let mut groups = Vec::with_capacity(8);
    for _ in 0..8 {
        let group: u16 = rng.gen_range(0, 65535);
        groups.push(format!("{:04x}", group));
    }
    groups.join(":")
}

/// Generate a batch of random MAC addresses.
pub fn generate_mac_addresses(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut macs = Vec::with_capacity(n);
    for _ in 0..n {
        macs.push(generate_mac_address(rng));
    }
    macs
}

/// Generate a single random MAC address.
#[inline]
pub fn generate_mac_address(rng: &mut ForgeryRng) -> String {
    let mut bytes = [0u8; 6];
    rng.fill_bytes(&mut bytes);
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // Domain name tests
    #[test]
    fn test_generate_domain_names_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let domains = generate_domain_names(&mut rng, 100);
        assert_eq!(domains.len(), 100);
    }

    #[test]
    fn test_domain_name_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let domains = generate_domain_names(&mut rng, 50);
        for domain in &domains {
            assert!(domain.contains('.'), "Domain should have dot: {}", domain);
        }
    }

    // URL tests
    #[test]
    fn test_generate_urls_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let urls = generate_urls(&mut rng, 100);
        assert_eq!(urls.len(), 100);
    }

    #[test]
    fn test_url_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let urls = generate_urls(&mut rng, 50);
        for url in &urls {
            assert!(
                url.starts_with("https://"),
                "URL should start with https://: {}",
                url
            );
        }
    }

    // IPv4 tests
    #[test]
    fn test_generate_ipv4s_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ips = generate_ipv4s(&mut rng, 100);
        assert_eq!(ips.len(), 100);
    }

    #[test]
    fn test_ipv4_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ips = generate_ipv4s(&mut rng, 100);
        for ip in &ips {
            let parts: Vec<&str> = ip.split('.').collect();
            assert_eq!(parts.len(), 4, "IPv4 should have 4 parts: {}", ip);
            for part in &parts {
                let num: u8 = part.parse().expect("Should be valid number");
                assert!(num <= 255, "Each octet should be 0-255: {}", ip);
            }
        }
    }

    // IPv6 tests
    #[test]
    fn test_generate_ipv6s_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ips = generate_ipv6s(&mut rng, 100);
        assert_eq!(ips.len(), 100);
    }

    #[test]
    fn test_ipv6_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let ips = generate_ipv6s(&mut rng, 50);
        for ip in &ips {
            let parts: Vec<&str> = ip.split(':').collect();
            assert_eq!(parts.len(), 8, "IPv6 should have 8 groups: {}", ip);
            for part in &parts {
                assert_eq!(part.len(), 4, "Each group should be 4 hex chars: {}", ip);
                assert!(
                    part.chars().all(|c| c.is_ascii_hexdigit()),
                    "Should be hex: {}",
                    ip
                );
            }
        }
    }

    // MAC address tests
    #[test]
    fn test_generate_mac_addresses_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let macs = generate_mac_addresses(&mut rng, 100);
        assert_eq!(macs.len(), 100);
    }

    #[test]
    fn test_mac_address_format() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let macs = generate_mac_addresses(&mut rng, 50);
        for mac in &macs {
            assert_eq!(mac.len(), 17, "MAC should be 17 chars: {}", mac);
            let parts: Vec<&str> = mac.split(':').collect();
            assert_eq!(parts.len(), 6, "MAC should have 6 parts: {}", mac);
            for part in &parts {
                assert_eq!(part.len(), 2, "Each part should be 2 hex chars: {}", mac);
            }
        }
    }

    #[test]
    fn test_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        assert_eq!(generate_ipv4s(&mut rng1, 50), generate_ipv4s(&mut rng2, 50));
    }

    #[test]
    fn test_empty_batches() {
        let mut rng = ForgeryRng::new();

        assert!(generate_domain_names(&mut rng, 0).is_empty());
        assert!(generate_urls(&mut rng, 0).is_empty());
        assert!(generate_ipv4s(&mut rng, 0).is_empty());
        assert!(generate_ipv6s(&mut rng, 0).is_empty());
        assert!(generate_mac_addresses(&mut rng, 0).is_empty());
    }

    #[test]
    fn test_different_seeds_different_ips() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(1);
        rng2.seed(2);

        let i1 = generate_ipv4s(&mut rng1, 100);
        let i2 = generate_ipv4s(&mut rng2, 100);

        assert_ne!(i1, i2, "Different seeds should produce different IPs");
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: IPv4 batch size is always respected
        #[test]
        fn prop_ipv4_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let ips = generate_ipv4s(&mut rng, n);
            prop_assert_eq!(ips.len(), n);
        }

        /// Property: IPv6 batch size is always respected
        #[test]
        fn prop_ipv6_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let ips = generate_ipv6s(&mut rng, n);
            prop_assert_eq!(ips.len(), n);
        }

        /// Property: MAC address batch size is always respected
        #[test]
        fn prop_mac_batch_size(n in 0usize..500) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let macs = generate_mac_addresses(&mut rng, n);
            prop_assert_eq!(macs.len(), n);
        }

        /// Property: IPv4 addresses have correct format
        #[test]
        fn prop_ipv4_format(n in 1usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let ips = generate_ipv4s(&mut rng, n);
            for ip in ips {
                let parts: Vec<&str> = ip.split('.').collect();
                prop_assert_eq!(parts.len(), 4);
            }
        }

        /// Property: same seed produces same output
        #[test]
        fn prop_seed_determinism(seed_val in any::<u64>(), n in 1usize..50) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let i1 = generate_ipv4s(&mut rng1, n);
            let i2 = generate_ipv4s(&mut rng2, n);

            prop_assert_eq!(i1, i2);
        }
    }
}
