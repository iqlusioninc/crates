//! Tendermint RPC endpoints

mod net_info;
mod status;

pub use self::{
    net_info::{NetInfo, NetInfoResponse},
    status::{Status, StatusResponse},
};
