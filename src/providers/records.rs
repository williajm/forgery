//! Structured data generation provider.
//!
//! Generates records based on a schema specification.

use crate::providers::{
    address, colors, company, datetime, finance, identifiers, internet, names, network, numbers,
    phone, text,
};
use crate::rng::ForgeryRng;
use std::collections::HashMap;
use std::fmt;

/// Error type for schema-related errors.
#[derive(Debug, Clone)]
pub struct SchemaError {
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
    IntRange { min: i64, max: i64 },
    /// Float range: ("float", min, max)
    FloatRange { min: f64, max: f64 },
    /// Text with character limits: ("text", min_chars, max_chars)
    Text { min_chars: usize, max_chars: usize },
    /// Date range: ("date", start, end)
    DateRange { start: String, end: String },
    /// Choice from options: ("choice", ["a", "b", "c"])
    Choice(Vec<String>),
}

/// A generated value that can be various types.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Tuple3U8(u8, u8, u8), // For RGB colors
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
    let valid_types = [
        "name",
        "first_name",
        "last_name",
        "email",
        "safe_email",
        "free_email",
        "uuid",
        "int",
        "float",
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
        "rgb_color",
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

    if valid_types.contains(&type_name) {
        Ok(FieldSpec::Simple(type_name.to_string()))
    } else {
        Err(SchemaError {
            message: format!("Unknown type: {}", type_name),
        })
    }
}

/// Generate a value based on a field specification.
pub fn generate_value(rng: &mut ForgeryRng, spec: &FieldSpec) -> Result<Value, SchemaError> {
    match spec {
        FieldSpec::Simple(type_name) => generate_simple_value(rng, type_name),
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
            let val = text::generate_text(rng, *min_chars, *max_chars);
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
    }
}

/// Generate a value for a simple type.
fn generate_simple_value(rng: &mut ForgeryRng, type_name: &str) -> Result<Value, SchemaError> {
    match type_name {
        // Names
        "name" => Ok(Value::String(names::generate_name(rng))),
        "first_name" => Ok(Value::String(names::generate_first_name(rng))),
        "last_name" => Ok(Value::String(names::generate_last_name(rng))),

        // Internet
        "email" => Ok(Value::String(internet::generate_email(rng))),
        "safe_email" => Ok(Value::String(internet::generate_safe_email(rng))),
        "free_email" => Ok(Value::String(internet::generate_free_email(rng))),

        // Identifiers
        "uuid" => Ok(Value::String(identifiers::generate_uuid(rng))),
        "md5" => Ok(Value::String(identifiers::generate_md5(rng))),
        "sha256" => Ok(Value::String(identifiers::generate_sha256(rng))),

        // Numbers (defaults)
        "int" => Ok(Value::Int(
            numbers::generate_integer(rng, 0, 1000).map_err(|e| SchemaError {
                message: e.to_string(),
            })?,
        )),
        "float" => Ok(Value::Float(
            numbers::generate_float(rng, 0.0, 1.0).map_err(|e| SchemaError {
                message: e.to_string(),
            })?,
        )),

        // Phone
        "phone" => Ok(Value::String(phone::generate_phone_number(rng))),

        // Address
        "address" => Ok(Value::String(address::generate_address(rng))),
        "street_address" => Ok(Value::String(address::generate_street_address(rng))),
        "city" => Ok(Value::String(address::generate_city(rng))),
        "state" => Ok(Value::String(address::generate_state(rng))),
        "country" => Ok(Value::String(address::generate_country(rng))),
        "zip_code" => Ok(Value::String(address::generate_zip_code(rng))),

        // Company
        "company" => Ok(Value::String(company::generate_company(rng))),
        "job" => Ok(Value::String(company::generate_job(rng))),
        "catch_phrase" => Ok(Value::String(company::generate_catch_phrase(rng))),

        // Network
        "url" => Ok(Value::String(network::generate_url(rng))),
        "domain_name" => Ok(Value::String(network::generate_domain_name(rng))),
        "ipv4" => Ok(Value::String(network::generate_ipv4(rng))),
        "ipv6" => Ok(Value::String(network::generate_ipv6(rng))),
        "mac_address" => Ok(Value::String(network::generate_mac_address(rng))),

        // Colors
        "color" => Ok(Value::String(colors::generate_color(rng))),
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
        "sentence" => Ok(Value::String(text::generate_sentence(rng, 10))),
        "paragraph" => Ok(Value::String(text::generate_paragraph(rng, 5))),
        "text" => Ok(Value::String(text::generate_text(rng, 50, 200))),

        _ => Err(SchemaError {
            message: format!("Unknown type: {}", type_name),
        }),
    }
}

