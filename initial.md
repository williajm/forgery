# forgery — Project Bootstrap Prompt

## Mission

Build **forgery**: a high-performance fake data generation library for Python, powered by Rust. The goal is to be the fastest fake data generator in the Python ecosystem while maintaining API familiarity for Faker users.

Target: **10-100x faster than Faker** for batch operations, with a clean Pythonic API.

---

## Project Identity

- **Name**: forgery
- **Tagline**: "Fake data at the speed of Rust"
- **PyPI**: `pip install forgery`
- **License**: MIT
- **Min Python**: 3.9+
- **Rust Edition**: 2021

---

## Core Design Principles

### 1. Batch-First Architecture
The fundamental insight is that Faker's per-call overhead is the bottleneck. forgery treats batch generation as the primary use case:

```python
# `fake` is a pre-instantiated Faker("en_US") for convenience
from forgery import fake

names = fake.names(10_000)           # Returns list[str] in one call
emails = fake.emails(10_000)         # All generated in Rust, returned as list
rows = fake.records(10_000, {        # Generate structured data
    "name": "name",
    "email": "email",
    "age": ("int", 18, 65),
})

# Secondary API — single values (convenience, still fast)
name = fake.name()
email = fake.email()

# Module-level aliases for quick access (use default `fake` instance)
from forgery import names, emails
names(100)  # Equivalent to fake.names(100)
```

### 2. Zero-Copy Where Possible
- Use PyO3's efficient string handling
- Return Python lists directly from Rust vectors
- Consider Arrow/numpy array outputs for numerical data

### 3. Faker-Compatible Provider Names
Users migrating from Faker should recognize the vocabulary:
- `name`, `first_name`, `last_name`
- `email`, `safe_email`, `free_email`
- `address`, `street_address`, `city`, `country`
- `phone_number`
- `date`, `date_of_birth`, `date_time`
- `text`, `sentence`, `paragraph`
- `uuid4`, `md5`, `sha256`
- `credit_card_number`, `iban`
- `company`, `job`, `catch_phrase`
- `url`, `domain_name`, `ipv4`, `ipv6`, `mac_address`
- `color`, `hex_color`, `rgb_color`

### 4. Deterministic Seeding
Reproducibility is critical for testing:

```python
from forgery import fake, seed

seed(42)
data1 = fake.names(100)

seed(42)
data2 = fake.names(100)

assert data1 == data2  # Always true
```

**Seeding Contract:**
- `seed(n)` affects the default `fake` instance only
- Each `Faker` instance has its own independent RNG state
- `faker_instance.seed(n)` seeds a specific instance
- **Single-threaded determinism only**: results are reproducible within one thread; multi-threaded usage may interleave and produce non-deterministic ordering
- **No cross-version guarantee**: output may differ between forgery versions (we reserve the right to improve generators)

### 5. Locale Support
Start with `en_US` as default, architecture should support locales:

```python
from forgery import Faker

fake_de = Faker("de_DE")
fake_de.name()  # German names
```

---

## Technical Architecture

### Project Layout (Single Root Crate)

```
forgery/
├── Cargo.toml              # Rust package config
├── pyproject.toml          # Python package config
├── src/                    # Rust source
│   ├── lib.rs              # PyO3 module definition
│   ├── rng.rs              # Per-instance RNG, seeding
│   ├── providers/
│   │   ├── mod.rs
│   │   ├── names.rs        # Name generation
│   │   ├── internet.rs     # Email, URL, IP
│   │   ├── address.rs      # Address components
│   │   ├── datetime.rs     # Date/time generation
│   │   ├── text.rs         # Lorem ipsum, sentences
│   │   ├── numbers.rs      # Integers, floats, ranges
│   │   ├── identifiers.rs  # UUID, hashes
│   │   └── finance.rs      # Credit cards, IBAN
│   └── data/
│       ├── mod.rs
│       ├── en_us/          # Embedded data files
│       │   ├── first_names.rs
│       │   ├── last_names.rs
│       │   ├── cities.rs
│       │   └── ...
│       └── locales.rs      # Locale registry
├── python/
│   └── forgery/
│       ├── __init__.py     # Public API, re-exports, default `fake` instance
│       ├── _forgery.pyi    # Type stubs for Rust module
│       ├── py.typed        # PEP 561 marker
│       └── compat.py       # Optional Faker compatibility layer
└── tests/                  # Python tests (root level)
    ├── test_providers.py
    ├── test_batch.py
    ├── test_seeding.py
    └── benchmarks/
        └── bench_vs_faker.py
```

