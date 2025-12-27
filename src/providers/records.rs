//! Structured data generation provider.
//!
//! Generates records based on a schema specification.
//!
//! This module provides the `records()` and `records_tuples()` functions
//! for generating structured data based on a schema DSL.

use crate::locale::Locale;
use crate::providers::custom::CustomProvider;
use crate::providers::{
    address, colors, company, datetime, finance, identifiers, internet, names, network, numbers,
    phone, text,
};
use crate::rng::ForgeryRng;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;

/// Error type for schema-related errors.
#[derive(Debug, Clone)]
pub struct SchemaError {
    /// The error message.
    pub message: String,
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Schema error: {}", self.message)
    }
}

impl std::error::Error for SchemaError {}

/// A field specification in the schema.
#[derive(Debug, Clone)]
pub enum FieldSpec {
    /// Simple type (e.g., "name", "email", "uuid")
    Simple(String),
    /// Integer range: ("int", min, max)
    IntRange {
        /// Minimum value (inclusive).
        min: i64,
        /// Maximum value (inclusive).
        max: i64,
    },
    /// Float range: ("float", min, max)
    FloatRange {
        /// Minimum value (inclusive).
        min: f64,
        /// Maximum value (inclusive).
        max: f64,
    },
    /// Text with character limits: ("text", min_chars, max_chars)
    Text {
        /// Minimum number of characters.
        min_chars: usize,
        /// Maximum number of characters.
        max_chars: usize,
    },
    /// Date range: ("date", start, end)
    DateRange {
        /// Start date in YYYY-MM-DD format.
        start: String,
        /// End date in YYYY-MM-DD format.
        end: String,
    },
    /// Choice from options: ("choice", ["a", "b", "c"])
    Choice(Vec<String>),
    /// Name field type.
    Name,
    /// First name field type.
    FirstName,
    /// Last name field type.
    LastName,
    /// Email field type.
    Email,
    /// Safe email field type.
    SafeEmail,
    /// Free email field type.
    FreeEmail,
    /// Phone field type.
    Phone,
    /// UUID field type.
    Uuid,
    /// Integer with default range (0-100).
    Int,
    /// Float with default range (0.0-1.0).
    Float,
    /// Date with default range.
    Date,
    /// DateTime field type.
    DateTime,
    /// Street address field type.
    StreetAddress,
    /// City field type.
    City,
    /// State field type.
    State,
    /// Country field type.
    Country,
    /// Zip code field type.
    ZipCode,
    /// Full address field type.
    Address,
    /// Company name field type.
    Company,
    /// Job title field type.
    Job,
    /// Catch phrase field type.
    CatchPhrase,
    /// URL field type.
    Url,
    /// Domain name field type.
    DomainName,
    /// IPv4 address field type.
    Ipv4,
    /// IPv6 address field type.
    Ipv6,
    /// MAC address field type.
    MacAddress,
    /// Credit card number field type.
    CreditCard,
    /// IBAN field type.
    Iban,
    /// Sentence field type.
    Sentence,
    /// Paragraph field type.
    Paragraph,
    /// Color name field type.
    Color,
    /// Hex color field type.
    HexColor,
    /// RGB color field type.
    RgbColor,
    /// MD5 hash field type.
    Md5,
    /// SHA256 hash field type.
    Sha256,
    /// Custom provider by name.
    Custom(String),
}

/// A generated value that can be various types.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// A string value.
    String(String),
    /// An integer value.
    Int(i64),
    /// A floating-point value.
    Float(f64),
    /// A tuple of three u8 values (for RGB colors).
    Tuple3U8(u8, u8, u8),
}

impl Value {
    /// Get the value as a string.
    pub fn as_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Tuple3U8(r, g, b) => format!("({}, {}, {})", r, g, b),
        }
    }
}

/// Parse a simple type name into a FieldSpec.
pub fn parse_simple_type(type_name: &str) -> Result<FieldSpec, SchemaError> {
    match type_name {
        "name" => Ok(FieldSpec::Name),
        "first_name" => Ok(FieldSpec::FirstName),
        "last_name" => Ok(FieldSpec::LastName),
        "email" => Ok(FieldSpec::Email),
        "safe_email" => Ok(FieldSpec::SafeEmail),
        "free_email" => Ok(FieldSpec::FreeEmail),
        "uuid" => Ok(FieldSpec::Uuid),
        "int" => Ok(FieldSpec::Int),
        "float" => Ok(FieldSpec::Float),
        "phone" => Ok(FieldSpec::Phone),
        "address" => Ok(FieldSpec::Address),
        "street_address" => Ok(FieldSpec::StreetAddress),
        "city" => Ok(FieldSpec::City),
        "state" => Ok(FieldSpec::State),
        "country" => Ok(FieldSpec::Country),
        "zip_code" => Ok(FieldSpec::ZipCode),
        "company" => Ok(FieldSpec::Company),
        "job" => Ok(FieldSpec::Job),
        "catch_phrase" => Ok(FieldSpec::CatchPhrase),
        "url" => Ok(FieldSpec::Url),
        "domain_name" => Ok(FieldSpec::DomainName),
        "ipv4" => Ok(FieldSpec::Ipv4),
        "ipv6" => Ok(FieldSpec::Ipv6),
        "mac_address" => Ok(FieldSpec::MacAddress),
        "color" => Ok(FieldSpec::Color),
        "hex_color" => Ok(FieldSpec::HexColor),
        "rgb_color" => Ok(FieldSpec::RgbColor),
        "credit_card" => Ok(FieldSpec::CreditCard),
        "iban" => Ok(FieldSpec::Iban),
        "date" => Ok(FieldSpec::Date),
        "datetime" => Ok(FieldSpec::DateTime),
        "md5" => Ok(FieldSpec::Md5),
        "sha256" => Ok(FieldSpec::Sha256),
        "sentence" => Ok(FieldSpec::Sentence),
        "paragraph" => Ok(FieldSpec::Paragraph),
        "text" => Ok(FieldSpec::Simple("text".to_string())),
        _ => Err(SchemaError {
            message: format!("Unknown type: {}", type_name),
        }),
    }
}

