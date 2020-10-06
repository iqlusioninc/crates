//! Transaction message type i.e [`sdk.Msg`]
//!
//! [`sdk.Msg`]: https://godoc.org/github.com/cosmos/cosmos-sdk/types#Msg

mod builder;
mod field;
mod value;

pub use self::{builder::Builder, field::Field, value::Value};

use crate::{
    error::ErrorKind,
    schema::{Schema, ValueType},
    Address, Decimal, Error, TypeName,
};
use anomaly::{fail, format_err};
use prost_amino::encode_length_delimiter as encode_leb128; // Little-endian Base 128
use std::{collections::BTreeMap, iter::FromIterator};
use subtle_encoding::hex;

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
    /// Parse a [`Msg`] from a [`serde_json::Value`] following the provided
    /// [`Schema`] for field definitions.
    pub fn from_json_value(schema: &Schema, json_value: serde_json::Value) -> Result<Self, Error> {
        let json_obj = match json_value.as_object() {
            Some(obj) => obj,
            None => fail!(ErrorKind::Type, "expected JSON object"),
        };

        if json_obj.len() != 2 {
            fail!(ErrorKind::Parse, "unexpected keys in JSON object");
        }

        let type_name = match json_obj.get("type").and_then(|v| v.as_str()) {
            Some(name) => name.parse::<TypeName>()?,
            None => fail!(ErrorKind::Parse, "no `type` key in JSON object"),
        };

        let type_def = match schema.get_definition(&type_name) {
            Some(def) => def,
            None => fail!(
                ErrorKind::FieldName,
                "no type definition for `{}`",
                type_name
            ),
        };

        let value_obj = match json_obj.get("value").and_then(|v| v.as_object()) {
            Some(obj) => obj,
            None => fail!(
                ErrorKind::Parse,
                "missing or invalid `value` key in JSON object"
            ),
        };

        let mut fields = vec![];

        for (json_name, json_value) in value_obj {
            let field_name = json_name.parse::<TypeName>()?;

            let field_def = match type_def.get_field(&field_name) {
                Some(def) => def,
                None => fail!(ErrorKind::FieldName, "unknown field name: `{}`", field_name),
            };

            let value_str = match json_value.as_str() {
                Some(s) => s,
                None => fail!(
                    ErrorKind::Parse,
                    "couldn't parse JSON value: `{}`",
                    field_name
                ),
            };

            let value = match field_def.value_type() {
                ValueType::Bytes => hex::decode(value_str).map(Value::Bytes).map_err(|e| {
                    format_err!(ErrorKind::Parse, "invalid hex-encoded bytes: {}", e)
                })?,
                ValueType::SdkAccAddress => {
                    let (hrp, addr) = Address::from_bech32(value_str)?;

                    if schema.acc_prefix() != hrp {
                        fail!(ErrorKind::Parse, "invalid account prefix: {}", value_str);
                    }

                    Value::SdkAccAddress(addr)
                }
                ValueType::SdkValAddress => {
                    let (hrp, addr) = Address::from_bech32(value_str)?;

                    if schema.val_prefix() != hrp {
                        fail!(ErrorKind::Parse, "invalid validator prefix: {}", value_str);
                    }

                    Value::SdkValAddress(addr)
                }
                ValueType::SdkDecimal => Value::SdkDecimal(value_str.parse::<Decimal>()?),
                ValueType::String => Value::String(value_str.to_owned()),
            };

            fields.push(Field::new(field_def.tag(), field_name, value));
        }

        fields.sort_by(|f1, f2| f1.tag().cmp(&f2.tag()));
        Ok(Self { type_name, fields })
    }

    /// Get the type name for this [`Msg`]
    pub fn type_name(&self) -> &TypeName {
        &self.type_name
    }

    /// Get the [`Field`] entries in this [`Msg`]
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    /// Compute [`serde_json::Value`] representing a `sdk.Msg`
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

#[cfg(test)]
mod tests {
    use super::{Msg, Value};
    use crate::{Address, Schema};
    use serde_json::json;

    /// Path to an example schema TOML file
    const EXAMPLE_SCHEMA: &str = "tests/support/example_schema.toml";

    #[test]
    fn from_json_value() {
        let schema = Schema::load_toml(EXAMPLE_SCHEMA).unwrap();

        let example_json = json!({
            "type": "oracle/MsgExchangeRateVote",
            "value": {
                "denom": "umnt",
                "exchange_rate": "-1.000000000000000000",
                "feeder": "terra1t9et8wjeh8d0ewf4lldchterxsmhpcgg5auy47",
                "salt": "4V7A",
                "validator": "terravaloper1grgelyng2v6v3t8z87wu3sxgt9m5s03x2mfyu7"
            }
        });

        let msg = Msg::from_json_value(&schema, example_json.clone()).unwrap();

        assert_eq!(msg.type_name().as_str(), "oracle/MsgExchangeRateVote");
        assert_eq!(msg.fields().len(), 5);

        let field1 = &msg.fields()[0];
        assert_eq!(field1.tag(), 1);
        assert_eq!(field1.value(), &Value::SdkDecimal("-1".parse().unwrap()));

        let field2 = &msg.fields()[1];
        assert_eq!(field2.tag(), 2);
        assert_eq!(field2.value(), &Value::String("4V7A".to_owned()));

        let field3 = &msg.fields()[2];
        assert_eq!(field3.tag(), 3);
        assert_eq!(field3.value(), &Value::String("umnt".to_owned()));

        let field4 = &msg.fields()[3];
        assert_eq!(field4.tag(), 4);
        assert_eq!(
            field4.value(),
            &Value::SdkAccAddress(Address([
                89, 114, 179, 186, 89, 185, 218, 252, 185, 53, 255, 219, 139, 175, 35, 52, 55, 112,
                225, 8,
            ]))
        );

        let field5 = &msg.fields()[4];
        assert_eq!(field5.tag(), 5);
        assert_eq!(
            field5.value(),
            &Value::SdkValAddress(Address([
                64, 209, 159, 146, 104, 83, 52, 200, 172, 226, 63, 157, 200, 192, 200, 89, 119, 72,
                62, 38,
            ]))
        );

        assert_eq!(msg.to_json_value(&schema), example_json);
    }
}
