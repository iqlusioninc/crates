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

/// BIP85 purpose: `83696968'`.
const BIP85: u32 = 83_696_968;

/// Project Eleven's BIP85 application number: `503131'` ("P11").
const P11_APP: u32 = 503_131;

/// The `pqseed` BIP85 application number for scoped keys: `505153'`.
///
/// The decimal digits are the hex bytes of ASCII "PQS", following the
/// convention behind Project Eleven's `503131'` ("P11").
pub const PQSEED_APP: u32 = 505_153;

/// Algorithm IDs, shared by the Project Eleven and `pqseed` namespaces.
const ML_DSA_44: u32 = 0;
const SLH_DSA_SHA2_128S: u32 = 2;
const ML_DSA_65: u32 = 3;
const ML_DSA_87: u32 = 4;

/// A coin type, account, and key index for scoped derivation.
///
/// Scoped keys live at
/// `m/83696968'/505153'/coin_type'/algorithm'/account'/index'`. `coin_type`
/// is a [SLIP-44] registered coin type. Every level is hardened, so each value
/// must be below 2<sup>31</sup>.
///
/// [SLIP-44]: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Scope {
    /// SLIP-44 coin type.
    pub coin_type: u32,
    /// Account number.
    pub account: u32,
    /// Key index within the account.
    pub index: u32,
}

impl Scope {
    /// Create a scope from a SLIP-44 coin type, account, and key index.
    pub const fn new(coin_type: u32, account: u32, index: u32) -> Self {
        Self {
            coin_type,
            account,
            index,
        }
    }

    fn path(&self, algorithm: u32) -> [u32; 6] {
        [
            BIP85,
            PQSEED_APP,
            self.coin_type,
            algorithm,
            self.account,
            self.index,
        ]
    }
}

