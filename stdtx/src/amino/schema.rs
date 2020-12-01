//! Amino schema for an [`sdk.Msg`].
//!
//! Schema files are similar to Protobuf schemas, but use a TOML-based syntax.
//!
//! # Example TOML File
//!
//! Below is an example TOML file defining an `sdk.Msg`. This example defines
//! a type named `oracle/MsgExchangeRatePrevote`:
//!
//! ```toml
//! # Example StdTx message schema definition.
//! #
//! # Message types taken from Terra's oracle voter transactions:
//! # <https://docs.terra.money/docs/dev-spec-oracle#message-types>
//!
//! # StdTx namespace for schema definitions
//! # (e.g. `cosmos-sdk/StdTx` for Cosmos SDK)
//! namespace = "core/StdTx"
//!
//! # Bech32 address prefixes
//! acc_prefix = "terra"
//! val_prefix = "terravaloper"
//!
//! [[definition]]
//! type_name = "oracle/MsgExchangeRatePrevote"
//! fields = [
//!     { name = "hash",  type = "string" },
//!     { name = "denom", type = "string" },
//!     { name = "feeder", type = "sdk.AccAddress" },
//!     { name = "validator", type = "sdk.ValAddress" },
//! ]
//!
//! [[definition]]
//! type_name = "oracle/MsgExchangeRateVote"
//! fields = [
//!     # explicit field tag example - will start from "1" otherwise
//!     { name = "exchange_rate", type = "sdk.Dec", tag = 1 },
//!     { name = "salt", type = "string" },
//!     { name = "denom", type = "string" },
//!     { name = "feeder", type = "sdk.AccAddress" },
//!     { name = "validator", type = "sdk.ValAddress" },
//! ]
//! ```
//!
//! [`sdk.Msg`]: https://godoc.org/github.com/cosmos/cosmos-sdk/types#Msg

mod definition;
mod field;
mod value_type;

pub use self::{definition::Definition, field::Field, value_type::ValueType};

use super::TypeName;
use crate::Error;
use eyre::{Result, WrapErr};
use serde::Deserialize;
use std::{fs, path::Path, str::FromStr};

/// Schema definition for an [`sdk.Msg`] to be included in an [`StdTx`].
///
/// The schema includes information about field identifiers and associated types.
///
/// [`StdTx`]: https://godoc.org/github.com/cosmos/cosmos-sdk/x/auth/types#StdTx
/// [`sdk.Msg`]: https://godoc.org/github.com/cosmos/cosmos-sdk/types#Msg
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Schema {
    /// `StdTx` namespace for schema (e.g. `cosmos-sdk/StdTx`)
    namespace: TypeName,

    /// Bech32 prefix for account addresses
    acc_prefix: String,

    /// Bech32 prefix for validator consensus addresses
    val_prefix: String,

    /// Schema definitions
    #[serde(rename = "definition")]
    definitions: Vec<Definition>,
}

impl Schema {
    /// Create a new [`Schema`] with the given `StdTx` namespace and [`Definition`] set
    pub fn new(
        namespace: TypeName,
        acc_prefix: impl Into<String>,
        val_prefix: impl Into<String>,
        definitions: impl Into<Vec<Definition>>,
    ) -> Self {
        Self {
            namespace,
            acc_prefix: acc_prefix.into(),
            val_prefix: val_prefix.into(),
            definitions: definitions.into(),
        }
    }

    /// Load a TOML file describing a [`Schema`]
    pub fn load_toml(path: impl AsRef<Path>) -> Result<Self> {
        match fs::read_to_string(path.as_ref()) {
            Ok(s) => s.parse(),
            Err(e) => Err(Error::Io)
                .wrap_err_with(|| format!("couldn't open {}: {}", path.as_ref().display(), e)),
        }
    }

    /// Get the transaction namespace for this schema (e.g. `cosmos-sdk/StdTx`)
    pub fn namespace(&self) -> &TypeName {
        &self.namespace
    }

    /// Get the Bech32 prefix for account addresses
    pub fn acc_prefix(&self) -> &str {
        self.acc_prefix.as_ref()
    }

    /// Get the Bech32 prefix for validator addresses
    pub fn val_prefix(&self) -> &str {
        self.val_prefix.as_ref()
    }

    /// [`Definition`] types found in this [`Schema`]
    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    /// Get a schema [`Definition`] for the given [`TypeName`]
    pub fn get_definition(&self, type_name: &TypeName) -> Option<&Definition> {
        self.definitions
            .iter()
            .find(|def| def.type_name() == type_name)
    }
}

impl FromStr for Schema {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        Ok(toml::from_str(s)?)
    }
}
