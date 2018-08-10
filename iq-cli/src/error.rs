//! Error types used by this crate

pub use failure::{Backtrace, Context, Fail};
use std::{
    fmt::{self, Display},
    io,
    string::ToString,
};
use term;

/// Error types used by this library, generic around `Kind`s
#[derive(Debug)]
pub struct Error<Kind>
where
    Kind: Fail + Clone + Display + Eq + PartialEq,
{
    /// Contextual information about the error
    inner: Context<Kind>,

    /// Description of the error providing additional information
    description: Option<String>,
}

impl<Kind> Error<Kind>
where
    Kind: Fail + Clone + Display + Eq + PartialEq,
{
    /// Create a new error type from its kind
    pub fn new<Description>(kind: Kind, description: Option<&Description>) -> Self
    where
        Description: ToString,
    {
        Self {
            inner: Context::new(kind),
            description: description.map(|desc| desc.to_string()),
        }
    }

    /// Obtain the error's `Kind`
    pub fn kind(&self) -> &Kind {
        self.inner.get_context()
    }
}

impl<Kind> Display for Error<Kind>
where
    Kind: Fail + Clone + Display + Eq + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.description {
            Some(ref desc) => write!(f, "{}: {}", self.kind(), desc),
            None => write!(f, "{}", self.kind()),
        }
    }
}

impl<Kind> Fail for Error<Kind>
where
    Kind: Fail + Clone + Display + Eq + PartialEq,
{
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

/// Convert this type to an error of the given `Kind`
pub trait ToError<Kind, Description>
where
    Kind: Fail + Clone + Display + Eq + PartialEq,
    Description: ToString,
{
    /// Return an error of the given `Kind`
    fn to_error(self, description: Option<&Description>) -> Error<Kind>;
}

impl<Kind, Description> ToError<Kind, Description> for Kind
where
    Kind: Fail + Clone + Display + Eq + PartialEq,
    Description: ToString,
{
    fn to_error(self, description: Option<&Description>) -> Error<Kind> {
        Error::new(self, description)
    }
}

/// General CLI errors (useful if you don't want to define your own error type)
pub type CliError = Error<CliErrorKind>;

/// General kinds of CLI errors
#[derive(Fail, Clone, Debug, Eq, PartialEq)]
pub enum CliErrorKind {
    /// I/O operation failed
    #[fail(display = "I/O operation failed")]
    Io,

    /// Couldn't parse the given value
    #[fail(display = "parse error")]
    Parse,
}

impl From<io::Error> for CliError {
    fn from(other: io::Error) -> Self {
        CliErrorKind::Io.to_error(Some(&other))
    }
}

impl From<term::Error> for CliError {
    fn from(other: term::Error) -> Self {
        CliErrorKind::Io.to_error(Some(&other))
    }
}
