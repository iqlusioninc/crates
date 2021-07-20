//! Signatory: a multi-algorithm digital signature library.
//!
//! This crate provides a thread-and-object-safe API for both creating and
//! verifying elliptic curve digital signatures, using either software-based
//! or hardware-based providers.
//!
//! The following algorithms are supported:
//!
//! - [ecdsa]: Elliptic Curve Digital Signature Algorithm ([FIPS 186-4])
//! - [ed25519]: Edwards Digital Signature Algorithm (EdDSA) instantiated using
//!   the twisted Edwards form of Curve25519 ([RFC 8032]).
//!
//! [FIPS 186-4]: https://csrc.nist.gov/publications/detail/fips/186/4/final
//! [RFC 8032]: https://tools.ietf.org/html/rfc8032

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_root_url = "https://docs.rs/signatory/0.23.0",
    html_logo_url = "https://raw.githubusercontent.com/iqlusioninc/crates/main/signatory/img/signatory-rustacean.png"
)]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "ecdsa")]
#[cfg_attr(docsrs, doc(cfg(feature = "ecdsa")))]
pub mod ecdsa;

#[cfg(feature = "ed25519")]
#[cfg_attr(docsrs, doc(cfg(feature = "ed25519")))]
pub mod ed25519;

mod algorithm;
mod error;
mod key;

pub use self::{
    algorithm::Algorithm,
    error::{Error, Result},
    key::{
        handle::KeyHandle,
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

/// Map type.
pub type Map<K, V> = alloc::collections::BTreeMap<K, V>;
