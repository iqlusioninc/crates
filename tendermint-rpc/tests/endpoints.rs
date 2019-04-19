//! Tests for consuming endpoint JSON from fixtures

use std::{fs, path::PathBuf};
use tendermint_rpc::{
    endpoints::{NetInfoResponse, StatusResponse},
    jsonrpc::Response,
};

fn read_fixture(name: &str) -> String {
    fs::read_to_string(PathBuf::from("./tests/fixtures/").join(name.to_owned() + ".json")).unwrap()
}

#[test]
fn status_endpoint() {
    let status_json = read_fixture("status");
    let status_response = StatusResponse::from_json(&status_json).unwrap();

    assert_eq!(status_response.node_info.network.as_str(), "cosmoshub-1");
    assert_eq!(
        status_response.sync_info.latest_block_height.value(),
        410744
    );
    assert_eq!(status_response.validator_info.voting_power.value(), 0);
}

#[test]
fn net_info_endpoint() {
    let net_info_json = read_fixture("net_info");
    let net_info_response = NetInfoResponse::from_json(&net_info_json).unwrap();

    println!("net_info_response: {:?}", net_info_response);

    assert_eq!(net_info_response.n_peers, 2);
    assert_eq!(
        net_info_response.peers[0].node_info.network.as_str(),
        "cosmoshub-1"
    );
}
