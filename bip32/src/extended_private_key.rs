//! Extended private keys

use crate::{
    extended_key::ExtendedKey, private_key::PrivateKey, ChainCode, ChildNumber, DerivationPath,
    Error, Result, KEY_SIZE,
};
use core::{
    convert::{TryFrom, TryInto},
    fmt::{self, Debug},
    str::FromStr,
};
use hkd32::mnemonic::Seed;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha512;
use subtle::{Choice, ConstantTimeEq};

/// Derivation depth.
pub type Depth = u8;

/// Derivation domain separator for BIP39 keys.
#[cfg_attr(docsrs, doc(cfg(feature = "bip39")))]
const BIP39_DOMAIN_SEPARATOR: [u8; 12] = [
    0x42, 0x69, 0x74, 0x63, 0x6f, 0x69, 0x6e, 0x20, 0x73, 0x65, 0x65, 0x64,
];

/// Extended private keys derived using BIP32.
#[derive(Clone)]
pub struct ExtendedPrivateKey<K: PrivateKey> {
    /// Derived private key
    private_key: K,

    /// Chain code
    chain_code: ChainCode,

    /// Derivation depth
    depth: Depth,
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
            chain_code: chain_code.try_into()?,
            depth: 0,
        })
    }

    /// Derive a child key for a particular [`ChildNumber`].
    pub fn derive_child(&self, child: ChildNumber) -> Result<Self> {
        let mut hmac =
            Hmac::<Sha512>::new_from_slice(&self.chain_code).map_err(|_| Error::Crypto)?;

        if child.is_hardened() {
            hmac.update(&[0]);
            hmac.update(&self.private_key.to_bytes());
        } else {
            hmac.update(self.private_key.public_key().as_ref());
        }

        hmac.update(&child.to_bytes());

        let result = hmac.finalize().into_bytes();
        let (secret_key, chain_code) = result.split_at(KEY_SIZE);

        Ok(ExtendedPrivateKey {
            private_key: self.private_key.derive_child(secret_key.try_into()?)?,
            chain_code: chain_code.try_into()?,
            depth: self.depth.checked_add(1).ok_or(Error::Depth)?,
        })
    }

    /// Borrow the derived private key value.
    pub fn private_key(&self) -> &K {
        &self.private_key
    }

    /// Serialize the derived public key as bytes.
    pub fn public_key(&self) -> K::PublicKey {
        self.private_key.public_key()
    }

    /// Borrow the chain code for this extended private key.
    pub fn chain_code(&self) -> &ChainCode {
        &self.chain_code
    }

    /// Get the [`Depth`] of this extended private key.
    pub fn depth(&self) -> Depth {
        self.depth
    }

    /// Serialize this key as a byte array.
    pub fn to_bytes(&self) -> [u8; KEY_SIZE] {
        self.private_key.to_bytes()
    }
}

impl<K> ConstantTimeEq for ExtendedPrivateKey<K>
where
    K: PrivateKey,
{
    fn ct_eq(&self, other: &Self) -> Choice {
        // TODO(tarcieri): add `ConstantTimeEq` bound to `PrivateKey`
        self.to_bytes().ct_eq(&other.to_bytes())
            & self.chain_code.ct_eq(&other.chain_code)
            & self.depth.ct_eq(&other.depth)
    }
}

impl<K> Debug for ExtendedPrivateKey<K>
where
    K: PrivateKey,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExtendedPrivateKey")
            .field("private_key", &"...")
            .field("chain_code", &"...")
            .field("depth", &self.depth)
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
                chain_code: extended_key.chain_code,
                private_key: PrivateKey::from_bytes(extended_key.key_bytes[1..].try_into()?)?,
                depth: extended_key.depth,
            })
        } else {
            Err(Error::Crypto)
        }
    }
}
