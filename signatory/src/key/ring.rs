//! Signature key ring.

use crate::{Error, Result};

#[cfg(feature = "ecdsa")]
use crate::ecdsa;

/// Signature key ring which can contain signing keys for all supported algorithms.
#[derive(Debug, Default)]
pub struct KeyRing {
    /// ECDSA key ring.
    #[cfg(feature = "ecdsa")]
    pub ecdsa: ecdsa::KeyRing,
}

/// Support for loading PKCS#8 private keys.
pub trait LoadPkcs8 {
    /// Load a PKCS#8 key into the key ring.
    fn load_pkcs8(&mut self, private_key: pkcs8::PrivateKeyInfo<'_>) -> Result<()>;
}

impl LoadPkcs8 for KeyRing {
    fn load_pkcs8(&mut self, private_key: pkcs8::PrivateKeyInfo<'_>) -> Result<()> {
        match private_key.algorithm.oid {
            #[cfg(feature = "ecdsa")]
            ecdsa::elliptic_curve::ALGORITHM_OID => self.ecdsa.load_pkcs8(private_key),
            _ => Err(Error::AlgorithmInvalid),
        }
    }
}
