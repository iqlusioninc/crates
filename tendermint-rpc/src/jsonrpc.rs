//! JSONRPC types

use failure::{format_err, Error};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::{self, Display};
use tendermint::Address;

/// JSONRPC requests
pub trait Request {
    /// Response type for this command
    type Response: Response;

    /// Perform this request against the given RPC endpoint
    fn perform(&self, rpc_addr: &Address) -> Result<Self::Response, Error> {
        let (host, port) = match rpc_addr {
            Address::Tcp { host, port, .. } => (host, *port),
            Address::Unix { .. } => panic!("UNIX sockets presently unsupported"),
        };

        // TODO(tarcieri): persistent clients
        let http = gaunt::Connection::new(host, port, &Default::default())
            .map_err(|e| format_err!("error connecting to RPC service: {}", e))?;

        let response = http
            .get(self.path(), &self.body())
            .map_err(|e| format_err!("RPC HTTP error: {}", e))?
            .into_vec();

        Self::Response::from_json(&String::from_utf8(response)?)
    }

    /// Path for this request
    fn path(&self) -> gaunt::Path;

    /// HTTP request body for this request
    fn body(&self) -> gaunt::request::Body {
        gaunt::request::Body::from(b"".as_ref())
    }
}

/// JSONRPC responses
pub trait Response: Serialize + DeserializeOwned + Sized {
    /// Parse a JSONRPC response from a JSON string
    fn from_json(response: &str) -> Result<Self, Error> {
        let wrapper: ResponseWrapper<Self> =
            serde_json::from_str(response).map_err(|e| format_err!("error parsing JSON: {}", e))?;

        // TODO(tarcieri): check JSONRPC version/ID?
        Ok(wrapper.result)
    }
}

/// Wrapper for all JSONRPC responses
#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseWrapper<R> {
    /// JSONRPC version
    pub jsonrpc: Version,

    /// ID
    pub id: Id,

    /// Result
    pub result: R,
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
