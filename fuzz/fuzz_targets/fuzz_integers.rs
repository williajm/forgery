//! Fuzz testing for integer generation.
//!
//! This fuzz target tests the integer generation with arbitrary inputs
//! to find edge cases that might cause panics or unexpected behavior.
//!
//! Run with: cargo +nightly fuzz run fuzz_integers

#![no_main]

use _forgery::Faker;
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

/// Input structure for integer generation fuzzing.
#[derive(Arbitrary, Debug)]
struct IntegerInput {
    seed: u64,
    n: u16, // Limited to prevent OOM
    min: i64,
    max: i64,
}

fuzz_target!(|input: IntegerInput| {
    let mut faker = Faker::new_default();
    faker.seed(input.seed);

    // Test single integer generation
    // Should either succeed or return an error, never panic
    let _ = faker.integer(input.min, input.max);

    // Test batch integer generation with bounded size
    let batch_size = (input.n as usize).min(1000); // Cap at 1000 for fuzzing
    let _ = faker.integers(batch_size, input.min, input.max);
});
