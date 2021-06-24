//! Error type

use core::fmt::{self, Display};

/// Result type
pub type Result<T> = core::result::Result<T, Error>;

/// Error type
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Algorithm is invalid.
    AlgorithmInvalid,

    /// Key name is invalid.
    KeyNameInvalid,

    /// I/O errors
    #[cfg(feature = "std")]
    Io(std::io::Error),

    /// Expected a directory, found something else
    #[cfg(feature = "std")]
    NotADirectory,

    /// Permissions error, not required mode
    #[cfg(feature = "std")]
    Permissions,

    /// PKCS#8 errors
    Pkcs8(pkcs8::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlgorithmInvalid => f.write_str("invalid algorithm"),
            Self::KeyNameInvalid => f.write_str("invalid key name"),
            #[cfg(feature = "std")]
            Self::Io(err) => write!(f, "{}", err),
            #[cfg(feature = "std")]
            Self::NotADirectory => f.write_str("not a directory"),
            #[cfg(feature = "std")]
            Self::Permissions => f.write_str("invalid file permissions"),
            Self::Pkcs8(err) => write!(f, "{}", err),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<pkcs8::Error> for Error {
    fn from(err: pkcs8::Error) -> Error {
        Error::Pkcs8(err)
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}
