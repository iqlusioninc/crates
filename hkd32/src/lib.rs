//! HMAC-based Hierarchical Key Derivation: deterministically derive a
//! hierarchy of symmetric keys from initial keying material through
//! repeated applications of the Hash-based Message Authentication Code
//! (HMAC) construction.
//!
//! This library implements a fully symmetric construction inspired by
//! [BIP-0032: Hierarchical Deterministic Wallets][bip32].
//!
//! # Usage
//!
//! To derive a key using HKD32, you'll need the following:
//!
//! - [`KeyMaterial`]: a 32-byte (256-bit) uniformly random value
//! - [`Path`] or [`PathBuf`]: path to the child key
//!
//! Derivation paths can be raw bytestrings but also support a Unix path-like
//! syntax which can be parsed using the `String::parse` method:
//!
//! ```rust
//! let path = "/foo/bar/baz".parse::<hkd32::PathBuf>().unwrap();
//! ```
//!
//! # Example
//!
//! ```rust
//! use rand_core::OsRng;
//!
//! // Parent key
//! let input_key_material = hkd32::KeyMaterial::random(&mut OsRng);
//!
//! // Path to the child key
//! let derivation_path = "/foo/bar/baz".parse::<hkd32::PathBuf>().unwrap();
//!
//! // Derive subkey from the parent key. Call `as_bytes()` on this to obtain
//! // a byte slice containing the derived key.
//! let output_key_material = input_key_material.derive_subkey(derivation_path);
//! ```
//!
//! [bip32]: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(feature = "alloc")]
#[cfg_attr(any(feature = "bip39", test), macro_use)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "mnemonic")]
pub mod mnemonic;

mod key_material;
mod path;

#[cfg(feature = "alloc")]
mod pathbuf;

pub use self::{key_material::*, path::*};

#[cfg(feature = "alloc")]
pub use self::pathbuf::PathBuf;

/// Delimiter used for strings containing paths
pub const DELIMITER: char = '/';

/// Size of input key material and derived keys.
///
/// Note: the name HKD32 is both a play on this size and "BIP32".
pub const KEY_SIZE: usize = 32;

/// Opaque error type
#[derive(Copy, Clone, Debug)]
pub struct Error;

#[cfg(feature = "std")]
impl std::error::Error for Error {}

use core::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error")
    }
}
