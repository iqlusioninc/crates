//! Transaction message type i.e [`sdk.Msg`]
//!
//! [`sdk.Msg`]: https://godoc.org/github.com/cosmos/cosmos-sdk/types#Msg

mod builder;
mod field;
mod value;

pub use self::{builder::Builder, field::Field, value::Value};

use crate::{Schema, TypeName};
use prost_amino::encode_length_delimiter as encode_leb128; // Little-endian Base 128
use std::{collections::BTreeMap, iter::FromIterator};

/// Tags are indexes which identify message fields
pub type Tag = u64;

/// Transaction message type i.e. [`sdk.Msg`].
/// These serve as the payload for [`StdTx`] transactions.
///
/// [`StdTx`]: https://godoc.org/github.com/cosmos/cosmos-sdk/x/auth/types#StdTx
/// [`sdk.Msg`]: https://godoc.org/github.com/cosmos/cosmos-sdk/types#Msg
#[derive(Clone, Debug)]
pub struct Msg {
    /// Name of the message type
    type_name: TypeName,

    /// Fields in the message
    fields: Vec<Field>,
}

impl Msg {
    /// Compute `serde_json::Value` representing a `sdk.Msg`
    pub fn to_json_value(&self, schema: &Schema) -> serde_json::Value {
        // `BTreeMap` ensures fields are ordered for Cosmos's Canonical JSON
        let mut values = BTreeMap::new();

        for field in &self.fields {
            values.insert(
                field.name().to_string(),
                field.value().to_json_value(schema),
            );
        }

        let mut json = serde_json::Map::new();
        json.insert(
            "type".to_owned(),
            serde_json::Value::String(self.type_name.to_string()),
        );
        json.insert(
            "value".to_owned(),
            serde_json::Map::from_iter(values.into_iter()).into(),
        );
        serde_json::Value::Object(json)
    }

    /// Encode this message in the Amino wire format
    pub fn to_amino_bytes(&self) -> Vec<u8> {
        let mut result = self.type_name.amino_prefix();

        for field in &self.fields {
            // Compute the field prefix, which encodes the tag and wire type code
            let prefix = field.tag() << 3 | field.value().wire_type();
            encode_leb128(prefix as usize, &mut result).expect("LEB128 encoding error");

            let mut encoded_value = field.value().to_amino_bytes();
            encode_leb128(encoded_value.len(), &mut result).expect("LEB128 encoding error");
            result.append(&mut encoded_value);
        }

        result
    }
}
