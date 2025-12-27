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


def run_benchmark(
    label: str,
    forgery_func: Callable[[], T],
    faker_func: Callable[[], T] | None,
    results: dict[str, dict[str, float]],
    key: str,
) -> None:
    """Run a single benchmark comparison.

    Args:
        label: Display label for the benchmark.
        forgery_func: Forgery function to benchmark.
        faker_func: Faker function to benchmark (None if Faker not available).
        results: Dictionary to store results.
        key: Key for storing results.
    """
    print(f"{label}:")
    forgery_time = bench(f"forgery.{key}()", forgery_func)
    results[key] = {"forgery": forgery_time}

    if faker_func is not None:
        faker_time = bench(f"Faker.{key}()", faker_func)
        results[key]["faker"] = faker_time
        print(f"  Speedup: {faker_time / forgery_time:.1f}x\n")
    else:
        print()


def main() -> None:
    """Run all benchmarks."""
    print(f"Benchmarking with N={N:,}\n")

    try:
        from faker import Faker as OriginalFaker

        has_faker = True
        faker = OriginalFaker()
        faker.seed_instance(42)
    except ImportError:
        print("Faker not installed. Install with: pip install faker")
        print("Running forgery benchmarks only.\n")
        has_faker = False
        faker = None

    import forgery

    results: dict[str, dict[str, float]] = {}

    # ==========================================================================
    # Phase 1 Providers
    # ==========================================================================
    print("=" * 60)
    print("PHASE 1 PROVIDERS")
    print("=" * 60 + "\n")

    # Names
    forgery.seed(42)
    run_benchmark(
        "Names",
        lambda: forgery.names(N),
        (lambda: [faker.name() for _ in range(N)]) if has_faker else None,
        results,
        "names",
    )

    # Emails
    forgery.seed(42)
    run_benchmark(
        "Emails",
        lambda: forgery.emails(N),
        (lambda: [faker.email() for _ in range(N)]) if has_faker else None,
        results,
        "emails",
    )

    # Integers
    forgery.seed(42)
    run_benchmark(
        "Integers",
        lambda: forgery.integers(N, 0, 1000),
        (lambda: [faker.random_int(0, 1000) for _ in range(N)]) if has_faker else None,
        results,
        "integers",
    )

    # UUIDs
    forgery.seed(42)
    run_benchmark(
        "UUIDs",
        lambda: forgery.uuids(N),
        (lambda: [faker.uuid4() for _ in range(N)]) if has_faker else None,
        results,
        "uuids",
    )

    # ==========================================================================
    # Phase 2 Providers
    # ==========================================================================
    print("=" * 60)
    print("PHASE 2 PROVIDERS")
    print("=" * 60 + "\n")

    # --- DateTime ---
    print("-" * 40)
    print("DateTime")
    print("-" * 40 + "\n")

    forgery.seed(42)
    run_benchmark(
        "Dates",
        lambda: forgery.dates(N, "2000-01-01", "2024-12-31"),
        (lambda: [faker.date_between(start_date="-20y", end_date="today") for _ in range(N)])
        if has_faker
        else None,
        results,
        "dates",
    )

    forgery.seed(42)
    run_benchmark(
        "Datetimes",
        lambda: forgery.datetimes(N, "2000-01-01", "2024-12-31"),
        (lambda: [faker.date_time() for _ in range(N)]) if has_faker else None,
        results,
        "datetimes",
    )

    forgery.seed(42)
    run_benchmark(
        "Dates of Birth",
        lambda: forgery.dates_of_birth(N, 18, 80),
        (lambda: [faker.date_of_birth(minimum_age=18, maximum_age=80) for _ in range(N)])
        if has_faker
        else None,
        results,
        "dates_of_birth",
    )

    # --- Finance ---
    print("-" * 40)
    print("Finance")
    print("-" * 40 + "\n")

    forgery.seed(42)
    run_benchmark(
        "Credit Cards",
        lambda: forgery.credit_cards(N),
        (lambda: [faker.credit_card_number() for _ in range(N)]) if has_faker else None,
        results,
        "credit_cards",
    )

    forgery.seed(42)
    run_benchmark(
        "IBANs",
        lambda: forgery.ibans(N),
        (lambda: [faker.iban() for _ in range(N)]) if has_faker else None,
        results,
        "ibans",
    )

    # --- Network ---
    print("-" * 40)
    print("Network")
    print("-" * 40 + "\n")

    forgery.seed(42)
    run_benchmark(
        "IPv4 Addresses",
        lambda: forgery.ipv4s(N),
        (lambda: [faker.ipv4() for _ in range(N)]) if has_faker else None,
        results,
        "ipv4s",
    )

    forgery.seed(42)
    run_benchmark(
        "IPv6 Addresses",
        lambda: forgery.ipv6s(N),
        (lambda: [faker.ipv6() for _ in range(N)]) if has_faker else None,
        results,
        "ipv6s",
    )

    forgery.seed(42)
    run_benchmark(
        "MAC Addresses",
        lambda: forgery.mac_addresses(N),
        (lambda: [faker.mac_address() for _ in range(N)]) if has_faker else None,
        results,
        "mac_addresses",
    )

    forgery.seed(42)
    run_benchmark(
        "URLs",
        lambda: forgery.urls(N),
        (lambda: [faker.url() for _ in range(N)]) if has_faker else None,
        results,
        "urls",
    )

    forgery.seed(42)
    run_benchmark(
        "Domain Names",
        lambda: forgery.domain_names(N),
        (lambda: [faker.domain_name() for _ in range(N)]) if has_faker else None,
        results,
        "domain_names",
    )

    # --- Phone ---
    print("-" * 40)
    print("Phone")
    print("-" * 40 + "\n")

    forgery.seed(42)
    run_benchmark(
        "Phone Numbers",
        lambda: forgery.phone_numbers(N),
        (lambda: [faker.phone_number() for _ in range(N)]) if has_faker else None,
        results,
        "phone_numbers",
    )

    # --- Text ---
    print("-" * 40)
    print("Text")
    print("-" * 40 + "\n")

    forgery.seed(42)
    run_benchmark(
        "Sentences",
        lambda: forgery.sentences(N, 10),
        (lambda: [faker.sentence(nb_words=10) for _ in range(N)]) if has_faker else None,
        results,
        "sentences",
    )

    forgery.seed(42)
    run_benchmark(
        "Paragraphs",
        lambda: forgery.paragraphs(N, 5),
        (lambda: [faker.paragraph(nb_sentences=5) for _ in range(N)]) if has_faker else None,
        results,
        "paragraphs",
    )

    forgery.seed(42)
    run_benchmark(
        "Texts",
        lambda: forgery.texts(N, 50, 200),
        (lambda: [faker.text(max_nb_chars=200) for _ in range(N)]) if has_faker else None,
        results,
        "texts",
    )

    # --- Colors ---
    print("-" * 40)
    print("Colors")
    print("-" * 40 + "\n")

    forgery.seed(42)
    run_benchmark(
        "Color Names",
        lambda: forgery.colors(N),
        (lambda: [faker.color_name() for _ in range(N)]) if has_faker else None,
        results,
        "colors",
    )

    forgery.seed(42)
    run_benchmark(
        "Hex Colors",
        lambda: forgery.hex_colors(N),
        (lambda: [faker.hex_color() for _ in range(N)]) if has_faker else None,
        results,
        "hex_colors",
    )

    forgery.seed(42)
    run_benchmark(
        "RGB Colors",
        lambda: forgery.rgb_colors(N),
        (lambda: [faker.rgb_color() for _ in range(N)]) if has_faker else None,
        results,
        "rgb_colors",
    )

    # --- Company ---
    print("-" * 40)
    print("Company")
    print("-" * 40 + "\n")

    forgery.seed(42)
    run_benchmark(
        "Companies",
        lambda: forgery.companies(N),
        (lambda: [faker.company() for _ in range(N)]) if has_faker else None,
        results,
        "companies",
    )

    forgery.seed(42)
    run_benchmark(
        "Jobs",
        lambda: forgery.jobs(N),
        (lambda: [faker.job() for _ in range(N)]) if has_faker else None,
        results,
        "jobs",
    )

    forgery.seed(42)
    run_benchmark(
        "Catch Phrases",
        lambda: forgery.catch_phrases(N),
        (lambda: [faker.catch_phrase() for _ in range(N)]) if has_faker else None,
        results,
        "catch_phrases",
    )

    # --- Address ---
    print("-" * 40)
    print("Address")
    print("-" * 40 + "\n")

    forgery.seed(42)
    run_benchmark(
        "Street Addresses",
        lambda: forgery.street_addresses(N),
        (lambda: [faker.street_address() for _ in range(N)]) if has_faker else None,
        results,
        "street_addresses",
    )

    forgery.seed(42)
    run_benchmark(
        "Cities",
        lambda: forgery.cities(N),
        (lambda: [faker.city() for _ in range(N)]) if has_faker else None,
        results,
        "cities",
    )

    forgery.seed(42)
    run_benchmark(
        "States",
        lambda: forgery.states(N),
        (lambda: [faker.state() for _ in range(N)]) if has_faker else None,
        results,
        "states",
    )

    forgery.seed(42)
    run_benchmark(
        "Countries",
        lambda: forgery.countries(N),
        (lambda: [faker.country() for _ in range(N)]) if has_faker else None,
        results,
        "countries",
    )

    forgery.seed(42)
    run_benchmark(
        "Zip Codes",
        lambda: forgery.zip_codes(N),
        (lambda: [faker.zipcode() for _ in range(N)]) if has_faker else None,
        results,
        "zip_codes",
    )

    forgery.seed(42)
    run_benchmark(
        "Full Addresses",
        lambda: forgery.addresses(N),
        (lambda: [faker.address() for _ in range(N)]) if has_faker else None,
        results,
        "addresses",
    )

    # ==========================================================================
    # Structured Data Generation
    # ==========================================================================
    print("=" * 60)
    print("STRUCTURED DATA GENERATION")
    print("=" * 60 + "\n")

    # Define a typical schema for benchmarking
    schema = {
        "id": "uuid",
        "name": "name",
        "email": "email",
        "age": ("int", 18, 65),
        "salary": ("float", 30000.0, 150000.0),
        "status": ("choice", ["active", "inactive", "pending"]),
    }

    def faker_records(n: int) -> list[dict[str, object]]:
        """Generate records using Faker (the slow way)."""
        records = []
        statuses = ["active", "inactive", "pending"]
        for _ in range(n):
            records.append(
                {
                    "id": faker.uuid4(),
                    "name": faker.name(),
                    "email": faker.email(),
                    "age": faker.random_int(18, 65),
                    "salary": faker.pyfloat(min_value=30000.0, max_value=150000.0),
                    "status": faker.random_element(statuses),
                }
            )
        return records

    forgery.seed(42)
    run_benchmark(
        "Records (6-field schema)",
        lambda: forgery.records(N, schema),
        (lambda: faker_records(N)) if has_faker else None,
        results,
        "records",
    )

    forgery.seed(42)
    run_benchmark(
        "Records Tuples (6-field schema)",
        lambda: forgery.records_tuples(N, schema),
        None,  # No direct Faker equivalent
        results,
        "records_tuples",
    )

    # Records Arrow (Phase 3.3)
    try:
        import pyarrow

        _ = pyarrow  # Silence unused import warning
        forgery.seed(42)
        run_benchmark(
            "Records Arrow (6-field schema)",
            lambda: forgery.records_arrow(N, schema),
            None,  # No Faker equivalent
            results,
            "records_arrow",
        )
    except ImportError:
        print("PyArrow not installed - skipping records_arrow benchmark\n")

    # ==========================================================================
    # Async Generation (Phase 3.4)
    # ==========================================================================
    print("=" * 60)
    print("ASYNC GENERATION")
    print("=" * 60 + "\n")

    import asyncio

    # Helper to run async benchmark
    def bench_async(
        name: str, coro_func: Callable[[], object], iterations: int = 3
    ) -> float:
        """Run an async benchmark and return the best time."""
        times = []
        for _ in range(iterations):
            start = time.perf_counter()
            asyncio.run(coro_func())  # type: ignore[arg-type]
            elapsed = time.perf_counter() - start
            times.append(elapsed)
        best = min(times)
        print(f"  {name}: {best:.3f}s")
        return best

    # Compare sync vs async records generation
    print("Sync vs Async Overhead (records):")
    forgery.seed(42)
    sync_time = bench(f"forgery.records({N})", lambda: forgery.records(N, schema))

    async def async_records() -> list[dict[str, object]]:
        return await forgery.records_async(N, schema)

    forgery.seed(42)
    async_time = bench_async(f"forgery.records_async({N})", async_records)
    overhead = ((async_time / sync_time) - 1) * 100
    print(f"  Async overhead: {overhead:+.1f}%\n")

    results["records_sync"] = {"forgery": sync_time}
    results["records_async"] = {"forgery": async_time}

    # Records Arrow async
    try:
        import pyarrow

        _ = pyarrow  # Silence unused import warning
        print("Sync vs Async Overhead (records_arrow):")
        forgery.seed(42)
        sync_arrow_time = bench(
            f"forgery.records_arrow({N})", lambda: forgery.records_arrow(N, schema)
        )

        async def async_records_arrow() -> object:
            return await forgery.records_arrow_async(N, schema)

        forgery.seed(42)
        async_arrow_time = bench_async(
            f"forgery.records_arrow_async({N})", async_records_arrow
        )
        overhead = ((async_arrow_time / sync_arrow_time) - 1) * 100
        print(f"  Async overhead: {overhead:+.1f}%\n")

        results["records_arrow_sync"] = {"forgery": sync_arrow_time}
        results["records_arrow_async"] = {"forgery": async_arrow_time}
    except ImportError:
        print("PyArrow not installed - skipping async Arrow benchmark\n")

    # ==========================================================================
    # Summary
    # ==========================================================================
    if has_faker:
        print("=" * 70)
        print("SUMMARY")
        print("=" * 70)
        print(f"{'Operation':<20} {'Faker':>12} {'forgery':>12} {'Speedup':>12}")
        print("-" * 70)

        total_faker = 0.0
        total_forgery = 0.0

        for op, times in results.items():
            faker_t = times.get("faker", 0)
            forgery_t = times["forgery"]
            speedup = faker_t / forgery_t if faker_t > 0 else 0
            total_faker += faker_t
            total_forgery += forgery_t
            print(f"{op:<20} {faker_t:>11.3f}s {forgery_t:>11.3f}s {speedup:>11.1f}x")

        print("-" * 70)
        avg_speedup = total_faker / total_forgery if total_forgery > 0 else 0
        print(f"{'TOTAL':<20} {total_faker:>11.3f}s {total_forgery:>11.3f}s {avg_speedup:>11.1f}x")
        print()

        # Find best and worst speedups
        speedups = [
            (op, times.get("faker", 0) / times["forgery"])
            for op, times in results.items()
            if times.get("faker", 0) > 0
        ]
        speedups.sort(key=lambda x: x[1], reverse=True)

        print("Top 5 speedups:")
        for op, speedup in speedups[:5]:
            print(f"  {op}: {speedup:.1f}x")

        print("\nLowest 5 speedups:")
        for op, speedup in speedups[-5:]:
            print(f"  {op}: {speedup:.1f}x")


if __name__ == "__main__":
    main()
