//! Types of values that can be present in an `sdk.Msg`

use crate::Error;
use eyre::{Result, WrapErr};
use serde::{de, Deserialize};
use std::{
    fmt::{self, Display},
    str::FromStr,
};

/// Types of Amino values which can be included in a [`sdk.Msg`]
///
/// [`sdk.Msg`]: https://godoc.org/github.com/cosmos/cosmos-sdk/types#Msg
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValueType {
    /// Bytes
    Bytes,

    /// `sdk.AccAddress`: Cosmos SDK account addresses
    /// <https://godoc.org/github.com/cosmos/cosmos-sdk/types#AccAddress>
    SdkAccAddress,

    /// `sdk.Dec`: Cosmos SDK decimals
    /// <https://godoc.org/github.com/cosmos/cosmos-sdk/types#Dec>
    SdkDecimal,

    /// `sdk.ValAddress`: Cosmos SDK validator addresses
    /// <https://godoc.org/github.com/cosmos/cosmos-sdk/types#ValAddress>
    SdkValAddress,

    /// Strings
    String,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ValueType::Bytes => "bytes",
            ValueType::SdkAccAddress => "sdk.AccAddress",
            ValueType::SdkDecimal => "sdk.Dec",
            ValueType::SdkValAddress => "sdk.ValAddress",
            ValueType::String => "string",
        })
    }
}

impl FromStr for ValueType {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "bytes" => Ok(ValueType::Bytes),
            "sdk.AccAddress" => Ok(ValueType::SdkAccAddress),
            "sdk.Dec" => Ok(ValueType::SdkDecimal),
            "sdk.ValAddress" => Ok(ValueType::SdkValAddress),
            "string" => Ok(ValueType::String),
            _ => Err(Error::Parse).wrap_err_with(|| format!("unknown value type: `{}`", s)),
        }
    }
}

impl<'de> Deserialize<'de> for ValueType {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}
