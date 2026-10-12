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
use sha3::{Shake256, digest::ExtendableOutput};
use zeroize::{Zeroize, Zeroizing};

/// BIP85 purpose: `83696968'`.
const BIP85: u32 = 83_696_968;

/// Project Eleven's BIP85 application number: `503131'` ("P11").
const P11_APP: u32 = 503_131;

// The only supported unscoped Project Eleven ID.
const P11_ML_DSA_44: u32 = 0;

/// The [pqhd draft]'s proposed BIP85 application number for scoped keys: `505153'`.
///
/// The decimal digits are the hex bytes of ASCII "PQS", following the
/// convention behind Project Eleven's `503131'` ("P11"). This application
/// number is not yet registered with BIP85. Do not use draft keys for real assets.
///
/// [pqhd draft]: https://github.com/pqdh/pqhd/blob/befdce14cecbb5d485d3239982bfa5b207cdb922/spec/README.md
pub const PQSEED_APP: u32 = 505_153;

// Active pqhd scheme IDs, separate from the unscoped Project Eleven IDs.
const ML_DSA_44: u32 = 200;
const ML_DSA_65: u32 = 201;
const ML_DSA_87: u32 = 202;
const SLH_DSA_SHA2_128S: u32 = 300;

/// A coin type, account, and key index for scoped derivation.
///
/// Scoped keys follow [pqhd]: they live at
/// `m/83696968'/505153'/coin_type'/scheme'/account'/index'`, and the BIP85
/// entropy there seeds BIP85-DRNG-SHAKE256. `coin_type` is a [SLIP-44]
/// registered coin type. Every level is hardened, so each value must be below
/// 2<sup>31</sup>. Bounds are checked when deriving a key, not on construction.
///
/// [pqhd]: https://github.com/pqdh/pqhd/blob/befdce14cecbb5d485d3239982bfa5b207cdb922/spec/README.md
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

/// SLH-DSA parameter sets in the [pqhd registry], for scoped derivation.
///
/// [pqhd registry]: https://github.com/pqdh/pqhd/blob/befdce14cecbb5d485d3239982bfa5b207cdb922/registry/README.md
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SlhDsa {
    /// SLH-DSA-SHA2-128s, pqhd scheme 300.
    Sha2_128s,
    /// SLH-DSA-SHAKE-128s, pqhd scheme 306.
    Shake128s,
    /// SLH-DSA-SHA2-128f, pqhd scheme 301.
    Sha2_128f,
    /// SLH-DSA-SHAKE-128f, pqhd scheme 307.
    Shake128f,
    /// SLH-DSA-SHA2-192s, pqhd scheme 302.
    Sha2_192s,
    /// SLH-DSA-SHAKE-192s, pqhd scheme 308.
    Shake192s,
    /// SLH-DSA-SHA2-192f, pqhd scheme 303.
    Sha2_192f,
    /// SLH-DSA-SHAKE-192f, pqhd scheme 309.
    Shake192f,
    /// SLH-DSA-SHA2-256s, pqhd scheme 304.
    Sha2_256s,
    /// SLH-DSA-SHAKE-256s, pqhd scheme 310.
    Shake256s,
    /// SLH-DSA-SHA2-256f, pqhd scheme 305.
    Sha2_256f,
    /// SLH-DSA-SHAKE-256f, pqhd scheme 311.
    Shake256f,
}

impl SlhDsa {
    /// The pqhd scheme index: the `scheme'` level of the derivation path.
    pub const fn scheme(self) -> u32 {
        match self {
            Self::Sha2_128s => SLH_DSA_SHA2_128S,
            Self::Shake128s => 306,
            Self::Sha2_128f => 301,
            Self::Shake128f => 307,
            Self::Sha2_192s => 302,
            Self::Shake192s => 308,
            Self::Sha2_192f => 303,
            Self::Shake192f => 309,
            Self::Sha2_256s => 304,
            Self::Shake256s => 310,
            Self::Sha2_256f => 305,
            Self::Shake256f => 311,
        }
    }

    /// The FIPS 205 security parameter `n` in bytes: 16, 24, or 32.
    pub const fn n(self) -> usize {
        match self {
            Self::Sha2_128s | Self::Shake128s | Self::Sha2_128f | Self::Shake128f => 16,
            Self::Sha2_192s | Self::Shake192s | Self::Sha2_192f | Self::Shake192f => 24,
            Self::Sha2_256s | Self::Shake256s | Self::Sha2_256f | Self::Shake256f => 32,
        }
    }
}

