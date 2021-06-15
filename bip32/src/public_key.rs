//! Trait for deriving child keys on a given type.

use crate::{KeyFingerprint, Result, KEY_SIZE};
use core::convert::TryInto;
use ripemd160::Ripemd160;
use sha2::{Digest, Sha256};

#[cfg(feature = "secp256k1")]
use k256::elliptic_curve::sec1::ToEncodedPoint;

/// Bytes which represent a public key.
///
/// Includes an extra byte for an SEC1 tag.
pub type PublicKeyBytes = [u8; KEY_SIZE + 1];

/// Trait for key types which can be derived using BIP32.
pub trait PublicKey: Sized {
    /// Initialize this key from bytes.
    fn from_bytes(bytes: PublicKeyBytes) -> Result<Self>;

    /// Serialize this key as bytes.
    fn to_bytes(&self) -> PublicKeyBytes;

    /// Compute a 4-byte key fingerprint for this public key.
    ///
    /// Default implementation uses `RIPEMD160(SHA256(public_key))`.
    fn fingerprint(&self) -> KeyFingerprint {
        let digest = Ripemd160::digest(&Sha256::digest(&self.to_bytes()));
        digest[..4].try_into().expect("digest truncated")
    }
}

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
impl PublicKey for k256::PublicKey {
    fn from_bytes(bytes: PublicKeyBytes) -> Result<Self> {
        Ok(k256::PublicKey::from_sec1_bytes(&bytes)?)
    }

    fn to_bytes(&self) -> PublicKeyBytes {
        self.to_encoded_point(true)
            .as_bytes()
            .try_into()
            .expect("malformed public key")
    }
}

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
impl PublicKey for k256::ecdsa::VerifyingKey {
    fn from_bytes(bytes: PublicKeyBytes) -> Result<Self> {
        Ok(k256::ecdsa::VerifyingKey::from_sec1_bytes(&bytes)?)
    }

    fn to_bytes(&self) -> PublicKeyBytes {
        self.to_bytes()
    }
}
