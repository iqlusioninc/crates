//! HMAC-based Hierarchical Key Derivation: deterministically derive a
//! hierarchy of symmetric keys from initial keying material through
//! repeated applications of the Hash-based Message Authentication Code
//! (HMAC) construction.
//!
//! This library implements a fully symmetric construction inspired by
//! [BIP-0032: Hierarchical Deterministic Wallets][bip32].
//!
//! [bip32]: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki

#![no_std]
#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/hkd32/0.0.0")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{str::FromStr, vec::Vec};
use core::slice::Iter;
use hmac::{Hmac, Mac};
use sha2::Sha512;
use zeroize::Zeroize;

/// Delimiter used for strings containing paths
pub const DELIMITER: char = '/';

/// Size of input key material and derived keys
pub const KEY_SIZE: usize = 32;

/// Opaque error type
#[derive(Copy, Clone, Debug)]
pub struct Error;

/// Symmetric key material: either input or output key material
#[derive(Clone, Zeroize)]
#[zeroize(drop)]
pub struct KeyMaterial([u8; KEY_SIZE]);

impl KeyMaterial {
    /// Create random key material using the operating system CSRNG
    #[cfg(feature = "getrandom")]
    pub fn random() -> Self {
        let mut bytes = [0u8; KEY_SIZE];
        getrandom::getrandom(&mut bytes).expect("getrandom failure!");
        Self::new(bytes)
    }

    /// Create new key material
    pub fn new(bytes: [u8; KEY_SIZE]) -> KeyMaterial {
        KeyMaterial(bytes)
    }

    /// Create new key material from a byte slice
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        if slice.len() == KEY_SIZE {
            let mut bytes = [0u8; KEY_SIZE];
            bytes.copy_from_slice(slice);
            Ok(Self::new(bytes))
        } else {
            Err(Error)
        }
    }
}

impl AsRef<[u8]> for KeyMaterial {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<[u8; KEY_SIZE]> for KeyMaterial {
    fn from(bytes: [u8; KEY_SIZE]) -> Self {
        Self::new(bytes)
    }
}

/// Components of a path: byte slices
pub type Component<'a> = &'a [u8];

/// Derivation paths: a list of bytestring components describing the path to
/// a location within the key hierarchy.
pub struct Path<'a> {
    /// Slice of path components
    components: &'a [Component<'a>],
}

impl<'a> Path<'a> {
    /// Create a new path
    pub fn new(components: &'a [Component<'a>]) -> Self {
        Self { components }
    }

    /// Iterate over the components of this derivation path
    pub fn iter(&self) -> Iter<Component<'a>> {
        self.components.iter()
    }

    /// Derive an output key from the given input key material
    pub fn derive(&self, input_key_material: &KeyMaterial) -> KeyMaterial {
        self.iter()
            .enumerate()
            .fold(input_key_material.clone(), |parent_key, (i, elem)| {
                let mut hmac = Hmac::<Sha512>::new_varkey(parent_key.as_ref()).unwrap();
                hmac.input(elem);

                let mut hmac_result = hmac.result().code();
                let (secret_key, chain_code) = hmac_result.split_at_mut(KEY_SIZE);
                let mut child_key = [0u8; KEY_SIZE];

                if i < self.components.len() - 1 {
                    // Use chain code for all but the last element
                    child_key.copy_from_slice(chain_code);
                } else {
                    // Use secret key for the last element
                    child_key.copy_from_slice(secret_key);
                }

                secret_key.zeroize();
                chain_code.zeroize();

                KeyMaterial(child_key)
            })
    }
}

impl<'a> From<&'a [Component<'a>]> for Path<'a> {
    fn from(components: &'a [Component<'a>]) -> Path<'a> {
        Path { components }
    }
}

/// Owned derivation path components
pub type ComponentBuf = Vec<u8>;

/// Owned derivation paths
#[cfg(feature = "alloc")]
pub struct PathBuf {
    components: Vec<ComponentBuf>,
}

#[cfg(feature = "alloc")]
impl PathBuf {
    /// Create a new derivation path
    pub fn new<I, T>(components: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ComponentBuf>,
    {
        Self {
            components: components
                .into_iter()
                .map(|component| component.into())
                .collect(),
        }
    }

    /// Iterate over the components of this derivation path
    pub fn iter(&self) -> Iter<ComponentBuf> {
        self.components.iter()
    }

    /// Derive an output key from the given key material
    pub fn derive(&self, input_key_material: &KeyMaterial) -> KeyMaterial {
        let component_refs = self.iter().map(|c| c.as_ref()).collect::<Vec<_>>();

        Path::new(component_refs.as_ref()).derive(input_key_material)
    }
}

#[cfg(feature = "alloc")]
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
        let mut components = s.split(DELIMITER);

        // Path strings must start with a leading `/`
        if components.next() != Some("") {
            return Err(Error);
        }

        let result = Self::new(components);

        if result.components.len() == 1 && result.components[0].is_empty() {
            // Root derivation path
            Ok(PathBuf {
                components: Vec::new(),
            })
        } else if result.components.iter().any(|c| c.is_empty()) {
            // Derivation path string components cannot be empty
            Err(Error)
        } else {
            Ok(result)
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for PathBuf {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap_or_else(|_| panic!("invalid derivation path: {:?}", s))
    }
}

/// Derivation paths may potentially contain secrets
#[cfg(feature = "alloc")]
impl Zeroize for PathBuf {
    fn zeroize(&mut self) {
        for component in &mut self.components {
            component.zeroize();
        }
    }
}

#[cfg(feature = "alloc")]
impl Drop for PathBuf {
    fn drop(&mut self) {
        self.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VECTOR_KEY: KeyMaterial = KeyMaterial([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ]);

    /// Root path outputs the original IKM
    #[test]
    fn test_vector_0_empty_path() {
        let output_key = PathBuf::from("/").derive(&TEST_VECTOR_KEY);

        assert_eq!(
            output_key.as_ref(),
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31
            ]
        );
    }

    #[test]
    fn test_vector_1() {
        let output_key = PathBuf::from("/1").derive(&TEST_VECTOR_KEY);

        assert_eq!(
            output_key.as_ref(),
            [
                132, 75, 58, 18, 91, 107, 10, 110, 128, 162, 98, 177, 192, 212, 50, 101, 136, 46,
                46, 83, 179, 150, 64, 68, 250, 57, 101, 1, 227, 159, 148, 20
            ]
        );
    }

    #[test]
    fn test_vector_2() {
        let output_key = PathBuf::from("/1/2").derive(&TEST_VECTOR_KEY);

        assert_eq!(
            output_key.as_ref(),
            [
                110, 41, 196, 37, 188, 239, 92, 14, 14, 8, 176, 199, 3, 232, 46, 214, 237, 183, 11,
                238, 110, 19, 100, 64, 191, 71, 221, 96, 0, 165, 202, 6
            ]
        );
    }

    #[test]
    fn test_vector_3() {
        let output_key = PathBuf::from("/1/2/3").derive(&TEST_VECTOR_KEY);

        assert_eq!(
            output_key.as_ref(),
            [
                17, 67, 145, 251, 66, 229, 67, 213, 30, 37, 15, 106, 223, 215, 34, 87, 221, 46,
                192, 225, 50, 153, 127, 65, 168, 152, 14, 237, 100, 231, 142, 3
            ]
        );
    }
}