/// Generate records based on a schema.
///
/// Returns a vector of HashMaps, where each HashMap represents a record
/// with field names as keys and generated values.
pub fn generate_records(
    rng: &mut ForgeryRng,
    n: usize,
    schema: &HashMap<String, FieldSpec>,
) -> Result<Vec<HashMap<String, Value>>, SchemaError> {
    let mut records = Vec::with_capacity(n);

    for _ in 0..n {
        let mut record = HashMap::new();
        for (field_name, spec) in schema {
            let value = generate_value(rng, spec)?;
            record.insert(field_name.clone(), value);
        }
        records.push(record);
    }

    Ok(records)
}

/// Generate records as tuples based on a schema.
///
/// Returns a vector of vectors, where each inner vector contains values
/// in the same order as the provided field order.
pub fn generate_records_tuples(
    rng: &mut ForgeryRng,
    n: usize,
    schema: &HashMap<String, FieldSpec>,
    field_order: &[String],
) -> Result<Vec<Vec<Value>>, SchemaError> {
    // Validate field order matches schema
    for field in field_order {
        if !schema.contains_key(field) {
            return Err(SchemaError {
                message: format!("Field '{}' not in schema", field),
            });
        }
    }

    let mut records = Vec::with_capacity(n);

    for _ in 0..n {
        let mut record = Vec::with_capacity(field_order.len());
        for field_name in field_order {
            let spec = schema.get(field_name).unwrap();
            let value = generate_value(rng, spec)?;
            record.push(value);
        }
        records.push(record);
    }

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_schema() -> HashMap<String, FieldSpec> {
        let mut schema = HashMap::new();
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
        let records = generate_records(&mut rng, 100, &schema).unwrap();

        assert_eq!(records.len(), 100);
    }

    #[test]
    fn test_generate_records_has_all_fields() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let schema = create_test_schema();
        let records = generate_records(&mut rng, 10, &schema).unwrap();

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
        let records = generate_records(&mut rng, 10, &schema).unwrap();

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

        let records = generate_records_tuples(&mut rng, 10, &schema, &field_order).unwrap();

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
        let r1 = generate_records(&mut rng1, 50, &schema).unwrap();
        let r2 = generate_records(&mut rng2, 50, &schema).unwrap();

        assert_eq!(r1, r2);
    }

    #[test]
    fn test_empty_records() {
        let mut rng = ForgeryRng::new();

        let schema = create_test_schema();
        let records = generate_records(&mut rng, 0, &schema).unwrap();

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
            let result = generate_value(&mut rng, &spec);
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
        let result = generate_value(&mut rng, &spec).unwrap();

        if let Value::Tuple3U8(r, g, b) = result {
            assert!(r <= 255);
            assert!(g <= 255);
            assert!(b <= 255);
        } else {
            panic!("RGB color should be a tuple");
        }
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
            let result = generate_value(&mut rng, &spec).unwrap();
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

        let result = generate_value(&mut rng, &spec).unwrap();
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
            let result = generate_value(&mut rng, &spec).unwrap();
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
        let result = generate_value(&mut rng, &spec);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_float_range() {
        let mut rng = ForgeryRng::new();

        let spec = FieldSpec::FloatRange {
            min: 100.0,
            max: 10.0,
        };
        let result = generate_value(&mut rng, &spec);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_choice() {
        let mut rng = ForgeryRng::new();

        let spec = FieldSpec::Choice(vec![]);
        let result = generate_value(&mut rng, &spec);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_field_in_order() {
        let mut rng = ForgeryRng::new();

        let schema = create_test_schema();
        let field_order = vec!["id".to_string(), "nonexistent".to_string()];

        let result = generate_records_tuples(&mut rng, 10, &schema, &field_order);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: record count is always respected
        #[test]
        fn prop_record_count(n in 0usize..100) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let mut schema = HashMap::new();
            schema.insert("id".to_string(), FieldSpec::Simple("uuid".to_string()));

            let records = generate_records(&mut rng, n, &schema).unwrap();
            prop_assert_eq!(records.len(), n);
        }

        /// Property: integer ranges are respected
        #[test]
        fn prop_int_range(min in -1000i64..500, max in 500i64..1000) {
            let mut rng = ForgeryRng::new();
            rng.seed(42);

            let spec = FieldSpec::IntRange { min, max };

            for _ in 0..100 {
                let result = generate_value(&mut rng, &spec).unwrap();
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
                let result = generate_value(&mut rng, &spec).unwrap();
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

            let mut schema = HashMap::new();
            schema.insert("id".to_string(), FieldSpec::Simple("uuid".to_string()));
            schema.insert("name".to_string(), FieldSpec::Simple("name".to_string()));

            let r1 = generate_records(&mut rng1, n, &schema).unwrap();
            let r2 = generate_records(&mut rng2, n, &schema).unwrap();

            prop_assert_eq!(r1, r2);
        }
    }
}