/// Secret SLH-DSA key-generation input: `SK.seed || SK.prf || PK.seed`.
///
/// Zeroized on drop. Its `Debug` output is redacted; do not log its bytes.
pub struct SlhDsaSeed {
    set: SlhDsa,
    bytes: Zeroizing<[u8; 96]>,
}

impl SlhDsaSeed {
    /// The parameter set this seed was derived for.
    pub fn set(&self) -> SlhDsa {
        self.set
    }

    /// All `3n` bytes: `SK.seed || SK.prf || PK.seed`.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..3 * self.set.n()]
    }

    /// `SK.seed`, the first `n` bytes.
    pub fn sk_seed(&self) -> &[u8] {
        &self.bytes[..self.set.n()]
    }

    /// `SK.prf`, the second `n` bytes.
    pub fn sk_prf(&self) -> &[u8] {
        let n = self.set.n();
        &self.bytes[n..2 * n]
    }

    /// `PK.seed`, the third `n` bytes.
    pub fn pk_seed(&self) -> &[u8] {
        let n = self.set.n();
        &self.bytes[2 * n..3 * n]
    }
}

impl fmt::Debug for SlhDsaSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlhDsaSeed")
            .field("set", &self.set)
            .finish_non_exhaustive()
    }
}

/// RustCrypto SLH-DSA parameter sets that have a pqhd scheme index.
///
/// Implemented for every `slh_dsa` parameter set; this trait is sealed.
#[cfg(feature = "slh-dsa")]
pub trait SlhDsaParams: slh_dsa::ParameterSet + sealed::Sealed {
    /// The corresponding registry entry.
    const SET: SlhDsa;
}

#[cfg(feature = "slh-dsa")]
mod sealed {
    pub trait Sealed {}
}

#[cfg(feature = "slh-dsa")]
macro_rules! slh_dsa_params {
    ($($set:ident),* $(,)?) => {$(
        impl sealed::Sealed for slh_dsa::$set {}
        impl SlhDsaParams for slh_dsa::$set {
            const SET: SlhDsa = SlhDsa::$set;
        }
    )*};
}

#[cfg(feature = "slh-dsa")]
slh_dsa_params!(
    Sha2_128s, Shake128s, Sha2_128f, Shake128f, Sha2_192s, Shake192s, Sha2_192f, Shake192f,
    Sha2_256s, Shake256s, Sha2_256f, Shake256f,
);

