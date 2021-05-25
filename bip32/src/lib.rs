//! Pure Rust implementation of
//! [BIP-0032: Hierarchical Deterministic Wallets][bip32].
//!
//! [bip32]: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/bip32/0.0.0")]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

extern crate alloc;

mod child_number;
mod derivation_path;
mod error;
mod extended_key;
mod extended_private_key;
mod private_key;
mod version;

pub use self::{
    child_number::ChildNumber,
    derivation_path::DerivationPath,
    error::{Error, Result},
    extended_private_key::{Depth, ExtendedPrivateKey},
    version::Version,
};
pub use hkd32::{
    mnemonic::{Language, Phrase as Mnemonic, Seed},
    KEY_SIZE,
};

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
pub use k256 as secp256k1;

/// Chain code: extension for both private and public keys which provides an
/// additional 256-bits of entropy.
pub type ChainCode = [u8; KEY_SIZE];

/// Extended private secp256k1 ECDSA signing key.
#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
pub type XPrv = ExtendedPrivateKey<secp256k1::ecdsa::SigningKey>;
