//! Secret `Vec` types

use super::{DebugSecret, Secret};
use alloc::vec::Vec;
use zeroize::Zeroize;

/// `Vec` types containing secret value
#[cfg(feature = "alloc")]
pub type SecretVec<S> = Secret<Vec<S>>;

#[cfg(feature = "alloc")]
impl<S: DebugSecret + Zeroize> DebugSecret for Vec<S> {}
