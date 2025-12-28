//! Macros for reducing code duplication in locale data modules.

/// Macro to define company-related data constants.
///
/// This macro generates the standard company data constants used by all locales:
/// - `COMPANY_PREFIXES`
/// - `COMPANY_SUFFIXES`
/// - `JOB_TITLES`
/// - `CATCH_PHRASE_ADJECTIVES`
/// - `CATCH_PHRASE_NOUNS`
///
/// # Example
///
/// ```ignore
/// define_company_data! {
///     prefixes: ["Alpha", "Beta", "Global"],
///     suffixes: ["Inc", "LLC", "Corp"],
///     job_titles: ["Engineer", "Manager"],
///     adjectives: ["Innovative", "Dynamic"],
///     nouns: ["solution", "strategy"],
/// }
/// ```
#[macro_export]
macro_rules! define_company_data {
    (
        prefixes: [$($prefix:literal),* $(,)?],
        suffixes: [$($suffix:literal),* $(,)?],
        job_titles: [$($job:literal),* $(,)?],
        adjectives: [$($adj:literal),* $(,)?],
        nouns: [$($noun:literal),* $(,)?] $(,)?
    ) => {
        /// Company name prefixes.
        pub const COMPANY_PREFIXES: &[&str] = &[$($prefix),*];

        /// Company name suffixes.
        pub const COMPANY_SUFFIXES: &[&str] = &[$($suffix),*];

        /// Job titles.
        pub const JOB_TITLES: &[&str] = &[$($job),*];

        /// Catch phrase adjectives.
        pub const CATCH_PHRASE_ADJECTIVES: &[&str] = &[$($adj),*];

        /// Catch phrase nouns.
        pub const CATCH_PHRASE_NOUNS: &[&str] = &[$($noun),*];
    };
}

