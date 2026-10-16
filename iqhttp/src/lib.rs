#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
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
pub use hyper::{self, HeaderMap, Request, Response, Uri, header};

/// HTTP body types used in requests and responses.
pub mod body {
    pub use http_body_util::{BodyExt, Full};
    pub use hyper::body::{Buf, Bytes, Incoming};
}

/// User-Agent to send in HTTP request
pub const USER_AGENT: &str = concat!("iqhttp/", env!("CARGO_PKG_VERSION"));

/// HTTP request path type
// TODO(tarcieri): real path type
pub type Path = str;
