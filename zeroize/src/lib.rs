//! Securely zero memory with a simple trait ([Zeroize]) built on stable Rust
//! primitives which guarantee the operation will not be 'optimized away'.
//!
//! ## Usage
//!
//! ```
//! extern crate zeroize;
//! use zeroize::Zeroize;
//!
//! fn main() {
//!     // Protip: don't embed secrets in your source code.
//!     // This is just an example.
//!     let mut secret = b"Air shield password: 1,2,3,4,5".to_vec();
//!     // [ ... ] open the air shield here
//!
//!     // Now that we're done using the secret, zero it out.
//!     secret.zeroize();
//! }
//! ```
//!
//! The [Zeroize] trait is impl'd on all of Rust's core scalar types including
//! integers, floats, `bool`, and `char`.
//!
//! Additionally, it's implemented on slices and `IterMut`s of the above types.
//!
//! When the `std` feature is enabled (which it is by default), it's also impl'd
//! for `Vec`s of the above types as well as `String`, where it provides
//! [Vec::clear()] / [String::clear()]-like behavior (truncating to zero-length)
//! but ensures the backing memory is securely zeroed.
//!
//! The [ZeroizeWithDefault] marker trait can be impl'd on types which also
//! impl [Default], which implements [Zeroize] by overwriting a value with
//! the default value.
//!
//! ## About
//!
//! [Zeroing memory securely is hard] - compilers optimize for performance, and
//! in doing so they love to "optimize away" unnecessary zeroing calls. There are
//! many documented "tricks" to attempt to avoid these optimizations and ensure
//! that a zeroing routine is performed reliably.
//!
//! This crate isn't about tricks: it uses [core::ptr::write_volatile]
//! and [core::sync::atomic] memory fences to provide easy-to-use, portable
//! zeroing behavior which works on all of Rust's core number types and slices
//! thereof, implemented in pure Rust with no usage of FFI or assembly.
//!
//! - **No insecure fallbacks!**
//! - **No dependencies!**
//! - **No FFI or inline assembly!**
//! - `#![no_std]` **i.e. embedded-friendly**!
//! - **No functionality besides securely zeroing memory!**
//!
//! ## What guarantees does this crate provide?
//!
//! Ideally a secure memory-zeroing function would guarantee the following:
//!
//! 1. Ensure the zeroing operation can't be "optimized away" by the compiler.
//! 2. Ensure all subsequent reads to the memory following the zeroing operation
//!    will always see zeroes.
//!
//! This crate guarantees #1 is true: LLVM's volatile semantics ensure it.
//!
//! The story around #2 is much more complicated. In brief, it should be true that
//! LLVM's current implementation does not attempt to perform optimizations which
//! would allow a subsequent (non-volatile) read to see the original value prior
//! to zeroization. However, this is not a guarantee, but rather an LLVM
//! implementation detail.
//!
//! For more background, we can look to the [core::ptr::write_volatile]
//! documentation:
//!
//! > Volatile operations are intended to act on I/O memory, and are guaranteed
//! > to not be elided or reordered by the compiler across other volatile
//! > operations.
//! >
//! > Memory accessed with `read_volatile` or `write_volatile` should not be
//! > accessed with non-volatile operations.
//!
//! Uhoh! This crate does not guarantee all reads to the memory it operates on
//! are volatile, and the documentation for [core::ptr::write_volatile]
//! explicitly warns against mixing volatile and non-volatile operations.
//! Perhaps we'd be better off with something like a `VolatileCell`
//! type which owns the associated data and ensures all reads and writes are
//! volatile so we don't have to worry about the semantics of mixing volatile and
//! non-volatile accesses.
//!
//! While that's a strategy worth pursuing (and something we may investigate
//! separately from this crate), it comes with some onerous API requirements:
//! it means any data that we might ever desire to zero is owned by a
//! `VolatileCell`. However, this does not make it possible for this crate
//! to act on references, which severely limits its applicability. In fact
//! a `VolatileCell` can only act on values, i.e. to read a value from it,
//! we'd need to make a copy of it, and that's literally the opposite of
//! what we want.
//!
//! It's worth asking what the precise semantics of mixing volatile and
//! non-volatile reads actually are, and whether a less obtrusive API which
//! can act entirely on mutable references is possible, safe, and provides the
//! desired behavior.
//!
//! Unfortunately, that's a tricky question, because
//! [Rust does not have a formally defined memory model][memory-model],
//! and the behavior of mixing volatile and non-volatile memory accesses is
//! therefore not rigorously specified and winds up being an LLVM
//! implementation detail. The semantics were discussed extensively in this
//! thread, specifically in the context of zeroing secrets from memory:
//!
//! <https://internals.rust-lang.org/t/volatile-and-sensitive-memory/3188/24>
//!
//! Some notable details from this thread:
//!
//! - Rust/LLVM's notion of "volatile" is centered around data *accesses*, not
//!   the data itself. Specifically it maps to flags in LLVM IR which control
//!   the behavior of the optimizer, and is therefore a bit different from the
//!   typical C notion of "volatile".
//! - As mentioned earlier, LLVM does not presently contain optimizations which
//!   would reorder a non-volatile read to occurs before a volatile write if
//!   it is written with the opposite ordering in the original code. However,
//!   there is nothing precluding such optimizations from being added. The
//!   current implementation presently appears to exhibit the desired behavior
//!   for both points #1 and #2 above, but there is nothing preventing future
//!   versions of Rust and/or LLVM from changing that.
//!
//! To help mitigate concerns about reordering potentially exposing secrets
//! after they have been zeroed, this crate leverages the [core::sync::atomic]
//! memory fence functions including [compiler_fence] and [fence] (which uses
//! the CPU's native fence instructions). These fences are leveraged with the
//! strictest ordering guarantees, [Ordering::SeqCst], which ensures no
//! accesses are reordered. Without a formally defined memory model we can't
//! guarantee these will be effective, but we hope they will cover most cases.
//!
//! Concretely the threat of leaking "zeroized" secrets (via reordering by
//! LLVM and/or the CPU via out-of-order or speculative execution) would
//! require a non-volatile access to be reordered ahead of the following:
//!
//! 1. before an [Ordering::SeqCst] compiler fence
//! 2. before an [Ordering::SeqCst] runtime fence
//! 3. before a volatile write
//!
//! This seems unlikely, but our usage of mixed non-volatile and volatile
//! accesses is technically undefined behavior, at least until guarantees
//! about this particular mixture of operations is formally defined in a
//! Rust memory model.
//!
//! Furthermore, given the recent history of microarchitectural attacks
//! (Spectre, Meltdown, etc), there is also potential for "zeroized" secrets
//! to be leaked through covert channels (e.g. memory fences have been used
//! as a covert channel), so we are wary to make guarantees unless they can
//! be made firmly in terms of both a formal Rust memory model and the
//! generated code for a particular CPU architecture.
//!
//! In conclusion, this crate guarantees the zeroize operation will not be
//! elided or "optimized away", makes a "best effort" to ensure that
//! memory accesses will not be reordered ahead of the "zeroize" operation,
//! but **cannot** yet guarantee that such reordering will not occur.
//!
//! ## Stack/Heap Zeroing Notes
//!
//! This crate can be used to zero values from either the stack or the heap.
//!
//! However, be aware that Rust's current memory semantics (e.g. `Copy` types)
//! can leave copies of data in memory, and there isn't presently a good solution
//! for ensuring all copies of data on the stack are properly cleared.
//!
//! The [`Pin` RFC][pin] proposes a method for avoiding this.
//!
//! ## What about: clearing registers, mlock, mprotect, etc?
//!
//! This crate is laser-focused on being a simple, unobtrusive crate for zeroing
//! memory in as reliable a manner as is possible on stable Rust.
//!
//! Clearing registers is a difficult problem that can't easily be solved by
//! something like a crate, and requires either inline ASM or rustc support.
//! See <https://github.com/rust-lang/rust/issues/17046> for background on
//! this particular problem.
//!
//! Other memory protection mechanisms are interesting and useful, but often
//! overkill (e.g. defending against RAM scraping or attackers with swap access).
//! In as much as there may be merit to these approaches, there are also many
//! other crates that already implement more sophisticated memory protections.
//! Such protections are explicitly out-of-scope for this crate.
//!
//! Zeroing memory is [good cryptographic hygiene] and this crate seeks to promote
//! it in the most unobtrusive manner possible. This includes omitting complex
//! `unsafe` memory protection systems and just trying to make the best memory
//! zeroing crate available.
//!
//! [Zeroize]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html
//! [Zeroing memory securely is hard]: http://www.daemonology.net/blog/2014-09-04-how-to-zero-a-buffer.html
//! [Vec::clear()]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.clear
//! [String::clear()]: https://doc.rust-lang.org/std/string/struct.String.html#method.clear
//! [ZeroizeWithDefault]: https://docs.rs/zeroize/latest/zeroize/trait.ZeroizeWithDefault.html
//! [Default]: https://doc.rust-lang.org/std/default/trait.Default.html
//! [core::ptr::write_volatile]: https://doc.rust-lang.org/core/ptr/fn.write_volatile.html
//! [core::sync::atomic]: https://doc.rust-lang.org/stable/core/sync/atomic/index.html
//! [Ordering::SeqCst]: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.SeqCst
//! [compiler_fence]: https://doc.rust-lang.org/stable/core/sync/atomic/fn.compiler_fence.html
//! [fence]: https://doc.rust-lang.org/stable/core/sync/atomic/fn.fence.html
//! [memory-model]: https://github.com/nikomatsakis/rust-memory-model
//! [pin]: https://github.com/rust-lang/rfcs/blob/master/text/2349-pin.md
//! [good cryptographic hygiene]: https://cryptocoding.net/index.php/Coding_rules#Clean_memory_of_secret_data

