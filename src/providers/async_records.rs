//! Async structured data generation provider.
//!
//! Provides async versions of the `records()`, `records_tuples()`, and
//! `records_arrow()` functions that yield control between chunks to allow
//! other async tasks to run.
//!
//! # Non-blocking Behavior
//!
//! For very large batches (millions of records), synchronous generation can
//! block the Python event loop. These async versions break generation into
//! chunks and yield control between them, allowing other coroutines to run.
//!
//! # Determinism
//!
//! ## Dict and Tuple Generation
//!
//! For `records_async` and `records_tuples_async`, the same seed produces
//! identical output regardless of chunk size. These methods generate data
//! row-by-row, so chunking doesn't affect RNG consumption order.
//!
//! ## Arrow Generation
//!
//! **Important:** `records_arrow_async` may produce different results than
//! the sync version when `n > chunk_size`. This is because:
//!
//! - Sync Arrow generates all data column-by-column (column-major order)
//! - Async Arrow generates each chunk column-by-column, then concatenates
//!
//! When `n <= chunk_size`, async and sync produce identical results.
//! When `n > chunk_size`, the RNG consumption order differs, producing
//! different (but equally valid) random data.
//!
//! For reproducibility with the sync version, either:
//! - Use `chunk_size >= n` to avoid chunking
//! - Use the sync `records_arrow()` method directly

use crate::locale::Locale;
use crate::providers::custom::CustomProvider;
use crate::providers::records::{
    generate_value_with_custom, validate_schema_with_custom, FieldSpec, SchemaError, Value,
};
use crate::rng::ForgeryRng;
use arrow_array::RecordBatch;
use std::collections::{BTreeMap, HashMap};
use tokio::task::yield_now;

/// Default chunk size for async generation.
///
/// This value balances:
/// - Small enough to yield frequently (responsive to other tasks)
/// - Large enough to amortize overhead (efficient generation)
pub const DEFAULT_CHUNK_SIZE: usize = 10_000;

/// Normalize chunk size, using default if zero.
#[inline]
fn normalize_chunk_size(chunk_size: usize) -> usize {
    if chunk_size == 0 {
        DEFAULT_CHUNK_SIZE
    } else {
        chunk_size
    }
}

/// Generate records asynchronously with chunking.
///
/// This function generates records in chunks, yielding control between
/// each chunk to allow other async tasks to run. The result is identical
/// to the synchronous version when using the same seed.
///
/// # Arguments
///
/// * `rng` - The random number generator (will be mutated)
/// * `locale` - The locale for locale-aware generation
/// * `n` - Total number of records to generate
/// * `schema` - The schema specification
/// * `chunk_size` - Number of records per chunk
/// * `custom_providers` - Custom provider definitions
///
/// # Returns
///
/// A vector of records as BTreeMaps.
pub async fn generate_records_async(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    schema: &BTreeMap<String, FieldSpec>,
    chunk_size: usize,
    custom_providers: &HashMap<String, CustomProvider>,
) -> Result<Vec<BTreeMap<String, Value>>, SchemaError> {
    // Validate schema upfront (even when n=0)
    validate_schema_with_custom(schema, custom_providers)?;

    let chunk_size = normalize_chunk_size(chunk_size);
    let mut records = Vec::with_capacity(n);
    let mut remaining = n;

    while remaining > 0 {
        let this_chunk = remaining.min(chunk_size);

        // Generate chunk synchronously
        for _ in 0..this_chunk {
            let mut record = BTreeMap::new();
            for (field_name, spec) in schema {
                let value = generate_value_with_custom(rng, locale, spec, custom_providers)?;
                record.insert(field_name.clone(), value);
            }
            records.push(record);
        }

        remaining -= this_chunk;

        // Yield control to the event loop between chunks
        if remaining > 0 {
            yield_now().await;
        }
    }

    Ok(records)
}

