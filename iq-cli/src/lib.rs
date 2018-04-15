//! Cargo-like command-line interfaces with selectively-colored status output

#![crate_name = "iq_cli"]
#![crate_type = "rlib"]
#![deny(warnings, missing_docs, unsafe_code, unused_import_braces, unused_qualifications)]
#![doc(html_root_url = "https://docs.rs/iq-cli/0.0.0")]

extern crate libc;
extern crate term;

mod shell;

pub use shell::{create, ColorConfig, Shell, ShellConfig};
pub use term::color;
pub use term::color::Color;
