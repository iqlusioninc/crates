//! Signature key ring.

#[cfg(feature = "ecdsa")]
use crate::ecdsa;

/// Signature key ring which can contain signing keys for all supported algorithms.
#[derive(Debug, Default)]
pub struct KeyRing {
    /// ECDSA key ring.
    #[cfg(feature = "ecdsa")]
    pub ecdsa: ecdsa::KeyRing,
}
