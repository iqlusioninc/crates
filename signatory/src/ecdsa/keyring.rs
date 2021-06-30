//! ECDSA key ring.

use crate::{Error, LoadPkcs8, Result};

#[allow(unused_imports)]
use ecdsa::elliptic_curve::AlgorithmParameters;

#[cfg(feature = "secp256k1")]
use super::secp256k1;

/// ECDSA key ring.
#[derive(Debug, Default)]
pub struct KeyRing {
    /// ECDSA/secp256k1 keys.
    #[cfg(feature = "secp256k1")]
    pub secp256k1: secp256k1::KeyRing,
}

impl LoadPkcs8 for KeyRing {
    fn load_pkcs8(&mut self, private_key: pkcs8::PrivateKeyInfo<'_>) -> Result<()> {
        if private_key.algorithm.oid != ecdsa::elliptic_curve::ALGORITHM_OID {
            return Err(Error::AlgorithmInvalid);
        }

        match private_key.algorithm.parameters_oid()? {
            #[cfg(feature = "secp256k1")]
            k256::Secp256k1::OID => self.secp256k1.load_pkcs8(private_key),
            _ => Err(Error::AlgorithmInvalid),
        }
    }
}
