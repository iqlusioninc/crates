//! ECDSA/secp256k1 support.

use crate::key::store::GeneratePkcs8;
use alloc::{boxed::Box, vec::Vec};
use core::fmt;
use ecdsa::signature::{Error, Signer};
pub use k256::ecdsa::{
    recoverable::{Id as RecoveryId, Signature as RecoverableSignature},
    Signature, VerifyingKey,
};
use pkcs8::ToPrivateKey;

/// ECDSA/secp256k1 key ring.
#[derive(Debug, Default)]
pub struct KeyRing {
    // TODO(tarcieri): map of verifying key -> signing key (needs Ord impl)
    keys: Vec<SigningKey>,
}

impl KeyRing {
    /// Create new ECDSA/secp256k1 keystore.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Transaction signing key (ECDSA/secp256k1)
pub struct SigningKey {
    inner: Box<dyn Secp256k1Signer>,
}

impl SigningKey {
    /// Get the verifying key that corresponds to this signing key.
    pub fn verifying_key(&self) -> VerifyingKey {
        self.inner.verifying_key()
    }
}

#[cfg(feature = "std")]
impl GeneratePkcs8 for SigningKey {
    /// Randomly generate a new PKCS#8 private key.
    fn generate_pkcs8() -> pkcs8::PrivateKeyDocument {
        k256::SecretKey::random(&mut rand_core::OsRng)
            .to_pkcs8_der()
            .expect("DER error")
    }
}

impl Signer<Signature> for SigningKey {
    fn try_sign(&self, msg: &[u8]) -> Result<Signature, Error> {
        self.inner.try_sign(msg)
    }
}

impl Signer<RecoverableSignature> for SigningKey {
    fn try_sign(&self, msg: &[u8]) -> Result<RecoverableSignature, Error> {
        let sig: Signature = self.inner.try_sign(msg)?;
        RecoverableSignature::from_trial_recovery(&self.verifying_key(), msg, &sig)
    }
}

impl fmt::Debug for SigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SigningKey")
            .field("verifying_key", &self.verifying_key())
            .finish()
    }
}

/// ECDSA/secp256k1 signer
pub trait Secp256k1Signer: Signer<Signature> {
    /// Get the ECDSA verifying key for this signer
    fn verifying_key(&self) -> VerifyingKey;
}

impl<T> Secp256k1Signer for T
where
    T: Signer<Signature>,
    VerifyingKey: for<'a> From<&'a T>,
{
    fn verifying_key(&self) -> VerifyingKey {
        self.into()
    }
}
