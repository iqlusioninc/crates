//! StdTx Amino types

use crate::{Signature, TypeName};
use prost_amino::{encode_length_delimiter, Message};
use prost_amino_derive::Message;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

use base64;
use serde_json::json;

/// StdTx Amino type
#[derive(Clone, Message)]
pub struct StdTx {
    /// Messages in transction
    #[prost_amino(bytes, repeated, tag = "1")]
    pub msg: Vec<Vec<u8>>,

    /// Feeds to be paid
    #[prost_amino(message)]
    pub fee: Option<StdFee>,

    /// Signatures
    #[prost_amino(message, repeated)]
    pub signatures: Vec<StdSignature>,

    /// Memo field
    #[prost_amino(string)]
    pub memo: String,
}

impl StdTx {
    /// Encode this [`StdTx`] in Amino encoding identifying it with the given
    /// type name (e.g. `cosmos-sdk/StdTx`)
    pub fn to_amino_bytes(&self, type_name: &TypeName) -> Vec<u8> {
        let mut amino_tx = type_name.amino_prefix();
        self.encode(&mut amino_tx).expect("LEB128 encoding error");

        let mut amino_encoded = vec![];
        encode_length_delimiter(amino_tx.len(), &mut amino_encoded).expect("LEB128 encoding error");
        amino_encoded.append(&mut amino_tx);
        amino_encoded
    }
}

/// StdFee amino type
#[derive(Clone, Message, Serialize, Deserialize)]
pub struct StdFee {
    /// Fee to be paid
    #[prost_amino(message, repeated, tag = "1")]
    pub amount: Vec<Coin>,

    /// Gas requested for transaction
    #[prost_amino(uint64)]
    #[serde(serialize_with = "serialize_u64", deserialize_with = "parse_u64")]
    pub gas: u64,
}

/// Serialize u64 as a string for proto3 JSON encoding.
pub(crate) fn serialize_u64<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    format!("{}", value).serialize(serializer)
}

/// Parse u64 from a string for proto3 JSON decoding.
pub(crate) fn parse_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse::<u64>()
        .map_err(|e| D::Error::custom(format!("{}", e)))
}

impl StdFee {
    /// Create a [`StdFee`] for a gas-only transaction
    pub fn for_gas(gas: u64) -> Self {
        StdFee {
            amount: vec![],
            gas,
        }
    }
    /// Compute `serde_json::Value` representing this fee
    pub fn to_json_value(&self) -> serde_json::Value {
        let amount = self
            .amount
            .iter()
            .map(|amt| amt.to_json_value())
            .collect::<Vec<_>>();

        json!({
            "amount": amount,
            "gas": self.gas.to_string()
        })
    }
}

/// Coin Amino type
#[derive(Clone, Message, Serialize, Deserialize)]
pub struct Coin {
    /// Denomination of coin
    #[prost_amino(string, tag = "1")]
    pub denom: String,

    /// Amount of the given denomination
    #[prost_amino(string)]
    pub amount: String,
}

impl Coin {
    /// Compute `serde_json::Value` representing this coin
    pub fn to_json_value(&self) -> serde_json::Value {
        json!({
            "denom": self.denom,
            "amount": self.amount
        })
    }
}

/// StdSignature amino type
#[derive(Clone, Message, Serialize, Deserialize)]
pub struct StdSignature {
    /// Public key which can verify this signature
    #[prost_amino(bytes, tag = "1", amino_name = "tendermint/PubKeySecp256k1")]
    #[serde(serialize_with = "serialize_base64", deserialize_with = "parse_base64")]
    pub public_key: Vec<u8>,

    /// Serialized signature
    #[prost_amino(bytes)]
    #[serde(serialize_with = "serialize_base64", deserialize_with = "parse_base64")]
    pub signature: Vec<u8>,
}

/// Serialize bytes as base64 string for proto3 JSON encoding.
pub(crate) fn serialize_base64<S>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&base64::encode(value))
}

/// Parse bytes from a base64 string for proto3 JSON decoding.
pub(crate) fn parse_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).and_then(|string| {
        base64::decode(&string).map_err(|err| D::Error::custom(format!("{}", err)))
    })
}

impl From<Signature> for StdSignature {
    fn from(signature: Signature) -> StdSignature {
        StdSignature {
            public_key: vec![],
            signature: signature.as_ref().to_vec(),
        }
    }
}
