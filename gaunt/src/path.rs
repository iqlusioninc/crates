//! Remote paths on HTTP servers

#[cfg(feature = "alloc")]
use {
    crate::error::Error,
    alloc::{borrow::ToOwned, string::String},
    core::{
        fmt::{self, Display},
        str::FromStr,
    },
};

/// Paths to HTTP resources (owned buffer)
// TODO: corresponding borrowed `Path` type
#[cfg(feature = "alloc")]
pub struct PathBuf(String);

#[cfg(feature = "alloc")]
impl FromStr for PathBuf {
    type Err = Error;

    /// Create a path from the given string
    fn from_str(path: &str) -> Result<Self, Error> {
        // TODO: validate path
        Ok(PathBuf(path.to_owned()))
    }
}

#[cfg(feature = "alloc")]
impl AsRef<str> for PathBuf {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[cfg(feature = "alloc")]
impl Display for PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a str> for PathBuf {
    fn from(path: &str) -> Self {
        Self::from_str(path).unwrap()
    }
}
