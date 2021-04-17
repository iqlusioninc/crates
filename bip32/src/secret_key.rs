//! Trait for deriving child keys on a given type.

use crate::{Result, KEY_SIZE};

/// Bytes which represent a secret key.
type SecretKeyBytes = [u8; KEY_SIZE];

/// Derive a child key for this key.
pub trait SecretKey: Sized {
    /// Serialized public key type.
    type PublicKey: AsRef<[u8]> + Sized;

    /// Initialize this key from bytes.
    fn from_bytes(bytes: &SecretKeyBytes) -> Result<Self>;

    /// Serialize this key as bytes.
    fn to_bytes(&self) -> SecretKeyBytes;

    /// Derive a child key from this key and a provided [`ChainCode`].
    fn derive_child(&self, derivation_key: &SecretKeyBytes) -> Result<Self>;

    /// Serialize the public key for this [`SecretKey`].
    fn public_key(&self) -> Self::PublicKey;
}

#[cfg(feature = "secp256k1")]
impl SecretKey for k256::SecretKey {
    type PublicKey = [u8; 33];

    fn from_bytes(bytes: &SecretKeyBytes) -> Result<Self> {
        Ok(k256::SecretKey::from_bytes(bytes)?)
    }

    fn to_bytes(&self) -> SecretKeyBytes {
        k256::SecretKey::to_bytes(self).into()
    }

    fn derive_child(&self, derivation_key: &SecretKeyBytes) -> Result<Self> {
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
