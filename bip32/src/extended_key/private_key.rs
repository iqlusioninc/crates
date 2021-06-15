//! Extended private keys

use crate::{
    ChainCode, ChildNumber, Depth, DerivationPath, Error, ExtendedKey, ExtendedPublicKey,
    KeyFingerprint, Prefix, PrivateKey, PrivateKeyBytes, PublicKey, Result, KEY_SIZE,
};
use alloc::string::{String, ToString};
use core::{
    convert::{TryFrom, TryInto},
    fmt::{self, Debug},
    str::FromStr,
};
use hkd32::mnemonic::Seed;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha512;
use subtle::{Choice, ConstantTimeEq};
use zeroize::Zeroizing;

/// Derivation domain separator for BIP39 keys.
#[cfg_attr(docsrs, doc(cfg(feature = "bip39")))]
const BIP39_DOMAIN_SEPARATOR: [u8; 12] = [
    0x42, 0x69, 0x74, 0x63, 0x6f, 0x69, 0x6e, 0x20, 0x73, 0x65, 0x65, 0x64,
];

/// Extended private secp256k1 ECDSA signing key.
#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
pub type XPrv = ExtendedPrivateKey<k256::ecdsa::SigningKey>;

/// Extended private keys derived using BIP32.
///
/// Generic around a [`PrivateKey`] type. When the `secp256k1` feature of this
/// crate is enabled, the [`XPrv`] type provides a convenient alias for
/// extended ECDSA/secp256k1 private keys.
#[derive(Clone)]
pub struct ExtendedPrivateKey<K: PrivateKey> {
    /// Derived private key
    private_key: K,

    /// Derivation depth
    depth: Depth,

    /// Key fingerprint of this key's parent
    parent_fingerprint: KeyFingerprint,

    /// Child number.
    child_number: ChildNumber,

    /// Chain code
    chain_code: ChainCode,
}

impl<K> ExtendedPrivateKey<K>
where
    K: PrivateKey,
{
    /// Maximum derivation depth.
    pub const MAX_DEPTH: Depth = u8::MAX;

    /// Derive a child key from the given [`DerivationPath`].
    pub fn derive_child_from_path<S>(seed: S, path: &DerivationPath) -> Result<Self>
    where
        S: AsRef<[u8; Seed::SIZE]>,
    {
        path.as_ref()
            .iter()
            .fold(Self::new(seed), |sk, &child_num| {
                sk?.derive_child(child_num)
            })
    }

    /// Create the root extended key for the given seed value.
    pub fn new<S>(seed: S) -> Result<Self>
    where
        S: AsRef<[u8; Seed::SIZE]>,
    {
        // TODO(tarcieri): unify this with the equivalent logic in `hkd32`
        let mut hmac = Hmac::<Sha512>::new_from_slice(&BIP39_DOMAIN_SEPARATOR)?;
        hmac.update(seed.as_ref());

        let result = hmac.finalize().into_bytes();
        let (secret_key, chain_code) = result.split_at(KEY_SIZE);

        Ok(ExtendedPrivateKey {
            private_key: PrivateKey::from_bytes(secret_key.try_into()?)?,
            depth: 0,
            parent_fingerprint: KeyFingerprint::default(),
            child_number: ChildNumber::default(),
            chain_code: chain_code.try_into()?,
        })
    }

    /// Derive a child key for a particular [`ChildNumber`].
    pub fn derive_child(&self, child_number: ChildNumber) -> Result<Self> {
        let depth = self.depth.checked_add(1).ok_or(Error::Depth)?;

        let mut hmac =
            Hmac::<Sha512>::new_from_slice(&self.chain_code).map_err(|_| Error::Crypto)?;

        if child_number.is_hardened() {
            hmac.update(&[0]);
            hmac.update(&self.private_key.to_bytes());
        } else {
            hmac.update(&self.private_key.public_key().to_bytes());
        }

        hmac.update(&child_number.to_bytes());

        let result = hmac.finalize().into_bytes();
        let (secret_key, chain_code) = result.split_at(KEY_SIZE);

        // We should technically loop here if a `secret_key` is zero or overflows
        // the order of the underlying elliptic curve group, however per
        // "Child key derivation (CKD) functions":
        // https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#child-key-derivation-ckd-functions
        //
        // > "Note: this has probability lower than 1 in 2^127."
        //
        // ...so instead, we simply return an error if this were ever to happen,
        // as the chances of it happening are vanishingly small.
        let private_key = self.private_key.derive_child(secret_key.try_into()?)?;

        Ok(ExtendedPrivateKey {
            private_key,
            parent_fingerprint: self.private_key.public_key().fingerprint(),
            child_number,
            chain_code: chain_code.try_into()?,
            depth,
        })
    }

    /// Borrow the derived private key value.
    pub fn private_key(&self) -> &K {
        &self.private_key
    }

    /// Serialize the derived public key as bytes.
    pub fn public_key(&self) -> ExtendedPublicKey<K::PublicKey> {
        self.into()
    }

    /// Key fingerprint of this key's parent.
    pub fn parent_fingerprint(&self) -> KeyFingerprint {
        self.parent_fingerprint
    }

    /// Child number used to derive this key from its parent.
    pub fn child_number(&self) -> ChildNumber {
        self.child_number
    }

    /// Borrow the chain code for this extended private key.
    pub fn chain_code(&self) -> &ChainCode {
        &self.chain_code
    }

    /// Get the [`Depth`] of this extended private key.
    pub fn depth(&self) -> Depth {
        self.depth
    }

    /// Serialize the raw private key as a byte array.
    pub fn to_bytes(&self) -> PrivateKeyBytes {
        self.private_key.to_bytes()
    }

    /// Serialize this key as an [`ExtendedKey`].
    pub fn to_extended_key(&self, prefix: Prefix) -> ExtendedKey {
        // Add leading `0` byte
        let mut key_bytes = [0u8; KEY_SIZE + 1];
        key_bytes[1..].copy_from_slice(&self.to_bytes());

        ExtendedKey {
            prefix,
            depth: self.depth,
            parent_fingerprint: self.parent_fingerprint,
            child_number: self.child_number,
            chain_code: self.chain_code,
            key_bytes,
        }
    }

    /// Serialize this key as a self-[`Zeroizing`] `String`.
    pub fn to_string(&self, prefix: Prefix) -> Zeroizing<String> {
        Zeroizing::new(self.to_extended_key(prefix).to_string())
    }
}

