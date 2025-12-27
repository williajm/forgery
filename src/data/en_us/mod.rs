//! English (US) locale data.
//!
//! Contains name lists and other data for en_US locale.

mod cities;
mod color_names;
mod companies;
mod countries;
mod first_names;
mod last_names;
mod lorem;
mod states;
mod streets;
mod tlds;

pub use cities::CITIES;
pub use color_names::COLOR_NAMES;
pub use companies::{
    CATCH_PHRASE_ADJECTIVES, CATCH_PHRASE_NOUNS, COMPANY_PREFIXES, COMPANY_SUFFIXES, JOB_TITLES,
};
pub use countries::COUNTRIES;
pub use first_names::FIRST_NAMES;
pub use last_names::LAST_NAMES;
pub use lorem::LOREM_WORDS;
pub use states::{STATES, STATE_ABBRS};
pub use streets::{STREET_NAMES, STREET_SUFFIXES};
pub use tlds::{FREE_EMAIL_DOMAINS, SAFE_EMAIL_DOMAINS, TLDS};
