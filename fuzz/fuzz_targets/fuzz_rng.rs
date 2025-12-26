//! Fuzz testing for Faker operations.
//!
//! This fuzz target tests various Faker operations with arbitrary inputs
//! to ensure no panics occur under any input conditions.
//!
//! Run with: cargo +nightly fuzz run fuzz_rng

#![no_main]

use _forgery::Faker;
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

/// Input structure for Faker fuzzing.
#[derive(Arbitrary, Debug)]
struct FakerInput {
    seed: u64,
    operations: Vec<FakerOperation>,
}

#[derive(Arbitrary, Debug)]
enum FakerOperation {
    Name,
    FirstName,
    LastName,
    Email,
    Uuid,
    Integer { min: i64, max: i64 },
    Names { n: u8 },
    FirstNames { n: u8 },
    LastNames { n: u8 },
    Emails { n: u8 },
    Uuids { n: u8 },
    Integers { n: u8, min: i64, max: i64 },
    Reseed { new_seed: u64 },
}

fuzz_target!(|input: FakerInput| {
    let mut faker = Faker::new_default();
    faker.seed(input.seed);

    // Limit operations to prevent timeouts
    let max_ops = input.operations.len().min(100);

    for op in input.operations.into_iter().take(max_ops) {
        match op {
            FakerOperation::Name => {
                let _ = faker.name();
            }
            FakerOperation::FirstName => {
                let _ = faker.first_name();
            }
            FakerOperation::LastName => {
                let _ = faker.last_name();
            }
            FakerOperation::Email => {
                let _ = faker.email();
            }
            FakerOperation::Uuid => {
                let _ = faker.uuid();
            }
            FakerOperation::Integer { min, max } => {
                // Should return error for invalid range, not panic
                let _ = faker.integer(min, max);
            }
            FakerOperation::Names { n } => {
                // Cap batch size for fuzzing
                let _ = faker.names((n as usize).min(100));
            }
            FakerOperation::FirstNames { n } => {
                let _ = faker.first_names((n as usize).min(100));
            }
            FakerOperation::LastNames { n } => {
                let _ = faker.last_names((n as usize).min(100));
            }
            FakerOperation::Emails { n } => {
                let _ = faker.emails((n as usize).min(100));
            }
            FakerOperation::Uuids { n } => {
                let _ = faker.uuids((n as usize).min(100));
            }
            FakerOperation::Integers { n, min, max } => {
                let _ = faker.integers((n as usize).min(100), min, max);
            }
            FakerOperation::Reseed { new_seed } => {
                faker.seed(new_seed);
            }
        }
    }
});
