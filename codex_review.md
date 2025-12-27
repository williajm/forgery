# Code Review: `phase3.3-records-arrow` vs `main`

## Scope summary

- **Commits:** 11 (feature + CI/coverage improvements)
- **Diffstat:** 15 files changed, **+1677/-11**
- **Primary change:** new Arrow/Polars integration via `records_arrow()` returning a PyArrow `RecordBatch`

## High-level feedback

This is a solid feature addition: the API is consistent with existing `records()` / `records_tuples()`, the Rust implementation is straightforward and testable, and the Python surface area is documented and covered with good tests. The DoS-focused guardrail (`MAX_SCHEMA_SIZE`) is a nice touch for user-controlled schemas.

## What’s working well

- **Clear API shape**
  - `Faker.records_arrow()` mirrors the existing record APIs and is exposed at the module level (`forgery.records_arrow`).
  - Docs added in `README.md` make the intended Arrow/Polars workflow obvious.

- **Good validation and determinism**
  - `validate_batch_size()` is reused, and Arrow generation validates the schema even when `n=0`.
  - Determinism tests exist on both Rust and Python sides.

- **Reasonable Arrow typing choices**
  - `int`/`float` map to `Int64`/`Float64`, and `rgb_color` maps to a struct with `r/g/b` `UInt8`, which is ergonomic for Arrow consumers.

- **Coverage and CI improvements**
  - Adding Rust coverage via `cargo llvm-cov` and uploading separate Codecov flags for Rust and Python is a good direction.

## Issues / risks to consider before merge

### 1) PyArrow installation vs Python CI matrix (potential CI fragility)

- `pyproject.toml` adds `pyarrow>=18.0` to the **`dev`** extra, and CI installs `-e ".[dev]"` across **3.11–3.14**.
- If PyArrow wheels aren’t available for the newest/pre-release interpreters (notably 3.14), CI may fail during dependency install even though tests are written to skip cleanly when PyArrow is absent.

**Suggestion:** move PyArrow into a separate optional extra (e.g. `arrow = ["pyarrow>=..."]`) and install it only in one job (e.g. `3.11`) or gate it with environment markers (e.g. `<3.14`) to keep the broader matrix stable and faster.

### 2) `expect()` in Arrow generation (panic surface)

In `src/providers/records.rs`, Arrow generation uses `expect()` for integer/float generation based on the assumption that ranges were prevalidated.

- This is likely safe today (the error path is “invalid range”), but it does mean a future behavior change in `numbers::generate_integer/float` could turn into a **hard panic** in library code.

**Suggestion:** consider converting these to an internal `debug_assert!` plus a fallback `map_err(...)` to preserve “should be unreachable” while keeping the public API non-panicking.

### 3) Minor cleanup opportunity: unused `field_names`

`generate_records_arrow_with_custom()` builds `field_names` but doesn’t use it. It’s harmless but allocates and pushes strings unnecessarily.

## Design / future-proofing suggestions (nice-to-have)

- **Feature-gate Arrow deps**: `arrow-*` + `pyo3-arrow` are non-trivial dependencies. If users may want the core faker without Arrow, consider a Cargo feature (and/or Python extra) to opt into Arrow support.
- **Document column ordering**: schemas are parsed into a `BTreeMap`, so output columns are **sorted by field name**, not Python dict insertion order. Existing docs mention tuple order; consider calling this out for Arrow as well.
- **Consider richer Arrow physical types**: dates/datetimes are currently `Utf8`. In the future, mapping to Arrow date/time types would improve downstream analytics ergonomics.

## Files reviewed (not exhaustive, but the key deltas)

- Rust API + safety:
  - `src/lib.rs` (adds `records_arrow`, `MAX_SCHEMA_SIZE`, `validate_schema_size`)
  - `src/providers/records.rs` (Arrow RecordBatch generation + tests)
- Python surface + typing:
  - `python/forgery/__init__.py` (module-level `records_arrow`)
  - `python/forgery/__init__.pyi`, `python/forgery/_forgery.pyi`
  - `tests/test_records.py` (Arrow tests + schema-size test)
- CI/coverage:
  - `.github/workflows/ci.yml`, `codecov.yml`, `sonar-project.properties`, `.gitignore`

## Local verification

Ran locally in this workspace:

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
./.venv/bin/ruff check python/ tests/
./.venv/bin/ruff format --check python/ tests/
./.venv/bin/mypy --strict python/
./.venv/bin/python -m pytest
```

All commands completed successfully.