/// Macro to implement the LocaleData trait for a locale.
///
/// This macro generates the LocaleData trait implementation with all required methods.
/// It handles both standard locales and those with romanized name variants.
///
/// # Parameters
///
/// - `$struct_name`: The struct type to implement LocaleData for
/// - `first_names`: Expression returning first names
/// - `last_names`: Expression returning last names
/// - `cities`: Expression returning cities
/// - `regions`: Expression returning regions
/// - `region_abbrs`: Expression returning region abbreviations
/// - `street_names`: Expression returning street names
/// - `street_suffixes`: Expression returning street suffixes
/// - `countries`: Expression returning countries
/// - `postal_format`: Expression returning postal code format
/// - `address_format`: Expression returning address format
/// - `phone_format`: Expression returning phone format
/// - `company_prefixes`: Expression returning company prefixes
/// - `company_suffixes`: Expression returning company suffixes
/// - `job_titles`: Expression returning job titles
/// - `catch_phrase_adjectives`: Expression returning catch phrase adjectives
/// - `catch_phrase_nouns`: Expression returning catch phrase nouns
/// - `text_words`: Expression returning lorem ipsum words
/// - `tlds`: Expression returning TLDs
/// - `free_email_domains`: Expression returning free email domains
/// - `safe_email_domains`: Expression returning safe email domains
/// - `color_names`: Expression returning color names
/// - `romanized_first_names`: (optional) Expression for romanized first names
/// - `romanized_last_names`: (optional) Expression for romanized last names
#[macro_export]
macro_rules! impl_locale_data {
    (
        $struct_name:ty,
        first_names: $first_names:expr,
        last_names: $last_names:expr,
        cities: $cities:expr,
        regions: $regions:expr,
        region_abbrs: $region_abbrs:expr,
        street_names: $street_names:expr,
        street_suffixes: $street_suffixes:expr,
        countries: $countries:expr,
        postal_format: $postal_format:expr,
        address_format: $address_format:expr,
        phone_format: $phone_format:expr,
        company_prefixes: $company_prefixes:expr,
        company_suffixes: $company_suffixes:expr,
        job_titles: $job_titles:expr,
        catch_phrase_adjectives: $catch_phrase_adjectives:expr,
        catch_phrase_nouns: $catch_phrase_nouns:expr,
        text_words: $text_words:expr,
        tlds: $tlds:expr,
        free_email_domains: $free_email_domains:expr,
        safe_email_domains: $safe_email_domains:expr,
        color_names: $color_names:expr $(,)?
    ) => {
        impl $crate::data::traits::LocaleData for $struct_name {
            fn first_names(&self) -> Option<&'static [&'static str]> {
                Some($first_names)
            }

            fn last_names(&self) -> Option<&'static [&'static str]> {
                Some($last_names)
            }

            fn cities(&self) -> Option<&'static [&'static str]> {
                Some($cities)
            }

            fn regions(&self) -> Option<&'static [&'static str]> {
                Some($regions)
            }

            fn region_abbrs(&self) -> Option<&'static [&'static str]> {
                Some($region_abbrs)
            }

            fn street_names(&self) -> Option<&'static [&'static str]> {
                Some($street_names)
            }

            fn street_suffixes(&self) -> Option<&'static [&'static str]> {
                Some($street_suffixes)
            }

            fn countries(&self) -> Option<&'static [&'static str]> {
                Some($countries)
            }

            fn postal_code_format(&self) -> Option<$crate::data::formats::PostalCodeFormat> {
                Some($postal_format)
            }

            fn address_format(&self) -> Option<$crate::data::formats::AddressFormat> {
                Some($address_format)
            }

            fn phone_format(&self) -> Option<$crate::data::formats::PhoneFormat> {
                Some($phone_format)
            }

            fn company_prefixes(&self) -> Option<&'static [&'static str]> {
                Some($company_prefixes)
            }

            fn company_suffixes(&self) -> Option<&'static [&'static str]> {
                Some($company_suffixes)
            }

            fn job_titles(&self) -> Option<&'static [&'static str]> {
                Some($job_titles)
            }

            fn catch_phrase_adjectives(&self) -> Option<&'static [&'static str]> {
                Some($catch_phrase_adjectives)
            }

            fn catch_phrase_nouns(&self) -> Option<&'static [&'static str]> {
                Some($catch_phrase_nouns)
            }

            fn text_words(&self) -> Option<&'static [&'static str]> {
                Some($text_words)
            }

            fn tlds(&self) -> Option<&'static [&'static str]> {
                Some($tlds)
            }

            fn free_email_domains(&self) -> Option<&'static [&'static str]> {
                Some($free_email_domains)
            }

            fn safe_email_domains(&self) -> Option<&'static [&'static str]> {
                Some($safe_email_domains)
            }

            fn color_names(&self) -> Option<&'static [&'static str]> {
                Some($color_names)
            }
        }
    };

    // Variant with romanized first names only
    (
        $struct_name:ty,
        first_names: $first_names:expr,
        last_names: $last_names:expr,
        cities: $cities:expr,
        regions: $regions:expr,
        region_abbrs: $region_abbrs:expr,
        street_names: $street_names:expr,
        street_suffixes: $street_suffixes:expr,
        countries: $countries:expr,
        postal_format: $postal_format:expr,
        address_format: $address_format:expr,
        phone_format: $phone_format:expr,
        company_prefixes: $company_prefixes:expr,
        company_suffixes: $company_suffixes:expr,
        job_titles: $job_titles:expr,
        catch_phrase_adjectives: $catch_phrase_adjectives:expr,
        catch_phrase_nouns: $catch_phrase_nouns:expr,
        text_words: $text_words:expr,
        tlds: $tlds:expr,
        free_email_domains: $free_email_domains:expr,
        safe_email_domains: $safe_email_domains:expr,
        color_names: $color_names:expr,
        romanized_first_names: $romanized_first_names:expr $(,)?
    ) => {
        impl $crate::data::traits::LocaleData for $struct_name {
            fn first_names(&self) -> Option<&'static [&'static str]> {
                Some($first_names)
            }

            fn last_names(&self) -> Option<&'static [&'static str]> {
                Some($last_names)
            }

            fn cities(&self) -> Option<&'static [&'static str]> {
                Some($cities)
            }

            fn regions(&self) -> Option<&'static [&'static str]> {
                Some($regions)
            }

            fn region_abbrs(&self) -> Option<&'static [&'static str]> {
                Some($region_abbrs)
            }

            fn street_names(&self) -> Option<&'static [&'static str]> {
                Some($street_names)
            }

            fn street_suffixes(&self) -> Option<&'static [&'static str]> {
                Some($street_suffixes)
            }

            fn countries(&self) -> Option<&'static [&'static str]> {
                Some($countries)
            }

            fn postal_code_format(&self) -> Option<$crate::data::formats::PostalCodeFormat> {
                Some($postal_format)
            }

            fn address_format(&self) -> Option<$crate::data::formats::AddressFormat> {
                Some($address_format)
            }

            fn phone_format(&self) -> Option<$crate::data::formats::PhoneFormat> {
                Some($phone_format)
            }

            fn company_prefixes(&self) -> Option<&'static [&'static str]> {
                Some($company_prefixes)
            }

            fn company_suffixes(&self) -> Option<&'static [&'static str]> {
                Some($company_suffixes)
            }

            fn job_titles(&self) -> Option<&'static [&'static str]> {
                Some($job_titles)
            }

            fn catch_phrase_adjectives(&self) -> Option<&'static [&'static str]> {
                Some($catch_phrase_adjectives)
            }

            fn catch_phrase_nouns(&self) -> Option<&'static [&'static str]> {
                Some($catch_phrase_nouns)
            }

            fn text_words(&self) -> Option<&'static [&'static str]> {
                Some($text_words)
            }

            fn tlds(&self) -> Option<&'static [&'static str]> {
                Some($tlds)
            }

            fn free_email_domains(&self) -> Option<&'static [&'static str]> {
                Some($free_email_domains)
            }

            fn safe_email_domains(&self) -> Option<&'static [&'static str]> {
                Some($safe_email_domains)
            }

            fn color_names(&self) -> Option<&'static [&'static str]> {
                Some($color_names)
            }

            fn romanized_first_names(&self) -> Option<&'static [&'static str]> {
                Some($romanized_first_names)
            }
        }
    };

    // Variant with both romanized first and last names
    (
        $struct_name:ty,
        first_names: $first_names:expr,
        last_names: $last_names:expr,
        cities: $cities:expr,
        regions: $regions:expr,
        region_abbrs: $region_abbrs:expr,
        street_names: $street_names:expr,
        street_suffixes: $street_suffixes:expr,
        countries: $countries:expr,
        postal_format: $postal_format:expr,
        address_format: $address_format:expr,
        phone_format: $phone_format:expr,
        company_prefixes: $company_prefixes:expr,
        company_suffixes: $company_suffixes:expr,
        job_titles: $job_titles:expr,
        catch_phrase_adjectives: $catch_phrase_adjectives:expr,
        catch_phrase_nouns: $catch_phrase_nouns:expr,
        text_words: $text_words:expr,
        tlds: $tlds:expr,
        free_email_domains: $free_email_domains:expr,
        safe_email_domains: $safe_email_domains:expr,
        color_names: $color_names:expr,
        romanized_first_names: $romanized_first_names:expr,
        romanized_last_names: $romanized_last_names:expr $(,)?
    ) => {
        impl $crate::data::traits::LocaleData for $struct_name {
            fn first_names(&self) -> Option<&'static [&'static str]> {
                Some($first_names)
            }

            fn last_names(&self) -> Option<&'static [&'static str]> {
                Some($last_names)
            }

            fn cities(&self) -> Option<&'static [&'static str]> {
                Some($cities)
            }

            fn regions(&self) -> Option<&'static [&'static str]> {
                Some($regions)
            }

            fn region_abbrs(&self) -> Option<&'static [&'static str]> {
                Some($region_abbrs)
            }

            fn street_names(&self) -> Option<&'static [&'static str]> {
                Some($street_names)
            }

            fn street_suffixes(&self) -> Option<&'static [&'static str]> {
                Some($street_suffixes)
            }

            fn countries(&self) -> Option<&'static [&'static str]> {
                Some($countries)
            }

            fn postal_code_format(&self) -> Option<$crate::data::formats::PostalCodeFormat> {
                Some($postal_format)
            }

            fn address_format(&self) -> Option<$crate::data::formats::AddressFormat> {
                Some($address_format)
            }

            fn phone_format(&self) -> Option<$crate::data::formats::PhoneFormat> {
                Some($phone_format)
            }

            fn company_prefixes(&self) -> Option<&'static [&'static str]> {
                Some($company_prefixes)
            }

            fn company_suffixes(&self) -> Option<&'static [&'static str]> {
                Some($company_suffixes)
            }

            fn job_titles(&self) -> Option<&'static [&'static str]> {
                Some($job_titles)
            }

            fn catch_phrase_adjectives(&self) -> Option<&'static [&'static str]> {
                Some($catch_phrase_adjectives)
            }

            fn catch_phrase_nouns(&self) -> Option<&'static [&'static str]> {
                Some($catch_phrase_nouns)
            }

            fn text_words(&self) -> Option<&'static [&'static str]> {
                Some($text_words)
            }

            fn tlds(&self) -> Option<&'static [&'static str]> {
                Some($tlds)
            }

            fn free_email_domains(&self) -> Option<&'static [&'static str]> {
                Some($free_email_domains)
            }

            fn safe_email_domains(&self) -> Option<&'static [&'static str]> {
                Some($safe_email_domains)
            }

            fn color_names(&self) -> Option<&'static [&'static str]> {
                Some($color_names)
            }

            fn romanized_first_names(&self) -> Option<&'static [&'static str]> {
                Some($romanized_first_names)
            }

            fn romanized_last_names(&self) -> Option<&'static [&'static str]> {
                Some($romanized_last_names)
            }
        }
    };
}

// Re-export macros for use in submodules
pub use define_company_data;
pub use impl_locale_data;
