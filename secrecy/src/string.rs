//! Secret strings

use super::{DebugSecret, Secret};
use alloc::string::String;

/// Secret strings
pub type SecretString = Secret<String>;

impl DebugSecret for String {}
