//! Handle to a particular key.

#[cfg(feature = "ecdsa")]
#[allow(unused_imports)]
use crate::ecdsa;

/// Handle to a particular key.
///
/// Uniquely identifies a particular key in the keyring.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum KeyHandle {
    /// ECDSA/P-256.
    #[cfg(feature = "nistp256")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nistp256")))]
    EcdsaNistP256(ecdsa::nistp256::VerifyingKey),

    /// ECDSA/secp256k1.
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    EcdsaSecp256k1(ecdsa::secp256k1::VerifyingKey),
}

impl KeyHandle {
    /// Get the ECDSA/P-256 verifying key, if this is an ECDSA/P-256 key
    #[cfg(feature = "nistp256")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nistp256")))]
    pub fn ecdsa_nistp256(&self) -> Option<ecdsa::nistp256::VerifyingKey> {
        match self {
            KeyHandle::EcdsaNistP256(pk) => Some(*pk),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }

    /// Get the ECDSA/secp256k1 verifying key, if this is an ECDSA/secp256k1 key
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    pub fn ecdsa_secp256k1(&self) -> Option<ecdsa::secp256k1::VerifyingKey> {
        match self {
            KeyHandle::EcdsaSecp256k1(pk) => Some(*pk),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
