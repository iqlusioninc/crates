//! Error types

use core_foundation::error::CFErrorRef;

/// Error type
pub struct Error {}

impl From<CFErrorRef> for Error {
    fn from(_cferror: CFErrorRef) -> Error {
        Error {}
    }
}
