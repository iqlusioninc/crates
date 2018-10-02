//! Encoders and decoders for common data encodings (WIP) which avoid
//! branching or performing table lookups based on their inputs
//! (a.k.a. constant time-ish).

#![crate_name = "subtle_encoding"]
#![crate_type = "rlib"]
#![no_std]
#![cfg_attr(
    all(feature = "nightly", not(feature = "std")),
    feature(alloc)
)]
#![deny(
    warnings,
    missing_docs,
    unused_import_braces,
    unused_qualifications
)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/subtle-encoding/0.0.0")]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

extern crate failure;
#[macro_use]
extern crate failure_derive;

#[macro_use]
mod error;

mod encoding;
mod identity;
mod prelude;

pub use self::{encoding::Encoding, error::Error, identity::*};
