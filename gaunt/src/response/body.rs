//! Response body types.
// TODO: support for streaming response bodies

use prelude::*;

/// Response body
#[derive(Debug)]
pub struct Body(pub(super) Vec<u8>);

impl Body {
    /// Buffer the response body into a `Vec<u8>` and return it
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}
