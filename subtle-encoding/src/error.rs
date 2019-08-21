//! Error type

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::FromUtf8Error;
use core::fmt;
#[cfg(feature = "std")]
use std::{io, string::FromUtf8Error};

/// Error type
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Error {
    /// Checksum fdoes not match expected value
    ChecksumInvalid,

    /// Data is not encoded correctly
    EncodingInvalid,

    /// Error performing I/O operation
    IoError,

    /// Input or output buffer is an incorrect length
    LengthInvalid,

    /// Padding missing/invalid
    PaddingInvalid,

    /// Trailing whitespace detected
    // TODO: handle trailing whitespace?
    TrailingWhitespace,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            Error::ChecksumInvalid => "checksum mismatch",
            Error::EncodingInvalid => "bad encoding",
            Error::IoError => "I/O error",
            Error::LengthInvalid => "invalid length",
            Error::PaddingInvalid => "padding invalid",
            Error::TrailingWhitespace => "trailing whitespace",
        };

        write!(f, "{}", description)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Assert that the provided condition is true, or else return the given error
macro_rules! ensure {
    ($condition:expr, $err:path) => {
        if !($condition) {
            Err($err)?;
        }
    };
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Error {
        // TODO: preserve cause or error message?
        Error::IoError
    }
}

#[cfg(feature = "alloc")]
impl From<FromUtf8Error> for Error {
    fn from(_err: FromUtf8Error) -> Error {
        // TODO: preserve cause or error message?
        Error::EncodingInvalid
    }
}
