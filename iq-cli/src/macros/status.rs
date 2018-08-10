//! Macros for ergonomic status messages printed to stdout/stderr
//!
//! # `status_ok!`: Successful status messages
//!
//! ```
//! # #[macro_use] extern crate iq_cli;
//! # fn main() {
//! // Print a Cargo-like justified status to STDOUT
//! status_ok!("Loaded", "app loaded successfully");
//! # }
//! ```
//!
//! # `status_err!`: Error messages
//!
//! ```
//! # #[macro_use] extern crate iq_cli;
//! # fn main() {
//! // Print an error message
//! status_err!("something bad happened");
//! # }
//! ```
//!
//! # `status_attr_ok!`: Successful attributes
//!
//! ```
//! # #[macro_use] extern crate iq_cli;
//! # fn main() {
//! // Print an indented attribute to STDOUT
//! status_attr_ok!("good", "yep");
//! # }
//! ```
//!
//! # `status_attr_error!`: Error attributes
//!
//! ```
//! # #[macro_use] extern crate iq_cli;
//! # fn main() {
//! // Print an error attribute to STDERR
//! status_attr_err!("error", "yep");
//! # }
//! ```

/// Print a status message (in the given color if colors are enabled)
#[macro_export]
macro_rules! status {
    ($stream:expr, $color:expr, $status:expr, $msg:expr) => {
        $crate::status($stream, $color, $status, $msg, false);
    };
    ($stream:expr, $color:expr, $status:expr, $fmt:expr, $($arg:tt)+) => {
        status!($stream, $color, $status, format!($fmt, $($arg)+));
    };
}

/// Print a success status message (in green if colors are enabled)
///
/// ```
/// # #[macro_use] extern crate iq_cli;
/// # fn main() {
/// // Print a Cargo-like justified status to STDOUT
/// status_ok!("Loaded", "app loaded successfully");
/// # }
/// ```
#[macro_export]
macro_rules! status_ok {
    ($status:expr, $msg:expr) => {
        $crate::status($crate::Stream::Stdout, $crate::color::GREEN, $status, $msg, true);
    };
    ($status:expr, $fmt:expr, $($arg:tt)+) => {
        status_ok!($status, format!($fmt, $($arg)+));
    };
}

/// Print an error message (in red if colors are enabled)
///
/// ```
/// # #[macro_use] extern crate iq_cli;
/// # fn main() {
/// // Print an error message
/// status_err!("something bad happened");
/// # }
/// ```
#[macro_export]
macro_rules! status_err {
    ($msg:expr) => {
        $crate::status($crate::Stream::Stderr, $crate::color::RED, "error:", $msg, false);
    };
    ($fmt:expr, $($arg:tt)+) => {
        status_err!(format!($fmt, $($arg)+));
    };
}

/// Print a tab-delimited status (with the given color if enabled)
#[macro_export]
macro_rules! status_attr {
    ($stream:expr, $color:expr, $attr:expr, $msg:expr) => {
        // TODO: this is kind of hax... use a better format string?
        let attr_delimited = if $attr.len() >= 7 {
            format!("{}:", $attr)
        } else {
            format!("{}:\t", $attr)
        };

        $crate::status(
            $stream,
            $color,
            attr_delimited,
            $msg,
            false,
        );
    };
    ($stream:expr, $color:expr, $attr: expr, $fmt:expr, $($arg:tt)+) => {
        status_attr!($stream, $attr, format!($fmt, $($arg)+));
    }
}

/// Print a tab-delimited status attribute (in green if colors are enabled)
///
/// ```
/// # #[macro_use] extern crate iq_cli;
/// # fn main() {
/// // Print an indented attribute to STDOUT
/// status_attr_ok!("good", "yep");
/// # }
/// ```
#[macro_export]
macro_rules! status_attr_ok {
    ($attr:expr, $msg:expr) => {
        status_attr!($crate::Stream::Stdout, $crate::color::GREEN, $attr, $msg);
    };
    ($attr: expr, $fmt:expr, $($arg:tt)+) => {
        status_attr_ok!($attr, format!($fmt, $($arg)+));
    }
}

/// Print a tab-delimited status attribute (in red if colors are enabled)
///
/// ```
/// # #[macro_use] extern crate iq_cli;
/// # fn main() {
/// // Print an error attribute to STDERR
/// status_attr_err!("error", "yep");
/// # }
/// ```
#[macro_export]
macro_rules! status_attr_err {
    ($attr:expr, $msg:expr) => {
        status_attr!($crate::Stream::Stderr, $crate::color::RED, $attr, $msg);
    };
    ($attr: expr, $fmt:expr, $($arg:tt)+) => {
        status_attr_err!($attr, format!($fmt, $($arg)+));
    }
}
