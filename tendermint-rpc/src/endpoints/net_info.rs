//! RPC wrapper for `/net_info` endpoint

use crate::{channel::Channel, jsonrpc, node_info::NodeInfo};
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt::{self, Display},
    net::IpAddr,
    time::Duration,
};

/// Request the status of the node
#[derive(Default)]
pub struct NetInfo;

impl jsonrpc::Request for NetInfo {
    type Response = NetInfoResponse;

    fn path(&self) -> gaunt::Path {
        "/net_info".into()
    }
}

/// Status responses
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetInfoResponse {
    /// Are we presently listening?
    pub listening: bool,

    /// Active listeners
    pub listeners: Vec<Listener>,

    /// Number of connected peers
    #[serde(
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    pub n_peers: u64,

    /// Peer information
    pub peers: Vec<PeerInfo>,
}

impl jsonrpc::Response for NetInfoResponse {}

/// Listener information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Listener(String);

impl Display for Listener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Peer information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PeerInfo {
    /// Node information
    pub node_info: NodeInfo,

    /// Is this an outbound connection?
    pub is_outbound: bool,

    /// Connection status
    pub connection_status: ConnectionStatus,

    /// Remote IP address
    pub remote_ip: IpAddr,
}

/// Connection status information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConnectionStatus {
    /// Duration of this connection
    #[serde(
        rename = "Duration",
        serialize_with = "serialize_duration",
        deserialize_with = "deserialize_duration"
    )]
    pub duration: Duration,

    /// Send monitor
    #[serde(rename = "SendMonitor")]
    pub send_monitor: Monitor,

    /// Receive monitor
    #[serde(rename = "RecvMonitor")]
    pub recv_monitor: Monitor,

    /// Channels
    #[serde(rename = "Channels")]
    pub channels: Vec<Channel>,
}

/// Monitor
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Monitor {
    /// Is this monitor active?
    #[serde(rename = "Active")]
    pub active: bool,

    /// When the monitor started
    #[serde(rename = "Start")]
    pub start: tendermint::Timestamp,

    /// Duration of this monitor
    #[serde(
        rename = "Duration",
        serialize_with = "serialize_duration",
        deserialize_with = "deserialize_duration"
    )]
    pub duration: Duration,

    /// Idle duration for this monitor
    #[serde(
        rename = "Idle",
        serialize_with = "serialize_duration",
        deserialize_with = "deserialize_duration"
    )]
    pub idle: Duration,

    /// Bytes
    #[serde(
        rename = "Bytes",
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    bytes: u64,

    /// Samples
    #[serde(
        rename = "Samples",
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    samples: u64,

    /// Instant rate
    #[serde(
        rename = "InstRate",
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    inst_rate: u64,

    /// Current rate
    #[serde(
        rename = "CurRate",
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    cur_rate: u64,

    /// Average rate
    #[serde(
        rename = "AvgRate",
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    avg_rate: u64,

    /// Peak rate
    #[serde(
        rename = "PeakRate",
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    peak_rate: u64,

    /// Bytes remaining
    #[serde(
        rename = "BytesRem",
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    bytes_rem: u64,

    /// Time remaining
    #[serde(
        rename = "TimeRem",
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    time_rem: u64,

    /// Progress
    #[serde(rename = "Progress")]
    progress: u64,
}

/// Serialize from a `Duration` to a count of nanoseconds
fn serialize_duration<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    format!("{}", duration.as_nanos()).serialize(serializer)
}

/// Deserialize a `Duration` from a string containing a nanosecond count
fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    // TODO(tarcieri): handle 64-bit overflow?
    let nanos = String::deserialize(deserializer)?
        .parse::<u64>()
        .map_err(|e| D::Error::custom(format!("{}", e)))?;

    Ok(Duration::from_nanos(nanos))
}