/// Derives post-quantum keys using Project Eleven's scheme and `pqseed` extensions.
///
/// Holds a secret 64-byte seed, zeroized on drop. No extended public keys or
/// non-hardened derivation are exposed. Unscoped methods use the article's
/// single fixed path per algorithm; `*_for` methods derive keys scoped to a
/// coin type, account, and index in the separate `pqseed` namespace.
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
        self.derive_seed(&[BIP85, P11_APP, ML_DSA_44])
    }

    /// Derive the 32-byte ML-DSA-65 seed at `m/83696968'/503131'/3'`.
    ///
    /// Algorithm ID 3 is a `pqseed` extension, not assigned by Project Eleven.
    /// Returns the first 32 bytes of the BIP85 HMAC output in a secret buffer
    /// that zeroizes on drop; do not log it or its `Debug` value.
    pub fn ml_dsa65_seed(&self) -> Result<Zeroizing<[u8; 32]>> {
        self.derive_seed(&[BIP85, P11_APP, ML_DSA_65])
    }

    /// Derive the 32-byte ML-DSA-87 seed at `m/83696968'/503131'/4'`.
    ///
    /// Algorithm ID 4 is a `pqseed` extension, not assigned by Project Eleven.
    /// Returns the first 32 bytes of the BIP85 HMAC output in a secret buffer
    /// that zeroizes on drop; do not log it or its `Debug` value.
    pub fn ml_dsa87_seed(&self) -> Result<Zeroizing<[u8; 32]>> {
        self.derive_seed(&[BIP85, P11_APP, ML_DSA_87])
    }

    /// Derive the 48-byte SLH-DSA-SHA2-128s seed at `m/83696968'/503131'/2'`.
    ///
    /// The bytes are `SK.seed || SK.prf || PK.seed`, each 16 bytes long.
    /// The returned buffer is secret and zeroizes on drop; do not log it.
    pub fn slh_dsa_sha2_128s_seed(&self) -> Result<Zeroizing<[u8; 48]>> {
        self.derive_seed(&[BIP85, P11_APP, SLH_DSA_SHA2_128S])
    }

    /// Derive the 32-byte ML-DSA-44 seed for a [`Scope`].
    ///
    /// Path: `m/83696968'/505153'/coin_type'/0'/account'/index'`. The returned
    /// buffer is secret and zeroizes on drop; do not log it.
    pub fn ml_dsa44_seed_for(&self, scope: &Scope) -> Result<Zeroizing<[u8; 32]>> {
        self.derive_seed(&scope.path(ML_DSA_44))
    }

    /// Derive the 32-byte ML-DSA-65 seed for a [`Scope`].
    ///
    /// Path: `m/83696968'/505153'/coin_type'/3'/account'/index'`. The returned
    /// buffer is secret and zeroizes on drop; do not log it.
    pub fn ml_dsa65_seed_for(&self, scope: &Scope) -> Result<Zeroizing<[u8; 32]>> {
        self.derive_seed(&scope.path(ML_DSA_65))
    }

    /// Derive the 32-byte ML-DSA-87 seed for a [`Scope`].
    ///
    /// Path: `m/83696968'/505153'/coin_type'/4'/account'/index'`. The returned
    /// buffer is secret and zeroizes on drop; do not log it.
    pub fn ml_dsa87_seed_for(&self, scope: &Scope) -> Result<Zeroizing<[u8; 32]>> {
        self.derive_seed(&scope.path(ML_DSA_87))
    }

    /// Derive the 48-byte SLH-DSA-SHA2-128s seed for a [`Scope`].
    ///
    /// Path: `m/83696968'/505153'/coin_type'/2'/account'/index'`, split as
    /// `SK.seed || SK.prf || PK.seed`. The returned buffer is secret and
    /// zeroizes on drop; do not log it.
    pub fn slh_dsa_sha2_128s_seed_for(&self, scope: &Scope) -> Result<Zeroizing<[u8; 48]>> {
        self.derive_seed(&scope.path(SLH_DSA_SHA2_128S))
    }

    /// Derive an ML-DSA-44 signing key, from which the public key is available
    /// through [`ml_dsa::Keypair::verifying_key`].
    #[cfg(feature = "ml-dsa")]
    pub fn ml_dsa44(&self) -> Result<ml_dsa::SigningKey<ml_dsa::MlDsa44>> {
        let seed = self.ml_dsa44_seed()?;
        Ok(ml_dsa_key(&seed))
    }

    /// Derive an ML-DSA-65 signing key using the `pqseed` algorithm ID 3.
    ///
    /// The public key is available through [`ml_dsa::Keypair::verifying_key`].
    #[cfg(feature = "ml-dsa")]
    pub fn ml_dsa65(&self) -> Result<ml_dsa::SigningKey<ml_dsa::MlDsa65>> {
        let seed = self.ml_dsa65_seed()?;
        Ok(ml_dsa_key(&seed))
    }

    /// Derive an ML-DSA-87 signing key using the `pqseed` algorithm ID 4.
    ///
    /// The public key is available through [`ml_dsa::Keypair::verifying_key`].
    #[cfg(feature = "ml-dsa")]
    pub fn ml_dsa87(&self) -> Result<ml_dsa::SigningKey<ml_dsa::MlDsa87>> {
        let seed = self.ml_dsa87_seed()?;
        Ok(ml_dsa_key(&seed))
    }

    /// Derive an SLH-DSA-SHA2-128s signing key, from which the public key is
    /// available through [`slh_dsa::signature::Keypair::verifying_key`].
    #[cfg(feature = "slh-dsa")]
    pub fn slh_dsa_sha2_128s(&self) -> Result<slh_dsa::SigningKey<slh_dsa::Sha2_128s>> {
        let seed = self.slh_dsa_sha2_128s_seed()?;
        Ok(slh_dsa_key(&seed))
    }

    /// Derive an ML-DSA-44 signing key for a [`Scope`].
    #[cfg(feature = "ml-dsa")]
    pub fn ml_dsa44_for(&self, scope: &Scope) -> Result<ml_dsa::SigningKey<ml_dsa::MlDsa44>> {
        let seed = self.ml_dsa44_seed_for(scope)?;
        Ok(ml_dsa_key(&seed))
    }

    /// Derive an ML-DSA-65 signing key for a [`Scope`].
    #[cfg(feature = "ml-dsa")]
    pub fn ml_dsa65_for(&self, scope: &Scope) -> Result<ml_dsa::SigningKey<ml_dsa::MlDsa65>> {
        let seed = self.ml_dsa65_seed_for(scope)?;
        Ok(ml_dsa_key(&seed))
    }

    /// Derive an ML-DSA-87 signing key for a [`Scope`].
    #[cfg(feature = "ml-dsa")]
    pub fn ml_dsa87_for(&self, scope: &Scope) -> Result<ml_dsa::SigningKey<ml_dsa::MlDsa87>> {
        let seed = self.ml_dsa87_seed_for(scope)?;
        Ok(ml_dsa_key(&seed))
    }

    /// Derive an SLH-DSA-SHA2-128s signing key for a [`Scope`].
    #[cfg(feature = "slh-dsa")]
    pub fn slh_dsa_sha2_128s_for(
        &self,
        scope: &Scope,
    ) -> Result<slh_dsa::SigningKey<slh_dsa::Sha2_128s>> {
        let seed = self.slh_dsa_sha2_128s_seed_for(scope)?;
        Ok(slh_dsa_key(&seed))
    }

    fn derive_seed<const N: usize>(&self, path: &[u32]) -> Result<Zeroizing<[u8; N]>> {
        let mut node = XPrv::new(&*self.seed)?;
        for &index in path {
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

#[cfg(feature = "ml-dsa")]
fn ml_dsa_key<P: ml_dsa::MlDsaParams>(seed: &[u8; 32]) -> ml_dsa::SigningKey<P> {
    ml_dsa::SigningKey::from_seed(seed.into())
}

#[cfg(feature = "slh-dsa")]
fn slh_dsa_key(seed: &[u8; 48]) -> slh_dsa::SigningKey<slh_dsa::Sha2_128s> {
    slh_dsa::SigningKey::slh_keygen_internal(&seed[..16], &seed[16..32], &seed[32..48])
}
