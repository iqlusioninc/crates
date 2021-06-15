//! Support for BIP39 mnemonic phrases.
//!
//! These enable deriving `hkd32::KeyMaterial` from a 24-word BIP39 phrase.
//!
//! Adapted from the `bip39` crate.
//! Copyright © 2017-2018 Stephen Oliver with contributions by Maciej Hirsz.

mod bits;
mod language;
mod phrase;

#[cfg(feature = "bip39")]
pub(crate) mod seed;

pub use self::{language::Language, phrase::Phrase};

#[cfg(feature = "bip39")]
pub use self::seed::Seed;
