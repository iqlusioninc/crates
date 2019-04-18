//! RPC wrapper for `/status` endpoint

use crate::jsonrpc;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display};

/// Request the status of the node
#[derive(Default)]
pub struct Status;

impl jsonrpc::Request for Status {
    type Response = StatusResponse;

    fn path(&self) -> gaunt::Path {
        "/status".into()
    }
}

/// Status responses
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatusResponse {
    /// Node information
    pub node_info: NodeInfo,

    /// Sync information
    pub sync_info: SyncInfo,

    /// Validator information
    pub validator_info: ValidatorInfo,
}

impl jsonrpc::Response for StatusResponse {}

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

/// Sync information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncInfo {
    /// Latest block hash
    pub latest_block_hash: tendermint::Hash,

    /// Latest app hash
    pub latest_app_hash: tendermint::Hash,

    /// Latest block height
    pub latest_block_height: tendermint::block::Height,

    /// Latest block time
    pub latest_block_time: tendermint::Timestamp,

    /// Are we catching up?
    pub catching_up: bool,
}

/// Validator information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidatorInfo {
    /// Validator account address
    pub address: tendermint::account::Id,

    /// Validator public key
    pub pub_key: tendermint::PublicKey,

    /// Validator voting power
    pub voting_power: VotingPower,
}

/// Protocol version information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProtocolVersionInfo {
    /// P2P protocol version
    pub p2p: ProtocolVersion,

    /// Block version
    pub block: ProtocolVersion,

    /// App version
    pub app: ProtocolVersion,
}

/// Version value for versions in `ProtocolVersionInfo`
// TODO(tarcieri): separate types for different kinds of versions?
/// Voting power
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ProtocolVersion(usize);

impl From<ProtocolVersion> for usize {
    fn from(power: ProtocolVersion) -> usize {
        power.0
    }
}

impl<'de> Deserialize<'de> for ProtocolVersion {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(ProtocolVersion(
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e| D::Error::custom(format!("{}", e)))?,
        ))
    }
}

impl Serialize for ProtocolVersion {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.to_string().serialize(serializer)
    }
}

/// Channels
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Channels(String);

impl Display for Channels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
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

/// Voting power
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct VotingPower(usize);

impl VotingPower {
    /// Get the current voting power as an integer
    pub fn value(self) -> usize {
        self.0
    }
}

impl From<VotingPower> for usize {
    fn from(power: VotingPower) -> usize {
        power.value()
    }
}

impl<'de> Deserialize<'de> for VotingPower {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(VotingPower(
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e| D::Error::custom(format!("{}", e)))?,
        ))
    }
}

impl Serialize for VotingPower {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.to_string().serialize(serializer)
    }
}
