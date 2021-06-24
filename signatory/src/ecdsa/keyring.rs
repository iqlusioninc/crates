//! ECDSA key ring.

#[cfg(feature = "secp256k1")]
use super::secp256k1;

/// ECDSA key ring.
#[derive(Debug, Default)]
pub struct KeyRing {
    /// ECDSA/secp256k1 keys.
    #[cfg(feature = "secp256k1")]
    pub secp256k1: secp256k1::KeyRing,
}
