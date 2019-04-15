//! Tests for consuming endpoint JSON from fixtures

use std::{fs, path::PathBuf};
use tendermint_rpc::{endpoints::StatusResponse, jsonrpc};

fn read_fixture(name: &str) -> String {
    fs::read_to_string(PathBuf::from("./tests/fixtures/").join(name.to_owned() + ".json")).unwrap()
}

#[test]
fn status_endpoint() {
    let status_json = read_fixture("status");
    let status_response: jsonrpc::ResponseWrapper<StatusResponse> =
        tendermint_rpc::jsonrpc::parse_response(&status_json).unwrap();

    let StatusResponse {
        node_info,
        sync_info,
        validator_info,
    } = status_response.result;

    assert_eq!(node_info.network.as_str(), "cosmoshub-1");
    assert_eq!(sync_info.latest_block_height.value(), 410744);
    assert_eq!(validator_info.voting_power.value(), 0);
}
