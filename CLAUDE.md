# CLAUDE.md - Project Instructions for AI Assistants

## Project Overview

**forgery** is a high-performance fake data generation library for Python, powered by Rust via PyO3. Target: 50-100x faster than Faker for batch operations.

## Architecture

```
forgery/
├── src/                    # Rust source code
│   ├── lib.rs              # PyO3 module entry point, Faker class
│   ├── rng.rs              # RNG wrapper (ChaCha8)
│   ├── error.rs            # Unified error types
│   ├── locale.rs           # Locale definitions
│   ├── data/               # Static data for generation (embedded at compile time)
│   │   ├── en_us/          # US English locale data
│   │   ├── en_gb/          # UK English locale data
│   │   ├── de_de/          # German locale data
│   │   ├── fr_fr/          # French locale data
│   │   ├── es_es/          # Spanish locale data
│   │   ├── it_it/          # Italian locale data
│   │   └── ja_jp/          # Japanese locale data
│   └── providers/          # Data generation providers
│       ├── names.rs        # Name generation
│       ├── internet.rs     # Email generation
│       ├── address.rs      # Address generation
│       ├── records.rs      # Structured data generation (records/tuples)
│       ├── custom.rs       # Custom provider support
│       └── ...             # Other providers
├── python/forgery/         # Python wrapper and type stubs
├── tests/                  # Python tests (pytest)
└── benches/                # Rust benchmarks (Criterion)
```

## Key Design Principles

1. **Batch-First**: All generators accept `n` parameter for bulk generation
2. **Deterministic**: ChaCha8 RNG with seedable state per instance
3. **Faker-Compatible**: Method names match Faker for easy migration
4. **Type-Safe**: Full type stub support (PEP 561)

## Development Workflow

```bash
# Build and install locally
maturin develop --release

# Run tests
cargo test                    # Rust tests
pytest                        # Python tests (requires 90%+ coverage)

# Linting
cargo fmt --check && cargo clippy --all-targets -- -D warnings
ruff check python/ tests/ && ruff format --check python/ tests/
mypy --strict python/

# Security
cargo audit
bandit -r python/ -ll
```

## Adding a New Provider

1. Create `src/providers/new_provider.rs`
2. Add `pub mod new_provider;` to `src/providers/mod.rs`
3. Add data files to `src/data/en_us/` if needed
4. Add methods to `impl Faker` in `src/lib.rs` (both Rust API and #[pymethods])
5. Add convenience functions to `python/forgery/__init__.py`
6. Add type stubs to `.pyi` files
7. Add tests to `tests/`
8. Update README.md with new generators

## Provider Implementation Pattern

```rust
use crate::rng::ForgeryRng;
use crate::validate_batch_size;

pub fn generate_items(rng: &mut ForgeryRng, n: usize) -> Vec<String> {
    let mut results = Vec::with_capacity(n);
    for _ in 0..n {
        results.push(generate_item(rng));
    }
    results
}

pub fn generate_item(rng: &mut ForgeryRng) -> String {
    // Single item generation logic
}
```

## Quality Standards

- **Rust**: `cargo clippy -- -D warnings`, `cargo fmt`, no unnecessary `unwrap()`
- **Python**: `mypy --strict`, `ruff check`, 90%+ test coverage
- **Testing**: Property-based tests with proptest, unit tests for all providers
- **Security**: No secrets, validate all inputs, batch size limit of 10M

## Common Gotchas

- `gen_range(min, max)` is INCLUSIVE on both ends (`min..=max`)
- `date_of_birth` uses fixed reference date (2024-01-01) for determinism
- Each `Faker` instance has its own RNG state (not thread-safe)
- `records_tuples()` returns values in alphabetical key order (uses BTreeMap)

## CI Requirements

All PRs must pass:
- rust-check: fmt, clippy, cargo test
- python-check: ruff, mypy, pytest with 90%+ coverage
- security: cargo audit, bandit
- codeql: static analysis

## Branch Policy

- Never commit directly to `main`
- Use feature branches and PRs
- No force pushes
