//! Extended secret keys

use crate::{
    secret_key::SecretKey, ChainCode, ChildNumber, DerivationPath, Error, Result, KEY_SIZE,
};
use core::{convert::TryInto, str::FromStr};
use hkd32::BIP39_BASE_DERIVATION_KEY;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha512;

/// Derivation depth.
pub type Depth = u8;

/// Extended secret keys derived using BIP32.
#[derive(Clone, Debug)]
pub struct ExtendedSecretKey<K: SecretKey> {
    /// Secret key
    secret_key: K,

    /// Chain code
    chain_code: ChainCode,

    /// Derivation depth
    depth: Depth,
}

impl<K> ExtendedSecretKey<K>
where
    K: SecretKey,
{
    /// Maximum derivation depth.
    pub const MAX_DEPTH: Depth = u8::MAX;

    /// Derive a child key from the given [`DerivationPath`].
    pub fn derive_child_from_path(seed: &[u8], path: &DerivationPath) -> Result<Self> {
        // TODO(tarcieri): unify this with the equivalent logic in `hkd32`
        let mut hmac = Hmac::<Sha512>::new_from_slice(&BIP39_BASE_DERIVATION_KEY)?;
        hmac.update(seed);

        let result = hmac.finalize().into_bytes();
        let (secret_key, chain_code) = result.split_at(KEY_SIZE);

        let mut sk = ExtendedSecretKey {
            secret_key: SecretKey::from_bytes(secret_key.try_into()?)?,
            chain_code: chain_code.try_into()?,
            depth: 0,
        };

        for child in path.as_ref() {
            sk = sk.derive_child(*child)?;
        }

        Ok(sk)
    }

    /// Derive a child key for a particular [`ChildNumber`].
    pub fn derive_child(&self, child: ChildNumber) -> Result<Self> {
        let mut hmac: Hmac<Sha512> = Hmac::new_from_slice(&self.chain_code).map_err(|_| Error)?;

        if child.is_hardened() {
            hmac.update(&[0]);
            hmac.update(&self.secret_key.to_bytes());
        } else {
            hmac.update(self.secret_key.public_key().as_ref());
        }

        hmac.update(&child.to_bytes());

        let result = hmac.finalize().into_bytes();
        let (secret_key, chain_code) = result.split_at(KEY_SIZE);

        Ok(ExtendedSecretKey {
            secret_key: self.secret_key.derive_child(secret_key.try_into()?)?,
            chain_code: chain_code.try_into()?,
            depth: self.depth.checked_add(1).ok_or(Error)?,
        })
    }

    /// Borrow the derived secret key value.
    pub fn secret_key(&self) -> &K {
        &self.secret_key
    }

    /// Serialize the derived public key as bytes.
    pub fn public_key(&self) -> K::PublicKey {
        self.secret_key.public_key()
    }

    /// Borrow the chain code for this extended secret key.
    pub fn chain_code(&self) -> &ChainCode {
        &self.chain_code
    }

    /// Get the [`Depth`] of this extended secret key.
    pub fn depth(&self) -> Depth {
        self.depth
    }

    /// Serialize this key as a byte array.
    pub fn to_bytes(&self) -> [u8; KEY_SIZE] {
        self.secret_key.to_bytes()
    }
}

impl<K> FromStr for ExtendedSecretKey<K>
where
    K: SecretKey,
{
    type Err = Error;

    // TODO(tarcieri): yprv, zprv
    fn from_str(xprv: &str) -> Result<Self> {
        let data = bs58::decode(xprv).into_vec().map_err(|_| Error)?;

        if data.len() == 82 {
            Ok(ExtendedSecretKey {
                chain_code: data[13..45].try_into()?,
                secret_key: SecretKey::from_bytes(data[46..78].try_into()?)?,
                depth: data[4],
            })
        } else {
            Err(Error)
        }
    }
}
