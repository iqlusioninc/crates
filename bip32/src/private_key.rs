//! Trait for deriving child keys on a given type.

use crate::{PublicKey, Result, KEY_SIZE};

#[cfg(feature = "secp256k1")]
use crate::Error;

/// Bytes which represent a private key.
pub type PrivateKeyBytes = [u8; KEY_SIZE];

/// Trait for key types which can be derived using BIP32.
// TODO(tarcieri): add `ConstantTimeEq` bound
pub trait PrivateKey: Sized {
    /// Public key type which corresponds to this private key.
    type PublicKey: PublicKey;

    /// Initialize this key from bytes.
    fn from_bytes(bytes: &PrivateKeyBytes) -> Result<Self>;

    /// Serialize this key as bytes.
    fn to_bytes(&self) -> PrivateKeyBytes;

    /// Derive a child key from a parent key and the left-half of the output
    /// of HMAC-SHA512 (where `left_half` is referred to as "I sub L" in BIP32)
    fn derive_child(&self, left_half: PrivateKeyBytes) -> Result<Self>;

    /// Get the [`PublicKey`] that corresponds to this private key.
    fn public_key(&self) -> Self::PublicKey;
}

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
impl PrivateKey for k256::SecretKey {
    type PublicKey = k256::PublicKey;

    fn from_bytes(bytes: &PrivateKeyBytes) -> Result<Self> {
        Ok(k256::SecretKey::from_bytes(bytes)?)
    }

    fn to_bytes(&self) -> PrivateKeyBytes {
        k256::SecretKey::to_bytes(self).into()
    }

    fn derive_child(&self, left_half: PrivateKeyBytes) -> Result<Self> {
        let child_scalar = k256::NonZeroScalar::from_repr(left_half.into()).ok_or(Error::Crypto)?;
        let derived_scalar = self.secret_scalar().as_ref() + child_scalar.as_ref();

        k256::NonZeroScalar::new(derived_scalar)
            .map(Self::new)
            .ok_or(Error::Crypto)
    }

    fn public_key(&self) -> Self::PublicKey {
        k256::SecretKey::public_key(self)
    }
}

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
impl PrivateKey for k256::ecdsa::SigningKey {
    type PublicKey = k256::ecdsa::VerifyingKey;

    fn from_bytes(bytes: &PrivateKeyBytes) -> Result<Self> {
        Ok(k256::ecdsa::SigningKey::from_bytes(bytes)?)
    }

    fn to_bytes(&self) -> PrivateKeyBytes {
        k256::ecdsa::SigningKey::to_bytes(self).into()
    }

    fn derive_child(&self, left_half: PrivateKeyBytes) -> Result<Self> {
        k256::SecretKey::from(self)
            .derive_child(left_half)
            .map(Into::into)
    }

    fn public_key(&self) -> Self::PublicKey {
        self.verify_key()
    }
}
