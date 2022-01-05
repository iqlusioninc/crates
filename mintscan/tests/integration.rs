//! Integration tests.
//!
//! Performed live against <https://api.cosmostation.io>

use mintscan::Mintscan;

/// API host to test against.
const API_HOST: &str = "api.cosmostation.io";

/// Example validator (iqlusion).
const VALIDATOR_ADDR: &str = "cosmosvaloper1grgelyng2v6v3t8z87wu3sxgt9m5s03xfytvz7";

#[tokio::test]
async fn status() {
    let result = Mintscan::new(API_HOST).status().await;

    // TODO(tarcieri): better assertions
    assert!(result.is_ok());
}

#[tokio::test]
async fn validator() {
    let validator = Mintscan::new(API_HOST)
        .validator(VALIDATOR_ADDR)
        .await
        .unwrap();

    assert_eq!(
        validator.account_address,
        "cosmos1grgelyng2v6v3t8z87wu3sxgt9m5s03xvslewd"
    );
    assert_eq!(validator.operator_address, VALIDATOR_ADDR);
    assert_eq!(
        validator.consensus_pubkey,
        "cosmosvalconspub1dgvppnyr5c9pulsrmzr9e9rp7qpgm9jwp5yu8g3aumekgjugxacqg9u7gq"
    );
    assert_eq!(validator.moniker.as_ref(), "iqlusion");
    assert_eq!(validator.identity, "DCB176E79AE7D51F");
    assert_eq!(validator.website, "iqlusion.io");
}

#[tokio::test]
async fn validator_uptime() {
    let result = Mintscan::new(API_HOST)
        .validator_uptime(VALIDATOR_ADDR)
        .await;

    // TODO(tarcieri): better assertions
    assert!(result.is_ok());
}