/// Generate records as tuples asynchronously with chunking.
///
/// This function generates records as tuples in chunks, yielding control
/// between each chunk. Values in each tuple are ordered according to
/// `field_order`.
///
/// # Arguments
///
/// * `rng` - The random number generator (will be mutated)
/// * `locale` - The locale for locale-aware generation
/// * `n` - Total number of records to generate
/// * `schema` - The schema specification
/// * `field_order` - Order of fields in output tuples
/// * `chunk_size` - Number of records per chunk
/// * `custom_providers` - Custom provider definitions
///
/// # Returns
///
/// A vector of tuples (as Vecs).
pub async fn generate_records_tuples_async(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    schema: &BTreeMap<String, FieldSpec>,
    field_order: &[String],
    chunk_size: usize,
    custom_providers: &HashMap<String, CustomProvider>,
) -> Result<Vec<Vec<Value>>, SchemaError> {
    // Validate schema upfront
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

    let chunk_size = normalize_chunk_size(chunk_size);
    let mut records = Vec::with_capacity(n);
    let mut remaining = n;

    while remaining > 0 {
        let this_chunk = remaining.min(chunk_size);

        // Generate chunk synchronously
        for _ in 0..this_chunk {
            let mut record = Vec::with_capacity(field_order.len());
            for field_name in field_order {
                let spec = schema
                    .get(field_name)
                    .expect("field_name was validated to exist in schema");
                let value = generate_value_with_custom(rng, locale, spec, custom_providers)?;
                record.push(value);
            }
            records.push(record);
        }

        remaining -= this_chunk;

        // Yield control to the event loop between chunks
        if remaining > 0 {
            yield_now().await;
        }
    }

    Ok(records)
}

