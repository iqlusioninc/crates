//! Shell support bindings

use iq_cli::{self, Color, ColorConfig, Shell};
pub use iq_cli::color;
use std::fmt;
use std::process;
use std::sync::Mutex;

lazy_static! {
    static ref SHELL: Mutex<Shell> = Mutex::new(iq_cli::create_shell(ColorConfig::Auto));
}

/// Say something with the given color
pub fn say<T: ToString>(message: &T, color: Color) {
    SHELL.lock().unwrap().say(message, color).unwrap();
}

/// Say a status message with the given color
pub fn say_status<T, U>(status: T, message: U, color: Color, justified: bool)
where
    T: fmt::Display,
    U: fmt::Display,
{
    SHELL
        .lock()
        .unwrap()
        .say_status(status, message, color, justified)
        .unwrap();
}

/// Print a warning message
pub fn warning<T: fmt::Display>(msg: T) {
    say_status("warning:", msg, color::YELLOW, false);
}

/// Print the given error message to the shell and exit
pub fn exit_error<T: fmt::Display>(msg: T) -> ! {
    say_status("error:", msg, color::RED, false);
    process::exit(1);
}
