//! StdTx Amino types

use crate::{Signature, TypeName};
use prost_amino::{encode_length_delimiter, Message};
use prost_amino_derive::Message;
use serde::{de, ser, Deserialize, Serialize};

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
#[derive(Clone, Message, Deserialize, Serialize)]
pub struct StdFee {
    /// Fee to be paid
    #[prost_amino(message, repeated, tag = "1")]
    pub amount: Vec<Coin>,

    /// Gas requested for transaction
    #[prost_amino(uint64)]
    #[serde(serialize_with = "serialize_gas", deserialize_with = "parse_gas")]
    pub gas: u64,
}

impl StdFee {
    /// Create a [`StdFee`] for a gas-only transaction
    pub fn for_gas(gas: u64) -> Self {
        StdFee {
            amount: vec![],
            gas,
        }
    }
}

/// Coin Amino type
#[derive(Clone, Message, Deserialize, Serialize)]
pub struct Coin {
    /// Denomination of coin
    #[prost_amino(string, tag = "1")]
    pub denom: String,

    /// Amount of the given denomination
    #[prost_amino(string)]
    pub amount: String,
}

/// StdSignature amino type
#[derive(Clone, Message)]
pub struct StdSignature {
    /// Public key which can verify this signature
    #[prost_amino(bytes, tag = "1", amino_name = "tendermint/PubKeySecp256k1")]
    pub pub_key: Vec<u8>,

    /// Serialized signature
    #[prost_amino(bytes)]
    pub signature: Vec<u8>,
}

impl From<Signature> for StdSignature {
    fn from(signature: Signature) -> StdSignature {
        StdSignature {
            pub_key: vec![],
            signature: signature.as_ref().to_vec(),
        }
    }
}

fn parse_gas<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: de::Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)
}

fn serialize_gas<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    serializer.serialize_str(&value.to_string())
}
