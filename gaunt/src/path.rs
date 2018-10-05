//! Remote paths on HTTP servers

use std::fmt::{self, Display};

use error::Error;

/// Paths requested via HTTP
pub struct Path(String);

impl Path {
    /// Create a path from the given string
    pub fn new(path: &str) -> Result<Self, Error> {
        // TODO: validate path
        Ok(Path(path.to_owned()))
    }
}

impl AsRef<str> for Path {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> From<&'a str> for Path {
    fn from(path: &str) -> Self {
        Self::new(path).unwrap()
    }
}
