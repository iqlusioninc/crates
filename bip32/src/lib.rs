//! Pure Rust implementation of
//! [BIP-0032: Hierarchical Deterministic Wallets][bip32].
//!
//! # About
//! BIP32 is an algorithm for generating a hierarchy of elliptic curve keys,
//! a.k.a. "wallets", from a single seed value. A related algorithm also
//! implemented by this crate, BIP39, provides a way to derive the seed value
//! from a set of 24-words from a preset list, a.k.a. a "mnemonic".
//!
//! # Backends
//! This crate provides a generic implementation of BIP32 which can be used
//! with any backing provider which implements the [`PrivateKey`] and
//! [`PublicKey`] traits. The following providers are built into this crate,
//! under the following crate features:
//!
//! - `secp256k1` (enabled by default): support for the pure Rust [`k256`]
//!   crate, with [`XPrv`] and [`XPub`] type aliases.
//! - `secp256k1-ffi`: support for Bitcoin Core's [libsecp256k1 C library],
//!   as wrapped by the [`secp256k1` Rust crate].
//!
//! [bip32]: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
//! [libsecp256k1 C library]: https://github.com/bitcoin-core/secp256k1
//! [`secp256k1` Rust crate]: https://github.com/rust-bitcoin/rust-secp256k1/

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/bip32/0.1.0")]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod child_number;
mod error;
mod extended_key;
mod prefix;
mod private_key;
mod public_key;

#[cfg(feature = "alloc")]
mod derivation_path;

pub use crate::{
    child_number::ChildNumber,
    error::{Error, Result},
    extended_key::{
        attrs::ExtendedKeyAttrs, private_key::ExtendedPrivateKey, public_key::ExtendedPublicKey,
        ExtendedKey,
    },
    prefix::Prefix,
    private_key::{PrivateKey, PrivateKeyBytes},
    public_key::{PublicKey, PublicKeyBytes},
};
pub use hkd32::KEY_SIZE;

#[cfg(feature = "alloc")]
pub use crate::derivation_path::DerivationPath;

#[cfg(feature = "bip39")]
#[cfg_attr(docsrs, doc(cfg(feature = "bip39")))]
pub use hkd32::mnemonic::{Language, Phrase as Mnemonic, Seed};

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
pub use {
    crate::extended_key::{private_key::XPrv, public_key::XPub},
    k256 as secp256k1,
};

/// Chain code: extension for both private and public keys which provides an
/// additional 256-bits of entropy.
pub type ChainCode = [u8; KEY_SIZE];

/// Derivation depth.
pub type Depth = u8;

/// BIP32 key fingerprints.
pub type KeyFingerprint = [u8; 4];

/// BIP32 "versions": integer representation of the key prefix.
pub type Version = u32;
