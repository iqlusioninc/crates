//! `Secret<T>` wrapper type for more carefully handling secret values
//! (e.g. passwords, cryptographic keys, access tokens or other credentials)

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/secrecy/0.6.0")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

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
pub use self::bytes::SecretBytesMut;

use core::fmt::{self, Debug};
#[cfg(feature = "serde")]
use serde::{de, ser, Deserialize, Serialize};
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

/// Implement `CloneableSecret` on arrays of types that impl `Clone` and
/// `Zeroize`.
macro_rules! impl_cloneable_secret_for_array {
    ($($size:expr),+) => {
        $(
            impl<T: Clone + Zeroize> CloneableSecret for [T; $size] {}
        )+
     };
}

// TODO(tarcieri): const generics
impl_cloneable_secret_for_array!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);

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
        "[REDACTED]"
    }
}

/// Implement `DebugSecret` on arrays of types that impl `Debug`.
macro_rules! impl_debug_secret_for_array {
    ($($size:expr),+) => {
        $(
            impl<T: Debug> DebugSecret for [T; $size] {}
        )+
     };
}

// TODO(tarcieri): const generics
impl_debug_secret_for_array!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);

/// Marker trait for secret types which can be [`Serialize`]-d by [`serde`].
///
/// When the `serde` feature of this crate is enabled and types are marked with
/// this trait, they receive a [`Serialize` impl][1] for `Secret<T>`.
/// (NOTE: all types which impl `DeserializeOwned` receive a [`Deserialize`]
/// impl)
///
/// This is done deliberately to prevent accidental exfiltration of secrets
/// via `serde` serialization.
///
/// If you are working with [`SecretString`] or [`SecretVec`], not that
/// by design these types do *NOT* impl this trait.
///
/// If you really want to have `serde` serialize those types, use the
/// [`serialize_with`][2] attribute to specify a serializer that exposes the secret.
///
/// [1]: https://docs.rs/secrecy/latest/secrecy/struct.Secret.html#implementations
/// [2]: https://serde.rs/field-attrs.html#serialize_with
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub trait SerializableSecret: Serialize {}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for Secret<T>
where
    T: Zeroize + Clone + de::DeserializeOwned + Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Secret::new)
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Secret<T>
where
    T: Zeroize + SerializableSecret + Serialize + Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.expose_secret().serialize(serializer)
    }
}
