//! Extended public keys

use crate::{
    ChainCode, ChildNumber, Depth, Error, ExtendedKey, ExtendedPrivateKey, KeyFingerprint, Prefix,
    PrivateKey, PublicKey, PublicKeyBytes, Result,
};
use alloc::string::{String, ToString};
use core::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

/// Extended public keys derived using BIP32.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ExtendedPublicKey<K: PublicKey> {
    /// Derived public key
    public_key: K,

    /// Derivation depth
    depth: Depth,

    /// Key fingerprint of this key's parent
    parent_fingerprint: KeyFingerprint,

    /// Child number.
    child_number: ChildNumber,

    /// Chain code
    chain_code: ChainCode,
}

impl<K> ExtendedPublicKey<K>
where
    K: PublicKey,
{
    /// Serialize the derived public key as bytes.
    pub fn public_key(&self) -> &K {
        &self.public_key
    }

    /// Get the [`Depth`] of this extended private key.
    pub fn depth(&self) -> Depth {
        self.depth
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

    /// Serialize the raw public key as a byte array (e.g. SEC1-encoded).
    pub fn to_bytes(&self) -> PublicKeyBytes {
        self.public_key.to_bytes()
    }

    /// Serialize this key as an [`ExtendedKey`].
    pub fn to_extended_key(&self, prefix: Prefix) -> ExtendedKey {
        ExtendedKey {
            prefix,
            depth: self.depth,
            parent_fingerprint: self.parent_fingerprint,
            child_number: self.child_number,
            chain_code: self.chain_code,
            key_bytes: self.to_bytes(),
        }
    }

    /// Serialize this key as a `String`.
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
            depth: xprv.depth(),
            parent_fingerprint: xprv.parent_fingerprint(),
            child_number: xprv.child_number(),
            chain_code: *xprv.chain_code(),
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
            Ok(Self {
                public_key: PublicKey::from_bytes(extended_key.key_bytes)?,
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
