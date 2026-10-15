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

    /// Hyper errors (e.g. while reading a response body)
    Hyper(hyper::Error),

    /// HTTP client errors (e.g. connecting, TLS handshake, sending a request)
    Client(hyper_util::client::legacy::Error),

    /// I/O errors (e.g. loading the operating system's root certificates)
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "json")]
            Error::Json(err) => err.fmt(f),
            Error::HeaderValue => f.write_str("invalid header value"),
            Error::Http(err) => err.fmt(f),
            Error::Hyper(err) => err.fmt(f),
            Error::Client(err) => err.fmt(f),
            Error::Io(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(feature = "json")]
            Error::Json(err) => Some(err),
            Error::HeaderValue => None,
            Error::Http(err) => Some(err),
            Error::Hyper(err) => Some(err),
            Error::Client(err) => Some(err),
            Error::Io(err) => Some(err),
        }
    }
}

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

impl From<hyper_util::client::legacy::Error> for Error {
    fn from(err: hyper_util::client::legacy::Error) -> Error {
        Error::Client(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}
