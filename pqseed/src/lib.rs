#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_qualifications
)]

mod error;

pub use crate::error::{Error, Result};

#[cfg(feature = "ml-dsa")]
pub use ml_dsa;
#[cfg(feature = "slh-dsa")]
pub use slh_dsa;

use bip32::{ChildNumber, XPrv};
use core::fmt;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha512;
use zeroize::{Zeroize, Zeroizing};

/// Derives Project Eleven post-quantum keys from a BIP39 seed.
///
/// Holds a secret 64-byte seed, zeroized on drop. No extended public keys or
/// non-hardened derivation are exposed. Each algorithm has exactly one fixed
/// path; adding an index would produce keys incompatible with the article.
pub struct KeyDeriver {
    seed: Zeroizing<[u8; 64]>,
}

impl KeyDeriver {
    /// Import an already expanded, 64-byte BIP39 seed (not mnemonic entropy).
    ///
    /// The caller must ensure this came from a securely generated 24-word
    /// mnemonic and must erase its own copy. This method cannot verify the
    /// original mnemonic's entropy, checksum, or word count.
    pub fn from_seed(seed: &[u8; 64]) -> Self {
        Self {
            seed: Zeroizing::new(*seed),
        }
    }

    /// Parse a 24-word English BIP39 mnemonic and expand it with a passphrase.
    ///
    /// Validates the word list, checksum, and word count. BIP39 Unicode NFKD
    /// normalization is applied. Use an empty passphrase to match the article.
    /// The caller remains responsible for erasing the input strings.
    #[cfg(feature = "bip39")]
    pub fn from_mnemonic(phrase: &str, passphrase: &str) -> Result<Self> {
        let mnemonic = bip39::Mnemonic::parse_in(bip39::Language::English, phrase)?;
        if mnemonic.word_count() != 24 {
            return Err(Error::WordCount);
        }

        Ok(Self {
            seed: Zeroizing::new(mnemonic.to_seed(passphrase)),
        })
    }

    /// Derive the 32-byte ML-DSA-44 seed at `m/83696968'/503131'/0'`.
    ///
    /// Returns the first 32 bytes of the BIP85 HMAC output. The returned
    /// buffer is secret and zeroizes on drop; do not log it or its `Debug` value.
    pub fn ml_dsa44_seed(&self) -> Result<Zeroizing<[u8; 32]>> {
        self.derive_seed(0)
    }

    /// Derive the 48-byte SLH-DSA-SHA2-128s seed at `m/83696968'/503131'/2'`.
    ///
    /// The bytes are `SK.seed || SK.prf || PK.seed`, each 16 bytes long.
    /// The returned buffer is secret and zeroizes on drop; do not log it.
    pub fn slh_dsa_sha2_128s_seed(&self) -> Result<Zeroizing<[u8; 48]>> {
        self.derive_seed(2)
    }

    /// Derive an ML-DSA-44 signing key, from which the public key is available
    /// through [`ml_dsa::Keypair::verifying_key`].
    #[cfg(feature = "ml-dsa")]
    pub fn ml_dsa44(&self) -> Result<ml_dsa::SigningKey<ml_dsa::MlDsa44>> {
        let seed = self.ml_dsa44_seed()?;
        Ok(ml_dsa::SigningKey::from_seed((&*seed).into()))
    }

    /// Derive an SLH-DSA-SHA2-128s signing key, from which the public key is
    /// available through [`slh_dsa::signature::Keypair::verifying_key`].
    #[cfg(feature = "slh-dsa")]
    pub fn slh_dsa_sha2_128s(&self) -> Result<slh_dsa::SigningKey<slh_dsa::Sha2_128s>> {
        let seed = self.slh_dsa_sha2_128s_seed()?;
        Ok(slh_dsa::SigningKey::slh_keygen_internal(
            &seed[..16],
            &seed[16..32],
            &seed[32..48],
        ))
    }

    fn derive_seed<const N: usize>(&self, algorithm: u32) -> Result<Zeroizing<[u8; N]>> {
        let mut node = XPrv::new(&*self.seed)?;
        for index in [83_696_968, 503_131, algorithm] {
            node = node.derive_child(ChildNumber::new(index, true)?)?;
        }

        let private_key = Zeroizing::new(node.to_bytes());
        let mut mac = Hmac::<Sha512>::new_from_slice(b"bip-entropy-from-k")
            .expect("HMAC accepts this fixed key length");
        mac.update(&*private_key);
        let mut digest = mac.finalize().into_bytes();
        let mut seed = Zeroizing::new([0u8; N]);
        seed.copy_from_slice(&digest[..N]);
        digest.as_mut_slice().zeroize();
        Ok(seed)
    }
}

impl fmt::Debug for KeyDeriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyDeriver").finish_non_exhaustive()
    }
}
