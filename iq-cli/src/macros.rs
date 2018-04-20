//! Macros for generating status messages using the global static SHELL

/// Print a status message with the given color (if colors are enabled)
#[macro_export]
macro_rules! status {
    ($color:expr, $status:expr, $msg:expr) => {
        $crate::status($color, $status, $msg, true);
    };
    ($color:expr, $status:expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status($color, $status, format!($fmt, $($arg)+), true);
    };
}

/// Print a success status message (in green if colors are enabled)
#[macro_export]
macro_rules! status_ok {
    ($status:expr, $msg:expr) => {
        $crate::status($crate::color::GREEN, $status, $msg, true);
    };
    ($status:expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status($crate::color::GREEN, $status, format!($fmt, $($arg)+), true);
    };
}

/// Print a warning message (in yellow if colors are enabled)
#[macro_export]
macro_rules! status_warn {
    ($msg:expr) => {
        $crate::status($crate::color::YELLOW, "warning:", $msg, false);
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::status($crate::color::YELLOW, "warning:", format!($fmt, $($arg)+), false);
    };
}

/// Print an error message (in red if colors are enabled)
#[macro_export]
macro_rules! status_error {
    ($msg:expr) => {
        $crate::status($crate::color::RED, "error:", $msg, false);
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::status($crate::color::RED, "error:", format!($fmt, $($arg)+), false);
    };
}
