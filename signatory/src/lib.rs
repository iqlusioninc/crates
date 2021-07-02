//! Signatory

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/signatory/0.23.0-pre.1")]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "ecdsa")]
#[cfg_attr(docsrs, doc(cfg(feature = "ecdsa")))]
pub mod ecdsa;

mod algorithm;
mod error;
mod key;

pub use self::{
    algorithm::Algorithm,
    error::{Error, Result},
    key::{
        info::KeyInfo,
        name::KeyName,
        ring::{KeyRing, LoadPkcs8},
        store::GeneratePkcs8,
    },
};
pub use pkcs8;
pub use signature;

#[cfg(feature = "std")]
pub use key::store::fs::FsKeyStore;
