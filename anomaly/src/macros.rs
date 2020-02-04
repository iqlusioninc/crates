//! This crate defines error handling macros designed to produce formatted
//! error messages using the [`Message`] type.

#[allow(unused_imports)]
use crate::message::Message;

/// Create a new error (of a given kind) with a formatted [`Message`]
/// as its source.
///
/// If additional parameters are given, the second is used as a format string,
/// e.g. `format_err!(kind, "something went wrong: {}", &wrongness)`.
#[macro_export]
macro_rules! format_err {
    ($kind:expr, $msg:expr) => {
        $kind.context($crate::Message::new($msg))
    };
    ($kind:expr, $fmt:expr, $($arg:tt)+) => {
        format_err!($kind, &format!($fmt, $($arg)+))
    };
}

/// Create and return an error with a formatted [`Message`].
#[macro_export]
macro_rules! fail {
    ($kind:expr, $msg:expr) => {
        return Err($crate::format_err!($kind, $msg).into());
    };
    ($kind:expr, $fmt:expr, $($arg:tt)+) => {
        fail!($kind, &format!($fmt, $($arg)+));
    };
}

/// Ensure a condition holds, returning an error if it doesn't (ala `assert`)
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $kind:expr, $msg:expr) => {
        if !($cond) {
            return Err($crate::format_err!($kind, $msg).into());
        }
    };
    ($cond:expr, $kind:expr, $fmt:expr, $($arg:tt)+) => {
        ensure!($cond, $kind, format!($fmt, $($arg)+))
    };
}
