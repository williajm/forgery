//! Benchmarks for forgery generators.
//!
//! Run with: cargo bench

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::collections::BTreeMap;
use std::hint::black_box;

// Import the internal modules via the rlib
use _forgery::providers::records::FieldSpec;
use _forgery::Faker;

fn bench_name_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("names");

    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                let mut faker = Faker::new("en_US").unwrap();
                faker.seed(42);
                b.iter(|| {
                    let names = faker.names(black_box(size)).unwrap();
                    black_box(names)
                });
            },
        );
    }
    group.finish();
}

fn bench_email_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("emails");

    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                let mut faker = Faker::new("en_US").unwrap();
                faker.seed(42);
                b.iter(|| {
                    let emails = faker.emails(black_box(size)).unwrap();
                    black_box(emails)
                });
            },
        );
    }
    group.finish();
}

fn bench_uuid_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("uuids");

    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                let mut faker = Faker::new("en_US").unwrap();
                faker.seed(42);
                b.iter(|| {
                    let uuids = faker.uuids(black_box(size)).unwrap();
                    black_box(uuids)
                });
            },
        );
    }
    group.finish();
}

fn bench_integer_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("integers");

    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                let mut faker = Faker::new("en_US").unwrap();
                faker.seed(42);
                b.iter(|| {
                    let ints = faker.integers(black_box(size), 0, 1_000_000).unwrap();
                    black_box(ints)
                });
            },
        );
    }
    group.finish();
}

fn bench_single_value_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_value");

    group.bench_function("name", |b| {
        let mut faker = Faker::new("en_US").unwrap();
        faker.seed(42);
        b.iter(|| black_box(faker.name()));
    });

    group.bench_function("email", |b| {
        let mut faker = Faker::new("en_US").unwrap();
        faker.seed(42);
        b.iter(|| black_box(faker.email()));
    });

    group.bench_function("uuid", |b| {
        let mut faker = Faker::new("en_US").unwrap();
        faker.seed(42);
        b.iter(|| black_box(faker.uuid()));
    });

    group.bench_function("integer", |b| {
        let mut faker = Faker::new("en_US").unwrap();
        faker.seed(42);
        b.iter(|| black_box(faker.integer(0, 100).unwrap()));
    });

    group.finish();
}

fn bench_records_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("records");

    // Create a typical schema with 6 fields
    let mut schema = BTreeMap::new();
    schema.insert("id".to_string(), FieldSpec::Uuid);
    schema.insert("name".to_string(), FieldSpec::Name);
    schema.insert("email".to_string(), FieldSpec::Email);
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
        FieldSpec::Choice(vec![
            "active".to_string(),
            "inactive".to_string(),
            "pending".to_string(),
        ]),
    );

    for size in [100, 1_000, 10_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            criterion::BenchmarkId::new("records", size),
            size,
            |b, &size| {
                let mut faker = Faker::new("en_US").unwrap();
                faker.seed(42);
                b.iter(|| {
                    let records = faker.records(black_box(size), &schema).unwrap();
                    black_box(records)
                });
            },
        );
    }

    let field_order: Vec<String> = schema.keys().cloned().collect();
    for size in [100, 1_000, 10_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            criterion::BenchmarkId::new("records_tuples", size),
            size,
            |b, &size| {
                let mut faker = Faker::new("en_US").unwrap();
                faker.seed(42);
                b.iter(|| {
                    let records = faker
                        .records_tuples(black_box(size), &schema, &field_order)
                        .unwrap();
                    black_box(records)
                });
            },
        );
    }

    // Arrow generation benchmarks
    for size in [100, 1_000, 10_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            criterion::BenchmarkId::new("records_arrow", size),
            size,
            |b, &size| {
                let mut faker = Faker::new("en_US").unwrap();
                faker.seed(42);
                b.iter(|| {
                    let batch = faker.records_arrow(black_box(size), &schema).unwrap();
                    black_box(batch)
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_name_generation,
    bench_email_generation,
    bench_uuid_generation,
    bench_integer_generation,
    bench_single_value_generation,
    bench_records_generation
);
criterion_main!(benches);