/// Derives post-quantum keys using the pqhd draft and Project Eleven's scheme.
///
/// Holds a secret 64-byte seed, zeroized on drop. No extended public keys or
/// non-hardened derivation are exposed. Unscoped ML-DSA-44 methods use the
/// article's fixed path; `*_for` methods derive [pqhd] keys scoped
/// to a coin type, account, and index.
///
/// Scoped methods implement active draft registry entries 200-202 and 300-311: ML-DSA
/// and every SLH-DSA parameter set.
/// An invalid BIP32 master or child key returns an error without skipping to
/// another index. Scoped and unscoped derivations must not be interchanged.
///
/// [pqhd]: https://github.com/pqdh/pqhd/blob/befdce14cecbb5d485d3239982bfa5b207cdb922/spec/README.md
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
        self.derive_seed(&[BIP85, P11_APP, P11_ML_DSA_44])
    }

    /// Derive the 32-byte ML-DSA-44 seed for a [`Scope`].
    ///
    /// Path: `m/83696968'/505153'/coin_type'/200'/account'/index'`. The returned
    /// buffer is secret and zeroizes on drop; do not log it.
    pub fn ml_dsa44_seed_for(&self, scope: &Scope) -> Result<Zeroizing<[u8; 32]>> {
        self.derive_scoped(&scope.path(ML_DSA_44))
    }

    /// Derive the 32-byte ML-DSA-65 seed for a [`Scope`].
    ///
    /// Path: `m/83696968'/505153'/coin_type'/201'/account'/index'`. The returned
    /// buffer is secret and zeroizes on drop; do not log it.
    pub fn ml_dsa65_seed_for(&self, scope: &Scope) -> Result<Zeroizing<[u8; 32]>> {
        self.derive_scoped(&scope.path(ML_DSA_65))
    }

    /// Derive the 32-byte ML-DSA-87 seed for a [`Scope`].
    ///
    /// Path: `m/83696968'/505153'/coin_type'/202'/account'/index'`. The returned
    /// buffer is secret and zeroizes on drop; do not log it.
    pub fn ml_dsa87_seed_for(&self, scope: &Scope) -> Result<Zeroizing<[u8; 32]>> {
        self.derive_scoped(&scope.path(ML_DSA_87))
    }

    /// Derive the 48-byte SLH-DSA-SHA2-128s seed for a [`Scope`].
    ///
    /// Path: `m/83696968'/505153'/coin_type'/300'/account'/index'`, split as
    /// `SK.seed || SK.prf || PK.seed`. The returned buffer is secret and
    /// zeroizes on drop; do not log it.
    pub fn slh_dsa_sha2_128s_seed_for(&self, scope: &Scope) -> Result<Zeroizing<[u8; 48]>> {
        self.derive_scoped(&scope.path(SLH_DSA_SHA2_128S))
    }

    /// Derive the SLH-DSA key-generation seed for any registry parameter set
    /// and [`Scope`], at `m/83696968'/505153'/coin_type'/scheme'/account'/index'`
    /// with `scheme` from [`SlhDsa::scheme`].
    ///
    /// Available without the `slh-dsa` backend.
    pub fn slh_dsa_seed_for(&self, set: SlhDsa, scope: &Scope) -> Result<SlhDsaSeed> {
        // SHAKE256 output is prefix-consistent: the first 3n of these 96
        // bytes are exactly the first 3n bytes pqhd specifies for this set.
        let bytes = self.derive_scoped(&scope.path(set.scheme()))?;
        Ok(SlhDsaSeed { set, bytes })
    }

    /// Derive an ML-DSA-44 signing key, from which the public key is available
    /// through [`ml_dsa::Keypair::verifying_key`].
    #[cfg(feature = "ml-dsa")]
    pub fn ml_dsa44(&self) -> Result<ml_dsa::SigningKey<ml_dsa::MlDsa44>> {
        let seed = self.ml_dsa44_seed()?;
        Ok(ml_dsa_key(&seed))
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

    /// Derive an SLH-DSA signing key for any registry parameter set and
    /// [`Scope`], for example `keys.slh_dsa_for::<slh_dsa::Shake256f>(&scope)`.
    #[cfg(feature = "slh-dsa")]
    pub fn slh_dsa_for<P: SlhDsaParams>(&self, scope: &Scope) -> Result<slh_dsa::SigningKey<P>> {
        let seed = self.slh_dsa_seed_for(P::SET, scope)?;
        Ok(slh_dsa::SigningKey::slh_keygen_internal(
            seed.sk_seed(),
            seed.sk_prf(),
            seed.pk_seed(),
        ))
    }

    /// Project Eleven: the first `N` bytes of the BIP85 entropy.
    fn derive_seed<const N: usize>(&self, path: &[u32]) -> Result<Zeroizing<[u8; N]>> {
        let entropy = self.bip85_entropy(path)?;
        let mut seed = Zeroizing::new([0u8; N]);
        seed.copy_from_slice(&entropy[..N]);
        Ok(seed)
    }

    /// pqhd: the first `N` bytes of BIP85-DRNG-SHAKE256 seeded with the
    /// 64-byte BIP85 entropy.
    fn derive_scoped<const N: usize>(&self, path: &[u32]) -> Result<Zeroizing<[u8; N]>> {
        let entropy = self.bip85_entropy(path)?;
        let mut seed = Zeroizing::new([0u8; N]);
        Shake256::digest_xof(entropy.as_slice(), &mut *seed);
        Ok(seed)
    }

    /// BIP85: HMAC-SHA512 keyed with `bip-entropy-from-k` over the private key
    /// at a fully hardened path.
    fn bip85_entropy(&self, path: &[u32]) -> Result<Zeroizing<[u8; 64]>> {
        let mut node = XPrv::new(&*self.seed)?;
        for &index in path {
            node = node.derive_child(ChildNumber::new(index, true)?)?;
        }

        let private_key = Zeroizing::new(node.to_bytes());
        let mut mac = Hmac::<Sha512>::new_from_slice(b"bip-entropy-from-k")
            .expect("HMAC accepts this fixed key length");
        mac.update(&*private_key);
        let mut digest = mac.finalize().into_bytes();
        let mut entropy = Zeroizing::new([0u8; 64]);
        entropy.copy_from_slice(&digest);
        digest.as_mut_slice().zeroize();
        Ok(entropy)
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
