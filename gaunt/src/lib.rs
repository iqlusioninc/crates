//! **gaunt.rs**: high-level, self-contained, minimalist HTTP toolkit.

#![crate_name = "gaunt"]
#![crate_type = "rlib"]
#![deny(
    warnings,
    missing_docs,
    unused_import_braces,
    unused_qualifications,
)]
#![no_std]
#![cfg_attr(
    all(feature = "nightly", not(feature = "std")),
    feature(alloc)
)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/gaunt/0.0.1")]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

extern crate failure;
#[macro_use]
extern crate failure_derive;
#[cfg(feature = "logger")]
#[macro_use]
extern crate slog;

#[macro_use]
pub mod error;

#[cfg(feature = "std")]
pub mod connection;
pub mod path;
pub mod prelude;
#[cfg(feature = "alloc")]
pub mod request;
#[cfg(feature = "alloc")]
pub mod response;

#[cfg(feature = "std")]
pub use connection::*;
pub use error::*;
pub use path::*;

/// Version of HTTP supported by Gaunt.
/// NOTE: HTTP/2 support is not planned.
pub const HTTP_VERSION: &str = "HTTP/1.1";

/// Gaunt's default `User-Agent` string
pub const USER_AGENT: &str = concat!("gaunt.rs ", env!("CARGO_PKG_VERSION"));
