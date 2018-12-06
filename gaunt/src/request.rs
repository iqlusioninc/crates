//! HTTP request types

use crate::prelude::*;

/// Request bodies
#[derive(Debug, Default)]
pub struct Body(pub(crate) Vec<u8>);

impl Body {
    /// Create a new `Body` from the given byte slice
    pub fn new(bytes: &[u8]) -> Body {
        Body(bytes.into())
    }
}

impl From<Vec<u8>> for Body {
    fn from(bytes: Vec<u8>) -> Body {
        Body(bytes)
    }
}
