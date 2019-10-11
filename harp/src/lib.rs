//! **harp.rs**: high-level, self-contained, minimalist HTTP toolkit.

#![no_std]
#![deny(missing_docs, rust_2018_idioms, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/harp/0.1.0")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

#[cfg(feature = "logger")]
#[macro_use]
extern crate slog;

#[macro_use]
pub mod error;

#[cfg(feature = "std")]
pub mod connection;
pub mod path;
#[cfg(feature = "alloc")]
pub mod request;
#[cfg(feature = "alloc")]
pub mod response;

#[cfg(feature = "std")]
pub use crate::connection::*;
pub use crate::error::*;
pub use crate::path::*;

/// Version of HTTP supported by Harp.
/// NOTE: HTTP/2 support is not planned.
pub const HTTP_VERSION: &str = "HTTP/1.1";

/// Harp's default `User-Agent` string
pub const USER_AGENT: &str = concat!("harp.rs ", env!("CARGO_PKG_VERSION"));