#![no_std]
#![deny(warnings, missing_docs, unused_import_braces, unused_qualifications)]
#![cfg_attr(all(feature = "nightly", not(feature = "std")), feature(alloc))]
#![cfg_attr(feature = "nightly", feature(core_intrinsics))]
#![doc(html_root_url = "https://docs.rs/zeroize/0.5.1")]

#[cfg(any(feature = "std", test))]
#[cfg_attr(test, macro_use)]
extern crate std;

use core::{ptr, slice::IterMut, sync::atomic};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::prelude::*;
#[cfg(feature = "std")]
use std::prelude::v1::*;

/// Trait for securely erasing types from memory
pub trait Zeroize {
    /// Zero out this object from memory (using Rust or OS intrinsics which
    /// ensure the zeroization operation is not "optimized away")
    fn zeroize(&mut self);
}

/// Marker trait for types which can be zeroized with the `Default` value
pub trait ZeroizeWithDefault: Copy + Default + Sized {}

impl<Z> Zeroize for Z
where
    Z: ZeroizeWithDefault,
{
    fn zeroize(&mut self) {
        volatile_set(self, Z::default());
        atomic_fence();
    }
}

macro_rules! impl_zeroize_with_default {
    ($($type:ty),+) => {
        $(impl ZeroizeWithDefault for $type {})+
     };
}

