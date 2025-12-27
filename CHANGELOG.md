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

#### Name Generators
- `name()` / `names(n)`: Full names (first + last)
- `first_name()` / `first_names(n)`: First names only
- `last_name()` / `last_names(n)`: Last names only

#### Contact Information
- `email()` / `emails(n)`: Email addresses
- `safe_email()` / `safe_emails(n)`: Safe domain emails (@example.com, etc.)
- `free_email()` / `free_emails(n)`: Free provider emails (@gmail.com, etc.)
- `phone_number()` / `phone_numbers(n)`: Phone numbers in (XXX) XXX-XXXX format

#### Numbers & Identifiers
- `integer(min, max)` / `integers(n, min, max)`: Random integers in range
- `float_(min, max)` / `floats(n, min, max)`: Random floats in range
- `uuid()` / `uuids(n)`: Version 4 UUIDs
- `md5()` / `md5s(n)`: MD5 hash strings
- `sha256()` / `sha256s(n)`: SHA256 hash strings

#### Dates & Times
- `date(start, end)` / `dates(n, start, end)`: Random dates (YYYY-MM-DD)
- `datetime(start, end)` / `datetimes(n, start, end)`: Random datetimes (ISO 8601)
- `date_of_birth(min_age, max_age)` / `dates_of_birth(n, min_age, max_age)`: Birth dates for age range

#### Addresses
- `street_address()` / `street_addresses(n)`: Street addresses
- `city()` / `cities(n)`: City names
- `state()` / `states(n)`: State names
- `country()` / `countries(n)`: Country names
- `zip_code()` / `zip_codes(n)`: ZIP codes (5 or 9 digit)
- `address()` / `addresses(n)`: Full addresses

#### Company & Business
- `company()` / `companies(n)`: Company names
- `job()` / `jobs(n)`: Job titles
- `catch_phrase()` / `catch_phrases(n)`: Business catch phrases

#### Network
- `url()` / `urls(n)`: URLs with https://
- `domain_name()` / `domain_names(n)`: Domain names
- `ipv4()` / `ipv4s(n)`: IPv4 addresses
- `ipv6()` / `ipv6s(n)`: IPv6 addresses
- `mac_address()` / `mac_addresses(n)`: MAC addresses

#### Finance
- `credit_card()` / `credit_cards(n)`: Credit card numbers (Luhn-valid)
- `iban()` / `ibans(n)`: IBAN numbers (valid checksum)

#### Text & Lorem Ipsum
- `sentence(word_count)` / `sentences(n, word_count)`: Lorem ipsum sentences
- `paragraph(sentence_count)` / `paragraphs(n, sentence_count)`: Lorem ipsum paragraphs
- `text(min_chars, max_chars)` / `texts(n, min_chars, max_chars)`: Text blocks

#### Colors
- `color()` / `colors(n)`: Color names
- `hex_color()` / `hex_colors(n)`: Hex color codes (#RRGGBB)
- `rgb_color()` / `rgb_colors(n)`: RGB tuples (r, g, b)

#### Infrastructure
- Module-level convenience functions using a default `fake` instance
- Full type stub support (PEP 561)
- US English locale (`en_US`) with comprehensive data:
  - 200 first names, 168 last names
  - 140 color names
  - 200 lorem ipsum words
  - 100 street names, 200 cities, 50 states, 200 countries
  - Company name components, 20 TLDs

### Security

- Batch size limit of 10 million items to prevent memory exhaustion
- Input validation for integer ranges (min must be <= max)
- Input validation for date ranges
- Credit card numbers use Luhn algorithm for valid checksums
- IBAN numbers use ISO 7064 Mod 97-10 for valid check digits

### Performance

- 50-100x faster than Faker for batch operations
- Benchmarked at 600x+ speedup for name generation
- All providers use batch-first design with preallocated buffers

### Developer Experience

- Comprehensive test suite with 90%+ code coverage
- Property-based testing with proptest
- CI/CD with GitHub Actions
- Rust tests with cargo test
- Python tests with pytest
- Linting with clippy, ruff, and mypy --strict
- Security scanning with cargo audit and bandit
- SonarCloud integration for code quality
- CodeQL static analysis

[Unreleased]: https://github.com/williajm/forgery/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/williajm/forgery/releases/tag/v0.1.0
