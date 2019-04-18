#[cfg(feature = "integration")]
use tendermint_rpc::{endpoints::Status, jsonrpc::Request, Address};

/// Hit a locally running gaiad full node
#[cfg(feature = "integration")]
#[test]
fn integration_test() {
    let node_addr = "tcp://127.0.0.1:26657".parse::<Address>().unwrap();
    let status = Status.perform(&node_addr).unwrap();

    // For lack of better things to test
    assert_eq!(
        status.validator_info.voting_power.value(),
        0,
        "don't integration test against a validator"
    );
}
