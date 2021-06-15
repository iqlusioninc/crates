//! Child numbers

use crate::{Error, Result};
use core::{
    fmt::{self, Display},
    str::FromStr,
};

/// Index of a particular child key for a given (extended) private key.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ChildNumber(pub u32);

impl ChildNumber {
    /// Size of a child number when encoded as bytes.
    pub const BYTE_SIZE: usize = 4;

    /// Hardened child keys use indices 2^31 through 2^32-1.
    pub const HARDENED_FLAG: u32 = 1 << 31;

    /// Create new [`ChildNumber`] with the given index and hardened flag.
    ///
    /// Returns an error if it is equal to or greater than [`Self::HARDENED_FLAG`].
    pub fn new(index: u32, hardened: bool) -> Result<Self> {
        if index >= Self::HARDENED_FLAG {
            Err(Error::ChildNumber)
        } else if hardened {
            Ok(Self(index | Self::HARDENED_FLAG))
        } else {
            Ok(Self(index))
        }
    }

    /// Parse a child number from the byte encoding.
    pub fn from_bytes(bytes: [u8; Self::BYTE_SIZE]) -> Self {
        u32::from_be_bytes(bytes).into()
    }

    /// Serialize this child number as bytes.
    pub fn to_bytes(self) -> [u8; Self::BYTE_SIZE] {
        self.0.to_be_bytes()
    }

    /// Get the index number for this [`ChildNumber`], i.e. with
    /// [`Self::HARDENED_FLAG`] cleared.
    pub fn index(self) -> u32 {
        self.0 & !Self::HARDENED_FLAG
    }

    /// Is this child number within the hardened range?
    pub fn is_hardened(&self) -> bool {
        self.0 & Self::HARDENED_FLAG != 0
    }
}

impl Display for ChildNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index())?;

        if self.is_hardened() {
            f.write_str("\'")?;
        }

        Ok(())
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
        let (child, hardened) = match child.strip_suffix('\'') {
            Some(c) => (c, true),
            None => (child, false),
        };

        let index = child.parse().map_err(|_| Error::ChildNumber)?;
        ChildNumber::new(index, hardened)
    }
}

#[cfg(test)]
mod tests {
    use super::ChildNumber;
    use crate::Error;

    #[test]
    fn parse_non_hardened() {
        let n = "42".parse::<ChildNumber>().unwrap();
        assert_eq!(n, ChildNumber::new(42, false).unwrap());
        assert_eq!(n.index(), 42);
        assert!(!n.is_hardened());
    }

    #[test]
    fn parse_hardened() {
        let n = "42'".parse::<ChildNumber>().unwrap();
        assert_eq!(n, ChildNumber::new(42, true).unwrap());
        assert_eq!(n.index(), 42);
        assert!(n.is_hardened());
    }

    #[test]
    fn parse_rejects_invalid() {
        assert_eq!("42!".parse::<ChildNumber>(), Err(Error::ChildNumber));
    }

    #[test]
    fn index_overflow() {
        // Invalid index
        let index = ChildNumber::HARDENED_FLAG;
        assert_eq!(ChildNumber::new(index, false), Err(Error::ChildNumber));
        assert_eq!(ChildNumber::new(index, true), Err(Error::ChildNumber));
    }
}
