//! JSONRPC types

use failure::{format_err, Error};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::{self, Debug, Display};

/// Parse a JSONRPC response
pub fn parse_response<T>(response: &str) -> Result<ResponseWrapper<T>, Error>
where
    T: Debug + DeserializeOwned + Serialize,
{
    serde_json::from_str(response).map_err(|e| format_err!("error parsing JSON: {}", e))
}

/// Wrapper for all JSONRPC responses
#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseWrapper<T> {
    /// JSONRPC version
    pub jsonrpc: Version,

    /// ID
    pub id: Id,

    /// Result
    pub result: T,
}

/// JSONRPC version
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Version(String);

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// JSONRPC ID
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Id(String);

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
