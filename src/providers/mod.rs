//! Data generation providers.
//!
//! Each provider module implements generation functions for a specific category
//! of fake data. All generators follow the batch-first pattern, returning `Vec<T>`.

pub mod address;
pub mod colors;
pub mod company;
pub mod custom;
pub mod datetime;
pub mod finance;
pub mod identifiers;
pub mod internet;
pub mod names;
pub mod network;
pub mod numbers;
pub mod phone;
pub mod records;
pub mod text;
