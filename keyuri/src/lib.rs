//! Keys-as-URIs (with Bech32 binary data/checksums)

#![crate_name = "keyuri"]
#![crate_type = "rlib"]
#![allow(unknown_lints, suspicious_arithmetic_impl)]
#![deny(warnings, missing_docs, unsafe_code, unused_import_braces, unused_qualifications)]
#![doc(html_root_url = "https://docs.rs/keyuri/0.0.0")]

#[macro_use]
extern crate failure;

pub mod bech32k;
