//! Data generation providers.
//!
//! Each provider module implements generation functions for a specific category
//! of fake data. All generators follow the batch-first pattern, returning `Vec<T>`.

pub mod identifiers;
pub mod internet;
pub mod names;
pub mod numbers;