impl_zeroize_with_default!(i8, i16, i32, i64, i128, isize);
impl_zeroize_with_default!(u16, u32, u64, u128, usize);
impl_zeroize_with_default!(f32, f64, char, bool);

/// On non-nightly targets, avoid special-casing u8
#[cfg(not(feature = "nightly"))]
impl_zeroize_with_default!(u8);

/// On nightly targets, don't implement `ZeroizeWithDefault` so we can special
/// case using batch set operations.
#[cfg(feature = "nightly")]
impl Zeroize for u8 {
    fn zeroize(&mut self) {
        volatile_set(self, 0);
        atomic_fence();
    }
}

impl<'a, Z> Zeroize for IterMut<'a, Z>
where
    Z: ZeroizeWithDefault,
{
    fn zeroize(&mut self) {
        let default = Z::default();

        for elem in self {
            volatile_set(elem, default);
        }

        atomic_fence();
    }
}

/// Implement zeroize on all types that can be zeroized with the zero value
impl<Z> Zeroize for [Z]
where
    Z: ZeroizeWithDefault,
{
    fn zeroize(&mut self) {
        // TODO: batch volatile set operation?
        self.iter_mut().zeroize();
    }
}

/// On `nightly` Rust, `volatile_set_memory` provides fast byte slice zeroing
#[cfg(feature = "nightly")]
impl Zeroize for [u8] {
    fn zeroize(&mut self) {
        volatile_zero_bytes(self);
        atomic_fence();
    }
}

#[cfg(feature = "alloc")]
impl<Z> Zeroize for Vec<Z>
where
    Z: ZeroizeWithDefault,
{
    fn zeroize(&mut self) {
        self.as_mut_slice().zeroize();
        self.clear();
    }
}

#[cfg(feature = "alloc")]
impl Zeroize for String {
    fn zeroize(&mut self) {
        unsafe { self.as_bytes_mut() }.zeroize();
        self.clear();
    }
}

/// On `nightly` Rust, `volatile_set_memory` provides fast byte array zeroing
#[cfg(feature = "nightly")]
macro_rules! impl_zeroize_for_byte_array {
    ($($size:expr),+) => {
        $(
            impl Zeroize for [u8; $size] {
                fn zeroize(&mut self) {
                    volatile_zero_bytes(self.as_mut());
                    atomic_fence();
                }
            }
        )+
     };
}

#[cfg(feature = "nightly")]
impl_zeroize_for_byte_array!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);

/// Use fences to prevent accesses from being reordered before this
/// point, which should hopefully help ensure that all accessors
/// see zeroes after this point.
#[inline]
fn atomic_fence() {
    atomic::fence(atomic::Ordering::SeqCst);
    atomic::compiler_fence(atomic::Ordering::SeqCst);
}

/// Set a mutable reference to a value to the given replacement
#[inline]
fn volatile_set<T: Copy + Sized>(dst: &mut T, src: T) {
    unsafe { ptr::write_volatile(dst, src) }
}

#[cfg(feature = "nightly")]
#[inline]
fn volatile_zero_bytes(dst: &mut [u8]) {
    unsafe { core::intrinsics::volatile_set_memory(dst.as_mut_ptr(), 0, dst.len()) }
}

#[cfg(test)]
mod tests {
    use super::Zeroize;
    use std::prelude::v1::*;

    #[test]
    fn zeroize_byte_arrays() {
        let mut arr = [42u8; 64];
        arr.zeroize();
        assert_eq!(arr.as_ref(), [0u8; 64].as_ref());
    }

    #[test]
    fn zeroize_vec() {
        let mut vec = vec![42; 3];
        vec.zeroize();
        assert!(vec.is_empty());
    }

    #[test]
    fn zeroize_string() {
        let mut string = String::from("Hello, world!");
        string.zeroize();
        assert!(string.is_empty());
    }

    #[test]
    fn zeroize_box() {
        let mut boxed_arr = Box::new([42u8; 3]);
        boxed_arr.zeroize();
        assert_eq!(boxed_arr.as_ref(), &[0u8; 3]);
    }
}
