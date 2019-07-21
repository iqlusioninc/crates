//! Key derivation paths. This is the owned type for such paths, analogous
//! to the corresponding type in `std`.
//!
//! This type is only available when the `alloc` feature is enabled.

use crate::{
    path::{Component, Path},
    Error, DELIMITER,
};
use alloc::{borrow::ToOwned, str::FromStr, vec::Vec};
use core::fmt::{self, Debug};
use core::{borrow::Borrow, ops::Deref};
use zeroize::Zeroize;

/// Key derivation paths: location within a key hierarchy which
/// names/identifies a specific key.
///
/// This is the owned path type. The corresponding reference type is
/// `hkd32::Path` (ala the corresponding types in `std`).
#[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord, Zeroize)]
#[repr(transparent)]
#[zeroize(drop)]
pub struct PathBuf(Vec<u8>);

impl PathBuf {
    /// Parse a path from its bytestring serialization.
    pub fn from_bytes<B>(bytes: B) -> Result<Self, Error>
    where
        B: AsRef<[u8]>,
    {
        // Use `Path::new` to assert that the path is actually valid
        Path::new(bytes.as_ref())?;

        // If it parses successfully, we can use it wholesale
        Ok(PathBuf(bytes.as_ref().into()))
    }

    /// Create a new `PathBuf` representing the root derivation path.
    ///
    /// This is also the default value for `PathBuf`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Borrow this `PathBuf` as a `Path`
    pub fn as_path(&self) -> &Path {
        Path::new(&self.0).unwrap()
    }

    /// Extend this key derivation path with additional components.
    pub fn extend<'a, I, C>(&mut self, components: I)
    where
        I: IntoIterator<Item = C>,
        C: AsRef<Component<'a>>,
    {
        for component in components {
            self.push(component);
        }
    }

    /// Push an additional component onto this path.
    pub fn push<'a, C: AsRef<Component<'a>>>(&mut self, component: C) {
        self.0.append(&mut component.as_ref().to_bytes());
    }
}

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl Borrow<Path> for PathBuf {
    fn borrow(&self) -> &Path {
        self.as_path()
    }
}

impl Debug for PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hkd32::PathBuf")?;
        self.debug_components(f)
    }
}

impl Deref for PathBuf {
    type Target = Path;

    fn deref(&self) -> &Path {
        self.as_path()
    }
}

impl FromStr for PathBuf {
    type Err = Error;

    /// Parse a derivation path from a string.
    ///
    /// Derivation path strings look like Unix paths (e.g. "/foo/bar/baz").
    /// They are delimited by a slash character (`/` a.k.a. `hkd32::DELIMITER`)
    /// and must start with a leading slash.
    ///
    /// Empty path components are not allowed (e.g. `//`, `/foo//`)
    fn from_str(s: &str) -> Result<Self, Error> {
        let mut result = Self::new();

        // Special case for the root path
        if s.len() == 1 && s.chars().nth(0) == Some(DELIMITER) {
            return Ok(result);
        }

        let mut components = s.split(DELIMITER);

        // Path strings must start with a leading `/`
        if components.next() != Some("") {
            return Err(Error);
        }

        for component in components {
            result.push(Component::new(component.as_bytes())?);
        }

        Ok(result)
    }
}

impl ToOwned for Path {
    type Owned = PathBuf;

    fn to_owned(&self) -> PathBuf {
        PathBuf::from_bytes(self.as_bytes()).unwrap()
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use super::*;

    #[test]
    fn test_root() {
        let root_path = PathBuf::new();
        assert_eq!(root_path.components().count(), 0);
        assert_eq!(root_path.stringify().unwrap(), "/");
        assert_eq!(&format!("{:?}", root_path), "hkd32::PathBuf(\"/\")");
    }
}
