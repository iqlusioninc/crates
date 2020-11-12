//! Transaction message builder

use super::{Field, Msg, Value};
use crate::{
    address::Address,
    amino::{
        schema::{Definition, Schema, ValueType},
        type_name::TypeName,
    },
    decimal::Decimal,
    error::{Error, ErrorKind},
};
use anomaly::{ensure, format_err};
use std::convert::TryInto;

/// Transaction message builder
pub struct Builder<'a> {
    /// Schema for the message we're building
    schema_definition: &'a Definition,

    /// Name of the message type
    type_name: TypeName,

    /// Bech32 prefix for account addresses
    acc_prefix: String,

    /// Bech32 prefix for validator consensus addresses
    val_prefix: String,

    /// Fields in the message
    fields: Vec<Field>,
}

impl<'a> Builder<'a> {
    /// Create a new message builder for the given schema and message type
    pub fn new(
        schema: &'a Schema,
        type_name: impl TryInto<TypeName, Error = Error>,
    ) -> Result<Self, Error> {
        let type_name = type_name.try_into()?;

        let schema_definition = schema.get_definition(&type_name).ok_or_else(|| {
            format_err!(
                ErrorKind::Type,
                "type not found in schema: `{}`",
                &type_name
            )
        })?;

        Ok(Self {
            schema_definition,
            type_name,
            acc_prefix: schema.acc_prefix().to_owned(),
            val_prefix: schema.val_prefix().to_owned(),
            fields: vec![],
        })
    }

    /// `sdk.AccAddress`: Cosmos SDK account addresses
    /// <https://godoc.org/github.com/cosmos/cosmos-sdk/types#AccAddress>
    pub fn acc_address(
        &mut self,
        field_name: impl TryInto<TypeName, Error = Error>,
        address: Address,
    ) -> Result<&mut Self, Error> {
        let field_name = field_name.try_into()?;
        let tag = self
            .schema_definition
            .get_field_tag(&field_name, ValueType::SdkAccAddress)?;

        let field = Field::new(tag, field_name, Value::SdkAccAddress(address));
        self.fields.push(field);

        Ok(self)
    }

    /// `sdk.AccAddress` encoded as Bech32
    pub fn acc_address_bech32(
        &mut self,
        field_name: impl TryInto<TypeName, Error = Error>,
        addr_bech32: impl AsRef<str>,
    ) -> Result<&mut Self, Error> {
        let (hrp, address) = Address::from_bech32(addr_bech32)?;

        ensure!(
            hrp == self.acc_prefix,
            ErrorKind::Address,
            "invalid account address prefix: `{}` (expected `{}`)",
            hrp,
            self.acc_prefix,
        );

        self.acc_address(field_name, address)
    }

    /// Bytes
    pub fn bytes(
        &mut self,
        field_name: impl TryInto<TypeName, Error = Error>,
        b: impl Into<Vec<u8>>,
    ) -> Result<&mut Self, Error> {
        let field_name = field_name.try_into()?;
        let tag = self
            .schema_definition
            .get_field_tag(&field_name, ValueType::Bytes)?;

        let field = Field::new(tag, field_name, Value::Bytes(b.into()));
        self.fields.push(field);

        Ok(self)
    }

    /// `sdk.Dec`: Cosmos SDK decimals
    /// <https://godoc.org/github.com/cosmos/cosmos-sdk/types#Dec>s
    pub fn decimal(
        &mut self,
        field_name: impl TryInto<TypeName, Error = Error>,
        value: impl Into<Decimal>,
    ) -> Result<&mut Self, Error> {
        let field_name = field_name.try_into()?;

        let tag = self
            .schema_definition
            .get_field_tag(&field_name, ValueType::SdkDecimal)?;

        let field = Field::new(tag, field_name, Value::SdkDecimal(value.into()));
        self.fields.push(field);

        Ok(self)
    }

    /// `sdk.ValAddress`: Cosmos SDK validator addresses
    /// <https://godoc.org/github.com/cosmos/cosmos-sdk/types#ValAddress>
    pub fn val_address(
        &mut self,
        field_name: impl TryInto<TypeName, Error = Error>,
        address: Address,
    ) -> Result<&mut Self, Error> {
        let field_name = field_name.try_into()?;
        let tag = self
            .schema_definition
            .get_field_tag(&field_name, ValueType::SdkValAddress)?;

        let field = Field::new(tag, field_name, Value::SdkValAddress(address));
        self.fields.push(field);

        Ok(self)
    }

    /// `sdk.ValAddress` encoded as Bech32
    pub fn val_address_bech32(
        &mut self,
        field_name: impl TryInto<TypeName, Error = Error>,
        addr_bech32: impl AsRef<str>,
    ) -> Result<&mut Self, Error> {
        let (hrp, address) = Address::from_bech32(addr_bech32)?;

        ensure!(
            hrp == self.val_prefix,
            ErrorKind::Address,
            "invalid validator address prefix: `{}` (expected `{}`)",
            hrp,
            self.val_prefix,
        );

        self.val_address(field_name, address)
    }

    /// Strings
    pub fn string(
        &mut self,
        field_name: impl TryInto<TypeName, Error = Error>,
        s: impl Into<String>,
    ) -> Result<&mut Self, Error> {
        let field_name = field_name.try_into()?;
        let tag = self
            .schema_definition
            .get_field_tag(&field_name, ValueType::String)?;

        let field = Field::new(tag, field_name, Value::String(s.into()));
        self.fields.push(field);

        Ok(self)
    }

    /// Consume this builder and output a message
    pub fn to_msg(&self) -> Msg {
        Msg {
            type_name: self.type_name.clone(),
            fields: self.fields.clone(),
        }
    }
}
