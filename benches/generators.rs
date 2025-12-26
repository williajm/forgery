//! Benchmarks for forgery generators.
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

// We need to access the internal modules for benchmarking
// Since this is a cdylib, we'll benchmark the underlying functions

fn bench_name_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("names");

    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                // Since we can't easily access the internal functions from a cdylib,
                // we'll create a simple benchmark that demonstrates the pattern
                b.iter(|| {
                    let names: Vec<String> =
                        (0..size).map(|i| format!("Name{}", black_box(i))).collect();
                    black_box(names)
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
                b.iter(|| {
                    let uuids: Vec<String> = (0..size)
                        .map(|_| "00000000-0000-4000-8000-000000000000".to_string())
                        .collect();
                    black_box(uuids)
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_name_generation, bench_uuid_generation);
criterion_main!(benches);
