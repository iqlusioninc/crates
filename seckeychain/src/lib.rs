//! macOS keychain access

#![crate_name = "seckeychain"]
#![crate_type = "rlib"]
#![allow(unknown_lints, suspicious_arithmetic_impl)]
#![deny(
    warnings,
    missing_docs,
    unused_import_braces,
    unused_qualifications
)]
#![doc(html_root_url = "https://docs.rs/seckeychain/0.0.0")]

extern crate core_foundation;

pub mod access;
pub mod error;
pub mod key;