/// Parse a simple type name into a FieldSpec, with custom provider awareness.
///
/// If the type name matches a built-in type, returns the corresponding FieldSpec.
/// If the type name is in the custom_provider_names set, returns FieldSpec::Custom.
/// Otherwise, returns an error.
pub fn parse_simple_type_with_custom(
    type_name: &str,
    custom_provider_names: &HashSet<String>,
) -> Result<FieldSpec, SchemaError> {
    // First try to parse as a built-in type
    match parse_simple_type(type_name) {
        Ok(spec) => Ok(spec),
        Err(_) => {
            // Check if it's a custom provider
            if custom_provider_names.contains(type_name) {
                Ok(FieldSpec::Custom(type_name.to_string()))
            } else {
                Err(SchemaError {
                    message: format!("Unknown type: {}", type_name),
                })
            }
        }
    }
}

/// Validate a field specification without generating a value.
///
/// This allows schema validation to happen upfront, even when n=0.
pub fn validate_spec(spec: &FieldSpec) -> Result<(), SchemaError> {
    match spec {
        FieldSpec::Simple(type_name) => {
            // Validate that the type name is known
            parse_simple_type(type_name)?;
            Ok(())
        }
        FieldSpec::IntRange { min, max } => {
            if min > max {
                return Err(SchemaError {
                    message: format!("Invalid int range: {} > {}", min, max),
                });
            }
            Ok(())
        }
        FieldSpec::FloatRange { min, max } => {
            if min > max {
                return Err(SchemaError {
                    message: format!("Invalid float range: {} > {}", min, max),
                });
            }
            Ok(())
        }
        FieldSpec::Text {
            min_chars,
            max_chars,
        } => {
            if min_chars > max_chars {
                return Err(SchemaError {
                    message: format!("Invalid text range: {} > {}", min_chars, max_chars),
                });
            }
            Ok(())
        }
        FieldSpec::DateRange { .. } => {
            // Date validation requires parsing, which is done during generation
            // We could add date format validation here if needed
            Ok(())
        }
        FieldSpec::Choice(options) => {
            if options.is_empty() {
                return Err(SchemaError {
                    message: "Choice options cannot be empty".to_string(),
                });
            }
            Ok(())
        }
        // Custom providers are validated by the Faker when generating
        // (we check the provider exists during generation)
        FieldSpec::Custom(_) => Ok(()),
        // All direct type variants are always valid
        _ => Ok(()),
    }
}

/// Validate an entire schema without generating any values.
///
/// This ensures schema validation happens even when n=0.
pub fn validate_schema(schema: &BTreeMap<String, FieldSpec>) -> Result<(), SchemaError> {
    for (field_name, spec) in schema {
        validate_spec(spec).map_err(|e| SchemaError {
            message: format!("Field '{}': {}", field_name, e.message),
        })?;
    }
    Ok(())
}

/// Validate an entire schema with custom provider verification.
///
/// This validates the schema structure and also verifies that any custom
/// provider references exist in the provided custom_providers map.
/// This ensures validation happens even when n=0.
pub fn validate_schema_with_custom(
    schema: &BTreeMap<String, FieldSpec>,
    custom_providers: &HashMap<String, CustomProvider>,
) -> Result<(), SchemaError> {
    for (field_name, spec) in schema {
        validate_spec(spec).map_err(|e| SchemaError {
            message: format!("Field '{}': {}", field_name, e.message),
        })?;

        // Additionally validate that custom providers exist
        if let FieldSpec::Custom(provider_name) = spec {
            if !custom_providers.contains_key(provider_name) {
                return Err(SchemaError {
                    message: format!(
                        "Field '{}': custom provider '{}' not found",
                        field_name, provider_name
                    ),
                });
            }
        }
    }
    Ok(())
}

