//! Error types.

use core::fmt;

/// Result type.
pub type Result<T> = core::result::Result<T, Error>;

/// Key derivation errors.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Invalid BIP39 mnemonic.
    #[cfg(feature = "bip39")]
    Bip39(bip39::Error),

    /// A valid mnemonic did not contain exactly 24 words.
    #[cfg(feature = "bip39")]
    WordCount,

    /// BIP32 derivation failed.
    Bip32(bip32::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "bip39")]
            Self::Bip39(err) => write!(f, "invalid BIP39 mnemonic: {err}"),
            #[cfg(feature = "bip39")]
            Self::WordCount => f.write_str("expected a 24-word BIP39 mnemonic"),
            Self::Bip32(err) => write!(f, "BIP32 derivation failed: {err}"),
        }
    }
}

impl core::error::Error for Error {}

#[cfg(feature = "bip39")]
impl From<bip39::Error> for Error {
    fn from(err: bip39::Error) -> Self {
        Self::Bip39(err)
    }
}

impl From<bip32::Error> for Error {
    fn from(err: bip32::Error) -> Self {
        Self::Bip32(err)
    }
}
