//! Ed25519 keys.

use super::{Signature, ALGORITHM_ID, ALGORITHM_OID};
use crate::{Error, Result};
use core::cmp::Ordering;
use pkcs8::{DecodePublicKey, EncodePublicKey};
use signature::Verifier;

/// Ed25519 verifying key.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VerifyingKey {
    inner: ed25519_dalek::PublicKey,
}

impl VerifyingKey {
    /// Size of a serialized Ed25519 verifying key in bytes.
    pub const BYTE_SIZE: usize = 32;

    /// Parse an Ed25519 public key from raw bytes
    /// (i.e. compressed Edwards-y coordinate)
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        ed25519_dalek::PublicKey::from_bytes(bytes)
            .map(|inner| VerifyingKey { inner })
            .map_err(|_| Error::Parse)
    }

    /// Serialize this key as a byte array.
    pub fn to_bytes(self) -> [u8; Self::BYTE_SIZE] {
        self.inner.to_bytes()
    }
}

impl AsRef<[u8; Self::BYTE_SIZE]> for VerifyingKey {
    fn as_ref(&self) -> &[u8; Self::BYTE_SIZE] {
        self.inner.as_bytes()
    }
}

impl From<&ed25519_dalek::Keypair> for VerifyingKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> VerifyingKey {
        Self {
            inner: keypair.public,
        }
    }
}

impl DecodePublicKey for VerifyingKey {}

impl EncodePublicKey for VerifyingKey {
    fn to_public_key_der(&self) -> pkcs8::spki::Result<pkcs8::Document> {
        pkcs8::SubjectPublicKeyInfo {
            algorithm: ALGORITHM_ID,
            subject_public_key: self.inner.as_bytes(),
        }
        .try_into()
    }
}

impl TryFrom<pkcs8::SubjectPublicKeyInfo<'_>> for VerifyingKey {
    type Error = pkcs8::spki::Error;

    fn try_from(spki: pkcs8::SubjectPublicKeyInfo<'_>) -> pkcs8::spki::Result<Self> {
        spki.algorithm.assert_algorithm_oid(ALGORITHM_OID)?;

        if spki.algorithm.parameters.is_some() {
            return Err(pkcs8::spki::Error::OidUnknown {
                oid: spki.algorithm.parameters_oid()?,
            });
        }

        Self::from_bytes(spki.subject_public_key).map_err(|_| pkcs8::spki::Error::KeyMalformed)
    }
}

impl TryFrom<&[u8]> for VerifyingKey {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        Self::from_bytes(bytes)
    }
}

impl Verifier<Signature> for VerifyingKey {
    fn verify(&self, msg: &[u8], sig: &Signature) -> signature::Result<()> {
        self.inner.verify(msg, sig)
    }
}

impl Ord for VerifyingKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.as_bytes().cmp(other.inner.as_bytes())
    }
}

impl PartialOrd for VerifyingKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
