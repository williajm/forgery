# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - Unreleased

### Added

- Initial release of forgery
- Core `Faker` class with per-instance RNG state
- Batch-first API design for maximum performance
- Deterministic seeding with ChaCha8 RNG
- Name generators:
  - `name()` / `names(n)`: Full names (first + last)
  - `first_name()` / `first_names(n)`: First names only
  - `last_name()` / `last_names(n)`: Last names only
- Email generator:
  - `email()` / `emails(n)`: Email addresses
- Integer generator:
  - `integer(min, max)` / `integers(n, min, max)`: Random integers
- UUID generator:
  - `uuid()` / `uuids(n)`: Version 4 UUIDs
- Module-level convenience functions using a default `fake` instance
- Full type stub support (PEP 561)
- US English locale (`en_US`) with 200 first names and 168 last names

### Security

- Batch size limit of 10 million items to prevent memory exhaustion
- Input validation for integer ranges (min must be <= max)

### Performance

- 50-100x faster than Faker for batch operations
- Benchmarked at 600x+ speedup for name generation

### Developer Experience

- Comprehensive test suite with 100% code coverage
- CI/CD with GitHub Actions
- Rust tests with cargo test
- Python tests with pytest
- Linting with clippy, ruff, and mypy --strict
- Security scanning with cargo audit and bandit

[Unreleased]: https://github.com/williajm/forgery/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/williajm/forgery/releases/tag/v0.1.0