impl<K> ConstantTimeEq for ExtendedPrivateKey<K>
where
    K: PrivateKey,
{
    fn ct_eq(&self, other: &Self) -> Choice {
        // TODO(tarcieri): add `ConstantTimeEq` bound to `PrivateKey`
        self.to_bytes().ct_eq(&other.to_bytes())
            & self.depth.ct_eq(&other.depth)
            & self.parent_fingerprint.ct_eq(&other.parent_fingerprint)
            & self.child_number.0.ct_eq(&other.child_number.0)
            & self.chain_code.ct_eq(&other.chain_code)
    }
}

impl<K> Debug for ExtendedPrivateKey<K>
where
    K: PrivateKey,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExtendedPrivateKey")
            .field("private_key", &"...")
            .field("depth", &self.depth)
            .field("parent_fingerprint", &self.parent_fingerprint)
            .field("chain_code", &self.chain_code)
            .finish()
    }
}

/// NOTE: uses [`ConstantTimeEq`] internally
impl<K> Eq for ExtendedPrivateKey<K> where K: PrivateKey {}

/// NOTE: uses [`ConstantTimeEq`] internally
impl<K> PartialEq for ExtendedPrivateKey<K>
where
    K: PrivateKey,
{
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

impl<K> FromStr for ExtendedPrivateKey<K>
where
    K: PrivateKey,
{
    type Err = Error;

    fn from_str(xprv: &str) -> Result<Self> {
        ExtendedKey::from_str(xprv)?.try_into()
    }
}

impl<K> TryFrom<ExtendedKey> for ExtendedPrivateKey<K>
where
    K: PrivateKey,
{
    type Error = Error;

    fn try_from(extended_key: ExtendedKey) -> Result<ExtendedPrivateKey<K>> {
        if extended_key.prefix.is_private() && extended_key.key_bytes[0] == 0 {
            Ok(Self {
                private_key: PrivateKey::from_bytes(extended_key.key_bytes[1..].try_into()?)?,
                depth: extended_key.depth,
                parent_fingerprint: extended_key.parent_fingerprint,
                child_number: extended_key.child_number,
                chain_code: extended_key.chain_code,
            })
        } else {
            Err(Error::Crypto)
        }
    }
}
