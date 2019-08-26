//! Secret strings

use super::{CloneableSecret, DebugSecret, Secret};
use alloc::string::String;

/// Secret strings
pub type SecretString = Secret<String>;

impl DebugSecret for String {}
impl CloneableSecret for String {}
