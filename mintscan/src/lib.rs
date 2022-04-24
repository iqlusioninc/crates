#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_qualifications
)]

pub mod coin;
pub mod v1;

mod deserializers;

pub use self::coin::Coin;
pub use tendermint;

use iqhttp::{HttpsClient, Result};

/// Bech32-encoded address.
// TODO(tarcieri): use `cosmos_sdk`'s `AccountId` or upstream it to `tendermint`?
pub type Address = String;

/// Validator rates.
// TODO(tarcieri): real decimal type for this? (e.g. "0.100000000000000000")
pub type Rate = String;

/// Mintscan API client.
pub struct Mintscan {
    /// HTTP client.
    client: HttpsClient,
}

impl Mintscan {
    /// Create a new Mintscan client for the given API hostname
    /// (e.g. `api.cosmostation.io`)
    pub fn new(hostname: impl Into<String>) -> Self {
        let mut client = HttpsClient::new(hostname);
        client
            .add_header(iqhttp::header::REFERER, "https://mintscan.io/")
            .expect("couldn't add referer header");
        Self { client }
    }

    /// Get `/v1/status` endpoint.
    pub async fn status(&self) -> Result<v1::Status> {
        self.client
            .get_json("/v1/status", &Default::default())
            .await
    }

    /// Get `/v1/staking/validator` endpoint.
    ///
    /// Accepts a Bech32-encoded account address for the validator.
    pub async fn validator(&self, addr: impl Into<Address>) -> Result<v1::staking::Validator> {
        // TODO(tarcieri): path construction with proper escaping
        let path = format!("/v1/staking/validator/{}", &addr.into());
        self.client.get_json(&path, &Default::default()).await
    }

    /// Get `/v1/staking/validator/uptime` endpoint.
    ///
    /// Accepts a Bech32-encoded account address for the validator.
    pub async fn validator_uptime(
        &self,
        addr: impl Into<Address>,
    ) -> Result<v1::staking::validator::Uptime> {
        // TODO(tarcieri): path construction with proper escaping
        let path = format!("/v1/staking/validator/uptime/{}", &addr.into());
        self.client.get_json(&path, &Default::default()).await
    }
}

impl From<HttpsClient> for Mintscan {
    fn from(client: HttpsClient) -> Mintscan {
        Mintscan { client }
    }
}
