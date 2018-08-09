//! Macros for ergonomic status messages printed to stdout/stderr

/// Print a tab-delimited status (with the given color if enabled)
#[macro_export]
macro_rules! status_attr_clr {
    ($shell:expr, $color:expr, $attr:expr, $msg:expr) => {
        // TODO: this is kind of hax... use a better format string?
        let attr_delimited = if $attr.len() >= 7 {
            format!("{}:", $attr)
        } else {
            format!("{}:\t", $attr)
        };

        $crate::status(
            $shell,
            $color,
            attr_delimited,
            $msg,
            false,
        );
    };
    ($shell:expr, $color:expr, $attr: expr, $fmt:expr, $($arg:tt)+) => {
        status_attr_clr!($shell, $attr, format!($fmt, $($arg)+));
    }
}

/// Print a tab-delimited status attribute (in green if colors are enabled)
#[macro_export]
macro_rules! status_attr_ok {
    ($attr:expr, $msg:expr) => {
        status_attr_clr!($crate::Stream::Stdout, $crate::color::GREEN, $attr, $msg);
    };
    ($attr: expr, $fmt:expr, $($arg:tt)+) => {
        status_attr_ok!($attr, format!($fmt, $($arg)+));
    }
}

/// Print a tab-delimited status attribute (in red if colors are enabled)
#[macro_export]
macro_rules! status_attr_err {
    ($attr:expr, $msg:expr) => {
        status_attr_clr!($crate::Stream::Stderr, $crate::color::RED, $attr, $msg);
    };
    ($attr: expr, $fmt:expr, $($arg:tt)+) => {
        status_attr_err!($attr, format!($fmt, $($arg)+));
    }
}

/// Print an error message (in red if colors are enabled)
#[macro_export]
macro_rules! status_err {
    ($msg:expr) => {
        $crate::status($crate::Stream::Stderr, $crate::color::RED, "error:", $msg, false);
    };
    ($fmt:expr, $($arg:tt)+) => {
        status_err!(format!($fmt, $($arg)+));
    };
}

/// Print a success status message (in green if colors are enabled)
#[macro_export]
macro_rules! status_ok {
    ($status:expr, $msg:expr) => {
        $crate::status($crate::Stream::Stdout, $crate::color::GREEN, $status, $msg, true);
    };
    ($status:expr, $fmt:expr, $($arg:tt)+) => {
        status_ok!($status, format!($fmt, $($arg)+));
    };
}
