//! Address types (account or validator)

use crate::error::{Error, ErrorKind};
use anomaly::ensure;
use serde::{de, Deserialize};
use std::convert::TryInto;
use subtle_encoding::bech32;

/// Size of an address
pub const ADDRESS_SIZE: usize = 20;

/// Address type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Address(pub [u8; ADDRESS_SIZE]);

impl Address {
    /// Parse an address from its Bech32 form
    pub fn from_bech32(addr_bech32: impl AsRef<str>) -> Result<(String, Address), Error> {
        let (hrp, addr) = bech32::decode(addr_bech32.as_ref())?;

        ensure!(
            addr.len() == ADDRESS_SIZE,
            ErrorKind::Address,
            "invalid length for decoded address: {} (expected {})",
            addr.len(),
            ADDRESS_SIZE
        );

        Ok((hrp, Address(addr.as_slice().try_into().unwrap())))
    }

    /// Encode this address as Bech32
    pub fn to_bech32(&self, hrp: &str) -> String {
        bech32::encode(hrp, &self.0)
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; ADDRESS_SIZE]> for Address {
    fn from(addr: [u8; ADDRESS_SIZE]) -> Address {
        Address(addr)
    }
}

impl From<Address> for [u8; ADDRESS_SIZE] {
    fn from(addr: Address) -> [u8; ADDRESS_SIZE] {
        addr.0
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let bech32_addr = String::deserialize(deserializer)?;

        let (_, addr) = Self::from_bech32(&bech32_addr).map_err(|_| {
            de::Error::custom(format!("invalid bech32-encoded address: {}", bech32_addr))
        })?;

        Ok(addr)
    }
}
