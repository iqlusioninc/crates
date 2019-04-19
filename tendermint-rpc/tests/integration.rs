use tendermint_rpc::Address;
#[cfg(feature = "integration")]
use tendermint_rpc::{endpoints::Status, jsonrpc::Request};

/// Get the address of the local node
pub fn localhost_rpc_addr() -> Address {
    "tcp://127.0.0.1:26657".parse().unwrap()
}

// TODO(tarcieri): chunked encoding support
// `/net_info` endpoint integration test
// #[cfg(feature = "integration")]
// #[test]
// fn net_info_integration() {
//    let net_info = NetInfo.perform(&localhost_rpc_addr()).unwrap();
//    assert!(net_info.listening);
// }

/// `/status` endpoint integration test
#[cfg(feature = "integration")]
#[test]
fn status_integration() {
    let status = Status.perform(&localhost_rpc_addr()).unwrap();

    // For lack of better things to test
    assert_eq!(
        status.validator_info.voting_power.value(),
        0,
        "don't integration test against a validator"
    );
}
