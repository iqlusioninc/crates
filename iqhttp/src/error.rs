//! Error types.

use std::fmt::{self, Display};

/// Result type
pub type Result<T> = std::result::Result<T, Error>;

/// Error type
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// JSON errors
    #[cfg(feature = "json")]
    Json(serde_json::Error),

    /// Invalid header value
    HeaderValue,

    /// HTTP errors
    Http(hyper::http::Error),

    /// Hyper errors
    // TODO(tarcieri): rename this variant, possibly extracting the error?
    Hyper(hyper::Error),

    /// Proxy errors
    #[cfg(feature = "proxy")]
    Proxy(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "json")]
            Error::Json(err) => err.fmt(f),
            Error::HeaderValue => f.write_str("invalid header value"),
            Error::Http(err) => err.fmt(f),
            Error::Hyper(err) => err.fmt(f),
            #[cfg(feature = "proxy")]
            Error::Proxy(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<hyper::header::InvalidHeaderValue> for Error {
    fn from(_: hyper::header::InvalidHeaderValue) -> Error {
        Error::HeaderValue
    }
}

impl From<hyper::http::Error> for Error {
    fn from(err: hyper::http::Error) -> Error {
        Error::Http(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}
