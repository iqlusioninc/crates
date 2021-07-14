//! ECDSA key ring.

use crate::{Error, KeyHandle, LoadPkcs8, Result};

#[allow(unused_imports)]
use ecdsa::elliptic_curve::AlgorithmParameters;

#[cfg(feature = "nistp256")]
use super::nistp256;

#[cfg(feature = "secp256k1")]
use super::secp256k1;

/// ECDSA key ring.
#[derive(Debug, Default)]
pub struct KeyRing {
    /// ECDSA/P-256 keys.
    #[cfg(feature = "nistp256")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nistp256")))]
    pub nistp256: nistp256::KeyRing,

    /// ECDSA/secp256k1 keys.
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    pub secp256k1: secp256k1::KeyRing,
}

impl LoadPkcs8 for KeyRing {
    fn load_pkcs8(&mut self, private_key: pkcs8::PrivateKeyInfo<'_>) -> Result<KeyHandle> {
        if private_key.algorithm.oid != ecdsa::elliptic_curve::ALGORITHM_OID {
            return Err(Error::AlgorithmInvalid);
        }

        match private_key.algorithm.parameters_oid()? {
            #[cfg(feature = "nistp256")]
            p256::NistP256::OID => self.nistp256.load_pkcs8(private_key),
            #[cfg(feature = "secp256k1")]
            k256::Secp256k1::OID => self.secp256k1.load_pkcs8(private_key),
            _ => Err(Error::AlgorithmInvalid),
        }
    }
}
