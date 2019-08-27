//! Secret bytes

use super::{CloneableSecret, DebugSecret, Secret};
use bytes_crate::{Bytes, BytesMut};

/// Secret bytes
pub type SecretBytes = Secret<Bytes>;
/// Secret bytes_mut
pub type SecretBytesMut = Secret<BytesMut>;

impl DebugSecret for Bytes {}
impl CloneableSecret for Bytes {}
impl DebugSecret for BytesMut {}
impl CloneableSecret for BytesMut {}
