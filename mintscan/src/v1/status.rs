//! `/v1/status` endpoint.

use crate::{deserializers, Coin};
use serde::Deserialize;
use tendermint::{block, chain, Time};

/// Type used to represent inflation.
// TODO(tarcieri): parse this?
pub type Inflation = String;

/// `/v1/status` endpoint.
#[derive(Clone, Debug, Deserialize)]
pub struct Status {
    /// Chain ID
    pub chain_id: chain::Id,

    /// Block height.
    #[serde(deserialize_with = "deserializers::block_height")]
    pub block_height: block::Height,

    /// Block time in seconds.
    pub block_time: f64,

    /// Total number of transactions.
    pub total_txs_num: u64,

    /// Total number of validators.
    pub total_validator_num: u64,

    /// Number of unjailed validators.
    pub unjailed_validator_num: u64,

    /// Number of jailed validators.
    pub jailed_validator_num: u64,

    /// Total supply of tokens.
    pub total_supply_tokens: Supply,

    /// Total circulating tokens.
    pub total_circulating_tokens: Supply,

    /// Number of bonded tokens.
    pub bonded_tokens: u64,

    /// Number of tokens not bonded.
    pub not_bonded_tokens: u64,

    /// Inflation.
    pub inflation: Inflation,

    /// Community pool.
    pub community_pool: Vec<Coin>,

    /// Timestamp.
    pub timestamp: Time,
}

/// Token supply.
#[derive(Clone, Debug, Deserialize)]
pub struct Supply {
    /// Supply of coins.
    pub supply: Vec<Coin>,
}
