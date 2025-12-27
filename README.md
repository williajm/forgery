# forgery

[![CI](https://github.com/williajm/forgery/actions/workflows/ci.yml/badge.svg)](https://github.com/williajm/forgery/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/williajm/forgery/branch/main/graph/badge.svg)](https://codecov.io/gh/williajm/forgery)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python 3.11+](https://img.shields.io/badge/python-3.11+-blue.svg)](https://www.python.org/downloads/)
[![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)

**Fake data at the speed of Rust.**

A high-performance fake data generation library for Python, powered by Rust. Designed to be 50-100x faster than Faker for batch operations.

## Installation

```bash
pip install forgery
```

## Quick Start

```python
from forgery import fake

# Generate 10,000 names in one fast call
names = fake.names(10_000)

# Single values work too
email = fake.email()
name = fake.name()

# Deterministic output with seeding
fake.seed(42)
data1 = fake.names(100)
fake.seed(42)
data2 = fake.names(100)
assert data1 == data2
```

## Features

- **Batch-first design**: Generate thousands of values in a single call
- **50-100x faster** than Faker for batch operations
- **Deterministic seeding**: Reproducible output for testing
- **Type hints**: Full type stub support for IDE autocompletion
- **Familiar API**: Method names match Faker for easy migration

## API

### Module-level functions (use default instance)

```python
from forgery import seed, names, emails, integers, uuids

seed(42)  # Seed for reproducibility

# Batch generation (fast path)
names(1000)           # list[str] of full names
emails(1000)          # list[str] of email addresses
integers(1000, 0, 100)  # list[int] in range
uuids(1000)           # list[str] of UUIDv4

# Single values
name()                # str
email()               # str
integer(0, 100)       # int
uuid()                # str
```

### Faker class (independent instances)

```python
from forgery import Faker

# Each instance has its own RNG state
fake1 = Faker()
fake2 = Faker()

fake1.seed(42)
fake2.seed(99)

# Generate independently
fake1.names(100)
fake2.emails(100)
```

## Available Generators

### Names & Identity

| Batch | Single | Description |
|-------|--------|-------------|
| `names(n)` | `name()` | Full names (first + last) |
| `first_names(n)` | `first_name()` | First names |
| `last_names(n)` | `last_name()` | Last names |

### Contact Information

| Batch | Single | Description |
|-------|--------|-------------|
| `emails(n)` | `email()` | Email addresses |
| `safe_emails(n)` | `safe_email()` | Safe domain emails (@example.com, etc.) |
| `free_emails(n)` | `free_email()` | Free provider emails (@gmail.com, etc.) |
| `phone_numbers(n)` | `phone_number()` | Phone numbers in (XXX) XXX-XXXX format |

### Numbers & Identifiers

| Batch | Single | Description |
|-------|--------|-------------|
| `integers(n, min, max)` | `integer(min, max)` | Random integers in range |
| `floats(n, min, max)` | `float_(min, max)` | Random floats in range (Note: `float_` avoids shadowing Python's `float` builtin) |
| `uuids(n)` | `uuid()` | UUID v4 strings |
| `md5s(n)` | `md5()` | Random 32-char hex strings (MD5-like format, not cryptographic hashes) |
| `sha256s(n)` | `sha256()` | Random 64-char hex strings (SHA256-like format, not cryptographic hashes) |

### Dates & Times

| Batch | Single | Description |
|-------|--------|-------------|
| `dates(n, start, end)` | `date(start, end)` | Random dates (YYYY-MM-DD) |
| `datetimes(n, start, end)` | `datetime_(start, end)` | Random datetimes (ISO 8601). Note: `datetime_` avoids shadowing Python's `datetime` module |
| `dates_of_birth(n, min_age, max_age)` | `date_of_birth(min_age, max_age)` | Birth dates for given age range |

### Addresses

| Batch | Single | Description |
|-------|--------|-------------|
| `street_addresses(n)` | `street_address()` | Street addresses (e.g., "123 Main Street") |
| `cities(n)` | `city()` | City names |
| `states(n)` | `state()` | State names |
| `countries(n)` | `country()` | Country names |
| `zip_codes(n)` | `zip_code()` | ZIP codes (5 or 9 digit) |
| `addresses(n)` | `address()` | Full addresses |

### Company & Business

| Batch | Single | Description |
|-------|--------|-------------|
| `companies(n)` | `company()` | Company names |
| `jobs(n)` | `job()` | Job titles |
| `catch_phrases(n)` | `catch_phrase()` | Business catch phrases |

### Network

| Batch | Single | Description |
|-------|--------|-------------|
| `urls(n)` | `url()` | URLs with https:// |
| `domain_names(n)` | `domain_name()` | Domain names |
| `ipv4s(n)` | `ipv4()` | IPv4 addresses |
| `ipv6s(n)` | `ipv6()` | IPv6 addresses |
| `mac_addresses(n)` | `mac_address()` | MAC addresses |

### Finance

| Batch | Single | Description |
|-------|--------|-------------|
| `credit_cards(n)` | `credit_card()` | Credit card numbers (valid Luhn) |
| `ibans(n)` | `iban()` | IBAN numbers (valid checksum) |

### Text & Lorem Ipsum

| Batch | Single | Description |
|-------|--------|-------------|
| `sentences(n, word_count)` | `sentence(word_count)` | Lorem ipsum sentences |
| `paragraphs(n, sentence_count)` | `paragraph(sentence_count)` | Lorem ipsum paragraphs |
| `texts(n, min_chars, max_chars)` | `text(min_chars, max_chars)` | Text blocks with length limits |

### Colors

| Batch | Single | Description |
|-------|--------|-------------|
| `colors(n)` | `color()` | Color names |
| `hex_colors(n)` | `hex_color()` | Hex color codes (#RRGGBB) |
| `rgb_colors(n)` | `rgb_color()` | RGB tuples (r, g, b) |

## Structured Data Generation

Generate entire datasets with a single call using schema definitions:

### records()

Returns a list of dictionaries:

```python
from forgery import records, seed

seed(42)
data = records(1000, {
    "id": "uuid",
    "name": "name",
    "email": "email",
    "age": ("int", 18, 65),
    "salary": ("float", 30000.0, 150000.0),
    "hire_date": ("date", "2020-01-01", "2024-12-31"),
    "bio": ("text", 50, 200),
    "status": ("choice", ["active", "inactive", "pending"]),
})

# data[0] = {"id": "88917925-...", "name": "Austin Bell", "age": 50, ...}
```

### records_tuples()

Returns a list of tuples (faster, values in alphabetical key order):

```python
from forgery import records_tuples, seed

seed(42)
data = records_tuples(1000, {
    "age": ("int", 18, 65),
    "name": "name",
})
# data[0] = (50, "Ryan Grant")  # (age, name) - alphabetical order
```

### Schema Field Types

| Type | Syntax | Example |
|------|--------|---------|
| Simple types | `"type_name"` | `"name"`, `"email"`, `"uuid"`, `"int"`, `"float"` |
| Integer range | `("int", min, max)` | `("int", 18, 65)` |
| Float range | `("float", min, max)` | `("float", 0.0, 100.0)` |
| Text with limits | `("text", min_chars, max_chars)` | `("text", 50, 200)` |
| Date range | `("date", start, end)` | `("date", "2020-01-01", "2024-12-31")` |
| Choice | `("choice", [options])` | `("choice", ["a", "b", "c"])` |

All simple types from the generators above are supported: `name`, `first_name`, `last_name`, `email`, `safe_email`, `free_email`, `phone`, `uuid`, `int`, `float`, `date`, `datetime`, `street_address`, `city`, `state`, `country`, `zip_code`, `address`, `company`, `job`, `catch_phrase`, `url`, `domain_name`, `ipv4`, `ipv6`, `mac_address`, `credit_card`, `iban`, `sentence`, `paragraph`, `text`, `color`, `hex_color`, `rgb_color`, `md5`, `sha256`.

## Performance

Benchmark generating 100,000 items:

```
$ python tests/benchmarks/bench_vs_faker.py

Names:
  forgery.names(): 0.015s
  Faker.name():    1.523s
  Speedup: 101.5x

Emails:
  forgery.emails(): 0.021s
  Faker.email():    2.134s
  Speedup: 101.6x
```

## Seeding Contract

- `seed(n)` affects the default `fake` instance only
- Each `Faker` instance has its own independent RNG state
- **Single-threaded determinism only**: Results are reproducible within one thread
- **No cross-version guarantee**: Output may differ between forgery versions

## Thread Safety

**forgery is NOT thread-safe.** Each `Faker` instance maintains mutable RNG state.

For multi-threaded applications, create one `Faker` instance per thread:

```python
from concurrent.futures import ThreadPoolExecutor
from forgery import Faker

def generate_names(seed: int) -> list[str]:
    fake = Faker()  # Create per-thread instance
    fake.seed(seed)
    return fake.names(1000)

with ThreadPoolExecutor(max_workers=4) as executor:
    results = list(executor.map(generate_names, range(4)))
```

Do NOT share a `Faker` instance across threads.

## Development

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install maturin
pip install maturin

# Build and install locally
maturin develop --release

# Run tests
cargo test          # Rust tests
pytest              # Python tests

# Run benchmarks
python tests/benchmarks/bench_vs_faker.py
```

## License

MIT