/// Generate a value based on a field specification.
pub fn generate_value(
    rng: &mut ForgeryRng,
    locale: Locale,
    spec: &FieldSpec,
) -> Result<Value, SchemaError> {
    match spec {
        FieldSpec::Simple(type_name) => generate_simple_value(rng, locale, type_name),
        FieldSpec::IntRange { min, max } => {
            if min > max {
                return Err(SchemaError {
                    message: format!("Invalid int range: {} > {}", min, max),
                });
            }
            let val = numbers::generate_integer(rng, *min, *max).map_err(|e| SchemaError {
                message: e.to_string(),
            })?;
            Ok(Value::Int(val))
        }
        FieldSpec::FloatRange { min, max } => {
            if min > max {
                return Err(SchemaError {
                    message: format!("Invalid float range: {} > {}", min, max),
                });
            }
            let val = numbers::generate_float(rng, *min, *max).map_err(|e| SchemaError {
                message: e.to_string(),
            })?;
            Ok(Value::Float(val))
        }
        FieldSpec::Text {
            min_chars,
            max_chars,
        } => {
            let val = text::generate_text(rng, locale, *min_chars, *max_chars);
            Ok(Value::String(val))
        }
        FieldSpec::DateRange { start, end } => {
            let val = datetime::generate_date(rng, start, end).map_err(|e| SchemaError {
                message: e.to_string(),
            })?;
            Ok(Value::String(val))
        }
        FieldSpec::Choice(options) => {
            if options.is_empty() {
                return Err(SchemaError {
                    message: "Choice options cannot be empty".to_string(),
                });
            }
            let val = rng.choose(options).clone();
            Ok(Value::String(val))
        }
        // Direct type variants
        FieldSpec::Name => Ok(Value::String(names::generate_name(rng, locale))),
        FieldSpec::FirstName => Ok(Value::String(names::generate_first_name(rng, locale))),
        FieldSpec::LastName => Ok(Value::String(names::generate_last_name(rng, locale))),
        FieldSpec::Email => Ok(Value::String(internet::generate_email(rng, locale))),
        FieldSpec::SafeEmail => Ok(Value::String(internet::generate_safe_email(rng, locale))),
        FieldSpec::FreeEmail => Ok(Value::String(internet::generate_free_email(rng, locale))),
        FieldSpec::Phone => Ok(Value::String(phone::generate_phone_number(rng, locale))),
        FieldSpec::Uuid => Ok(Value::String(identifiers::generate_uuid(rng))),
        FieldSpec::Int => Ok(Value::Int(numbers::generate_integer(rng, 0, 100).map_err(
            |e| SchemaError {
                message: e.to_string(),
            },
        )?)),
        FieldSpec::Float => Ok(Value::Float(
            numbers::generate_float(rng, 0.0, 1.0).map_err(|e| SchemaError {
                message: e.to_string(),
            })?,
        )),
        FieldSpec::Date => {
            let val = datetime::generate_date(rng, "2000-01-01", "2030-12-31").map_err(|e| {
                SchemaError {
                    message: e.to_string(),
                }
            })?;
            Ok(Value::String(val))
        }
        FieldSpec::DateTime => {
            let val =
                datetime::generate_datetime(rng, "2000-01-01", "2030-12-31").map_err(|e| {
                    SchemaError {
                        message: e.to_string(),
                    }
                })?;
            Ok(Value::String(val))
        }
        FieldSpec::StreetAddress => {
            Ok(Value::String(address::generate_street_address(rng, locale)))
        }
        FieldSpec::City => Ok(Value::String(address::generate_city(rng, locale))),
        FieldSpec::State => Ok(Value::String(address::generate_state(rng, locale))),
        FieldSpec::Country => Ok(Value::String(address::generate_country(rng))),
        FieldSpec::ZipCode => Ok(Value::String(address::generate_zip_code(rng, locale))),
        FieldSpec::Address => Ok(Value::String(address::generate_address(rng, locale))),
        FieldSpec::Company => Ok(Value::String(company::generate_company(rng, locale))),
        FieldSpec::Job => Ok(Value::String(company::generate_job(rng, locale))),
        FieldSpec::CatchPhrase => Ok(Value::String(company::generate_catch_phrase(rng, locale))),
        FieldSpec::Url => Ok(Value::String(network::generate_url(rng))),
        FieldSpec::DomainName => Ok(Value::String(network::generate_domain_name(rng))),
        FieldSpec::Ipv4 => Ok(Value::String(network::generate_ipv4(rng))),
        FieldSpec::Ipv6 => Ok(Value::String(network::generate_ipv6(rng))),
        FieldSpec::MacAddress => Ok(Value::String(network::generate_mac_address(rng))),
        FieldSpec::CreditCard => Ok(Value::String(finance::generate_credit_card(rng))),
        FieldSpec::Iban => Ok(Value::String(finance::generate_iban(rng))),
        FieldSpec::Sentence => Ok(Value::String(text::generate_sentence(rng, locale, 10))),
        FieldSpec::Paragraph => Ok(Value::String(text::generate_paragraph(rng, locale, 5))),
        FieldSpec::Color => Ok(Value::String(colors::generate_color(rng, locale))),
        FieldSpec::HexColor => Ok(Value::String(colors::generate_hex_color(rng))),
        FieldSpec::RgbColor => {
            let (r, g, b) = colors::generate_rgb_color(rng);
            Ok(Value::Tuple3U8(r, g, b))
        }
        FieldSpec::Md5 => Ok(Value::String(identifiers::generate_md5(rng))),
        FieldSpec::Sha256 => Ok(Value::String(identifiers::generate_sha256(rng))),
        FieldSpec::Custom(name) => {
            // This should not be reached when calling generate_value directly
            // Use generate_value_with_custom for custom provider support
            Err(SchemaError {
                message: format!(
                    "Custom provider '{}' requires custom_providers map - use generate_value_with_custom",
                    name
                ),
            })
        }
    }
}

/// Generate a value based on a field specification, with custom provider support.
///
/// This variant of generate_value() can handle FieldSpec::Custom variants
/// by looking up the provider in the provided custom_providers map.
pub fn generate_value_with_custom(
    rng: &mut ForgeryRng,
    locale: Locale,
    spec: &FieldSpec,
    custom_providers: &HashMap<String, CustomProvider>,
) -> Result<Value, SchemaError> {
    if let FieldSpec::Custom(name) = spec {
        let provider = custom_providers.get(name).ok_or_else(|| SchemaError {
            message: format!("Custom provider '{}' not found", name),
        })?;
        Ok(Value::String(provider.generate(rng)))
    } else {
        generate_value(rng, locale, spec)
    }
}

/// Generate a value for a simple type.
fn generate_simple_value(
    rng: &mut ForgeryRng,
    locale: Locale,
    type_name: &str,
) -> Result<Value, SchemaError> {
    match type_name {
        // Names
        "name" => Ok(Value::String(names::generate_name(rng, locale))),
        "first_name" => Ok(Value::String(names::generate_first_name(rng, locale))),
        "last_name" => Ok(Value::String(names::generate_last_name(rng, locale))),

        // Internet
        "email" => Ok(Value::String(internet::generate_email(rng, locale))),
        "safe_email" => Ok(Value::String(internet::generate_safe_email(rng, locale))),
        "free_email" => Ok(Value::String(internet::generate_free_email(rng, locale))),

        // Identifiers
        "uuid" => Ok(Value::String(identifiers::generate_uuid(rng))),
        "md5" => Ok(Value::String(identifiers::generate_md5(rng))),
        "sha256" => Ok(Value::String(identifiers::generate_sha256(rng))),

        // Numbers (defaults)
        "int" => Ok(Value::Int(numbers::generate_integer(rng, 0, 100).map_err(
            |e| SchemaError {
                message: e.to_string(),
            },
        )?)),
        "float" => Ok(Value::Float(
            numbers::generate_float(rng, 0.0, 1.0).map_err(|e| SchemaError {
                message: e.to_string(),
            })?,
        )),

        // Phone
        "phone" => Ok(Value::String(phone::generate_phone_number(rng, locale))),

        // Address
        "address" => Ok(Value::String(address::generate_address(rng, locale))),
        "street_address" => Ok(Value::String(address::generate_street_address(rng, locale))),
        "city" => Ok(Value::String(address::generate_city(rng, locale))),
        "state" => Ok(Value::String(address::generate_state(rng, locale))),
        "country" => Ok(Value::String(address::generate_country(rng))),
        "zip_code" => Ok(Value::String(address::generate_zip_code(rng, locale))),

        // Company
        "company" => Ok(Value::String(company::generate_company(rng, locale))),
        "job" => Ok(Value::String(company::generate_job(rng, locale))),
        "catch_phrase" => Ok(Value::String(company::generate_catch_phrase(rng, locale))),

        // Network
        "url" => Ok(Value::String(network::generate_url(rng))),
        "domain_name" => Ok(Value::String(network::generate_domain_name(rng))),
        "ipv4" => Ok(Value::String(network::generate_ipv4(rng))),
        "ipv6" => Ok(Value::String(network::generate_ipv6(rng))),
        "mac_address" => Ok(Value::String(network::generate_mac_address(rng))),

        // Colors
        "color" => Ok(Value::String(colors::generate_color(rng, locale))),
        "hex_color" => Ok(Value::String(colors::generate_hex_color(rng))),
        "rgb_color" => {
            let (r, g, b) = colors::generate_rgb_color(rng);
            Ok(Value::Tuple3U8(r, g, b))
        }

        // Finance
        "credit_card" => Ok(Value::String(finance::generate_credit_card(rng))),
        "iban" => Ok(Value::String(finance::generate_iban(rng))),

        // DateTime (defaults)
        "date" => {
            let val = datetime::generate_date(rng, "2000-01-01", "2030-12-31").map_err(|e| {
                SchemaError {
                    message: e.to_string(),
                }
            })?;
            Ok(Value::String(val))
        }
        "datetime" => {
            let val =
                datetime::generate_datetime(rng, "2000-01-01", "2030-12-31").map_err(|e| {
                    SchemaError {
                        message: e.to_string(),
                    }
                })?;
            Ok(Value::String(val))
        }

        // Text (defaults)
        "sentence" => Ok(Value::String(text::generate_sentence(rng, locale, 10))),
        "paragraph" => Ok(Value::String(text::generate_paragraph(rng, locale, 5))),
        "text" => Ok(Value::String(text::generate_text(rng, locale, 50, 200))),

        _ => Err(SchemaError {
            message: format!("Unknown type: {}", type_name),
        }),
    }
}