### Build System

Use **maturin** for building:

```toml
# pyproject.toml
[build-system]
requires = ["maturin>=1.4,<2.0"]
build-backend = "maturin"

[project]
name = "forgery"
description = "Fake data at the speed of Rust"
readme = "README.md"
license = { text = "MIT" }
requires-python = ">=3.9"
classifiers = [
    "Development Status :: 4 - Beta",
    "Intended Audience :: Developers",
    "License :: OSI Approved :: MIT License",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.9",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
    "Programming Language :: Rust",
    "Topic :: Software Development :: Testing",
]
keywords = ["faker", "fake", "data", "testing", "mock", "rust"]

[project.urls]
Homepage = "https://github.com/USERNAME/forgery"
Documentation = "https://forgery.readthedocs.io"
Repository = "https://github.com/USERNAME/forgery"

[tool.maturin]
features = ["pyo3/extension-module"]
python-source = "python"
module-name = "forgery._forgery"
```

```toml
# Cargo.toml
[package]
name = "forgery"
version = "0.1.0"
edition = "2021"

[lib]
name = "_forgery"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.22", features = ["extension-module"] }
rand = "0.8"
rand_chacha = "0.3"  # For deterministic seeding (ChaCha8 is fast enough)
uuid = { version = "1.0", features = ["v4", "fast-rng"] }
chrono = "0.4"

[dev-dependencies]
criterion = "0.5"      # Benchmarking
proptest = "1.4"       # Property-based testing

[[bench]]
name = "generators"
harness = false

[profile.release]
lto = true
codegen-units = 1
opt-level = 3
```

---

## API Specification

### Module: `forgery`

```python
# Core functions — batch generation (returns list)
def names(n: int) -> list[str]: ...
def first_names(n: int) -> list[str]: ...
def last_names(n: int) -> list[str]: ...
def emails(n: int) -> list[str]: ...
def safe_emails(n: int) -> list[str]: ...
def phone_numbers(n: int) -> list[str]: ...
def addresses(n: int) -> list[str]: ...
def cities(n: int) -> list[str]: ...
def countries(n: int) -> list[str]: ...
def dates(n: int, start: str = "1970-01-01", end: str = "2030-12-31") -> list[str]: ...
def uuids(n: int) -> list[str]: ...
def integers(n: int, min: int = 0, max: int = 100) -> list[int]: ...
def floats(n: int, min: float = 0.0, max: float = 1.0) -> list[float]: ...
def sentences(n: int, words: int = 6) -> list[str]: ...
def paragraphs(n: int, sentences: int = 3) -> list[str]: ...

# Convenience functions — single value (calls batch with n=1)
def name() -> str: ...
def first_name() -> str: ...
def last_name() -> str: ...
def email() -> str: ...
# ... etc

# Structured data generation
def records(n: int, schema: dict[str, str | tuple]) -> list[dict]: ...
def records_tuples(n: int, schema: dict[str, str | tuple]) -> list[tuple]: ...  # Faster than dicts
def records_arrow(n: int, schema: dict[str, str | tuple]) -> "pyarrow.Table": ...  # Phase 3

# Seeding (affects default `fake` instance only)
def seed(value: int) -> None: ...

# Locale-aware generator (each instance has independent RNG)
class Faker:
    def __init__(self, locale: str = "en_US") -> None: ...
    def seed(self, value: int) -> None: ...  # Seed this instance
    # All the same batch/single methods as module-level

# Default instance (module-level functions delegate to this)
fake: Faker
```

### Schema DSL for `records()`

```python
# Simple field types
schema = {
    "id": "uuid",
    "name": "name",
    "email": "email",
}

# Parameterized types
schema = {
    "age": ("int", 18, 65),           # (type, min, max)
    "salary": ("float", 30000, 150000),
    "hire_date": ("date", "2020-01-01", "2024-12-31"),
    "bio": ("text", 50, 200),         # (type, min_chars, max_chars)
}

# Choice from list
schema = {
    "status": ("choice", ["active", "inactive", "pending"]),
    "department": ("choice", ["engineering", "sales", "hr"]),
}
```

---

## Performance Targets

**Primary KPI: Speedup vs Faker** (not absolute times, which depend on hardware and Python allocation overhead)

Benchmark against Faker on generating 100,000 records:

