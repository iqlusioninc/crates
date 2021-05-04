//! `/v1/staking/validator` endpoints.

pub mod uptime;

pub use self::uptime::Uptime;

use crate::{coin::Amount, deserializers, Address, Rate};
use serde::Deserialize;
use tendermint::{block, Moniker, Time};

/// `/v1/staking/validator` endpoint.
#[derive(Clone, Debug, Deserialize)]
pub struct Validator {
    /// Account address.
    pub account_address: Address,

    /// Operator address.
    pub operator_address: Address,

    /// Consensus public key (Bech32)
    // TODO(tarcieri): parse this to a `tendermint::TendermintKey`?
    pub consensus_pubkey: String,

    /// Validator rank.
    pub rank: u64,

    /// Bonded height.
    #[serde(deserialize_with = "deserializers::block_height")]
    pub bonded_height: block::Height,

    /// Bonded time.
    pub bonded_time: Time,

    /// Is this validator jailed?
    pub jailed: bool,

    /// Validator status.
    // TODO(tarcieri): should this map to an enum? (e.g. 3)
    pub status: u64,

    /// Tokens.
    pub tokens: Amount,

    /// Delegator shares.
    // TODO(tarcieri): parse this? (e.g. "4118602822199.711514641106188835")
    pub delegator_shares: String,

    /// Validator moniker.
    pub moniker: Moniker,

    /// Validator identity.
    // TODO(tarcieri): parse this? (e.g. "DCB176E79AE7D51F")
    pub identity: String,

    /// Validator web site.
    // TODO(tarcieri): parse this? (e.g. "iqlusion.io")
    pub website: String,

    /// Validator details.
    pub details: String,

    /// Unbonding height.
    // Note: For some reason unlike the other block heights, this one is quoted.
    pub unbonding_height: block::Height,

    /// Unbonding time.
    pub unbonding_time: Time,

    /// Validator commission rate.
    pub rate: Rate,

    /// Maximum validator commission rate.
    pub max_rate: Rate,

    /// Maximum rate at which validator commission can be changed.
    pub max_change_rate: Rate,

    /// Time when the validator was updated.
    pub update_time: Time,

    /// Uptime summary.
    pub uptime: uptime::Summary,

    /// Minimum self-delegation.
    pub min_self_delegation: Amount,

    /// Keybase URL.
    // TODO(tarcieri): parse this?
    pub keybase_url: String,
}
