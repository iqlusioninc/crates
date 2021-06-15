//! Pure Rust implementation of
//! [BIP-0032: Hierarchical Deterministic Wallets][bip32].
//!
//! [bip32]: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/bip32/0.0.0")]
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
