//! Optional `Secret` wrapper type for the `bytes::BytesMut` crate.

use super::{CloneableSecret, DebugSecret, Secret};
use bytes::BytesMut;

/// Alias for `Secret<BytesMut>`
pub type SecretBytesMut = Secret<BytesMut>;

impl DebugSecret for BytesMut {}
impl CloneableSecret for BytesMut {}
