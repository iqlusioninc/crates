//! Coin types.
// TODO(tarcieri): source these from the `cosmos_sdk` crate?

use serde::Deserialize;
use std::fmt::{self, Display};

/// Coin defines a token with a denomination and an amount.
#[derive(Clone, Debug, Deserialize)]
pub struct Coin {
    /// Denomination
    pub denom: Denom,

    /// Amount.
    pub amount: Amount,
}

/// Denomination.
#[derive(Clone, Debug, Deserialize)]
pub struct Denom(String);

impl AsRef<str> for Denom {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for Denom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Amount.
#[derive(Clone, Debug, Deserialize)]
pub struct Amount(String);

impl Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
