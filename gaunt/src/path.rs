//! Remote paths on HTTP servers

#[cfg(feature = "alloc")]
use prelude::*;

#[cfg(feature = "alloc")]
use core::fmt::{self, Display};

#[cfg(feature = "alloc")]
use error::Error;

/// Paths requested via HTTP
#[cfg(feature = "alloc")]
pub struct Path(String);

#[cfg(feature = "alloc")]
impl Path {
    /// Create a path from the given string
    pub fn new(path: &str) -> Result<Self, Error> {
        // TODO: validate path
        Ok(Path(path.to_owned()))
    }
}

#[cfg(feature = "alloc")]
impl AsRef<str> for Path {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[cfg(feature = "alloc")]
impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a str> for Path {
    fn from(path: &str) -> Self {
        Self::new(path).unwrap()
    }
}