/// Generate records based on a schema.
///
/// Returns a vector of BTreeMaps, where each BTreeMap represents a record
/// with field names as keys and generated values.
///
/// # Determinism
///
/// BTreeMap is used instead of HashMap to ensure deterministic iteration order,
/// which guarantees that the same seed produces the same output across runs.
pub fn generate_records(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    schema: &BTreeMap<String, FieldSpec>,
) -> Result<Vec<BTreeMap<String, Value>>, SchemaError> {
    // Delegate to the custom-aware version with empty providers map
    generate_records_with_custom(rng, locale, n, schema, &HashMap::new())
}

/// Generate records as tuples based on a schema.
///
/// Returns a vector of vectors, where each inner vector contains values
/// in the same order as the provided field order.
pub fn generate_records_tuples(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    schema: &BTreeMap<String, FieldSpec>,
    field_order: &[String],
) -> Result<Vec<Vec<Value>>, SchemaError> {
    // Delegate to the custom-aware version with empty providers map
    generate_records_tuples_with_custom(rng, locale, n, schema, field_order, &HashMap::new())
}

/// Generate records based on a schema, with custom provider support.
///
/// This variant of generate_records() can handle FieldSpec::Custom variants
/// by looking up providers in the provided custom_providers map.
pub fn generate_records_with_custom(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    schema: &BTreeMap<String, FieldSpec>,
    custom_providers: &HashMap<String, CustomProvider>,
) -> Result<Vec<BTreeMap<String, Value>>, SchemaError> {
    // Validate schema upfront (even when n=0), including custom provider existence
    validate_schema_with_custom(schema, custom_providers)?;

    let mut records = Vec::with_capacity(n);

    for _ in 0..n {
        let mut record = BTreeMap::new();
        for (field_name, spec) in schema {
            let value = generate_value_with_custom(rng, locale, spec, custom_providers)?;
            record.insert(field_name.clone(), value);
        }
        records.push(record);
    }

    Ok(records)
}

/// Generate records as tuples based on a schema, with custom provider support.
///
/// This variant of generate_records_tuples() can handle FieldSpec::Custom variants.
pub fn generate_records_tuples_with_custom(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    schema: &BTreeMap<String, FieldSpec>,
    field_order: &[String],
    custom_providers: &HashMap<String, CustomProvider>,
) -> Result<Vec<Vec<Value>>, SchemaError> {
    // Validate schema upfront (even when n=0), including custom provider existence
    validate_schema_with_custom(schema, custom_providers)?;

    // Validate field_order: check for duplicates
    let mut seen = std::collections::HashSet::new();
    for field in field_order {
        if !seen.insert(field) {
            return Err(SchemaError {
                message: format!("Duplicate field in field_order: '{}'", field),
            });
        }
    }

    // Validate field_order: all fields must exist in schema
    for field in field_order {
        if !schema.contains_key(field) {
            return Err(SchemaError {
                message: format!("Field '{}' not in schema", field),
            });
        }
    }

    // Validate field_order: must cover all schema fields
    if field_order.len() != schema.len() {
        let missing: Vec<_> = schema.keys().filter(|k| !field_order.contains(k)).collect();
        return Err(SchemaError {
            message: format!(
                "field_order must cover all schema fields. Missing: {:?}",
                missing
            ),
        });
    }

    let mut records = Vec::with_capacity(n);

    for _ in 0..n {
        let mut record = Vec::with_capacity(field_order.len());
        for field_name in field_order {
            // Field existence is validated at the start of this function
            let spec = schema
                .get(field_name)
                .expect("field_name was validated to exist in schema");
            let value = generate_value_with_custom(rng, locale, spec, custom_providers)?;
            record.push(value);
        }
        records.push(record);
    }

    Ok(records)
}

// ============================================================================
// Arrow/Polars Integration
// ============================================================================

use arrow_array::{
    ArrayRef, Float64Array, Int64Array, RecordBatch, StringArray, StructArray, UInt8Array,
};
use arrow_buffer::NullBuffer;
use arrow_schema::{DataType, Field, Schema};
use std::sync::Arc;

