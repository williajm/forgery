//! Top-level domain data.

/// Common top-level domains.
pub const TLDS: &[&str] = &[
    "com", "org", "net", "edu", "gov", "io", "co", "us", "uk", "de", "fr", "jp", "cn", "au", "ca",
    "in", "br", "ru", "it", "es", "nl", "se", "no", "fi", "dk", "pl", "cz", "at", "ch", "be",
];

/// Free email provider domains.
pub const FREE_EMAIL_DOMAINS: &[&str] = &[
    "gmail.com",
    "yahoo.com",
    "hotmail.com",
    "outlook.com",
    "aol.com",
    "icloud.com",
    "mail.com",
    "protonmail.com",
    "zoho.com",
    "yandex.com",
];

/// Safe email domains (for testing).
pub const SAFE_EMAIL_DOMAINS: &[&str] = &["example.com", "example.org", "example.net"];
