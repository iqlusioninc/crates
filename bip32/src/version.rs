//! Version support.

use crate::{Error, Result};
use core::convert::{TryFrom, TryInto};

/// BIP32 versions are the leading prefix of a Base58-encoded extended key
/// interpreted as a 32-bit big endian integer after decoding.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Version {
    /// Mainnet public key.
    XPub,

    /// Mainnet private key.
    XPrv,

    /// Testnet public key.
    TPub,

    /// Testnet private key.
    TPrv,

    /// Other types of keys.
    Other(u32),
}

impl Version {
    /// Is this a mainnet key?
    pub fn is_mainnet(self) -> bool {
        matches!(self, Version::XPub | Version::XPrv)
    }

    /// Is this a testnet key?
    pub fn is_testnet(self) -> bool {
        matches!(self, Version::TPub | Version::TPrv)
    }

    /// Is this a public key?
    pub fn is_public(self) -> bool {
        matches!(self, Version::XPub | Version::TPub)
    }

    /// Is this a private key?
    pub fn is_private(self) -> bool {
        matches!(self, Version::XPrv | Version::TPrv)
    }
}

impl From<u32> for Version {
    fn from(n: u32) -> Version {
        match n {
            // `xpub` (mainnet public)
            0x0488B21E => Version::XPub,
            // `xprv` (mainnet private)
            0x0488ADE4 => Version::XPrv,
            // `tpub` (testnet public)
            0x043587CF => Version::TPub,
            // `tprv` (testnet private)
            0x04358394 => Version::TPrv,
            _ => Version::Other(n),
        }
    }
}

impl From<Version> for u32 {
    fn from(v: Version) -> u32 {
        match v {
            Version::XPub => 0x0488B21E,
            Version::XPrv => 0x0488ADE4,
            Version::TPub => 0x043587CF,
            Version::TPrv => 0x04358394,
            Version::Other(n) => n,
        }
    }
}

impl TryFrom<&[u8]> for Version {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Version> {
        Ok(u32::from_be_bytes(bytes.try_into()?).into())
    }
}
