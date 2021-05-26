//! Trait for deriving child keys on a given type.

use crate::{Result, KEY_SIZE};

/// Bytes which represent a private key.
type KeyBytes = [u8; KEY_SIZE];

/// Trait for key types which can be derived using BIP32.
pub trait PrivateKey: Sized {
    /// Serialized public key type.
    type PublicKey: AsRef<[u8]> + Sized;

    /// Initialize this key from a the provided bytes, interpreting the
    /// provided value as a big endian integer and reducing it mod n.
    ///
    /// Returns `None` if the provided value is equal to zero.
    fn from_bytes(bytes: &KeyBytes) -> Option<Self>;

    /// Serialize this key as bytes.
    fn to_bytes(&self) -> KeyBytes;

    /// Derive a child key from this key and a provided [`ChainCode`].
    fn derive_child(&self, derivation_key: &KeyBytes) -> Result<Self>;

    /// Serialize the public key for this [`SecretKey`].
    fn public_key(&self) -> Self::PublicKey;
}

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
impl PrivateKey for k256::SecretKey {
    type PublicKey = [u8; 33];

    fn from_bytes(bytes: &KeyBytes) -> Option<Self> {
        let scalar = k256::Scalar::from_bytes_reduced(bytes.into());
        Some(k256::SecretKey::new(k256::NonZeroScalar::new(scalar)?))
    }

    fn to_bytes(&self) -> KeyBytes {
        k256::SecretKey::to_bytes(self).into()
    }

    fn derive_child(&self, derivation_key: &KeyBytes) -> Result<Self> {
        let child_scalar = k256::Scalar::from_bytes_reduced(derivation_key.into());
        let derived_scalar = self.secret_scalar().as_ref() + child_scalar;

        k256::NonZeroScalar::new(derived_scalar)
            .map(Self::new)
            .ok_or(crate::Error)
    }

    fn public_key(&self) -> Self::PublicKey {
        use core::convert::TryInto;
        use k256::elliptic_curve::sec1::ToEncodedPoint;

        self.public_key()
            .to_encoded_point(true)
            .as_ref()
            .try_into()
            .expect("malformed public key")
    }
}

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
impl PrivateKey for k256::ecdsa::SigningKey {
    type PublicKey = [u8; 33];

    fn from_bytes(bytes: &KeyBytes) -> Option<Self> {
        k256::NonZeroScalar::new(k256::Scalar::from_bytes_reduced(bytes.into())).map(Into::into)
    }

    fn to_bytes(&self) -> KeyBytes {
        k256::ecdsa::SigningKey::to_bytes(self).into()
    }

    fn derive_child(&self, derivation_key: &KeyBytes) -> Result<Self> {
        k256::SecretKey::from(self)
            .derive_child(derivation_key)
            .map(Into::into)
    }

    fn public_key(&self) -> Self::PublicKey {
        PrivateKey::public_key(&k256::SecretKey::from(self))
    }
}
