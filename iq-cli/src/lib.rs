//! Cargo-like command-line interfaces with selectively-colored status output

#![crate_name = "iq_cli"]
#![crate_type = "rlib"]
#![deny(warnings, missing_docs, unsafe_code, unused_import_braces, unused_qualifications)]
#![doc(html_root_url = "https://docs.rs/iq-cli/0.1.0")]

#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate term;

mod macros;
mod shell;

pub use shell::{ColorConfig, Shell, ShellConfig};
pub use term::color;
pub use term::color::Color;

use std::fmt;
use std::sync::Mutex;

lazy_static! {
    static ref SHELL: Mutex<Shell> = Mutex::new(Shell::default());
}

/// Say a status message with the given color
pub fn status<T, U>(color: Color, status: T, message: U, justified: bool)
where
    T: fmt::Display,
    U: fmt::Display,
{
    SHELL
        .lock()
        .unwrap()
        .status(color, status, message, justified)
        .unwrap();
}
