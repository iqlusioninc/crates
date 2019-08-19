//! Encoders and decoders for common data encodings (base64, hex) which avoid
//! branching or performing table lookups based on their inputs
//! (a.k.a. "constant time-ish").
//!
//! ## Supported Encodings
//!
//! - [hex]
//! - [base64]
//! - [bech32] (WARNING: preview! Not constant time yet)
//!
//! [hex]: https://docs.rs/subtle-encoding/latest/subtle_encoding/hex/index.html
//! [base64]: https://docs.rs/subtle-encoding/latest/subtle_encoding/base64/index.html
//! [bech32]: https://docs.rs/subtle-encoding/0.2.3/subtle_encoding/bech32/index.html

#![no_std]
#![deny(warnings, missing_docs, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/subtle-encoding/0.3.7")]

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(feature = "zeroize")]
extern crate zeroize;

#[macro_use]
mod error;

#[cfg(feature = "base64")]
pub mod base64;
#[cfg(feature = "bech32-preview")]
pub mod bech32;
pub mod encoding;
#[cfg(feature = "hex")]
pub mod hex;
pub mod identity;

#[cfg(feature = "base64")]
pub use crate::base64::Base64;
pub use crate::encoding::Encoding;
pub use crate::error::Error;
#[cfg(feature = "hex")]
pub use crate::hex::Hex;
pub use crate::identity::{Identity, IDENTITY};
