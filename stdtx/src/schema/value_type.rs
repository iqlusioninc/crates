//! Types of values that can be present in an `sdk.Msg`

use crate::error::{Error, ErrorKind};
use anomaly::fail;
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
            ValueType::SdkAccAddress => "sdk.AccAddress",
            ValueType::SdkDecimal => "sdk.Dec",
            ValueType::SdkValAddress => "sdk.ValAddress",
            ValueType::String => "string",
        })
    }
}

impl FromStr for ValueType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "sdk.AccAddress" => ValueType::SdkAccAddress,
            "sdk.Dec" => ValueType::SdkDecimal,
            "sdk.ValAddress" => ValueType::SdkValAddress,
            "string" => ValueType::String,
            _ => fail!(ErrorKind::Parse, "unknown value type: `{}`", s),
        })
    }
}

impl<'de> Deserialize<'de> for ValueType {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}
