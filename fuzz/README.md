# Fuzz Testing for Forgery

This directory contains fuzz testing targets for the forgery library using `cargo-fuzz`.

## Prerequisites

1. Install the nightly Rust toolchain:
   ```bash
   rustup install nightly
   ```

2. Install cargo-fuzz:
   ```bash
   cargo install cargo-fuzz
   ```

## Running Fuzz Tests

Run a specific fuzz target:

```bash
cd /path/to/forgery
cargo +nightly fuzz run fuzz_integers
cargo +nightly fuzz run fuzz_rng
```

Run with a time limit:

```bash
cargo +nightly fuzz run fuzz_integers -- -max_total_time=60
```

## Available Fuzz Targets

- `fuzz_integers`: Tests integer generation with arbitrary ranges and batch sizes
- `fuzz_rng`: Tests RNG operations including gen_range, choose, and fill_bytes

## Corpus

Fuzzing corpus files are stored in `fuzz/corpus/<target_name>/`. These are automatically
maintained by the fuzzer.

## Crashes

Any crashes found will be stored in `fuzz/artifacts/<target_name>/`.

## Notes

The fuzz targets test the library's robustness against arbitrary inputs. Key properties tested:

1. No panics on valid inputs
2. Graceful error handling for invalid inputs
3. No undefined behavior
4. Memory safety

For property-based testing of correctness, see the proptest tests in the main test suite.
