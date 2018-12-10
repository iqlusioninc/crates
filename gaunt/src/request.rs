//! HTTP request types

use crate::prelude::*;

/// Request bodies
#[derive(Debug, Default)]
pub struct Body(pub(crate) Vec<u8>);

impl<'a> From<&'a [u8]> for Body {
    fn from(bytes: &[u8]) -> Body {
        Body(Vec::from(bytes))
    }
}
