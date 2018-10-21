//! **Gaunt**: high-level, self-contained, minimalist HTTP toolkit.

#![crate_name = "gaunt"]
#![crate_type = "rlib"]
#![deny(
    warnings,
    missing_docs,
    unused_import_braces,
    unused_qualifications,
)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/gaunt/0.0.1")]

extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;

#[macro_use]
pub mod error;

pub mod connection;
pub mod path;
pub mod response;

pub use connection::*;
pub use error::*;
pub use path::*;
pub use response::*;

/// Gaunt's default `User-Agent` string
pub const USER_AGENT: &str = concat!("gaunt.rs ", env!("CARGO_PKG_VERSION"));
