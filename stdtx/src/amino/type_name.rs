//! Amino type names

use crate::Error;
use eyre::{Result, WrapErr};
use serde::{de, Deserialize};
use sha2::{Digest, Sha256};
use std::{
    fmt::{self, Display},
    str::FromStr,
};

/// Name of an Amino type
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct TypeName(String);

impl TypeName {
    /// Create a new `sdk.Msg` type name
    pub fn new(name: impl AsRef<str>) -> Result<Self> {
        name.as_ref().parse()
    }

    /// Borrow this [`TypeName`] as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Compute the Amino prefix for this [`TypeName`]
    pub fn amino_prefix(&self) -> Vec<u8> {
        Sha256::digest(self.0.as_bytes())
            .iter()
            .filter(|&x| *x != 0x00)
            .skip(3)
            .filter(|&x| *x != 0x00)
            .cloned()
            .take(4)
            .collect()
    }
}

impl AsRef<str> for TypeName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for TypeName {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

impl FromStr for TypeName {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        for c in s.chars() {
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '/' | '_' => (),
                _ => {
                    return Err(Error::Parse).wrap_err_with(|| {
                        format!("invalid character `{}` in type name: `{}`", c, s)
                    })
                }
            }
        }

        Ok(TypeName(s.to_owned()))
    }
}

impl TryFrom<&str> for TypeName {
    type Error = eyre::Report;

    fn try_from(s: &str) -> Result<Self> {
        s.parse()
    }
}
