//! Deserialization helpers

use serde::{de, Deserialize};
use tendermint::block;

/// Parse block height from an integer (ordinarily a string in Tendermint JSON)
pub(crate) fn block_height<'de, D>(deserializer: D) -> Result<block::Height, D::Error>
where
    D: de::Deserializer<'de>,
{
    let height = u64::deserialize(deserializer)?;
    height.try_into().map_err(de::Error::custom)
}
