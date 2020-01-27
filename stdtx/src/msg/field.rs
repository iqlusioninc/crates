//! Message fields

use super::{Tag, Value};
use crate::type_name::TypeName;

/// Message fields
#[derive(Clone, Debug)]
pub struct Field {
    /// Field number to use as the key in an Amino message.
    tag: Tag,

    /// Name of this field
    name: TypeName,

    /// Amino type to serialize this field as
    value: Value,
}

impl Field {
    /// Create a new message field
    pub fn new(tag: Tag, name: TypeName, value: impl Into<Value>) -> Self {
        Self {
            tag,
            name,
            value: value.into(),
        }
    }

    /// Get this field's [`Tag`]
    pub fn tag(&self) -> Tag {
        self.tag
    }

    /// Get this field's [`TypeName`]
    pub fn name(&self) -> &TypeName {
        &self.name
    }

    /// Get this field's [`Value`]
    pub fn value(&self) -> &Value {
        &self.value
    }
}
