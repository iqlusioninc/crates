/// Error type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Error;

/// Result type
pub type Result<T> = core::result::Result<T, Error>;

impl From<core::array::TryFromSliceError> for Error {
    fn from(_: core::array::TryFromSliceError) -> Error {
        Error
    }
}

impl From<hmac::crypto_mac::InvalidKeyLength> for Error {
    fn from(_: hmac::crypto_mac::InvalidKeyLength) -> Error {
        Error
    }
}

#[cfg(feature = "secp256k1")]
impl From<k256::elliptic_curve::Error> for Error {
    fn from(_: k256::elliptic_curve::Error) -> Error {
        Error
    }
}

#[cfg(feature = "secp256k1")]
impl From<k256::ecdsa::Error> for Error {
    fn from(_: k256::ecdsa::Error) -> Error {
        Error
    }
}
