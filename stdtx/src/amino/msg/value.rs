//! Message values

use crate::{
    address::Address,
    amino::schema::{Schema, ValueType},
    decimal::Decimal,
};
use subtle_encoding::hex;

/// Message values - data contained in fields of a message
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    /// Bytes
    Bytes(Vec<u8>),

    /// `sdk.AccAddress`: Cosmos SDK account addresses
    /// <https://godoc.org/github.com/cosmos/cosmos-sdk/types#AccAddress>
    SdkAccAddress(Address),

    /// `sdk.Dec`: Cosmos SDK decimals
    /// <https://godoc.org/github.com/cosmos/cosmos-sdk/types#Dec>
    SdkDecimal(Decimal),

    /// `sdk.ValAddress`: Cosmos SDK validator addresses
    /// <https://godoc.org/github.com/cosmos/cosmos-sdk/types#ValAddress>
    SdkValAddress(Address),

    /// Strings
    String(String),
}

impl Value {
    /// Get the type of this value
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Bytes(_) => ValueType::Bytes,
            Value::SdkAccAddress(_) => ValueType::SdkAccAddress,
            Value::SdkDecimal(_) => ValueType::SdkDecimal,
            Value::SdkValAddress(_) => ValueType::SdkValAddress,
            Value::String(_) => ValueType::String,
        }
    }

    /// Get the Amino/Proto wire type for this field
    /// See: <https://developers.google.com/protocol-buffers/docs/encoding#structure>
    pub(crate) fn wire_type(&self) -> u64 {
        match self {
            // Length-delimited types
            Value::Bytes(_)
            | Value::SdkAccAddress(_)
            | Value::SdkDecimal(_)
            | Value::SdkValAddress(_)
            | Value::String(_) => 2,
        }
    }

    /// Encode this value as Amino bytes
    pub(crate) fn to_amino_bytes(&self) -> Vec<u8> {
        match self {
            Value::Bytes(b) => b.clone(),
            Value::SdkAccAddress(addr) | Value::SdkValAddress(addr) => addr.as_ref().to_vec(),
            Value::SdkDecimal(decimal) => decimal.to_amino_bytes(),
            Value::String(s) => s.clone().into_bytes(),
        }
    }

    /// Encode this value as a [`serde_json::Value`]
    pub(crate) fn to_json_value(&self, schema: &Schema) -> serde_json::Value {
        serde_json::Value::String(match self {
            Value::Bytes(b) => String::from_utf8(hex::encode(b)).unwrap(),
            Value::SdkAccAddress(addr) => addr.to_bech32(schema.acc_prefix()),
            Value::SdkDecimal(decimal) => decimal.to_string(),
            Value::SdkValAddress(addr) => addr.to_bech32(schema.val_prefix()),
            Value::String(s) => s.clone(),
        })
    }
}

impl From<Decimal> for Value {
    fn from(dec: Decimal) -> Value {
        Value::SdkDecimal(dec)
    }
}

impl From<Vec<u8>> for Value {
    fn from(b: Vec<u8>) -> Value {
        Value::Bytes(b)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::String(s)
    }
}
