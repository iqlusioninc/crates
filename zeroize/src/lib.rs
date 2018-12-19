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
//!     let mut secret = b"Air shield password: 1,2,3,4,5".clone();
//!     // [ ... ] open the air shield here
//!
//!     // Now that we're done using the secret, we want to zero it out.
//!     // Actual call to zeroize here:
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
//! This crate isn't about tricks: it builds on the [std::ptr::write_volatile()]
//! function available in `stable` Rust (since 1.0.9) to provide easy-to-use,
//! portable zeroing behavior which works on all of Rust's core number types and
//! slices thereof.
//!
//! - **No insecure fallbacks!**
//! - **No dependencies!**
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
//! For more background, we can look to the [std::ptr::write_volatile()]
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
//! are volatile, and the documentation for [std::ptr::write_volatile()]
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
//! to act on references, which severely limits its applicability.
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
//! In conclusion, this crate guarantees the zeroization operation will not be
//! elided or "optimized away", but **cannot** guarantee that in future
//! versions of Rust and/or LLVM that all subsequent non-volatile reads will
//! see zeroes instead of the original data (though that appears to be the
//! case today).
//!
//! Whether or not this is satisfactory for your use cases will depend on what
//! they are. Notably it may be insufficient to pass rigorous cryptographic
//! audits where precise zeroization semantics are mandatory (e.g. FIPS).
//!
//! All that said, just by using this crate you can keep track of what memory
//! needs to be zeroed, and in the meantime, the implementation of [Zeroize]
//! can evolve into something that could potentially guarantee *all* subsequent
//! reads will be zeroes. For example, `core::sync::atomic` contains the
//! `compiler_fence` and `fence` functions which may be suitable to guarantee
//! the ordering via a combination of compile-time and runtime memory fences.
//! This approach warrants further investigation, and could potentially lead
//! to a sound solution for #2 with guaranteed semantics.
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
//! [std::ptr::write_volatile()]: https://doc.rust-lang.org/std/ptr/fn.write_volatile.html
//! [memory-model]: https://github.com/nikomatsakis/rust-memory-model
//! [pin]: https://github.com/rust-lang/rfcs/blob/master/text/2349-pin.md
//! [good cryptographic hygiene]: https://cryptocoding.net/index.php/Coding_rules#Clean_memory_of_secret_data

#![no_std]
#![deny(warnings, missing_docs, unused_import_braces, unused_qualifications)]
#![doc(html_root_url = "https://docs.rs/zeroize/0.4.2")]

#[cfg(test)]
#[macro_use]
extern crate std;

use core::{ptr, slice::IterMut};

/// Trait for securely erasing types from memory
pub trait Zeroize {
    /// Zero out this object from memory (using Rust or OS intrinsics which
    /// ensure the zeroization operation is not "optimized away")
    fn zeroize(&mut self);
}

macro_rules! impl_zeroize_for_num_types {
    ($($type:ty),+) => {
        $(
            impl Zeroize for $type {
                #[allow(clippy::cast_lossless)]
                fn zeroize(&mut self) {
                    unsafe { ptr::write_volatile(self, 0 as $type) }
                }
            }
        )+
     };
}

impl_zeroize_for_num_types!(i8, i16, i32, i64, i128, isize);
impl_zeroize_for_num_types!(u8, u16, u32, u64, u128, usize);
impl_zeroize_for_num_types!(f32, f64, char);

impl Zeroize for bool {
    fn zeroize(&mut self) {
        unsafe { ptr::write_volatile(self, false) }
    }
}

impl<'a, Z> Zeroize for IterMut<'a, Z>
where
    Z: Zeroize,
{
    fn zeroize(&mut self) {
        for elem in self {
            elem.zeroize()
        }
    }
}

impl<'a, Z> Zeroize for [Z]
where
    Z: Zeroize,
{
    fn zeroize(&mut self) {
        self.iter_mut().zeroize()
    }
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
        let mut vec = vec![42u8; 3];
        vec.zeroize();
        assert!(vec.as_slice().iter().all(|b| *b == 0));
    }

    #[test]
    fn zeroize_box() {
        let mut boxed_arr = Box::new([42u8; 3]);
        boxed_arr.zeroize();
        assert_eq!(boxed_arr.as_ref(), &[0u8; 3]);
    }
}
