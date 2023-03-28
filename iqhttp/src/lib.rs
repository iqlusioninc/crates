#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_qualifications
)]

#[cfg(feature = "serde")]
pub mod serializers;

mod error;
mod https_client;
mod query;

pub use self::{
    error::{Error, Result},
    https_client::HttpsClient,
    query::Query,
};
pub use hyper::{self, header, HeaderMap, Uri};

/// User-Agent to send in HTTP request
pub const USER_AGENT: &str = concat!("iqhttp/", env!("CARGO_PKG_VERSION"));

/// HTTP request path type
// TODO(tarcieri): real path type
pub type Path = str;
