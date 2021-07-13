//! Handle to a particular key.

/// Handle to a particular key.
///
/// Uniquely identifies a particular key in the keyring.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum KeyHandle {
    /// ECDSA with secp256k1.
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    EcdsaSecp256k1(k256::ecdsa::VerifyingKey),
}

impl KeyHandle {
    /// Get the ECDSA/secp256k1 verifying key, if this is an ECDSA/secp256k1 key
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    pub fn ecdsa_secp256k1(&self) -> Option<k256::ecdsa::VerifyingKey> {
        match self {
            KeyHandle::EcdsaSecp256k1(pk) => Some(*pk),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
