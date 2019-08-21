//! Error types used by **gaunt.rs**

#![allow(unused_macros)]

use core::fmt;

#[cfg(feature = "alloc")]
use core::{num::ParseIntError, str::Utf8Error};

#[cfg(feature = "std")]
use std::{
    io,
    string::{FromUtf8Error, String, ToString},
};

/// Error type
#[derive(Debug)]
pub struct Error {
    /// Error context and kind
    kind: ErrorKind,

    /// Optional description
    #[cfg(feature = "alloc")]
    description: Option<String>,
}

impl Error {
    /// Create a new error object with an optional error message
    #[allow(unused_variables)]
    pub fn new(kind: ErrorKind, description: Option<&str>) -> Self {
        #[cfg_attr(not(feature = "alloc"), allow(unused_mut))]
        let mut err = Self::from(kind);

        #[cfg(feature = "alloc")]
        {
            err.description = description.map(|desc| desc.into());
        }

        err
    }

    /// Obtain the inner `ErrorKind` for this `Error`
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            kind,
            #[cfg(feature = "alloc")]
            description: None,
        }
    }
}

/// Kinds of errors
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// Invalid address
    AddrInvalid,

    /// I/O operation failed
    IoError,

    /// Parsing data failed
    ParseError,

    /// Request failed
    RequestError,

    /// Error reading response
    ResponseError,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            ErrorKind::AddrInvalid => "address invalid",
            ErrorKind::IoError => "I/O error",
            ErrorKind::ParseError => "parse error",
            ErrorKind::RequestError => "request error",
            ErrorKind::ResponseError => "error reading response",
        };

        write!(f, "{}", description)
    }
}

/// Create a new error (of a given enum variant) with a formatted message
macro_rules! err {
    ($variant:ident, $msg:expr) => {
        crate::error::Error::new(
            crate::error::ErrorKind::$variant,
            Some($msg)
        )
    };
    ($variant:ident, $fmt:expr, $($arg:tt)+) => {
        err!($variant, &format!($fmt, $($arg)+))
    };
}

/// Create and return an error with a formatted message
macro_rules! fail {
    ($kind:ident, $msg:expr) => {
        return Err(err!($kind, $msg).into());
    };
    ($kind:ident, $fmt:expr, $($arg:tt)+) => {
        fail!($kind, &format!($fmt, $($arg)+));
    };
}

/// Assert a condition is true, returning an error type with a formatted message if not
macro_rules! ensure {
    ($condition: expr, $variant:ident, $msg:expr) => {
        if !($condition) {
            return Err(err!($variant, $msg).into());
        }
    };
    ($condition: expr, $variant:ident, $fmt:expr, $($arg:tt)+) => {
        ensure!($variant, $fmt, $($arg)+);
    };
}

#[cfg(feature = "alloc")]
impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        err!(ParseError, &err.to_string())
    }
}

#[cfg(feature = "std")]
impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        err!(ParseError, &err.to_string())
    }
}

#[cfg(feature = "alloc")]
impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        err!(ParseError, &err.to_string())
    }
}

#[cfg(feature = "std")]
impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        err!(RequestError, &err.to_string())
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        err!(IoError, &err.to_string())
    }
}
