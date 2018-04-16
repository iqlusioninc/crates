//! Cargo-like command-line interfaces with selectively-colored status output

#![crate_name = "iq_cli"]
#![crate_type = "rlib"]
#![deny(warnings, missing_docs, unsafe_code, unused_import_braces, unused_qualifications)]
#![doc(html_root_url = "https://docs.rs/iq-cli/0.0.0")]

extern crate libc;
extern crate term;

mod shell;

pub use shell::{ColorConfig, Shell, ShellConfig};
pub use term::color;
pub use term::color::Color;

use std::io;
use libc::isatty;

/// Create a new shell
pub fn create_shell(color_config: ColorConfig) -> Shell {
    let config = ShellConfig {
        color_config,
        tty: is_tty(),
    };
    Shell::create(|| Box::new(io::stdout()), config)
}

/// Is STDOUT a tty?
fn is_tty() -> bool {
    #[allow(unsafe_code)]
    unsafe {
        isatty(0) == 1
    }
}
