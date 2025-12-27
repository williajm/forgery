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
│   │       ├── last_names.rs
│   │       ├── color_names.rs
│   │       ├── lorem.rs
│   │       ├── streets.rs
│   │       ├── cities.rs
│   │       ├── states.rs
│   │       ├── countries.rs
│   │       ├── companies.rs
│   │       └── tlds.rs
│   └── providers/          # Data generation providers
│       ├── mod.rs
│       ├── names.rs        # Name generation
│       ├── internet.rs     # Email generation (standard, safe, free)
│       ├── numbers.rs      # Integer and float generation
│       ├── identifiers.rs  # UUID, MD5, SHA256 generation
│       ├── colors.rs       # Color names, hex, RGB
│       ├── datetime.rs     # Date, datetime, date of birth
│       ├── text.rs         # Sentence, paragraph, text blocks
│       ├── address.rs      # Street, city, state, country, zip, full
│       ├── phone.rs        # Phone number generation
│       ├── company.rs      # Company names, jobs, catch phrases
│       ├── network.rs      # URL, domain, IPv4, IPv6, MAC address
│       ├── finance.rs      # Credit cards (Luhn), IBANs
│       └── records.rs      # Structured data with schema DSL
├── python/                 # Python source code
│   └── forgery/
│       ├── __init__.py     # Public API, module-level functions
│       ├── __init__.pyi    # Type stubs for the package
│       ├── _forgery.pyi    # Type stubs for the Rust extension
│       └── py.typed        # PEP 561 marker
├── tests/                  # Python tests
│   ├── conftest.py
│   ├── test_basic.py
│   ├── test_providers.py
│   ├── test_phase2_providers.py
│   ├── test_seeding.py
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

| Provider | Functions | Description |
|----------|-----------|-------------|
| names.rs | name, first_name, last_name | Full and partial names |
| internet.rs | email, safe_email, free_email | Email addresses |
| numbers.rs | integer, float | Numeric values in ranges |
| identifiers.rs | uuid, md5, sha256 | Unique identifiers and hashes |
| colors.rs | color, hex_color, rgb_color | Color names and values |
| datetime.rs | date, datetime, date_of_birth | Dates and timestamps |
| text.rs | sentence, paragraph, text | Lorem ipsum text |
| address.rs | street_address, city, state, country, zip_code, address | Location data |
| phone.rs | phone_number | Phone numbers |
| company.rs | company, job, catch_phrase | Business data |
| network.rs | url, domain_name, ipv4, ipv6, mac_address | Network identifiers |
| finance.rs | credit_card, iban | Financial identifiers with valid checksums |
| records.rs | records | Structured data from schema DSL |

All providers follow the same pattern:
1. Accept an `&mut ForgeryRng` for randomness
2. Accept a count `n` for batch size
3. Return `Vec<T>` or `Result<Vec<T>, Error>`

### Data Layer (`src/data/`)

Static data organized by locale, embedded at compile time as `&'static [&str]`:

