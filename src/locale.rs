//! Locale support for forgery.
//!
//! This module provides the `Locale` enum and related types for
//! locale-specific data generation.

use std::fmt;
use std::str::FromStr;

/// Supported locales for data generation.
///
/// Each locale provides locale-specific data for names, addresses,
/// phone numbers, companies, and other generated content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Locale {
    /// English (United States)
    #[default]
    EnUS,
    /// German (Germany)
    DeDE,
    /// French (France)
    FrFR,
    /// Spanish (Spain)
    EsES,
    /// Italian (Italy)
    ItIT,
    /// Japanese (Japan)
    JaJP,
    /// English (United Kingdom)
    EnGB,
}

impl Locale {
    /// All supported locales.
    pub const ALL: &'static [Locale] = &[
        Locale::EnUS,
        Locale::DeDE,
        Locale::FrFR,
        Locale::EsES,
        Locale::ItIT,
        Locale::JaJP,
        Locale::EnGB,
    ];

    /// Get the string representation of the locale.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use forgery::locale::Locale;
    ///
    /// assert_eq!(Locale::EnUS.as_str(), "en_US");
    /// assert_eq!(Locale::DeDE.as_str(), "de_DE");
    /// ```
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Locale::EnUS => "en_US",
            Locale::DeDE => "de_DE",
            Locale::FrFR => "fr_FR",
            Locale::EsES => "es_ES",
            Locale::ItIT => "it_IT",
            Locale::JaJP => "ja_JP",
            Locale::EnGB => "en_GB",
        }
    }

    /// Check if this locale uses family name first (e.g., Japanese).
    #[inline]
    pub const fn family_name_first(&self) -> bool {
        matches!(self, Locale::JaJP)
    }

    /// Check if this locale places street number before street name.
    #[inline]
    pub const fn number_before_street(&self) -> bool {
        match self {
            Locale::EnUS | Locale::EnGB => true,
            Locale::DeDE | Locale::FrFR | Locale::EsES | Locale::ItIT | Locale::JaJP => false,
        }
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Locale {
    type Err = LocaleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en_US" => Ok(Locale::EnUS),
            "de_DE" => Ok(Locale::DeDE),
            "fr_FR" => Ok(Locale::FrFR),
            "es_ES" => Ok(Locale::EsES),
            "it_IT" => Ok(Locale::ItIT),
            "ja_JP" => Ok(Locale::JaJP),
            "en_GB" => Ok(Locale::EnGB),
            _ => Err(LocaleError {
                requested: s.to_string(),
            }),
        }
    }
}

/// Error type for unsupported locale.
#[derive(Debug, Clone)]
pub struct LocaleError {
    /// The requested locale that was not found.
    pub requested: String,
}

impl LocaleError {
    /// Get the list of supported locale strings.
    pub fn supported_locales() -> Vec<&'static str> {
        Locale::ALL.iter().map(|l| l.as_str()).collect()
    }
}

impl fmt::Display for LocaleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unsupported locale '{}', supported locales: {}",
            self.requested,
            Self::supported_locales().join(", ")
        )
    }
}

impl std::error::Error for LocaleError {}

/// Error type for methods not supported by a locale.
///
/// This error is returned when a generation method is called on a locale
/// that doesn't have the required data for that method.
#[derive(Debug, Clone)]
pub struct UnsupportedLocaleMethodError {
    /// The locale that was used.
    pub locale: Locale,
    /// The method that was called.
    pub method: &'static str,
}

impl fmt::Display for UnsupportedLocaleMethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "method '{}' is not supported for locale '{}'",
            self.method,
            self.locale.as_str()
        )
    }
}

impl std::error::Error for UnsupportedLocaleMethodError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_from_str() {
        assert_eq!(Locale::from_str("en_US").unwrap(), Locale::EnUS);
        assert_eq!(Locale::from_str("de_DE").unwrap(), Locale::DeDE);
        assert_eq!(Locale::from_str("fr_FR").unwrap(), Locale::FrFR);
        assert_eq!(Locale::from_str("es_ES").unwrap(), Locale::EsES);
        assert_eq!(Locale::from_str("it_IT").unwrap(), Locale::ItIT);
        assert_eq!(Locale::from_str("ja_JP").unwrap(), Locale::JaJP);
        assert_eq!(Locale::from_str("en_GB").unwrap(), Locale::EnGB);
    }

    #[test]
    fn test_locale_from_str_invalid() {
        let result = Locale::from_str("xx_XX");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.requested, "xx_XX");
        assert!(err.to_string().contains("unsupported locale"));
    }

    #[test]
    fn test_locale_as_str() {
        assert_eq!(Locale::EnUS.as_str(), "en_US");
        assert_eq!(Locale::DeDE.as_str(), "de_DE");
        assert_eq!(Locale::FrFR.as_str(), "fr_FR");
        assert_eq!(Locale::EsES.as_str(), "es_ES");
        assert_eq!(Locale::ItIT.as_str(), "it_IT");
        assert_eq!(Locale::JaJP.as_str(), "ja_JP");
        assert_eq!(Locale::EnGB.as_str(), "en_GB");
    }

    #[test]
    fn test_locale_display() {
        assert_eq!(format!("{}", Locale::EnUS), "en_US");
        assert_eq!(format!("{}", Locale::DeDE), "de_DE");
    }

    #[test]
    fn test_locale_default() {
        assert_eq!(Locale::default(), Locale::EnUS);
    }

    #[test]
    fn test_family_name_first() {
        assert!(!Locale::EnUS.family_name_first());
        assert!(!Locale::DeDE.family_name_first());
        assert!(Locale::JaJP.family_name_first());
    }

    #[test]
    fn test_number_before_street() {
        assert!(Locale::EnUS.number_before_street());
        assert!(Locale::EnGB.number_before_street());
        assert!(!Locale::DeDE.number_before_street());
        assert!(!Locale::FrFR.number_before_street());
        assert!(!Locale::JaJP.number_before_street());
    }

    #[test]
    fn test_all_locales() {
        assert_eq!(Locale::ALL.len(), 7);
        assert!(Locale::ALL.contains(&Locale::EnUS));
        assert!(Locale::ALL.contains(&Locale::JaJP));
    }

    #[test]
    fn test_unsupported_locale_method_error() {
        let err = UnsupportedLocaleMethodError {
            locale: Locale::JaJP,
            method: "state",
        };
        assert!(err.to_string().contains("state"));
        assert!(err.to_string().contains("ja_JP"));
    }
}
