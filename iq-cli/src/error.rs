//! Error types used by this crate

use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};
use std::io;
use term;

/// Error type
#[derive(Debug)]
pub struct Error {
    /// Contextual information about the error
    inner: Context<ErrorKind>,

    /// Description of the error providing additional information
    description: String,
}

impl Error {
    /// Create a new error with the given description
    pub fn new<S: ToString>(kind: ErrorKind, description: &S) -> Self {
        Self {
            inner: Context::new(kind),
            description: description.to_string(),
        }
    }

    /// Obtain the inner `ErrorKind` for this error
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", &self.inner, &self.description)
    }
}

/// Kinds of errors
#[derive(Copy, Clone, Debug, Eq, Fail, PartialEq)]
pub enum ErrorKind {
    /// I/O operation failed
    #[fail(display = "I/O operation failed")]
    Io,

    /// Couldn't parse the given value
    #[fail(display = "parse error")]
    Parse,
}

impl ErrorKind {
    /// Add a description to this ErrorKind, creating an Error
    pub fn to_error<S: ToString>(self, description: &S) -> Error {
        Error::new(self, description)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        ErrorKind::Io.to_error(&other)
    }
}

impl From<term::Error> for Error {
    fn from(other: term::Error) -> Self {
        ErrorKind::Io.to_error(&other)
    }
}
