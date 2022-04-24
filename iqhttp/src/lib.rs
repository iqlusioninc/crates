//! iqlusion's HTTP toolkit.
//!
//! Provides a high-level wrapper around [`hyper`], with built-in SSL/TLS support
//! based on [`rustls`].
//!
//! [`hyper`]: https://docs.rs/hyper
//! [`rustls`]: https://docs.rs/rustls

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/iqhttp/0.1.0")]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
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
