//! Type definition within a schema

use super::{field, Field, ValueType};
use crate::{
    amino::{msg::Tag, TypeName},
    Error,
};
use eyre::{Result, WrapErr};
use serde::Deserialize;

/// Definition of a particular type in the schema
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Definition {
    /// Name of the type this definition is for
    type_name: TypeName,

    /// Fields in this type definition
    #[serde(deserialize_with = "field::deserialize_vec")]
    fields: Vec<Field>,
}

impl Definition {
    /// Create a new schema [`Definition`] with the given type name and fields
    pub fn new(type_name: TypeName, fields: impl Into<Vec<Field>>) -> Result<Self> {
        let fields = fields.into();

        if let Err(e) = field::validate(&fields) {
            return Err(Error::Parse).wrap_err_with(|| format!("{}", e));
        }

        Ok(Self { type_name, fields })
    }

    /// Get the [`TypeName`] defined by this schema.
    pub fn type_name(&self) -> &TypeName {
        &self.type_name
    }

    /// Get a list of [`Field`] types in this schema.
    pub fn fields(&self) -> &[Field] {
        self.fields.as_slice()
    }

    /// Get a [`Field`] by its [`TypeName`]
    pub fn get_field(&self, field_name: &TypeName) -> Option<&Field> {
        self.fields.iter().find(|field| field.name() == field_name)
    }

    /// Get the [`Tag`] for a [`Field`], ensuring is of the given [`ValueType`]
    pub fn get_field_tag(&self, field_name: &TypeName, value_type: ValueType) -> Result<Tag> {
        let field = self
            .get_field(field_name)
            .ok_or_else(|| Error::Type)
            .wrap_err_with(|| {
                format!(
                    "field name not found in `{}` schema: `{}`",
                    &self.type_name, field_name
                )
            })?;

        if field.value_type() != value_type {
            return Err(Error::Type).wrap_err_with(|| {
                format!(
                    "field `{}` of `{}` is not an {} (expected {})",
                    field_name,
                    &self.type_name,
                    value_type,
                    field.value_type()
                )
            });
        }

        Ok(field.tag())
    }
}
