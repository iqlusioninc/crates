//! Securely zero memory using core or OS intrinsics. This crate wraps
//! facilities specifically designed to securely zero memory in a common,
//! safe API.
//!
//! This crate deliberately avoids use of `cc`, C shims, and other "tricks"
//! to "securely" zero memory, preferring to use the appropriate intrinsic
//! on nightly, or on stable using FFI bindings to specifically designed
//! OS APIs for securely zeroing memory.

#![crate_name = "zeroize"]
#![crate_type = "rlib"]
#![no_std]
#![deny(
    warnings,
    missing_docs,
    unused_import_braces,
    unused_qualifications,
)]
#![doc(html_root_url = "https://docs.rs/zeroize/0.0.0")]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

// nightly: use `volatile_set_memory`
#[cfg(feature = "nightly")]
mod nightly;
#[cfg(feature = "nightly")]
pub use nightly::secure_zero_memory;

// stable: use OS-specific APIs
#[cfg(not(feature = "nightly"))]
mod stable;
#[cfg(not(feature = "nightly"))]
pub use stable::secure_zero_memory;

#[cfg(test)]
mod tests {
    use super::secure_zero_memory;
    use std::prelude::v1::*;

    /// Ensure the selected implementation actually zeroes memory
    #[test]
    fn test_secure_zero_memory() {
        let mut buffer = Vec::from("DEADBEEFCAFE");
        secure_zero_memory(&mut buffer);
        assert_eq!(buffer, [0u8; 12]);
    }
}
