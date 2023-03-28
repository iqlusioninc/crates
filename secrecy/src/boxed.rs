//! `Box` types containing secrets

use super::{DebugSecret, Secret};
use alloc::boxed::Box;
use zeroize::Zeroize;

/// `Box` types containing a secret value
pub type SecretBox<S> = Secret<Box<S>>;

impl<S: DebugSecret + Zeroize> DebugSecret for Box<S> {}
