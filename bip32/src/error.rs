//! Error type.

use core::fmt::{self, Display};

/// Result type.
pub type Result<T> = core::result::Result<T, Error>;

/// Error type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Base58 errors.
    Base58,

    /// BIP39-related errors.
    Bip39,

    /// BIP39 can't find the word in the selected wordlist.
    Bip39InvalidWord,

    /// BIP39-related errors.
    Bip39InvalidPhraseSize,

    ///BIP39 entropy must have 16 or 32 bytes
    Bip39InvalidEntropySize,
    
    ///BIP39 invalid checksum
    Bip39InvalidChecksum,


    /// Child number-related errors.
    ChildNumber,

    /// Cryptographic errors.
    Crypto,

    /// Decoding errors (not related to Base58).
    Decode,

    /// Maximum derivation depth exceeded.
    Depth,

    /// Seed length invalid.
    SeedLength,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Base58 => f.write_str("base58 error"),
            Error::Bip39 => f.write_str("bip39 error"),
            Error::Bip39InvalidPhraseSize => f.write_str("bip39 invalid phrase size error"),
            Error::Bip39InvalidEntropySize => f.write_str("bip39 entropy must have 16 or 32 bytes"),
            Error::Bip39InvalidChecksum => f.write_str("bip39 invalid checksum"),
            Error::Bip39InvalidWord => f.write_str("bip39 can't find the word in the selected wordlist"),
            Error::ChildNumber => f.write_str("invalid child number"),
            Error::Crypto => f.write_str("cryptographic error"),
            Error::Decode => f.write_str("decoding error"),
            Error::Depth => f.write_str("maximum derivation depth exceeded"),
            Error::SeedLength => f.write_str("seed length invalid"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<bs58::decode::Error> for Error {
    fn from(_: bs58::decode::Error) -> Error {
        Error::Base58
    }
}

impl From<bs58::encode::Error> for Error {
    fn from(_: bs58::encode::Error) -> Error {
        Error::Base58
    }
}

impl From<core::array::TryFromSliceError> for Error {
    fn from(_: core::array::TryFromSliceError) -> Error {
        Error::Decode
    }
}

impl From<hmac::digest::InvalidLength> for Error {
    fn from(_: hmac::digest::InvalidLength) -> Error {
        Error::Crypto
    }
}

#[cfg(feature = "secp256k1")]
impl From<k256::elliptic_curve::Error> for Error {
    fn from(_: k256::elliptic_curve::Error) -> Error {
        Error::Crypto
    }
}

#[cfg(feature = "secp256k1")]
impl From<k256::ecdsa::Error> for Error {
    fn from(_: k256::ecdsa::Error) -> Error {
        Error::Crypto
    }
}

#[cfg(feature = "secp256k1-ffi")]
impl From<secp256k1_ffi::Error> for Error {
    fn from(_: secp256k1_ffi::Error) -> Error {
        Error::Crypto
    }
}

#[cfg(feature = "secp256k1-ffi")]
impl From<secp256k1_ffi::scalar::OutOfRangeError> for Error {
    fn from(_: secp256k1_ffi::scalar::OutOfRangeError) -> Error {
        Error::Crypto
    }
}
