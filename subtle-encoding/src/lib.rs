//! Encoders and decoders for common data encodings (WIP) which avoid
//! branching or performing table lookups based on their inputs
//! (a.k.a. constant time-ish).

#![crate_name = "subtle_encoding"]
#![crate_type = "rlib"]
#![deny(
    warnings,
    missing_docs,
    unused_import_braces,
    unused_qualifications
)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/subtle-encoding/0.0.0")]