/// Determine the Arrow DataType for a given FieldSpec.
///
/// Most field types map to Utf8 (strings), but integers and floats
/// have their own types, and RGB colors are stored as a Struct.
pub fn field_spec_to_arrow_type(spec: &FieldSpec) -> DataType {
    match spec {
        // Integer types
        FieldSpec::Int | FieldSpec::IntRange { .. } => DataType::Int64,

        // Float types
        FieldSpec::Float | FieldSpec::FloatRange { .. } => DataType::Float64,

        // RGB colors are stored as a struct with r, g, b uint8 fields
        FieldSpec::RgbColor => DataType::Struct(
            vec![
                Field::new("r", DataType::UInt8, false),
                Field::new("g", DataType::UInt8, false),
                Field::new("b", DataType::UInt8, false),
            ]
            .into(),
        ),

        // All other types produce strings
        _ => DataType::Utf8,
    }
}

/// Generate records as an Arrow RecordBatch.
///
/// This is the high-performance path for generating structured data
/// suitable for use with PyArrow, Polars, and other Arrow-compatible tools.
///
/// # Performance
///
/// This function generates data in columnar format, which is more efficient
/// for Arrow than row-by-row generation. Each column is built as a contiguous
/// array, avoiding Python object overhead.
pub fn generate_records_arrow(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    schema: &BTreeMap<String, FieldSpec>,
) -> Result<RecordBatch, SchemaError> {
    // Delegate to the custom-aware version with empty providers map
    generate_records_arrow_with_custom(rng, locale, n, schema, &HashMap::new())
}

/// Generate records as an Arrow RecordBatch, with custom provider support.
///
/// This variant of generate_records_arrow() can handle FieldSpec::Custom variants
/// by looking up providers in the provided custom_providers map.
pub fn generate_records_arrow_with_custom(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    schema: &BTreeMap<String, FieldSpec>,
    custom_providers: &HashMap<String, CustomProvider>,
) -> Result<RecordBatch, SchemaError> {
    // Validate schema upfront (even when n=0), including custom provider existence
    validate_schema_with_custom(schema, custom_providers)?;

    // Build Arrow schema and collect field specs
    let mut arrow_fields: Vec<Field> = Vec::with_capacity(schema.len());
    let mut field_specs: Vec<&FieldSpec> = Vec::with_capacity(schema.len());

    for (name, spec) in schema.iter() {
        let arrow_type = field_spec_to_arrow_type(spec);
        arrow_fields.push(Field::new(name, arrow_type, false));
        field_specs.push(spec);
    }

    let arrow_schema = Arc::new(Schema::new(arrow_fields));

    // Generate columns
    let mut columns: Vec<ArrayRef> = Vec::with_capacity(schema.len());

    for spec in field_specs.iter() {
        let column = generate_arrow_column(rng, locale, n, spec, custom_providers)?;
        columns.push(column);
    }

    // Build RecordBatch
    RecordBatch::try_new(arrow_schema, columns).map_err(|e| SchemaError {
        message: format!("Failed to create RecordBatch: {}", e),
    })
}

