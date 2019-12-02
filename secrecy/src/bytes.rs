//! Optional `Secret` wrapper type for the `bytes::BytesMut` crate.

use super::ExposeSecret;
use bytes::BytesMut;
use core::fmt;
use zeroize::Zeroize;

#[cfg(feature = "bytes-serde")]
use serde::de::{Deserialize, Deserializer};

/// Instance of `BytesMut` protected by a type that impls the `ExposeSecret`
/// trait like `Secret<T>`.
///
/// Because of the nature of how the `Bytes` type works, it needs some special
/// care in order to have a proper zeroizing drop handler.
#[derive(Clone)]
pub struct SecretBytesMut(BytesMut);

impl SecretBytesMut {
    /// Wrap bytes in `SecretBytesMut`
    pub fn new(bytes: impl Into<BytesMut>) -> SecretBytesMut {
        SecretBytesMut(bytes.into())
    }
}

impl ExposeSecret<BytesMut> for SecretBytesMut {
    fn expose_secret(&self) -> &BytesMut {
        &self.0
    }
}

impl fmt::Debug for SecretBytesMut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretBytesMut([REDACTED])")
    }
}

impl From<BytesMut> for SecretBytesMut {
    fn from(bytes: BytesMut) -> SecretBytesMut {
        SecretBytesMut::new(bytes)
    }
}

impl Drop for SecretBytesMut {
    fn drop(&mut self) {
        self.0.resize(self.0.capacity(), 0);
        self.0.as_mut().zeroize();
        debug_assert!(self.0.as_ref().iter().all(|b| *b == 0));
    }
}

#[cfg(feature = "bytes-serde")]
impl<'de> Deserialize<'de> for SecretBytesMut {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        BytesMut::deserialize(deserializer).map(SecretBytesMut::new)
    }
}
