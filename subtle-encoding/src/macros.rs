//! Macros which provide abort-on-overflow/underflow semantics.
// TODO: replace this with an `AbortOnOverflow<T>` type?

#![allow(dead_code, unused_macros)]

/// Message to abort with on overflow
pub(crate) const OVERFLOW_MSG: &str = "overflow";

/// Checked addition
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a.checked_add($b).expect($crate::macros::OVERFLOW_MSG)
    };
}

/// Checked subtraction
macro_rules! sub {
    ($a:expr, $b:expr) => {
        $a.checked_sub($b).expect($crate::macros::OVERFLOW_MSG)
    };
}

/// Checked multiplication
macro_rules! mul {
    ($a:expr, $b:expr) => {
        $a.checked_mul($b).expect($crate::macros::OVERFLOW_MSG)
    };
}

/// Checked division
macro_rules! div {
    ($a:expr, $b:expr) => {
        $a.checked_div($b).expect($crate::macros::OVERFLOW_MSG)
    };
}

/// Checked right shift
macro_rules! shr {
    ($a:expr, $b:expr) => {
        $a.checked_shr($b).expect($crate::macros::OVERFLOW_MSG)
    };
}

/// Checked left shift
macro_rules! shl {
    ($a:expr, $b:expr) => {
        $a.checked_shl($b).expect($crate::macros::OVERFLOW_MSG)
    };
}
