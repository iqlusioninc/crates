//! Securely zero memory using core or OS intrinsics. This crate wraps
//! facilities specifically designed to securely zero memory in a common,
//! safe API: [Zeroize].
//!
//! ## Usage
//!
//! ```
//! extern crate zeroize;
//! use zeroize::Zeroize;
//!
//! fn main() {
//!     let mut secret = Vec::from("The password to the air shield is 1,2,3,4,5...");
//!     // [ ... ] open the air shield here
//!
//!     // Actual call to zeroize here:
//!     // `Zeroize` is impl'd for any type which impls `AsMut<[u8]>`
//!     secret.zeroize();
//! }
//! ```
//!
//! ## About
//!
//! [Zeroing memory securely is hard] - compilers optimize for performance, and
//! in doing so they love to "optimize away" unnecessary zeroing calls. There are
//! many documented "tricks" to attempt to avoid these optimizations and ensure
//! that a zeroing routine is performed reliably.
//!
//! This crate isn't about tricks: instead it *only* invokes intrinsics (either
//! rustc/LLVM or OS) with a documented contract assuring the caller that after
//! the intrinsic is invoked, memory will be zeroed 100% of the time.
//!
//! **No insecure fallbacks. No dependencies. `#![no_std]`. No functionality
//! besides securely zeroing memory.**
//!
//! This crate provides the thinnest portable wrapper for secure zeroing
//! intrinsics. If it can't find a way to securely zero memory,
//! **it will refuse to compile**.
//!
//! Don't worry about that though: it supports almost every tier 1 and 2 Rust
//! platform (and even most of tier 3!). See below for compatiblity.
//!
//! ## Platform Support / Intrinsics
//!
//! This crate provides wrappers for the following intrinsics:
//!
//! - `stable` rust: OS intrinsics
//!   - [explicit_bzero():](http://man7.org/linux/man-pages/man3/bzero.3.html)
//!     Linux<sup>‡</sup>, FreeBSD, OpenBSD, DragonflyBSD
//!   - [explicit_memset():](http://netbsd.gw.com/cgi-bin/man-cgi?explicit_memset+3.i386+NetBSD-8.0)
//!     NetBSD
//!   - [memset_s():](https://www.unix.com/man-page/osx/3/memset_s/)
//!     Mac OS X/iOS, Solaris
//!   - [SecureZeroMemory():](https://msdn.microsoft.com/en-us/library/windows/desktop/aa366877(v=vs.85).aspx)
//!     Windows
//! - `nightly` rust: [volatile_set_memory()] (all platforms)
//!
//! Notable unsupported platforms at present: Fuchsia, Redox. PRs accepted!
//!
//! Enable the `nightly` cargo feature to take advantage of the Rust intrinsic
//! rather than using FFI to invoke OS intrinsics (requires a nightly rustc):
//!
//! `Cargo.toml` example:
//!
//! ```toml
//! [dependencies.zeroize]
//! version = "0"
//! features = ["nightly"]
//! ```
//!
//! ‡ NOTE: Linux w\ glibc versions earlier than 2.2.5 (i.e. when the
//! `linux-backport` cargo feature is enabled) uses `cc` as a build
//! dependency to link `explicit_bzero.c`, a backport of the method
//! glibc uses to implement `explicit_bzero()`.
//!
//! ## Stack/Heap Zeroing Notes
//!
//! This crate can be used to zero values from either the stack or the heap.
//!
//! However, be aware that Rust's current memory semantics (e.g. move)
//! can leave copies of data in memory, and there isn't presently a good solution
//! for ensuring all copies of data on the stack are properly cleared.
//!
//! The [`Pin` RFC][pin] proposes a method for avoiding this. See also:
//! <https://github.com/rust-lang/rust/issues/17046>.
//!
//! ## What about: clearing registers, mlock, mprotect, etc?
//!
//! This crate is laser focused on being a simple, unobtrusive crate for zeroing
//! memory reliably.
//!
//! Clearing registers is a difficult problem that can't easily be solved by
//! something like a crate, and requires either inline ASM or rustc support.
//!
//! Other memory protection mechanisms are interesting and useful, but often
//! overkill (e.g. defending against RAM scraping or attackers with swap access).
//! There are already many other crates that already implement more sophisticated
//! memory protections.
//!
//! Zeroing memory is [good cryptographic hygiene] and this crate seeks to promote
//! it in the most unobtrusive manner possible. This includes omitting complex
//! `unsafe` memory protection systems and just trying to make the best memory
//! zeroing crate available.
//!
//! [Zeroize]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html
//! [Zeroing memory securely is hard]: http://www.daemonology.net/blog/2014-09-04-how-to-zero-a-buffer.html
//! [volatile_set_memory()]: https://doc.rust-lang.org/std/intrinsics/fn.volatile_set_memory.html
//! [pin]: https://github.com/rust-lang/rfcs/blob/master/text/2349-pin.md
//! [good cryptographic hygiene]: https://cryptocoding.net/index.php/Coding_rules#Clean_memory_of_secret_data

#![crate_name = "zeroize"]
#![crate_type = "rlib"]
#![no_std]
#![deny(
    warnings,
    missing_docs,
    unused_import_braces,
    unused_qualifications,
)]
#![doc(html_root_url = "https://docs.rs/zeroize/0.3.0")]

#[cfg(any(feature = "std", test))]
#[allow(unused_imports)]
#[macro_use]
extern crate std;

/// Zeroization traits
mod zeroize;
pub use zeroize::*;

// nightly: use `volatile_set_memory`
#[cfg(feature = "nightly")]
mod nightly;
#[cfg(feature = "nightly")]
pub(crate) use nightly::secure_zero_memory;

// stable: use OS-specific APIs
#[cfg(not(feature = "nightly"))]
mod stable;
#[cfg(not(feature = "nightly"))]
pub(crate) use stable::secure_zero_memory;

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
