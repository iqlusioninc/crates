//! Rust API wrapper for the Tendermint JSONRPC/HTTP, with support for querying
//! state from a running full node.

#![deny(warnings, missing_docs, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/tendermint-rpc/0.0.0")]

#[macro_use]
extern crate serde_derive;

pub mod endpoints;
pub mod jsonrpc;

pub use tendermint::Address;
