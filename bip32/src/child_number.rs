//! Child numbers

use crate::{Error, Result};
use core::str::FromStr;

/// Index of a particular child key for a given (extended) private key.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ChildNumber(pub u32);

impl ChildNumber {
    /// Size of a child number when encoded as bytes.
    pub const BYTE_SIZE: usize = 4;

    /// Hardened child keys use indices 2^31 through 2^32-1.
    pub const HARDENED_FLAG: u32 = 1 << 31;

    /// Parse a child number from the byte encoding.
    pub fn from_bytes(bytes: [u8; Self::BYTE_SIZE]) -> Self {
        u32::from_be_bytes(bytes).into()
    }

    /// Serialize this child number as bytes.
    pub fn to_bytes(self) -> [u8; Self::BYTE_SIZE] {
        self.0.to_be_bytes()
    }

    /// Is this child number within the hardened range?
    pub fn is_hardened(&self) -> bool {
        self.0 & Self::HARDENED_FLAG != 0
    }
}

impl From<u32> for ChildNumber {
    fn from(n: u32) -> ChildNumber {
        ChildNumber(n)
    }
}

impl From<ChildNumber> for u32 {
    fn from(n: ChildNumber) -> u32 {
        n.0
    }
}

impl FromStr for ChildNumber {
    type Err = Error;

    fn from_str(child: &str) -> Result<ChildNumber> {
        let (child, mask) = match child.strip_suffix('\'') {
            Some(c) => (c, Self::HARDENED_FLAG),
            None => (child, 0),
        };

        let index = child.parse::<u32>().map_err(|_| Error::Decode)?;

        if index & Self::HARDENED_FLAG == 0 {
            Ok(ChildNumber(index | mask))
        } else {
            Err(Error::Decode)
        }
    }
}
