//! `/v1/staking/validator/uptime`

use crate::deserializers;
use serde::Deserialize;
use tendermint::{account, block, Time};

/// `/v1/staking/validator/uptime` endpoint.
#[derive(Clone, Debug, Deserialize)]
pub struct Uptime {
    /// Latest height for a given validator.
    #[serde(deserialize_with = "deserializers::block_height")]
    pub latest_height: block::Height,

    /// Uptime information
    pub uptime: Vec<MissedBlock>,
}

/// Information about a missed block.
#[derive(Clone, Debug, Deserialize)]
pub struct MissedBlock {
    /// Block height.
    #[serde(deserialize_with = "deserializers::block_height")]
    pub height: block::Height,

    /// Timestamp of the block.
    pub timestamp: Time,
}

/// Summary of a validator's uptime.
///
/// This is returned from the `/v1/staking/validator` endpoint.
#[derive(Clone, Debug, Deserialize)]
pub struct Summary {
    /// Address.
    pub address: account::Id,

    /// Number of missed blocks.
    pub missed_blocks: u64,

    /// Over blocks.
    pub over_blocks: u64,
}
