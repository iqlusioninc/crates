//! Node information included in `/net_info` and `/status` responses

use crate::{channel::Channels, jsonrpc};
use serde::{Deserialize, Serialize};

/// Node information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NodeInfo {
    /// Protocol version information
    pub protocol_version: ProtocolVersionInfo,

    /// Node ID
    pub id: tendermint::node::Id,

    /// Listen address
    pub listen_addr: tendermint::Address,

    /// Tendermint network / chain ID,
    pub network: tendermint::chain::Id,

    /// Tendermint version
    pub version: semver::Version,

    /// Channels
    pub channels: Channels,

    /// Moniker
    pub moniker: tendermint::Moniker,

    /// Other status information
    pub other: OtherInfo,
}

/// Protocol version information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProtocolVersionInfo {
    /// P2P protocol version
    #[serde(
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    pub p2p: u64,

    /// Block version
    #[serde(
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    pub block: u64,

    /// App version
    #[serde(
        serialize_with = "jsonrpc::serialize_u64_string",
        deserialize_with = "jsonrpc::deserialize_u64_string"
    )]
    pub app: u64,
}

/// Other information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OtherInfo {
    /// TX index status
    pub tx_index: TxIndexStatus,

    /// RPC address
    pub rpc_address: tendermint::Address,
}

/// Transaction index status
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum TxIndexStatus {
    /// Index is on
    #[serde(rename = "on")]
    On,

    /// Index is off
    #[serde(rename = "off")]
    Off,
}

impl From<TxIndexStatus> for bool {
    fn from(status: TxIndexStatus) -> bool {
        match status {
            TxIndexStatus::On => true,
            TxIndexStatus::Off => false,
        }
    }
}
