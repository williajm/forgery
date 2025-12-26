# forgery

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

| Batch | Single | Description |
|-------|--------|-------------|
| `names(n)` | `name()` | Full names (first + last) |
| `first_names(n)` | `first_name()` | First names |
| `last_names(n)` | `last_name()` | Last names |
| `emails(n)` | `email()` | Email addresses |
| `integers(n, min, max)` | `integer(min, max)` | Random integers |
| `uuids(n)` | `uuid()` | UUID v4 strings |

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