/// Generate records as Arrow RecordBatch asynchronously with chunking.
///
/// This function generates Arrow data in chunks, yielding control between
/// each chunk. For large datasets, this allows other async tasks to run
/// during generation.
///
/// # Important: Chunking Affects Output
///
/// Unlike the dict/tuple async versions which produce identical output to
/// their sync counterparts, **Arrow async may produce different data when
/// `n > chunk_size`**.
///
/// This is because Arrow generation is column-major (all values for column A,
/// then column B, etc.). When chunking:
/// - Each chunk generates its own column-major data
/// - Chunks are concatenated into the final RecordBatch
/// - The RNG consumption order differs from a single sync call
///
/// When `n <= chunk_size`, the function takes a fast path that delegates
/// directly to the sync implementation, producing identical results.
///
/// For reproducibility with the sync version, use `chunk_size >= n`.
///
/// # Arguments
///
/// * `rng` - The random number generator (will be mutated)
/// * `locale` - The locale for locale-aware generation
/// * `n` - Total number of records to generate
/// * `schema` - The schema specification
/// * `chunk_size` - Number of records per chunk
/// * `custom_providers` - Custom provider definitions
///
/// # Returns
///
/// A single Arrow RecordBatch containing all records.
pub async fn generate_records_arrow_async(
    rng: &mut ForgeryRng,
    locale: Locale,
    n: usize,
    schema: &BTreeMap<String, FieldSpec>,
    chunk_size: usize,
    custom_providers: &HashMap<String, CustomProvider>,
) -> Result<RecordBatch, SchemaError> {
    use crate::providers::records::generate_records_arrow_with_custom;
    use arrow_select::concat::concat_batches;

    // Validate schema upfront
    validate_schema_with_custom(schema, custom_providers)?;

    let chunk_size = normalize_chunk_size(chunk_size);

    // For small batches, just use the sync version
    if n <= chunk_size {
        return generate_records_arrow_with_custom(rng, locale, n, schema, custom_providers);
    }

    let mut batches: Vec<RecordBatch> = Vec::new();
    let mut remaining = n;

    while remaining > 0 {
        let this_chunk = remaining.min(chunk_size);

        // Generate chunk as a RecordBatch
        let batch =
            generate_records_arrow_with_custom(rng, locale, this_chunk, schema, custom_providers)?;

        batches.push(batch);
        remaining -= this_chunk;

        // Yield control to the event loop between chunks
        if remaining > 0 {
            yield_now().await;
        }
    }

    // Concatenate all batches into one
    if batches.len() == 1 {
        return Ok(batches.into_iter().next().unwrap());
    }

    let schema = batches[0].schema();

    concat_batches(&schema, batches.iter()).map_err(|e| SchemaError {
        message: format!("Failed to concatenate Arrow batches: {}", e),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_schema() -> BTreeMap<String, FieldSpec> {
        let mut schema = BTreeMap::new();
        schema.insert("name".to_string(), FieldSpec::Name);
        schema.insert("age".to_string(), FieldSpec::IntRange { min: 18, max: 65 });
        schema
    }

    #[tokio::test]
    async fn test_generate_records_async_basic() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let schema = create_test_schema();
        let records =
            generate_records_async(&mut rng, Locale::EnUS, 100, &schema, 10, &HashMap::new())
                .await
                .unwrap();

        assert_eq!(records.len(), 100);
        for record in &records {
            assert!(record.contains_key("name"));
            assert!(record.contains_key("age"));
        }
    }

    #[tokio::test]
    async fn test_generate_records_async_determinism() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();
        rng1.seed(42);
        rng2.seed(42);

        let schema = create_test_schema();

        let records1 =
            generate_records_async(&mut rng1, Locale::EnUS, 100, &schema, 10, &HashMap::new())
                .await
                .unwrap();

        let records2 =
            generate_records_async(&mut rng2, Locale::EnUS, 100, &schema, 10, &HashMap::new())
                .await
                .unwrap();

        assert_eq!(records1, records2);
    }

    #[tokio::test]
    async fn test_generate_records_async_same_as_sync() {
        use crate::providers::records::generate_records_with_custom;

        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();
        rng1.seed(42);
        rng2.seed(42);

        let schema = create_test_schema();

        // Sync version
        let sync_records =
            generate_records_with_custom(&mut rng1, Locale::EnUS, 100, &schema, &HashMap::new())
                .unwrap();

        // Async version
        let async_records =
            generate_records_async(&mut rng2, Locale::EnUS, 100, &schema, 10, &HashMap::new())
                .await
                .unwrap();

        assert_eq!(sync_records, async_records);
    }

    #[tokio::test]
    async fn test_generate_records_async_empty() {
        let mut rng = ForgeryRng::new();
        let schema = create_test_schema();

        let records =
            generate_records_async(&mut rng, Locale::EnUS, 0, &schema, 10, &HashMap::new())
                .await
                .unwrap();

        assert!(records.is_empty());
    }

    #[tokio::test]
    async fn test_generate_records_tuples_async_basic() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let schema = create_test_schema();
        let field_order = vec!["age".to_string(), "name".to_string()];

        let records = generate_records_tuples_async(
            &mut rng,
            Locale::EnUS,
            100,
            &schema,
            &field_order,
            10,
            &HashMap::new(),
        )
        .await
        .unwrap();

        assert_eq!(records.len(), 100);
        for record in &records {
            assert_eq!(record.len(), 2);
        }
    }

    #[tokio::test]
    async fn test_generate_records_arrow_async_basic() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let schema = create_test_schema();

        let batch =
            generate_records_arrow_async(&mut rng, Locale::EnUS, 100, &schema, 10, &HashMap::new())
                .await
                .unwrap();

        assert_eq!(batch.num_rows(), 100);
        assert_eq!(batch.num_columns(), 2);
    }

    #[tokio::test]
    async fn test_generate_records_arrow_async_determinism() {
        let mut rng1 = ForgeryRng::new();
        let mut rng2 = ForgeryRng::new();
        rng1.seed(42);
        rng2.seed(42);

        let schema = create_test_schema();

        let batch1 = generate_records_arrow_async(
            &mut rng1,
            Locale::EnUS,
            100,
            &schema,
            10,
            &HashMap::new(),
        )
        .await
        .unwrap();

        let batch2 = generate_records_arrow_async(
            &mut rng2,
            Locale::EnUS,
            100,
            &schema,
            10,
            &HashMap::new(),
        )
        .await
        .unwrap();

        assert_eq!(batch1, batch2);
    }

    #[tokio::test]
    async fn test_chunk_size_zero_uses_default() {
        let mut rng = ForgeryRng::new();
        rng.seed(42);

        let schema = create_test_schema();

        // chunk_size = 0 should use DEFAULT_CHUNK_SIZE
        let records = generate_records_async(
            &mut rng,
            Locale::EnUS,
            100,
            &schema,
            0, // Should use default
            &HashMap::new(),
        )
        .await
        .unwrap();

        assert_eq!(records.len(), 100);
    }
}
