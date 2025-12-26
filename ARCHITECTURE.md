# Architecture

This document describes the architecture of the forgery library.

## Overview

Forgery is a high-performance fake data generation library for Python, powered by Rust. It uses PyO3 to create Python bindings for Rust code, enabling 50-100x performance improvements over pure Python implementations.

## Directory Structure

```
forgery/
├── src/                    # Rust source code
│   ├── lib.rs              # PyO3 module entry point, Faker class
│   ├── rng.rs              # RNG wrapper (ChaCha8)
│   ├── data/               # Static data for generation
│   │   ├── mod.rs
│   │   └── en_us/          # US English locale data
│   │       ├── mod.rs
│   │       ├── first_names.rs
│   │       └── last_names.rs
│   └── providers/          # Data generation providers
│       ├── mod.rs
│       ├── names.rs        # Name generation
│       ├── internet.rs     # Email generation
│       ├── numbers.rs      # Integer generation
│       └── identifiers.rs  # UUID generation
├── python/                 # Python source code
│   └── forgery/
│       ├── __init__.py     # Public API, module-level functions
│       ├── __init__.pyi    # Type stubs for the package
│       ├── _forgery.pyi    # Type stubs for the Rust extension
│       └── py.typed        # PEP 561 marker
├── tests/                  # Python tests
│   ├── conftest.py
│   ├── test_providers.py
│   ├── test_seeding.py
│   ├── test_batch.py
│   └── benchmarks/
│       └── bench_vs_faker.py
├── benches/                # Rust benchmarks (Criterion)
│   └── generators.rs
└── .github/workflows/      # CI/CD
    ├── ci.yml
    ├── release.yml
    └── dependency-review.yml
```

## Core Components

### RNG Layer (`src/rng.rs`)

The `ForgeryRng` struct wraps ChaCha8Rng for deterministic, seedable random number generation:

- **ChaCha8**: Fast CSPRNG with good statistical properties
- **Per-instance state**: Each `Faker` instance has its own RNG
- **Seedable**: Calling `seed(n)` resets the RNG to a deterministic state

### Providers (`src/providers/`)

Each provider module generates a specific type of fake data:

- **names.rs**: First names, last names, full names
- **internet.rs**: Email addresses
- **numbers.rs**: Integers within ranges
- **identifiers.rs**: UUIDs (version 4)

All providers follow the same pattern:
1. Accept an `&mut ForgeryRng` for randomness
2. Accept a count `n` for batch size
3. Return `Vec<T>` or `Result<Vec<T>, Error>`

### Data Layer (`src/data/`)

Static data organized by locale:

- **en_us/**: US English locale
  - `first_names.rs`: ~200 first names
  - `last_names.rs`: ~170 last names

Each data file includes tests for uniqueness and non-empty values.

### Python Bindings (`src/lib.rs`)

The `Faker` class is exposed to Python via PyO3:

```rust
#[pyclass]
pub struct Faker {
    rng: ForgeryRng,
    locale: String,
}
```

Key design decisions:
- Batch-first API: All generators accept `n` parameter
- Single-value convenience: Methods like `name()` call `names(1)`
- Error handling: Returns `PyResult<T>` for validation errors

### Python Wrapper (`python/forgery/__init__.py`)

The Python layer provides:
- A default `fake` instance for convenience
- Module-level functions that delegate to `fake`
- Type hints for IDE support

## Performance Design

### Batch-First Generation

The primary performance optimization is batch generation:

```python
# Fast: single FFI call, preallocated Vec
names = fake.names(10000)

# Slow: 10000 FFI calls
names = [fake.name() for _ in range(10000)]
```

### Memory Efficiency

- `Vec::with_capacity(n)`: Preallocates output buffers
- Single allocation per batch
- No intermediate collections

### Safety Limits

- Maximum batch size: 10 million items
- Prevents memory exhaustion from accidental large requests

## Error Handling

### Rust Side

- Empty slice access: Panics with descriptive message
- Invalid integer range: Returns `Err(RangeError)`
- Batch size exceeded: Returns `Err` converted to `PyValueError`

### Python Side

- All batch methods can raise `ValueError`
- Error messages include the invalid values

## Thread Safety

**Important**: Forgery is NOT thread-safe.

- Each `Faker` instance has mutable RNG state
- Sharing a `Faker` across threads causes non-deterministic output and potential data races
- Python's GIL serializes calls, so there's no memory unsafety from Python, but results will be unpredictable
- Create one `Faker` per thread for deterministic, reproducible output

## Testing Strategy

### Rust Tests (`cargo test`)

- Unit tests for each provider
- RNG determinism tests
- Data uniqueness tests
- Edge case tests (empty batches, boundary values)

### Python Tests (`pytest`)

- Integration tests for the Python API
- Seeding determinism tests
- Batch/single consistency tests
- Error handling tests
- Large batch performance tests

### Property-Based Testing

Proptest is available for fuzz testing (in dev-dependencies).

## CI/CD Pipeline

### Continuous Integration (`ci.yml`)

1. **rust-check**: fmt, clippy, cargo test
2. **python-check**: ruff, mypy, pytest with coverage
3. **security**: cargo audit, bandit
4. **codeql**: Static analysis for Python

### Release (`release.yml`)

1. Build wheels for Linux, macOS, Windows
2. Build source distribution
3. Publish to PyPI using trusted publishing

## Future Considerations

- Additional locales (de_DE, fr_FR, etc.)
- More providers (addresses, phone numbers, dates)
- Arrow record batch output for analytics
- Parallel generation within batches
