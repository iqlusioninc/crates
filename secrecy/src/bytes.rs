//! Optional `Secret` wrapper type for the `bytes::BytesMut` crate.

use super::{CloneableSecret, DebugSecret, ExposeSecret, Secret};
use bytes_crate::{Bytes, BytesMut};
use core::fmt;
use zeroize::Zeroize;

#[cfg(feature = "serde")]
use serde::de::{Deserialize, Deserializer};

/// Instance of `Bytes` protected by a type that impls the `ExposeSecret`
/// trait like `Secret<T>`.
///
/// Because of the nature of how the `Bytes` type works, it needs some special
/// care in order to have a proper zeroizing drop handler.
#[derive(Clone)]
pub struct SecretBytes(Option<Bytes>);

impl SecretBytes {
    /// Wrap bytes in `SecretBytes`
    pub fn new(bytes: impl Into<Bytes>) -> SecretBytes {
        SecretBytes(Some(bytes.into()))
    }
}

impl ExposeSecret<Bytes> for SecretBytes {
    fn expose_secret(&self) -> &Bytes {
        self.0.as_ref().unwrap()
    }
}

impl fmt::Debug for SecretBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretBytes(...)")
    }
}

impl From<Bytes> for SecretBytes {
    fn from(bytes: Bytes) -> SecretBytes {
        SecretBytes::new(bytes)
    }
}

impl From<BytesMut> for SecretBytes {
    fn from(bytes: BytesMut) -> SecretBytes {
        SecretBytes::new(bytes)
    }
}

impl Drop for SecretBytes {
    fn drop(&mut self) {
        // To zero the contents of `Bytes`, we have to take ownership of it
        // and then attempt to convert it to a `BytesMut`. If that succeeds,
        // we are holding the last reference to the inner byte buffer, which
        // indicates its lifetime has ended and it's ready to be zeroed.
        if let Some(bytes) = self.0.take() {
            if let Ok(mut bytes_mut) = bytes.try_mut() {
                bytes_mut.zeroize();
            }
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for SecretBytes where {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Bytes::deserialize(deserializer).map(SecretBytes::new)
    }
}

/// Alias for `Secret<BytesMut>`
pub type SecretBytesMut = Secret<BytesMut>;

impl DebugSecret for BytesMut {}
impl CloneableSecret for BytesMut {}
