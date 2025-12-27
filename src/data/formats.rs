//! Format specifications for locale-specific data generation.
//!
//! These structs define patterns and formats used for generating
//! phone numbers, postal codes, and addresses in different locales.

/// Phone number format specification.
///
/// Defines patterns for generating locale-specific phone numbers.
/// Patterns use `#` as a placeholder for digits.
///
/// # Examples
///
/// - US: `(###) ###-####`
/// - Germany: `+49 ### #######`
/// - Japan: `+81 ##-####-####`
#[derive(Debug, Clone, Copy)]
pub struct PhoneFormat {
    /// Patterns with `#` for digits.
    /// Multiple patterns allow for variation (e.g., with/without area code).
    pub patterns: &'static [&'static str],
    /// Country code (e.g., "+1" for US, "+49" for Germany).
    pub country_code: &'static str,
}

/// Postal code format specification.
///
/// Defines patterns for generating locale-specific postal codes.
/// - `#` represents a digit (0-9)
/// - `A` or `@` represents a letter (A-Z)
/// - Other characters are included literally
///
/// # Examples
///
/// - US: `#####` or `#####-####`
/// - Germany: `#####`
/// - UK: `AA## #AA`
/// - Japan: `###-####`
#[derive(Debug, Clone, Copy)]
pub struct PostalCodeFormat {
    /// Patterns with `#` for digits, `A` for letters.
    pub patterns: &'static [&'static str],
}

/// Address format specification.
///
/// Defines how address components are assembled for each locale.
#[derive(Debug, Clone, Copy)]
pub struct AddressFormat {
    /// Format template with placeholders:
    /// - `{street}` - Street address (includes number)
    /// - `{city}` - City name
    /// - `{region}` - Region/state/prefecture name
    /// - `{region_abbr}` - Region abbreviation
    /// - `{postal}` - Postal/ZIP code
    /// - `{country}` - Country name
    ///
    /// # Examples
    ///
    /// - US: `{street}, {city}, {region_abbr} {postal}`
    /// - Germany: `{street}\n{postal} {city}`
    /// - Japan: `{postal} {region}{city}{street}`
    pub template: &'static str,

    /// Whether the street number comes before the street name.
    /// - `true` for US/UK: "123 Main Street"
    /// - `false` for Germany/France: "Hauptstraße 123"
    pub number_before_street: bool,

    /// Separator between street name and type.
    /// - `""` for German: "Haupt" + "straße" = "Hauptstraße"
    /// - `" "` for most others: "Main" + " " + "Street" = "Main Street"
    pub street_name_separator: &'static str,

    /// Whether the street type comes before the name (prefix) or after (suffix).
    /// - `false` (suffix): "Main Street", "Hauptstraße" - type after name
    /// - `true` (prefix): "Calle Mayor", "Via Roma", "rue de la République" - type before name
    pub street_type_prefix: bool,
}

impl PhoneFormat {
    /// Create a new phone format.
    pub const fn new(patterns: &'static [&'static str], country_code: &'static str) -> Self {
        Self {
            patterns,
            country_code,
        }
    }
}

impl PostalCodeFormat {
    /// Create a new postal code format.
    pub const fn new(patterns: &'static [&'static str]) -> Self {
        Self { patterns }
    }
}

impl AddressFormat {
    /// Create a new address format with default settings.
    /// Defaults: space separator, street type as suffix.
    pub const fn new(template: &'static str, number_before_street: bool) -> Self {
        Self {
            template,
            number_before_street,
            street_name_separator: " ",
            street_type_prefix: false,
        }
    }

    /// Create a new address format with custom street name separator.
    /// Defaults: street type as suffix.
    pub const fn with_separator(
        template: &'static str,
        number_before_street: bool,
        street_name_separator: &'static str,
    ) -> Self {
        Self {
            template,
            number_before_street,
            street_name_separator,
            street_type_prefix: false,
        }
    }

    /// Create a new address format with street type as prefix.
    /// Used for locales like French, Spanish, Italian where type comes first.
    /// Example: "Calle Mayor", "Via Roma", "rue de la République"
    pub const fn with_prefix_type(template: &'static str, number_before_street: bool) -> Self {
        Self {
            template,
            number_before_street,
            street_name_separator: " ",
            street_type_prefix: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_format_creation() {
        let format = PhoneFormat::new(&["(###) ###-####", "###-###-####"], "+1");
        assert_eq!(format.patterns.len(), 2);
        assert_eq!(format.country_code, "+1");
    }

    #[test]
    fn test_postal_code_format_creation() {
        let format = PostalCodeFormat::new(&["#####", "#####-####"]);
        assert_eq!(format.patterns.len(), 2);
    }

    #[test]
    fn test_address_format_creation() {
        let format = AddressFormat::new("{street}, {city}, {region_abbr} {postal}", true);
        assert!(format.template.contains("{street}"));
        assert!(format.number_before_street);
        assert_eq!(format.street_name_separator, " ");
    }

    #[test]
    fn test_address_format_with_separator() {
        let format = AddressFormat::with_separator("{street}\n{postal} {city}", false, "");
        assert!(!format.number_before_street);
        assert_eq!(format.street_name_separator, "");
    }
}
