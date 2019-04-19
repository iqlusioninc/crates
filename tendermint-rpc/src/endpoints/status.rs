//! RPC wrapper for `/status` endpoint

use crate::{jsonrpc, node_info::NodeInfo};
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

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

/// Voting power
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct VotingPower(u64);

impl VotingPower {
    /// Get the current voting power
    pub fn value(self) -> u64 {
        self.0
    }
}

impl From<VotingPower> for u64 {
    fn from(power: VotingPower) -> u64 {
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
