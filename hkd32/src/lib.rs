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
//! - `hkd32::KeyMaterial`: a 32-byte (256-bit) uniformly random value
//! - `hkd32::Path` or `hkd32::PathBuf`: path to the child key
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
//! // Parent key
//! let input_key_material = hkd32::KeyMaterial::random();
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
#![deny(warnings, missing_docs, unused_qualifications, unsafe_code)]
#![doc(html_root_url = "https://docs.rs/hkd32/0.1.0")]

#[cfg(feature = "alloc")]
#[cfg_attr(test, macro_use)]
extern crate alloc;

mod key_material;
#[cfg(feature = "mnemonic")]
pub mod mnemonic;
mod path;
#[cfg(feature = "alloc")]
mod pathbuf;

#[cfg(feature = "alloc")]
pub use self::pathbuf::PathBuf;
pub use self::{key_material::*, path::*};

/// Delimiter used for strings containing paths
pub const DELIMITER: char = '/';

/// Size of input key material and derived keys.
///
/// Note: the name HKD32 is both a play on this size and "BIP32".
pub const KEY_SIZE: usize = 32;

/// Opaque error type
#[derive(Copy, Clone, Debug)]
pub struct Error;
