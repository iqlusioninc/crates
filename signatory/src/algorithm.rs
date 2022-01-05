//! Algorithms supported by this library.

use crate::{Error, Result};

#[cfg(feature = "ed25519")]
use crate::ed25519;

/// Signature algorithms.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Algorithm {
    /// ECDSA with NIST P-256.
    #[cfg(feature = "nistp256")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nistp256")))]
    EcdsaNistP256,

    /// ECDSA with secp256k1.
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    EcdsaSecp256k1,

    /// Ed25519.
    #[cfg(feature = "ed25519")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ed25519")))]
    Ed25519,
}

impl Algorithm {
    /// Is the algorithm ECDSA?
    #[cfg(feature = "ecdsa")]
    pub fn is_ecdsa(self) -> bool {
        #[cfg(feature = "nistp256")]
        if self == Algorithm::EcdsaNistP256 {
            return true;
        }

        #[cfg(feature = "secp256k1")]
        if self == Algorithm::EcdsaSecp256k1 {
            return true;
        }

        false
    }
}

impl TryFrom<pkcs8::AlgorithmIdentifier<'_>> for Algorithm {
    type Error = Error;

    #[allow(unused_variables)]
    fn try_from(pkcs8_alg_id: pkcs8::AlgorithmIdentifier<'_>) -> Result<Self> {
        #[cfg(feature = "ecdsa")]
        if pkcs8_alg_id.oid == ecdsa::elliptic_curve::ALGORITHM_OID {
            #[cfg(any(feature = "nistp256", feature = "secp256k1"))]
            use ecdsa::elliptic_curve::AlgorithmParameters;

            #[cfg(feature = "nistp256")]
            if pkcs8_alg_id.parameters_oid() == Ok(crate::ecdsa::NistP256::OID) {
                return Ok(Self::EcdsaNistP256);
            }

            #[cfg(feature = "secp256k1")]
            if pkcs8_alg_id.parameters_oid() == Ok(crate::ecdsa::Secp256k1::OID) {
                return Ok(Self::EcdsaSecp256k1);
            }
        }

        #[cfg(feature = "ed25519")]
        if pkcs8_alg_id == ed25519::ALGORITHM_ID {
            return Ok(Self::Ed25519);
        }

        Err(Error::AlgorithmInvalid)
    }
}
