//! Algorithms supported by this library.

use crate::{Error, Result};
use core::convert::TryFrom;

/// Signature algorithms.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Algorithm {
    /// ECDSA with secp256k1.
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    EcdsaSecp256k1,
}

impl TryFrom<pkcs8::AlgorithmIdentifier<'_>> for Algorithm {
    type Error = Error;

    #[allow(unused_variables)]
    fn try_from(pkcs8_alg_id: pkcs8::AlgorithmIdentifier<'_>) -> Result<Self> {
        #[cfg(feature = "ecdsa")]
        if pkcs8_alg_id.oid == ecdsa::elliptic_curve::ALGORITHM_OID {
            #[cfg(any(feature = "secp256k1"))]
            use ecdsa::elliptic_curve::AlgorithmParameters;

            #[cfg(feature = "secp256k1")]
            if pkcs8_alg_id.parameters_oid() == Ok(crate::ecdsa::Secp256k1::OID) {
                return Ok(Self::EcdsaSecp256k1);
            }
        }

        Err(Error::AlgorithmInvalid)
    }
}