| Operation | Target Speedup |
|-----------|----------------|
| names | 50-100x |
| emails | 50-100x |
| integers | 100-200x |
| records (5 fields) → list[dict] | 20-50x |
| records_arrow (5 fields) → Arrow | 50-100x |

**Note:** `list[dict]` output is Python-object-bound; for maximum throughput on large batches, use Arrow/Polars outputs (Phase 3).

### Benchmark Script (include in repo)

```python
# benchmarks/bench_vs_faker.py
import time
from faker import Faker as OriginalFaker
import forgery

N = 100_000

def bench_faker():
    fake = OriginalFaker()
    start = time.perf_counter()
    names = [fake.name() for _ in range(N)]
    return time.perf_counter() - start

def bench_forgery():
    start = time.perf_counter()
    names = forgery.names(N)
    return time.perf_counter() - start

faker_time = bench_faker()
forgery_time = bench_forgery()

print(f"Faker:   {faker_time:.3f}s")
print(f"forgery: {forgery_time:.3f}s")
print(f"Speedup: {faker_time / forgery_time:.1f}x")
```

---

## Implementation Priorities

### Phase 1: Core (MVP)
1. Project scaffolding with maturin
2. RNG infrastructure with seeding (per-instance, deterministic)
3. Basic providers: names, emails, integers, uuids
4. Batch API + single-value convenience methods
5. Comprehensive tests (Rust unit tests + Python integration tests)
6. CI/CD with GitHub Actions (build wheels for Linux/macOS/Windows)
7. Branch protection ruleset configured

### Phase 2: Feature Parity
1. All major Faker providers
2. `records()` and `records_tuples()` structured generation
3. Type stubs (.pyi) + py.typed marker
4. Documentation

### Phase 3: Advanced
1. Locale support
2. Custom providers API
3. `records_arrow()` for Arrow/Polars integration (high-throughput path)
4. Async generation for very large batches

---

## Code Quality Standards

**Philosophy: No shortcuts. Exemplary implementation. Code that others can learn from.**

### Rust

**Code Quality**
- Idiomatic Rust: use iterators, proper error handling, no unnecessary `unwrap()`
- Clear module boundaries with single responsibility
- No clever hacks — prefer readable over clever
- `cargo clippy -- -D warnings` (deny all warnings)
- `cargo fmt` enforced (CI fails on unformatted code)
- No `unsafe` unless absolutely necessary and thoroughly documented

**Testing**
- Unit tests for every public function
- Property-based tests using `proptest` for generators (e.g., verify email format validity)
- Edge case coverage: empty inputs, maximum values, unicode handling
- Benchmarks using `criterion` for performance regression detection

**Documentation**
- Rustdoc for all public items with examples
- Module-level documentation explaining purpose and design decisions
- `#![deny(missing_docs)]` enforced

### Python

**Code Quality**
- Type hints everywhere — no `Any` unless unavoidable
- `ruff` for linting (strict mode)
- `mypy --strict` passes
- Clear, Pythonic API design

**Testing**
- `pytest` with 90%+ coverage target
- Test seeding determinism explicitly
- Test batch vs single-value consistency
- Integration tests verifying Rust bindings work correctly
- Regression tests added for every bug fix

**Documentation**
- Docstrings for all public functions (Google style)
- Usage examples in docstrings
- Type stubs (.pyi) complete and accurate

### Performance

**Benchmarking**
- Benchmark suite comparing against Faker (the primary KPI)
- Benchmarks run in CI to detect performance regressions
- Profile before optimizing — measure, don't guess
- Document any non-obvious performance optimizations

**Optimization Principles**
- Batch operations are the fast path — optimize these first
- Minimize Python ↔ Rust boundary crossings
- Prefer stack allocation over heap where possible
- Use `#[inline]` judiciously based on benchmark data
- Pre-allocate vectors when size is known

### Documentation

**README**
- Clear quick start (install → first use in <30 seconds)
- Performance comparison with methodology
- Migration guide from Faker with code examples

**API Reference**
- Auto-generated from docstrings/rustdoc
- Every function documented with parameters, return types, examples
- Edge cases and error conditions documented

**Architecture**
- ARCHITECTURE.md explaining design decisions
- Why batch-first, why these RNG choices, why this module structure

---

## CI/CD & Quality Gates

### GitHub Actions Workflows

