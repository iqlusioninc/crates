//! Extended public keys

use crate::{
    Error, ExtendedKey, ExtendedKeyAttrs, ExtendedPrivateKey, KeyFingerprint, Prefix, PrivateKey,
    PublicKey, PublicKeyBytes, Result,
};
use core::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};

/// Extended public secp256k1 ECDSA verification key.
#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
pub type XPub = ExtendedPublicKey<k256::ecdsa::VerifyingKey>;

/// Extended public keys derived using BIP32.
///
/// Generic around a [`PublicKey`] type. When the `secp256k1` feature of this
/// crate is enabled, the [`XPub`] type provides a convenient alias for
/// extended ECDSA/secp256k1 public keys.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ExtendedPublicKey<K: PublicKey> {
    /// Derived public key
    public_key: K,

    /// Extended key attributes.
    attrs: ExtendedKeyAttrs,
}

impl<K> ExtendedPublicKey<K>
where
    K: PublicKey,
{
    /// Serialize the derived public key as bytes.
    pub fn public_key(&self) -> &K {
        &self.public_key
    }

    /// Get attributes for this key such as depth, parent fingerprint,
    /// child number, and chain code.
    pub fn attrs(&self) -> &ExtendedKeyAttrs {
        &self.attrs
    }

    /// Compute a 4-byte key fingerprint for this extended public key.
    pub fn fingerprint(&self) -> KeyFingerprint {
        self.public_key().fingerprint()
    }

    /// Serialize the raw public key as a byte array (e.g. SEC1-encoded).
    pub fn to_bytes(&self) -> PublicKeyBytes {
        self.public_key.to_bytes()
    }

    /// Serialize this key as an [`ExtendedKey`].
    pub fn to_extended_key(&self, prefix: Prefix) -> ExtendedKey {
        ExtendedKey {
            prefix,
            attrs: self.attrs.clone(),
            key_bytes: self.to_bytes(),
        }
    }

    /// Serialize this key as a `String`.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn to_string(&self, prefix: Prefix) -> String {
        self.to_extended_key(prefix).to_string()
    }
}

impl<K> From<&ExtendedPrivateKey<K>> for ExtendedPublicKey<K::PublicKey>
where
    K: PrivateKey,
{
    fn from(xprv: &ExtendedPrivateKey<K>) -> ExtendedPublicKey<K::PublicKey> {
        ExtendedPublicKey {
            public_key: xprv.private_key().public_key(),
            attrs: xprv.attrs().clone(),
        }
    }
}

impl<K> FromStr for ExtendedPublicKey<K>
where
    K: PublicKey,
{
    type Err = Error;

    fn from_str(xpub: &str) -> Result<Self> {
        ExtendedKey::from_str(xpub)?.try_into()
    }
}

impl<K> TryFrom<ExtendedKey> for ExtendedPublicKey<K>
where
    K: PublicKey,
{
    type Error = Error;

    fn try_from(extended_key: ExtendedKey) -> Result<ExtendedPublicKey<K>> {
        if extended_key.prefix.is_public() {
            Ok(ExtendedPublicKey {
                public_key: PublicKey::from_bytes(extended_key.key_bytes)?,
                attrs: extended_key.attrs.clone(),
            })
        } else {
            Err(Error::Crypto)
        }
    }
}
