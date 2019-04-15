//! RPC wrapper for `/status` endpoint

use crate::address::Address;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt::{self, Display},
    ops::Deref,
};

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

/// Node information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NodeInfo {
    /// Protocol version information
    pub protocol_version: ProtocolVersionInfo,

    /// Node ID
    pub id: NodeId,

    /// Listen address
    pub listen_addr: Address,

    /// Tendermint network / chain ID,
    pub network: tendermint::chain::Id,

    /// Tendermint version
    pub version: semver::Version,

    /// Channels
    pub channels: Channels,

    /// Moniker
    // TODO(tarcieri): get this into the `tendermint` crate?
    pub moniker: Moniker,

    /// Other status information
    pub other: OtherInfo,
}

/// Sync information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncInfo {
    /// Latest block hash
    pub latest_block_hash: Hash,

    /// Latest app hash
    pub latest_app_hash: Hash,

    /// Latest block height
    pub latest_block_height: BlockHeight,

    /// Latest block time
    pub latest_block_time: tendermint::Timestamp,

    /// Are we catching up?
    pub catching_up: bool,
}

/// Validator information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidatorInfo {
    /// Validator account address
    pub address: AccountAddr,

    /// Validator public key
    pub pub_key: PublicKey,

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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProtocolVersion(String);

/// Node ID
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeId(String);

impl Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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

/// Moniker
// TODO(tarcieri): use an upstream type from the `tendermint` crate
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Moniker(String);

impl Display for Moniker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Hashes
// TODO(tarcieri): use upstream `tendermint::hash::Hash` type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Hash(String);

impl Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Block height wrapper type
// TODO(tarcieri): serde deserialize impl for `tendermint::block::Height`
#[derive(Clone, Debug)]
pub struct BlockHeight(tendermint::block::Height);

impl Deref for BlockHeight {
    type Target = tendermint::block::Height;

    fn deref(&self) -> &tendermint::block::Height {
        &self.0
    }
}

impl<'de> Deserialize<'de> for BlockHeight {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let height = String::deserialize(deserializer)?
            .parse::<u64>()
            .map_err(|e| D::Error::custom(format!("{}", e)))?;

        Ok(BlockHeight(tendermint::block::Height::from(height)))
    }
}

impl Serialize for BlockHeight {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value().to_string().serialize(serializer)
    }
}

/// Other information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OtherInfo {
    /// TX index status
    pub tx_index: TxIndexStatus,

    /// RPC address
    pub rpc_address: Address,
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

/// Hashes
// TODO(tarcieri): use upstream `tendermint` crate type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AccountAddr(String);

impl Display for AccountAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Public keys allowed in Tendermint protocols
/// TODO(tarcieri): use upstream `tendermint::public_keys::PublicKey` type
#[derive(Clone, Debug, Deserialize, Hash, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum PublicKey {
    /// Ed25519 keys
    #[serde(rename = "tendermint/PubKeyEd25519")]
    Ed25519(String),

    /// Secp256k1 keys
    #[serde(rename = "tendermint/PubKeySecp256k1")]
    Secp256k1(String),
}

/// Voting power
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct VotingPower(usize);

impl From<VotingPower> for usize {
    fn from(power: VotingPower) -> usize {
        power.0
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
