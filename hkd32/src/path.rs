//! Key derivation paths: locations within the hierarchical derivation tree.
//!
//! Paths are lists of "binary safe" (a.k.a. "8-bit clean") components that
//! specify a location within a tree of keys. Every path within the hierarchy
//! describes a unique, unrelated key.
//!
//! For convenience, HKD32 also supports a string notation inspired by BIP32:
//! <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#the-key-tree>
//!
//! This notation is similar to Unix paths:
//!
//! `/first-component/second-component/.../nth-component`
//!
//! This corresponds to the following bytestring segments:
//!
//! `[b"first-component", b"second-component", ..., b"nth-component"]`
//!
//! Paths also support a binary serialization, in which each component is
//! prefixed by its length. Note that this requires the maximum component
//! length is 256, a limitation generally enforced by the protocol.

use super::Error;
#[cfg(feature = "alloc")]
use crate::{pathbuf::PathBuf, DELIMITER};
#[cfg(feature = "alloc")]
use alloc::{str, string::String, vec::Vec};
#[cfg(feature = "alloc")]
use core::fmt::{self, Debug};

/// Maximum length of a derivation component
pub const MAX_COMPONENT_LENGTH: usize = 256;

/// Key derivation paths: location within a key hierarchy which
/// names/identifies a specific key.
///
/// This is the reference type. The corresponding owned type is `hkd32::Path`
/// (ala the corresponding types in `std`).
#[derive(Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Path([u8]);

impl Path {
    /// Create a path from a byte slice.
    ///
    /// Returns `Error` if the path is malformed.
    // TODO(tarcieri): use a safe transparent wrapper type
    // Pre-RFC: <https://internals.rust-lang.org/t/pre-rfc-patterns-allowing-transparent-wrapper-types/10229>
    #[allow(unsafe_code)]
    pub fn new<P>(path: &P) -> Result<&Self, Error>
    where
        P: AsRef<[u8]> + ?Sized,
    {
        if Components::new(path.as_ref()).validate() {
            Ok(unsafe { &*(path.as_ref() as *const [u8] as *const Self) })
        } else {
            Err(Error)
        }
    }

    /// Obtain a reference to this path's bytestring serialization.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    /// Obtain a component iterator for this path.
    pub fn components(&self) -> Components {
        Components::new(&self.0)
    }

    /// Is this path the root path?
    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    /// Create a `PathBuf` with `path` joined to `self`.
    ///
    /// Requires the `alloc` feature is enabled.
    #[cfg(feature = "alloc")]
    pub fn join<P>(&self, path: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let mut result = PathBuf::new();
        result.extend(self.components());
        result.extend(path.as_ref().components());
        result
    }

    /// Get the parent path for this path
    pub fn parent(&self) -> Option<&Path> {
        let mut tail = None;

        for component in self.components() {
            tail = Some(component)
        }

        tail.map(|t| {
            let tail_len = self.0.len() - t.len() - 1;
            Path::new(&self.0[..tail_len]).unwrap()
        })
    }

    /// Attempt to convert this path to an `/x/y/z` string.
    ///
    /// This will only succeed if the path components are all ASCII.
    ///
    /// Requires the `alloc` feature is enabled.
    #[cfg(feature = "alloc")]
    pub fn stringify(&self) -> Result<String, Error> {
        let mut result = String::new();

        if self.is_root() {
            result.push(DELIMITER);
            return Ok(result);
        }

        for component in self.components() {
            result.push(DELIMITER);
            result.push_str(component.stringify()?.as_ref());
        }

        Ok(result)
    }

    /// Serialize this `Path` as a byte vector
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Internal function for debug printing just the path components
    #[cfg(feature = "alloc")]
    pub(crate) fn debug_components(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;

        if let Ok(s) = self.stringify() {
            s.fmt(f)?;
        } else {
            let component_count = self.components().count();
            for (i, component) in self.components().enumerate() {
                write!(f, "{:?}", component)?;
                if i < component_count - 1 {
                    write!(f, ", ")?;
                }
            }
        }

        write!(f, ")")
    }
}

#[cfg(feature = "alloc")]
impl Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hkd32::Path")?;
        self.debug_components(f)
    }
}

/// Component of a derivation path
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Component<'a>(&'a [u8]);

// Components are not allowed to be empty
#[allow(clippy::len_without_is_empty)]
impl<'a> Component<'a> {
    /// Create a new component.
    ///
    /// Returns `Error` if the component is empty or is longer than
    /// `MAX_COMPONENT_LENGTH`.
    pub fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        if !bytes.is_empty() && bytes.len() <= MAX_COMPONENT_LENGTH {
            Ok(Component(bytes))
        } else {
            Err(Error)
        }
    }

    /// Borrow the contents of this component as a byte slice.
    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    /// Get the length of this component in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Attempt to convert component to an ASCII string.
    ///
    /// Requires the `alloc` feature is enabled.
    #[cfg(feature = "alloc")]
    pub fn stringify(&self) -> Result<String, Error> {
        str::from_utf8(self.as_bytes())
            .map(String::from)
            .map_err(|_| Error)
    }

    /// Serialize this component as a length-prefixed bytestring.
    #[cfg(feature = "alloc")]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut serialized = Vec::with_capacity(1 + self.len());
        serialized.push((self.len() - 1) as u8);
        serialized.extend_from_slice(&self.0);
        serialized
    }
}

impl<'a> AsRef<Component<'a>> for Component<'a> {
    fn as_ref(&self) -> &Component<'a> {
        self
    }
}

#[cfg(feature = "alloc")]
impl<'a> Debug for Component<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hkd32::Component")?;

        if let Ok(s) = self.stringify() {
            write!(f, "({:?})", s)
        } else {
            write!(f, "({:?})", self.as_bytes())
        }
    }
}

/// Iterator over the components of a path
#[derive(Clone)]
pub struct Components<'a> {
    /// Remaining data in the path
    path: &'a [u8],

    /// Flag set to false if the path is malformed/truncated
    valid: bool,
}

impl<'a> Components<'a> {
    /// Create a new component iterator from the given byte slice
    fn new(path: &'a [u8]) -> Self {
        Self { path, valid: true }
    }

    /// Iterate over the components until the end, checking that the path
    /// is valid. This consumes the path in the process, and is only used
    /// internally by `Path::new` to ensure the path is well-formed.
    fn validate(mut self) -> bool {
        while self.next().is_some() {}
        self.valid
    }
}

impl<'a> Iterator for Components<'a> {
    type Item = Component<'a>;

    fn next(&mut self) -> Option<Component<'a>> {
        // Read the length prefix for the next component
        let component_len = *self.path.first()? as usize + 1;
        self.path = &self.path[1..];

        if self.path.len() < component_len {
            // If the path appears truncated, mark it internally as such
            // This is checked in `Path::new` and an error returned to the user
            self.valid = false;
            return None;
        }

        let (component_bytes, remaining) = self.path.split_at(component_len);
        self.path = remaining;

        if let Ok(component) = Component::new(component_bytes) {
            Some(component)
        } else {
            self.valid = false;
            None
        }
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use super::*;

    #[test]
    fn test_root() {
        let root_path = Path::new(&[]).unwrap();
        assert_eq!(root_path.components().count(), 0);
        assert_eq!(root_path.stringify().unwrap(), "/");
        assert_eq!(&format!("{:?}", root_path), "hkd32::Path(\"/\")");
    }
}
