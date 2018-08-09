//! Command line application microframework which supports command-line
//! option parsing, basic terminal management, and Cargo-like status output.
//!
//! # Usage
//!
//! ```
//! # #[macro_use] extern crate iq_cli;
//! # fn main() {
//! use iq_cli::{self, ColorConfig};
//!
//! // Initialize the terminal (uses autodetection for if colors are supported)
//! iq_cli::init(ColorConfig::default());
//!
//! // Print a Cargo-like justified status to STDOUT
//! status_ok!("Loaded", "app loaded successfully");
//!
//! // Print an error message
//! status_err!("something bad happened");
//!
//! // Print an indented attribute to STDOUT
//! status_attr_ok!("good", "yep");
//!
//! // Print an error attribute to STDERR
//! status_attr_err!("error", "yep");
//! # }
//! ```

#![crate_name = "iq_cli"]
#![crate_type = "rlib"]
#![deny(
    warnings,
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#![doc(html_root_url = "https://docs.rs/iq-cli/0.1.0")]

extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate lazy_static;
extern crate term;

pub use term::color;
pub use term::color::Color;

mod error;
mod macros;
mod shell;

pub use error::Error;
pub use shell::{init, status, ColorConfig, Stream};