/// Generate an Arrow array for a single column based on the field spec.
fn generate_arrow_column(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    spec: &FieldSpec,
    custom_providers: &HashMap<String, CustomProvider>,
) -> Result<ArrayRef, SchemaError> {
    match spec {
        // Integer types -> Int64Array
        // Note: Ranges are validated in validate_spec() before generation, so these can't fail
        FieldSpec::Int => {
            let values: Vec<i64> = (0..n)
                .map(|_| {
                    numbers::generate_integer(rng, 0, 100)
                        .expect("default range 0-100 is always valid")
                })
                .collect();
            Ok(Arc::new(Int64Array::from(values)))
        }
        FieldSpec::IntRange { min, max } => {
            let values: Vec<i64> = (0..n)
                .map(|_| {
                    numbers::generate_integer(rng, *min, *max)
                        .expect("range validated in validate_spec")
                })
                .collect();
            Ok(Arc::new(Int64Array::from(values)))
        }

        // Float types -> Float64Array
        // Note: Ranges are validated in validate_spec() before generation, so these can't fail
        FieldSpec::Float => {
            let values: Vec<f64> = (0..n)
                .map(|_| {
                    numbers::generate_float(rng, 0.0, 1.0)
                        .expect("default range 0.0-1.0 is always valid")
                })
                .collect();
            Ok(Arc::new(Float64Array::from(values)))
        }
        FieldSpec::FloatRange { min, max } => {
            let values: Vec<f64> = (0..n)
                .map(|_| {
                    numbers::generate_float(rng, *min, *max)
                        .expect("range validated in validate_spec")
                })
                .collect();
            Ok(Arc::new(Float64Array::from(values)))
        }

        // RGB color -> Struct with r, g, b UInt8 fields
        FieldSpec::RgbColor => {
            let mut r_values: Vec<u8> = Vec::with_capacity(n);
            let mut g_values: Vec<u8> = Vec::with_capacity(n);
            let mut b_values: Vec<u8> = Vec::with_capacity(n);

            for _ in 0..n {
                let (r, g, b) = colors::generate_rgb_color(rng);
                r_values.push(r);
                g_values.push(g);
                b_values.push(b);
            }

            let r_array = Arc::new(UInt8Array::from(r_values)) as ArrayRef;
            let g_array = Arc::new(UInt8Array::from(g_values)) as ArrayRef;
            let b_array = Arc::new(UInt8Array::from(b_values)) as ArrayRef;

            let struct_fields: Vec<Field> = vec![
                Field::new("r", DataType::UInt8, false),
                Field::new("g", DataType::UInt8, false),
                Field::new("b", DataType::UInt8, false),
            ];

            let struct_array = StructArray::new(
                struct_fields.into(),
                vec![r_array, g_array, b_array],
                None::<NullBuffer>,
            );

            Ok(Arc::new(struct_array))
        }

        // All other types produce string arrays
        _ => {
            let values: Result<Vec<String>, SchemaError> = (0..n)
                .map(|_| {
                    generate_value_with_custom(rng, locale, spec, custom_providers)
                        .map(|v| v.as_string())
                })
                .collect();
            Ok(Arc::new(StringArray::from(values?)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::locale::Locale;

    fn create_test_schema() -> BTreeMap<String, FieldSpec> {
        let mut schema = BTreeMap::new();
        schema.insert("id".to_string(), FieldSpec::Simple("uuid".to_string()));
        schema.insert("name".to_string(), FieldSpec::Simple("name".to_string()));
        schema.insert("age".to_string(), FieldSpec::IntRange { min: 18, max: 65 });
        schema.insert(
            "salary".to_string(),
            FieldSpec::FloatRange {
                min: 30000.0,
                max: 150000.0,
            },
        );
        schema.insert(
            "status".to_string(),
            FieldSpec::Choice(vec!["active".to_string(), "inactive".to_string()]),
        );
        schema
    }

    #[test]
    fn test_generate_records_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let schema = create_test_schema();
        let records = generate_records(&mut rng, Locale::EnUS, 100, &schema).unwrap();

        assert_eq!(records.len(), 100);
    }

    #[test]
    fn test_generate_records_has_all_fields() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let schema = create_test_schema();
        let records = generate_records(&mut rng, Locale::EnUS, 10, &schema).unwrap();

        for record in &records {
            assert!(record.contains_key("id"));
            assert!(record.contains_key("name"));
            assert!(record.contains_key("age"));
            assert!(record.contains_key("salary"));
            assert!(record.contains_key("status"));
        }
    }

    #[test]
    fn test_generate_records_correct_types() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let schema = create_test_schema();
        let records = generate_records(&mut rng, Locale::EnUS, 10, &schema).unwrap();

        for record in &records {
            // UUID should be a string with dashes
            if let Value::String(id) = &record["id"] {
                assert!(id.contains('-'));
            } else {
                panic!("ID should be a string");
            }

            // Age should be an integer in range
            if let Value::Int(age) = &record["age"] {
                assert!(*age >= 18 && *age <= 65);
            } else {
                panic!("Age should be an integer");
            }

            // Salary should be a float in range
            if let Value::Float(salary) = &record["salary"] {
                assert!(*salary >= 30000.0 && *salary <= 150000.0);
            } else {
                panic!("Salary should be a float");
            }

            // Status should be one of the choices
            if let Value::String(status) = &record["status"] {
                assert!(status == "active" || status == "inactive");
            } else {
                panic!("Status should be a string");
            }
        }
    }

    #[test]
    fn test_generate_records_tuples() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let schema = create_test_schema();
        let field_order = vec![
            "id".to_string(),
            "name".to_string(),
            "age".to_string(),
            "salary".to_string(),
            "status".to_string(),
        ];

        let records =
            generate_records_tuples(&mut rng, Locale::EnUS, 10, &schema, &field_order).unwrap();

        assert_eq!(records.len(), 10);
        for record in &records {
            assert_eq!(record.len(), 5);
        }
    }

    #[test]
    fn test_generate_records_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let schema = create_test_schema();
        let r1 = generate_records(&mut rng1, Locale::EnUS, 50, &schema).unwrap();
        let r2 = generate_records(&mut rng2, Locale::EnUS, 50, &schema).unwrap();

        assert_eq!(r1, r2);
    }

    #[test]
    fn test_empty_records() {
        let mut rng = ForgeryRng::new();

        let schema = create_test_schema();
        let records = generate_records(&mut rng, Locale::EnUS, 0, &schema).unwrap();

        assert!(records.is_empty());
    }

    #[test]
    fn test_all_simple_types() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let types = [
            "name",
            "first_name",
            "last_name",
            "email",
            "safe_email",
            "free_email",
            "uuid",
            "phone",
            "address",
            "street_address",
            "city",
            "state",
            "country",
            "zip_code",
            "company",
            "job",
            "catch_phrase",
            "url",
            "domain_name",
            "ipv4",
            "ipv6",
            "mac_address",
            "color",
            "hex_color",
            "credit_card",
            "iban",
            "date",
            "datetime",
            "md5",
            "sha256",
            "sentence",
            "paragraph",
            "text",
        ];

        for type_name in types {
            let spec = parse_simple_type(type_name).unwrap();
            let result = generate_value(&mut rng, Locale::EnUS, &spec);
            assert!(
                result.is_ok(),
                "Failed to generate {}: {:?}",
                type_name,
                result
            );
        }
    }

    #[test]
    fn test_rgb_color_type() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let spec = parse_simple_type("rgb_color").unwrap();
        let result = generate_value(&mut rng, Locale::EnUS, &spec).unwrap();

        // Just verify we get a Tuple3U8 variant (u8 values are always 0-255)
        assert!(
            matches!(result, Value::Tuple3U8(_, _, _)),
            "RGB color should be a tuple"
        );
    }

    #[test]
    fn test_text_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let spec = FieldSpec::Text {
            min_chars: 50,
            max_chars: 100,
        };

        for _ in 0..100 {
            let result = generate_value(&mut rng, Locale::EnUS, &spec).unwrap();
            if let Value::String(text) = result {
                assert!(text.len() >= 50 && text.len() <= 100);
            } else {
                panic!("Text should be a string");
            }
        }
    }

    #[test]
    fn test_date_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let spec = FieldSpec::DateRange {
            start: "2020-01-01".to_string(),
            end: "2020-12-31".to_string(),
        };

        let result = generate_value(&mut rng, Locale::EnUS, &spec).unwrap();
        if let Value::String(date) = result {
            assert!(date.starts_with("2020-"));
        } else {
            panic!("Date should be a string");
        }
    }

    #[test]
    fn test_choice() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let options = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let spec = FieldSpec::Choice(options.clone());

        for _ in 0..100 {
            let result = generate_value(&mut rng, Locale::EnUS, &spec).unwrap();
            if let Value::String(val) = result {
                assert!(options.contains(&val));
            } else {
                panic!("Choice should be a string");
            }
        }
    }

    #[test]
    fn test_invalid_type() {
        let result = parse_simple_type("invalid_type");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_int_range() {
        let mut rng = ForgeryRng::new();

        let spec = FieldSpec::IntRange { min: 100, max: 10 };
        let result = generate_value(&mut rng, Locale::EnUS, &spec);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_float_range() {
        let mut rng = ForgeryRng::new();

        let spec = FieldSpec::FloatRange {
            min: 100.0,
            max: 10.0,
        };
        let result = generate_value(&mut rng, Locale::EnUS, &spec);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_choice() {
        let mut rng = ForgeryRng::new();

        let spec = FieldSpec::Choice(vec![]);
        let result = generate_value(&mut rng, Locale::EnUS, &spec);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_field_in_order() {
        let mut rng = ForgeryRng::new();

        let schema = create_test_schema();
        let field_order = vec!["id".to_string(), "nonexistent".to_string()];

        let result = generate_records_tuples(&mut rng, Locale::EnUS, 10, &schema, &field_order);
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_field_in_order() {
        let mut rng = ForgeryRng::new();

        let schema = create_test_schema();
        let field_order = vec![
            "id".to_string(),
            "name".to_string(),
            "id".to_string(), // duplicate
            "age".to_string(),
            "salary".to_string(),
            "status".to_string(),
        ];

        let result = generate_records_tuples(&mut rng, Locale::EnUS, 10, &schema, &field_order);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .message
            .contains("Duplicate field in field_order"));
    }

    #[test]
    fn test_field_order_missing_fields() {
        let mut rng = ForgeryRng::new();

        let schema = create_test_schema();
        // Only 3 fields, but schema has 5
        let field_order = vec!["id".to_string(), "name".to_string(), "age".to_string()];

        let result = generate_records_tuples(&mut rng, Locale::EnUS, 10, &schema, &field_order);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .message
            .contains("field_order must cover all schema fields"));
    }

    #[test]
    fn test_schema_validation_when_n_is_zero() {
        let mut rng = ForgeryRng::new();

        let mut schema = BTreeMap::new();
        schema.insert("age".to_string(), FieldSpec::IntRange { min: 100, max: 10 }); // invalid

        // Even with n=0, schema should be validated
        let result = generate_records(&mut rng, Locale::EnUS, 0, &schema);
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Invalid int range"));
    }

    #[test]
    fn test_tuples_schema_validation_when_n_is_zero() {
        let mut rng = ForgeryRng::new();

        let mut schema = BTreeMap::new();
        schema.insert("status".to_string(), FieldSpec::Choice(vec![])); // empty choice

        let field_order = vec!["status".to_string()];

        // Even with n=0, schema should be validated
        let result = generate_records_tuples(&mut rng, Locale::EnUS, 0, &schema, &field_order);
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("empty"));
    }

    // Arrow-specific tests
    #[test]
    fn test_generate_records_arrow_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let schema = create_test_schema();
        let batch = generate_records_arrow(&mut rng, Locale::EnUS, 100, &schema).unwrap();

        assert_eq!(batch.num_rows(), 100);
    }

    #[test]
    fn test_generate_records_arrow_column_count() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let schema = create_test_schema();
        let batch = generate_records_arrow(&mut rng, Locale::EnUS, 10, &schema).unwrap();

        assert_eq!(batch.num_columns(), 5);
    }

    #[test]
    fn test_generate_records_arrow_deterministic() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();

        rng1.seed(12345);
        rng2.seed(12345);

        let schema = create_test_schema();
        let batch1 = generate_records_arrow(&mut rng1, Locale::EnUS, 50, &schema).unwrap();
        let batch2 = generate_records_arrow(&mut rng2, Locale::EnUS, 50, &schema).unwrap();

        // Compare the actual data in the batches
        assert_eq!(batch1.num_rows(), batch2.num_rows());
        assert_eq!(batch1.num_columns(), batch2.num_columns());
        for i in 0..batch1.num_columns() {
            assert_eq!(batch1.column(i).as_ref(), batch2.column(i).as_ref());
        }
    }

    #[test]
    fn test_generate_records_arrow_empty() {
        let mut rng = ForgeryRng::new();

        let schema = create_test_schema();
        let batch = generate_records_arrow(&mut rng, Locale::EnUS, 0, &schema).unwrap();

        assert_eq!(batch.num_rows(), 0);
        assert_eq!(batch.num_columns(), 5);
    }

    #[test]
    fn test_generate_records_arrow_schema_validation_when_n_is_zero() {
        let mut rng = ForgeryRng::new();

        let mut schema = BTreeMap::new();
        schema.insert("age".to_string(), FieldSpec::IntRange { min: 100, max: 10 });

        let result = generate_records_arrow(&mut rng, Locale::EnUS, 0, &schema);
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Invalid int range"));
    }

    #[test]
    fn test_field_spec_to_arrow_type_int() {
        let spec = FieldSpec::Int;
        let arrow_type = field_spec_to_arrow_type(&spec);
        assert_eq!(arrow_type, DataType::Int64);

        let spec_range = FieldSpec::IntRange { min: 0, max: 100 };
        let arrow_type_range = field_spec_to_arrow_type(&spec_range);
        assert_eq!(arrow_type_range, DataType::Int64);
    }

    #[test]
    fn test_field_spec_to_arrow_type_float() {
        let spec = FieldSpec::Float;
        let arrow_type = field_spec_to_arrow_type(&spec);
        assert_eq!(arrow_type, DataType::Float64);

        let spec_range = FieldSpec::FloatRange { min: 0.0, max: 1.0 };
        let arrow_type_range = field_spec_to_arrow_type(&spec_range);
        assert_eq!(arrow_type_range, DataType::Float64);
    }

    #[test]
    fn test_field_spec_to_arrow_type_string() {
        let spec = FieldSpec::Name;
        let arrow_type = field_spec_to_arrow_type(&spec);
        assert_eq!(arrow_type, DataType::Utf8);
    }

    #[test]
    fn test_field_spec_to_arrow_type_rgb() {
        let spec = FieldSpec::RgbColor;
        let arrow_type = field_spec_to_arrow_type(&spec);
        assert!(matches!(arrow_type, DataType::Struct(_)));
    }

    #[test]
    fn test_generate_arrow_column_int() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let mut schema = BTreeMap::new();
        schema.insert("value".to_string(), FieldSpec::Int);

        let batch = generate_records_arrow(&mut rng, Locale::EnUS, 10, &schema).unwrap();
        assert_eq!(batch.num_rows(), 10);

        let column = batch.column(0);
        assert_eq!(*column.data_type(), DataType::Int64);
    }

    #[test]
    fn test_generate_arrow_column_int_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let mut schema = BTreeMap::new();
        schema.insert(
            "value".to_string(),
            FieldSpec::IntRange { min: 10, max: 20 },
        );

        let batch = generate_records_arrow(&mut rng, Locale::EnUS, 100, &schema).unwrap();

        let column = batch
            .column(0)
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();

        for i in 0..100 {
            let val = column.value(i);
            assert!((10..=20).contains(&val), "Value {} out of range", val);
        }
    }

    #[test]
    fn test_generate_arrow_column_float() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let mut schema = BTreeMap::new();
        schema.insert("value".to_string(), FieldSpec::Float);

        let batch = generate_records_arrow(&mut rng, Locale::EnUS, 10, &schema).unwrap();
        assert_eq!(batch.num_rows(), 10);

        let column = batch.column(0);
        assert_eq!(*column.data_type(), DataType::Float64);
    }

    #[test]
    fn test_generate_arrow_column_float_range() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let mut schema = BTreeMap::new();
        schema.insert(
            "value".to_string(),
            FieldSpec::FloatRange {
                min: 1.0,
                max: 10.0,
            },
        );

        let batch = generate_records_arrow(&mut rng, Locale::EnUS, 100, &schema).unwrap();

        let column = batch
            .column(0)
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap();

        for i in 0..100 {
            let val = column.value(i);
            assert!((1.0..=10.0).contains(&val), "Value {} out of range", val);
        }
    }

    #[test]
    fn test_generate_arrow_column_rgb_color() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let mut schema = BTreeMap::new();
        schema.insert("color".to_string(), FieldSpec::RgbColor);

        let batch = generate_records_arrow(&mut rng, Locale::EnUS, 10, &schema).unwrap();
        assert_eq!(batch.num_rows(), 10);

        let column = batch.column(0);
        assert!(matches!(column.data_type(), DataType::Struct(_)));
    }

    #[test]
    fn test_generate_arrow_column_string_types() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let mut schema = BTreeMap::new();
        schema.insert("name".to_string(), FieldSpec::Name);
        schema.insert("email".to_string(), FieldSpec::Email);
        schema.insert("uuid".to_string(), FieldSpec::Uuid);

        let batch = generate_records_arrow(&mut rng, Locale::EnUS, 10, &schema).unwrap();
        assert_eq!(batch.num_rows(), 10);
        assert_eq!(batch.num_columns(), 3);

        // All columns should be string type
        for i in 0..3 {
            assert_eq!(*batch.column(i).data_type(), DataType::Utf8);
        }
    }

    #[test]
    fn test_generate_arrow_with_custom_provider() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let mut custom_providers = HashMap::new();
        custom_providers.insert(
            "fruit".to_string(),
            CustomProvider::Uniform(vec![
                "apple".to_string(),
                "banana".to_string(),
                "cherry".to_string(),
            ]),
        );

        let mut schema = BTreeMap::new();
        schema.insert("fruit".to_string(), FieldSpec::Custom("fruit".to_string()));

        let batch = generate_records_arrow_with_custom(
            &mut rng,
            Locale::EnUS,
            10,
            &schema,
            &custom_providers,
        )
        .unwrap();

        assert_eq!(batch.num_rows(), 10);

        let column = batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();

        for i in 0..10 {
            let val = column.value(i);
            assert!(
                val == "apple" || val == "banana" || val == "cherry",
                "Unexpected value: {}",
                val
            );
        }
    }

    #[test]
    fn test_generate_arrow_choice_field() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let mut schema = BTreeMap::new();
        schema.insert(
            "status".to_string(),
            FieldSpec::Choice(vec![
                "active".to_string(),
                "inactive".to_string(),
                "pending".to_string(),
            ]),
        );

        let batch = generate_records_arrow(&mut rng, Locale::EnUS, 100, &schema).unwrap();

        let column = batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();

        for i in 0..100 {
            let val = column.value(i);
            assert!(
                val == "active" || val == "inactive" || val == "pending",
                "Unexpected value: {}",
                val
            );
        }
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use crate::locale::Locale;
    use proptest::prelude::*;

    proptest! {
        /// Property: record count is always respected
        #[test]
        fn prop_record_count(n in 0usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let mut schema = BTreeMap::new();
            schema.insert("id".to_string(), FieldSpec::Simple("uuid".to_string()));

            let records = generate_records(&mut rng, Locale::EnUS, n, &schema).unwrap();
            prop_assert_eq!(records.len(), n);
        }

        /// Property: integer ranges are respected
        #[test]
        fn prop_int_range(min in -1000i64..500, max in 500i64..1000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let spec = FieldSpec::IntRange { min, max };

            for _ in 0..100 {
                let result = generate_value(&mut rng, Locale::EnUS, &spec).unwrap();
                if let Value::Int(val) = result {
                    prop_assert!(val >= min && val <= max);
                } else {
                    prop_assert!(false, "Expected Int value");
                }
            }
        }

        /// Property: float ranges are respected
        #[test]
        fn prop_float_range(min in -1000.0f64..500.0, max in 500.0f64..1000.0) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let spec = FieldSpec::FloatRange { min, max };

            for _ in 0..100 {
                let result = generate_value(&mut rng, Locale::EnUS, &spec).unwrap();
                if let Value::Float(val) = result {
                    prop_assert!(val >= min && val <= max);
                } else {
                    prop_assert!(false, "Expected Float value");
                }
            }
        }

        /// Property: same seed produces same records
        #[test]
        fn prop_seed_determinism(seed_val in any::<u64>(), n in 1usize..20) {
            let mut rng1 = ForgeryRng::new();
            let mut rng2 = ForgeryRng::new();

            rng1.seed(seed_val);
            rng2.seed(seed_val);

            let mut schema = BTreeMap::new();
            schema.insert("id".to_string(), FieldSpec::Simple("uuid".to_string()));
            schema.insert("name".to_string(), FieldSpec::Simple("name".to_string()));

            let r1 = generate_records(&mut rng1, Locale::EnUS, n, &schema).unwrap();
            let r2 = generate_records(&mut rng2, Locale::EnUS, n, &schema).unwrap();

            prop_assert_eq!(r1, r2);
        }
    }
}
