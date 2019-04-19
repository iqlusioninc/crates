//! Network channels

use crate::jsonrpc;
use std::fmt::{self, Display};

/// Individual network channel (from `/net_info`)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Channel {
    /// Channel ID
    #[serde(rename = "ID")]
    pub id: Id,

    /// Capacity of the send queue
    #[serde(
        rename = "SendQueueCapacity",
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    pub send_queue_capacity: u64,

    /// Size of the send queue
    #[serde(
        rename = "SendQueueSize",
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    pub send_queue_size: u64,

    /// Priority value
    #[serde(
        rename = "Priority",
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    pub priority: u64,

    /// Amount of data recently sent
    #[serde(
        rename = "RecentlySent",
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    pub recently_sent: u64,
}

/// Channel IDs
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Id(pub u64);

impl Id {
    /// Get the current voting power as an integer
    pub fn value(self) -> u64 {
        self.0
    }
}

impl From<Id> for u64 {
    fn from(id: Id) -> u64 {
        id.value()
    }
}

impl From<u64> for Id {
    fn from(id: u64) -> Id {
        Id(id)
    }
}

/// Channels (from `/status`)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Channels(String);

impl Display for Channels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
