//! Cryptographic key material.
//!
//! Unlike traditional KDFs and XOFs, HKD32 acts on fixed-sized
//! 256-bit (32-byte) keys.
//!
//! The `KeyMaterial` type is used to represent both input and output key
//! material, and is the primary type useful for deriving other keys.

use crate::{path::Path, Error, KEY_SIZE};
use core::convert::TryFrom;
use hmac::crypto_mac::{Mac, NewMac};
use hmac::Hmac;
use rand_core::{CryptoRng, RngCore};
use sha2::Sha512;
use zeroize::Zeroize;

#[cfg(feature = "bech32")]
use {alloc::string::String, subtle_encoding::bech32::Bech32, zeroize::Zeroizing};

#[cfg(feature = "mnemonic")]
use crate::mnemonic;

/// Cryptographic key material: 256-bit (32-byte) uniformly random bytestring
/// generated either via a CSRNG or via hierarchical derivation.
///
/// This type provides the main key derivation functionality and is used to
/// represent both input and output key material.
#[derive(Clone, Zeroize)]
#[zeroize(drop)]
pub struct KeyMaterial([u8; KEY_SIZE]);

impl KeyMaterial {
    /// Create random key material using the operating system CSRNG
    pub fn random(mut rng: impl RngCore + CryptoRng) -> Self {
        let mut bytes = [0u8; KEY_SIZE];
        rng.fill_bytes(&mut bytes);
        Self::new(bytes)
    }

    /// Decode key material from a Bech32 representation
    #[cfg(feature = "bech32")]
    pub fn from_bech32<S>(encoded: S) -> Result<(String, Self), Error>
    where
        S: AsRef<str>,
    {
        let (hrp, mut key_bytes) = Bech32::default().decode(encoded).map_err(|_| Error)?;
        let key_result = Self::from_bytes(&key_bytes);
        key_bytes.zeroize();
        key_result.map(|key| (hrp, key))
    }

    /// Create key material from a 24-word BIP39 mnemonic phrase
    #[cfg(feature = "mnemonic")]
    pub fn from_mnemonic<S>(phrase: S, language: mnemonic::Language) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        Ok(mnemonic::Phrase::new(phrase, language)?.into())
    }

    /// Create new key material from a byte slice.
    ///
    /// Byte slice is expected to have been generated by a cryptographically
    /// secure random number generator.
    pub fn from_bytes(slice: &[u8]) -> Result<Self, Error> {
        if slice.len() == KEY_SIZE {
            let mut bytes = [0u8; KEY_SIZE];
            bytes.copy_from_slice(slice);
            Ok(Self::new(bytes))
        } else {
            Err(Error)
        }
    }

    /// Import existing key material - must be uniformly random!
    pub fn new(bytes: [u8; KEY_SIZE]) -> KeyMaterial {
        KeyMaterial(bytes)
    }

    /// Borrow the key material as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Derive an output key from the given input key material
    pub fn derive_subkey<P>(self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let component_count = path.as_ref().components().count();

        path.as_ref()
            .components()
            .enumerate()
            .fold(self, |parent_key, (i, component)| {
                let mut hmac = Hmac::<Sha512>::new_from_slice(parent_key.as_bytes())
                    .expect("HMAC key size incorrect");
                hmac.update(component.as_bytes());

                let mut hmac_result = hmac.finalize().into_bytes();
                let (secret_key, chain_code) = hmac_result.split_at_mut(KEY_SIZE);
                let mut child_key = [0u8; KEY_SIZE];

                if i < component_count - 1 {
                    // Use chain code for all but the last element
                    child_key.copy_from_slice(chain_code);
                } else {
                    // Use secret key for the last element
                    child_key.copy_from_slice(secret_key);
                }

                secret_key.zeroize();
                chain_code.zeroize();

                KeyMaterial(child_key)
            })
    }

    /// Serialize this `KeyMaterial` as a self-zeroizing Bech32 string
    #[cfg(feature = "bech32")]
    pub fn to_bech32<S>(&self, hrp: S) -> Zeroizing<String>
    where
        S: AsRef<str>,
    {
        let b32 = Bech32::default().encode(hrp, self.as_bytes());
        Zeroizing::new(b32)
    }
}

impl From<[u8; KEY_SIZE]> for KeyMaterial {
    fn from(bytes: [u8; KEY_SIZE]) -> Self {
        Self::new(bytes)
    }
}

impl<'a> TryFrom<&'a [u8]> for KeyMaterial {
    type Error = Error;

    fn try_from(slice: &'a [u8]) -> Result<Self, Error> {
        Self::from_bytes(slice)
    }
}
