#!/usr/bin/env python3
"""Benchmark forgery against Faker.

This script compares the performance of forgery batch generation
against Faker's traditional single-value generation.

Usage:
    python bench_vs_faker.py
"""

import time
from collections.abc import Callable
from typing import TypeVar

T = TypeVar("T")

# Number of items to generate in each benchmark
N = 100_000


def bench(name: str, func: Callable[[], T], iterations: int = 3) -> float:
    """Run a benchmark and return the best time.

    Args:
        name: Name of the benchmark.
        func: Function to benchmark.
        iterations: Number of iterations to run.

    Returns:
        Best time in seconds.
    """
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        func()
        elapsed = time.perf_counter() - start
        times.append(elapsed)

    best = min(times)
    print(f"  {name}: {best:.3f}s")
    return best


def main() -> None:
    """Run all benchmarks."""
    print(f"Benchmarking with N={N:,}\n")

    try:
        from faker import Faker as OriginalFaker

        has_faker = True
    except ImportError:
        print("Faker not installed. Install with: pip install faker")
        print("Running forgery benchmarks only.\n")
        has_faker = False

    import forgery

    results: dict[str, dict[str, float]] = {}

    # Names benchmark
    print("Names:")
    forgery.seed(42)
    forgery_time = bench("forgery.names()", lambda: forgery.names(N))
    results["names"] = {"forgery": forgery_time}

    if has_faker:
        faker = OriginalFaker()
        faker.seed_instance(42)
        faker_time = bench("Faker.name()", lambda: [faker.name() for _ in range(N)])
        results["names"]["faker"] = faker_time
        print(f"  Speedup: {faker_time / forgery_time:.1f}x\n")
    else:
        print()

    # Emails benchmark
    print("Emails:")
    forgery.seed(42)
    forgery_time = bench("forgery.emails()", lambda: forgery.emails(N))
    results["emails"] = {"forgery": forgery_time}

    if has_faker:
        faker = OriginalFaker()
        faker.seed_instance(42)
        faker_time = bench("Faker.email()", lambda: [faker.email() for _ in range(N)])
        results["emails"]["faker"] = faker_time
        print(f"  Speedup: {faker_time / forgery_time:.1f}x\n")
    else:
        print()

    # Integers benchmark
    print("Integers:")
    forgery.seed(42)
    forgery_time = bench("forgery.integers()", lambda: forgery.integers(N, 0, 1000))
    results["integers"] = {"forgery": forgery_time}

    if has_faker:
        faker = OriginalFaker()
        faker.seed_instance(42)
        faker_time = bench(
            "Faker.random_int()", lambda: [faker.random_int(0, 1000) for _ in range(N)]
        )
        results["integers"]["faker"] = faker_time
        print(f"  Speedup: {faker_time / forgery_time:.1f}x\n")
    else:
        print()

    # UUIDs benchmark
    print("UUIDs:")
    forgery.seed(42)
    forgery_time = bench("forgery.uuids()", lambda: forgery.uuids(N))
    results["uuids"] = {"forgery": forgery_time}

    if has_faker:
        faker = OriginalFaker()
        faker.seed_instance(42)
        faker_time = bench("Faker.uuid4()", lambda: [faker.uuid4() for _ in range(N)])
        results["uuids"]["faker"] = faker_time
        print(f"  Speedup: {faker_time / forgery_time:.1f}x\n")
    else:
        print()

    # Summary
    if has_faker:
        print("=" * 50)
        print("Summary:")
        print("=" * 50)
        print(f"{'Operation':<15} {'Faker':>10} {'forgery':>10} {'Speedup':>10}")
        print("-" * 50)
        for op, times in results.items():
            faker_t = times.get("faker", 0)
            forgery_t = times["forgery"]
            speedup = faker_t / forgery_t if faker_t > 0 else 0
            print(f"{op:<15} {faker_t:>9.3f}s {forgery_t:>9.3f}s {speedup:>9.1f}x")


if __name__ == "__main__":
    main()
