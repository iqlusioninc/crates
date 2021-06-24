//! Signatory

#![no_std]
#![doc(html_root_url = "https://docs.rs/signatory/0.23.0-pre")]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "ecdsa")]
pub mod ecdsa;

mod algorithm;
mod error;
mod key;

pub use self::{
    algorithm::Algorithm,
    error::{Error, Result},
    key::{info::KeyInfo, name::KeyName, ring::KeyRing, store::GeneratePkcs8},
};
pub use signature;

#[cfg(feature = "std")]
pub use key::store::fs::FsKeyStore;
