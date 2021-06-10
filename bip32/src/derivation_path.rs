//! Derivation paths

use crate::{ChildNumber, Error, Result};
use alloc::vec::Vec;
use core::str::FromStr;

/// Derivation paths within a hierarchical keyspace.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct DerivationPath {
    path: Vec<ChildNumber>,
}

impl FromStr for DerivationPath {
    type Err = Error;

    fn from_str(path: &str) -> Result<DerivationPath> {
        let mut path = path.split('/');

        if path.next() != Some("m") {
            return Err(Error::Decode);
        }

        Ok(DerivationPath {
            path: path.map(str::parse).collect::<Result<Vec<_>>>()?,
        })
    }
}

impl AsRef<[ChildNumber]> for DerivationPath {
    fn as_ref(&self) -> &[ChildNumber] {
        &self.path
    }
}
