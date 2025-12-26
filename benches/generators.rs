//! Benchmarks for forgery generators.
//!
//! Run with: cargo bench

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::hint::black_box;

// Import the internal modules via the rlib
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

criterion_group!(
    benches,
    bench_name_generation,
    bench_email_generation,
    bench_uuid_generation,
    bench_integer_generation,
    bench_single_value_generation
);
criterion_main!(benches);
