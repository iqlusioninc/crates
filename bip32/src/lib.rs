#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/bip32/0.3.0")]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

//! ## Backends
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
//! ## Limitations and further work
//! - Only 24-word BIP39 mnemonics are supported
//! - BIP43, BIP44, BIP49, BIP84 not yet properly supported
//!
//! # Usage
//! The following is an end-to-end example of how to generate a random BIP39
//! mnemonic and use it to derive child keys according to a provided BIP32
//! derivation path.
//!
//! ## Accessing `OsRng`
//! The following example uses `OsRng` for cryptographically secure random
//! number generation. To use it, you need to include `rand_core` with the
//! `std` feature by adding the following to `Cargo.toml`:
//!
//! ```toml
//! rand_core = { version = "0.6", features = ["std"] }
//! ```
//!
//! (on embedded platforms, you will need to supply our own RNG)
//!
//! ## Rust code example
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # #[cfg(all(feature = "bip39", feature = "secp256k1"))]
//! # {
//! use bip32::{Mnemonic, Prefix, XPrv};
//! use rand_core::OsRng;
//!
//! // Generate random Mnemonic using the default language (English)
//! let mnemonic = Mnemonic::random(&mut OsRng, Default::default());
//!
//! // Derive a BIP39 seed value using the given password
//! let seed = mnemonic.to_seed("password");
//!
//! // Derive the root `XPrv` from the `seed` value
//! let root_xprv = XPrv::new(&seed)?;
//! assert_eq!(root_xprv, XPrv::derive_from_path(&seed, &"m".parse()?)?);
//!
//! // Derive a child `XPrv` using the provided BIP32 derivation path
//! let child_path = "m/0/2147483647'/1/2147483646'";
//! let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse()?)?;
//!
//! // Get the `XPub` associated with `child_xprv`.
//! let child_xpub = child_xprv.public_key();
//!
//! // Serialize `child_xprv` as a string with the `xprv` prefix.
//! let child_xprv_str = child_xprv.to_string(Prefix::XPRV);
//! assert!(child_xprv_str.starts_with("xprv"));
//!
//! // Serialize `child_xpub` as a string with the `xpub` prefix.
//! let child_xpub_str = child_xpub.to_string(Prefix::XPUB);
//! assert!(child_xprv_str.starts_with("xprv"));
//!
//! // Get the ECDSA/secp256k1 signing and verification keys for the xprv and xpub
//! let signing_key = child_xprv.private_key();
//! let verification_key = child_xpub.public_key();
//!
//! // Sign and verify an example message using the derived keys.
//! use bip32::secp256k1::ecdsa::{
//!     signature::{Signer, Verifier},
//!     Signature
//! };
//!
//! let example_msg = b"Hello, world!";
//! let signature: Signature = signing_key.sign(example_msg);
//! assert!(verification_key.verify(example_msg, &signature).is_ok());
//! # }
//! # Ok(())
//! # }
//! ```
//!
//! [bip32]: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
//! [libsecp256k1 C library]: https://github.com/bitcoin-core/secp256k1
//! [`secp256k1` Rust crate]: https://github.com/rust-bitcoin/rust-secp256k1/

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

#[cfg(feature = "mnemonic")]
mod mnemonic;

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

#[cfg(feature = "alloc")]
pub use crate::derivation_path::DerivationPath;

#[cfg(feature = "bip39")]
#[cfg_attr(docsrs, doc(cfg(feature = "bip39")))]
pub use crate::mnemonic::{Language, Phrase as Mnemonic, Seed};

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

/// HMAC with SHA-512
type HmacSha512 = hmac::Hmac<sha2::Sha512>;

/// Size of input key material and derived keys.
pub const KEY_SIZE: usize = 32;
