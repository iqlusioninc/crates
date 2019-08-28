//! `Secret<T>` wrapper type for more carefully handling secret values
//! (e.g. passwords, cryptographic keys, access tokens or other credentials)

#![no_std]
#![deny(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/secrecy/0.3.1")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod boxed;
#[cfg(feature = "bytes")]
mod bytes;
#[cfg(feature = "alloc")]
mod string;
#[cfg(feature = "alloc")]
mod vec;

#[cfg(feature = "alloc")]
pub use self::{boxed::SecretBox, string::SecretString, vec::SecretVec};

#[cfg(feature = "bytes")]
pub use self::{bytes::SecretBytes, bytes::SecretBytesMut};

use core::fmt::{self, Debug};
#[cfg(feature = "serde")]
use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use zeroize::Zeroize;

/// Wrapper type for values that contains secrets, which attempts to limit
/// accidental exposure and ensure secrets are wiped from memory when dropped.
/// (e.g. passwords, cryptographic keys, access tokens or other credentials)
///
/// Access to the secret inner value occurs through the `ExposeSecret` trait,
/// which provides an `expose_secret()` method for accessing the inner secret
/// value.
pub struct Secret<S>
where
    S: Zeroize,
{
    /// Inner secret value
    inner_secret: S,
}

impl<S> Secret<S>
where
    S: Zeroize,
{
    /// Take ownership of a secret value
    pub fn new(secret: S) -> Self {
        Secret {
            inner_secret: secret,
        }
    }
}

impl<S> ExposeSecret<S> for Secret<S>
where
    S: Zeroize,
{
    fn expose_secret(&self) -> &S {
        &self.inner_secret
    }
}

impl<S> Clone for Secret<S>
where
    S: CloneableSecret,
{
    fn clone(&self) -> Self {
        Secret {
            inner_secret: self.inner_secret.clone(),
        }
    }
}

impl<S> Debug for Secret<S>
where
    S: Zeroize + DebugSecret,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Secret({})", S::debug_secret())
    }
}

#[cfg(feature = "serde")]
impl<'de, S> Deserialize<'de> for Secret<S>
where
    S: Zeroize + Clone + DebugSecret + DeserializeOwned + Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        S::deserialize(deserializer).map(Secret::new)
    }
}

impl<S> Drop for Secret<S>
where
    S: Zeroize,
{
    fn drop(&mut self) {
        // Zero the secret out from memory
        self.inner_secret.zeroize();
    }
}

/// Marker trait for secrets which are allowed to be cloned
pub trait CloneableSecret: Clone + Zeroize {}

/// Expose a reference to an inner secret
pub trait ExposeSecret<S> {
    /// Expose secret
    fn expose_secret(&self) -> &S;
}

/// Debugging trait which is specialized for handling secret values
pub trait DebugSecret {
    /// Information about what the secret contains.
    ///
    /// Static so as to discourage unintentional secret exposure.
    fn debug_secret() -> &'static str {
        "[SECRET]"
    }
}