- **en_us/**: US English locale
  - `first_names.rs`: ~200 first names
  - `last_names.rs`: ~170 last names
  - `color_names.rs`: ~140 color names
  - `lorem.rs`: ~200 lorem ipsum words
  - `streets.rs`: ~100 street names
  - `cities.rs`: ~200 US cities
  - `states.rs`: 50 US states
  - `countries.rs`: ~200 countries
  - `companies.rs`: Company name components
  - `tlds.rs`: ~20 top-level domains

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

## Provider Implementation Pattern

Each provider follows this structure:

```rust
// src/providers/example.rs

use crate::rng::ForgeryRng;
use crate::validate_batch_size;
use crate::BatchSizeError;

/// Generate a single item.
pub fn generate_item(rng: &mut ForgeryRng) -> String {
    generate_items(rng, 1).pop().unwrap()
}

/// Generate a batch of items.
pub fn generate_items(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut results = Vec::with_capacity(n);
    for _ in 0..n {
        // Generation logic
        results.push(item);
    }
    results
}

/// Validate and generate a batch.
pub fn generate_items_validated(
    rng: &mut ForgeryRng,
    n: usize,
) -> Result<Vec<String>, BatchSizeError> {
    validate_batch_size(n)?;
    Ok(generate_items(rng, n))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_generates_valid_output() { ... }

    proptest! {
        #[test]
        fn prop_always_valid(seed: u64) { ... }
    }
}
```

## Checksum Algorithms

### Luhn Algorithm (Credit Cards)

Used to generate valid credit card numbers:

```rust
fn luhn_checksum(number: &str) -> u8 {
    let mut sum: u32 = 0;
    let mut double = true;  // Start doubling from rightmost digit

    for c in number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            let mut d = digit;
            if double {
                d *= 2;
                if d > 9 { d -= 9; }
            }
            sum += d;
            double = !double;
        }
    }
    ((10 - (sum % 10)) % 10) as u8
}
```

### ISO 7064 Mod 97-10 (IBAN)

Used to generate valid IBAN check digits:

```rust
fn iban_checksum(country: &str, bban: &str) -> String {
    // Rearrange: BBAN + country code + "00"
    // Convert letters to numbers (A=10, B=11, ...)
    // Calculate: 98 - (numeric mod 97)
    format!("{:02}", 98 - remainder)
}
```

## Schema DSL for Structured Data

The `records()` method generates structured data from a Python dict schema:

```python
schema = {
    "id": "uuid",
    "name": "name",
    "age": ("integer", 18, 65),
    "salary": ("float", 30000.0, 150000.0),
    "status": ("choice", ["active", "inactive"]),
}

records = fake.records(1000, schema)
# Returns list of dicts with consistent schema
```

Supported types:
- **Simple**: `uuid`, `name`, `first_name`, `last_name`, `email`, `safe_email`, `free_email`
- **Simple**: `city`, `state`, `country`, `zip_code`, `street_address`, `address`
- **Simple**: `phone_number`, `company`, `job`, `catch_phrase`
- **Simple**: `url`, `domain_name`, `ipv4`, `ipv6`, `mac_address`
- **Simple**: `date`, `datetime`, `credit_card`, `iban`
- **Simple**: `color`, `hex_color`, `sentence`, `paragraph`, `text`
- **Simple**: `md5`, `sha256`
- **Parameterized**: `("integer", min, max)`, `("float", min, max)`
- **Parameterized**: `("choice", [options])`

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
- Static embedded data: Zero runtime I/O

### Safety Limits

- Maximum batch size: 10 million items
- Prevents memory exhaustion from accidental large requests

## Error Handling

### Rust Side

- Empty slice access: Panics with descriptive message
- Invalid integer range: Returns `Err(RangeError)`
- Invalid date range: Returns `Err(DateRangeError)`
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
- Property-based tests with proptest

### Python Tests (`pytest`)

- Integration tests for the Python API
- Seeding determinism tests
- Batch/single consistency tests
- Error handling tests
- Checksum validation tests (Luhn, IBAN)
- Large batch performance tests

## CI/CD Pipeline

### Continuous Integration (`ci.yml`)

1. **rust-check**: fmt, clippy, cargo test
2. **python-check**: ruff, mypy, pytest with coverage (90% minimum)
3. **security**: cargo audit, bandit
4. **codeql**: Static analysis for Python

### Release (`release.yml`)

1. Build wheels for Linux, macOS, Windows
2. Build source distribution
3. Publish to PyPI using trusted publishing

## Adding a New Provider

1. Create `src/providers/new_provider.rs`
2. Add `pub mod new_provider;` to `src/providers/mod.rs`
3. Add data file to `src/data/en_us/` if needed
4. Export data in `src/data/en_us/mod.rs`
5. Add Rust API methods to `impl Faker` in `src/lib.rs`
6. Add Python API methods to `#[pymethods] impl Faker` in `src/lib.rs`
7. Add convenience functions to `python/forgery/__init__.py`
8. Add type stubs to `.pyi` files
9. Add tests to `tests/`

## Dependencies

### Rust
- `pyo3`: Python bindings
- `rand_chacha`: ChaCha8 PRNG
- `rand`: Random number generation traits
- `chrono`: Date/time handling
- `uuid`: UUID generation

### Python (dev)
- `maturin`: Build system
- `pytest`: Testing
- `pytest-cov`: Coverage
- `mypy`: Type checking
- `ruff`: Linting/formatting

## Future Considerations

- Additional locales (de_DE, fr_FR, etc.)
- Arrow record batch output for analytics
- Parallel generation within batches