**`.github/workflows/ci.yml`** — Runs on every PR and push to main
```yaml
Jobs:
  rust-check:
    - cargo fmt --check
    - cargo clippy -- -D warnings
    - cargo test
    - cargo bench (baseline comparison)

  python-check:
    - maturin develop
    - ruff check
    - ruff format --check
    - mypy --strict python/
    - pytest --cov=forgery --cov-fail-under=90

  security:
    - cargo audit (dependency vulnerabilities)
    - bandit (Python security scan)

  codeql:
    - Static analysis for Rust and Python
```

**`.github/workflows/release.yml`** — Runs on version tags
```yaml
Jobs:
  build-wheels:
    - Build wheels for Linux (x86_64, aarch64), macOS (x86_64, arm64), Windows (x86_64)
    - Upload to PyPI
```

**`.github/workflows/dependency-review.yml`** — Runs on PRs
```yaml
Jobs:
  - Check for vulnerable dependencies
  - License compatibility check
```

### Branch Protection Ruleset

Configure via GitHub Settings → Rules → Rulesets:

**Target:** `main` branch

**Rules:**
- Require pull request before merging
- Require status checks to pass:
  - `rust-check`
  - `python-check`
  - `security`
  - `codeql`
- Require conversation resolution
- Block force pushes
- Require signed commits (optional but recommended)

### Quality Gate Summary

| Gate | Tool | Threshold |
|------|------|-----------|
| Rust formatting | `cargo fmt` | No diff |
| Rust linting | `cargo clippy` | Zero warnings |
| Rust tests | `cargo test` | 100% pass |
| Python formatting | `ruff format` | No diff |
| Python linting | `ruff check` | Zero errors |
| Type checking | `mypy --strict` | Zero errors |
| Test coverage | `pytest --cov` | ≥90% |
| Security (Rust) | `cargo audit` | No vulnerabilities |
| Security (Python) | `bandit` | No high/critical |
| Static analysis | CodeQL | No high/critical |

### Pre-commit Hooks (Optional Local Enforcement)

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt --
        language: system
        types: [rust]
      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy -- -D warnings
        language: system
        types: [rust]
  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.8.0
    hooks:
      - id: ruff
      - id: ruff-format
  - repo: https://github.com/pre-commit/mirrors-mypy
    rev: v1.13.0
    hooks:
      - id: mypy
        args: [--strict]
```

---

## Repository Structure

```
forgery/
├── .github/
│   └── workflows/
│       ├── ci.yml              # Test on PR (Rust + Python tests, linting, type checking)
│       ├── release.yml         # Build and publish wheels
│       └── dependency-review.yml
├── .pre-commit-config.yaml     # Local quality enforcement
├── src/                        # Rust source
│   ├── lib.rs
│   ├── rng.rs
│   └── providers/
├── benches/                    # Rust benchmarks (criterion)
│   └── generators.rs
├── python/
│   └── forgery/
│       ├── __init__.py         # Public API, default `fake` instance
│       ├── _forgery.pyi        # Type stubs
│       └── py.typed            # PEP 561 marker
├── tests/                      # Python tests (root level)
│   ├── test_providers.py
│   ├── test_seeding.py
│   ├── test_batch.py
│   └── benchmarks/
│       └── bench_vs_faker.py
├── Cargo.toml
├── pyproject.toml
├── README.md
├── ARCHITECTURE.md             # Design decisions documentation
├── LICENSE
└── CHANGELOG.md
```

---

## Getting Started Commands

```bash
# Development setup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
pip install maturin

# Build and install locally
maturin develop --release

# Run tests
pytest tests/

# Run benchmarks
python tests/benchmarks/bench_vs_faker.py

# Build wheels
maturin build --release
```

---

## Success Criteria

1. **Performance**: Demonstrably 10-100x faster than Faker in benchmarks
2. **Correctness**: All generated data passes validation (valid emails, UUIDs, etc.)
3. **Ergonomics**: API is intuitive, well-typed, well-documented
4. **Reliability**: Deterministic seeding works correctly
5. **Packaging**: Clean install via `pip install forgery`, wheels for all major platforms

---

## Reference Implementation Inspiration

Study these for patterns:
- **orjson**: How they structure PyO3 bindings for performance
- **pydantic-core**: Rust validation with Python ergonomics  
- **polars**: Batch-first API design, excellent PyO3 usage

---

## First Task

Bootstrap the project structure:
1. Initialize the repository with the directory structure above
2. Set up Cargo.toml and pyproject.toml
3. Implement a minimal proof-of-concept: `forgery.names(n)` that generates n random names
4. Verify it builds and runs
5. Write a simple benchmark comparing to Faker

Let's build the fastest fake data generator Python has ever seen.

