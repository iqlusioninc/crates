//! `Box` types containing secrets

use super::{DebugSecret, Secret};
use alloc::boxed::Box;
use zeroize::Zeroize;

/// `Box` types containing a secret value
#[cfg(feature = "alloc")]
pub type SecretBox<S> = Secret<Box<S>>;

#[cfg(feature = "alloc")]
impl<S: DebugSecret + Zeroize> DebugSecret for Box<S> {}
