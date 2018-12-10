//! Error types used by **gaunt.rs**

#![allow(unused_macros)]

#[cfg(feature = "alloc")]
use crate::prelude::*;

#[cfg(feature = "alloc")]
use core::{num::ParseIntError, str::Utf8Error};
use failure::Context;

#[cfg(feature = "std")]
use std::{
    error::Error as StdError,
    fmt::{self, Display},
    io,
    string::{FromUtf8Error, String, ToString},
};

/// Error type
#[derive(Debug)]
pub struct Error {
    /// Error context and kind
    inner: Context<ErrorKind>,

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
        *self.inner.get_context()
    }
}

#[cfg(feature = "alloc")]
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

#[cfg(feature = "alloc")]
impl StdError for Error {
    fn description(&self) -> &str {
        if let Some(ref desc) = self.description {
            desc
        } else {
            "(none)"
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
            #[cfg(feature = "alloc")]
            description: None,
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Self {
        Self {
            inner,
            #[cfg(feature = "alloc")]
            description: None,
        }
    }
}

/// Kinds of errors
#[derive(Copy, Clone, Debug, Fail, Eq, PartialEq)]
pub enum ErrorKind {
    /// Invalid address
    #[fail(display = "address invalid")]
    AddrInvalid,

    /// I/O operation failed
    #[fail(display = "I/O error")]
    IoError,

    /// Parsing data failed
    #[fail(display = "parse error")]
    ParseError,

    /// Request failed
    #[fail(display = "request error")]
    RequestError,

    /// Error reading response
    #[fail(display = "error reading response")]
    ResponseError,
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
