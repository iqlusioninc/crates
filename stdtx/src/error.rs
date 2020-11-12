//! Error types

pub use eyre::{Report, Result};

use thiserror::Error;

/// Kinds of errors
#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum Error {
    /// Malformed account or validator address
    #[error("address error")]
    Address,

    /// Invalid decimal value
    #[error("invalid decimal value")]
    Decimal,

    /// Input/output errors
    #[error("I/O error")]
    Io,

    /// Invalid field name
    #[error("unknown field name")]
    FieldName,

    /// Parse error
    #[error("parse error")]
    Parse,

    /// Signature error
    #[error("signature error")]
    Signature,

    /// Invalid type
    #[error("type error")]
    Type,
}
