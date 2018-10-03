# [zeroize].rs <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-prod-web-assets/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>🄌

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![MIT/Apache 2.0 Licensed][license-image]
[![Build Status][build-image]][build-link]

[crate-image]: https://img.shields.io/crates/v/zeroize.svg
[crate-link]: https://crates.io/crates/zeroize
[docs-image]: https://docs.rs/zeroize/badge.svg
[docs-link]: https://docs.rs/zeroize/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[build-image]: https://circleci.com/gh/iqlusioninc/crates.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusioninc/crates

*Alpha-quality preview*: Rust crate for securely zeroing memory while
avoiding compiler optimizations.

This crate provides a safe<sup>†</sup>, portable `secure_zero_memory()`
wrapper function for secure memory zeroing intrinsics which are
specifically documented as guaranteeing they won't be "optimized away".

[Documentation]

## About

[Zeroing memory securely is hard] - compilers optimize for performance, and
in doing so they love to "optimize away" unnecessary zeroing calls. There are
many documented "tricks" to attempt to avoid these optimizations and ensure
that a zeroing routine is performed reliably.

This crate isn't about tricks: instead it *only* invokes intrinsics (either
rustc/LLVM or OS) with a documented contract assuring the caller that after
the intrinsic is invoked, memory will be zeroed 100% of the time.

**No insecure fallbacks. No dependencies<sup>‡</sup>. `#![no_std]`. No
functionality besides securely zeroing memory.**

This crate has one job and one function: `secure_zero_memory()`, and it
provides the thinnest portable wrapper for secure zeroing intrinsics.

If it can't find a way to securely zero memory, **it will refuse to compile**.
Don't worry about that though: it supports almost every tier 1 and 2 Rust
platform (and even most of tier 3!). See below for compatiblity.

## Platform Support / Intrinsics

This crate provides wrappers for the following intrinsics:

- `stable` rust: OS intrinsics
  - [explicit_bzero()]: Linux<sup>‡</sup>, FreeBSD, OpenBSD, DragonflyBSD
  - [explicit_memset()]: NetBSD
  - [memset_s()]: Mac OS X/iOS, Solaris
  - [SecureZeroMemroy()]: Windows
- `nightly` rust: [volatile_set_memory()] (all platforms)

Notable unsupported platforms at present: Fuchsia, Redox. PRs accepted!

Enable the `nightly` cargo feature to take advantage of the Rust intrinsic
rather than using FFI to invoke OS intrinsics (requires a nightly rustc):

`Cargo.toml` example:

```toml
[dependencies.zeroize]
version = "0"
features = ["nightly"]
```

‡ NOTE: Linux w\ glibc versions earlier than 2.2.5 (i.e. when the
  `linux-backport` cargo feature is enabled) uses `cc` as a build
  dependency to link `explicit_bzero.c`, a backport of the method
  glibc uses to implement `explicit_bzero()`.

## Stack/Heap Zeroing Notes

This crate can be used to zero values from either the stack or the heap.

However, be aware that Rust's current memory semantics (e.g. move)
can leave copies of data in memory, and there isn't presently a good solution
for ensuring all copies of data on the stack are properly cleared.

The [`Pin` RFC][pin] proposes a method for avoiding this. See also:
<https://github.com/rust-lang/rust/issues/17046>.

## What about: clearing registers, mlock, mprotect, etc?

This crate is laser focused on being a simple, unobtrusive crate for zeroing
memory reliably.

Clearing registers is a difficult problem that can't easily be solved by
something like a crate, and requires either inline ASM or rustc support.

Other memory protection mechanisms are interesting and useful, but often
overkill (e.g. defending against RAM scraping or attackers with swap access).
There are already many other crates that already implement more sophisticated
memory protections.

Zeroing memory is [good cryptographic hygiene] and this crate seeks to promote
it in the most unobtrusive manner possible. This includes omitting complex
`unsafe` memory protection systems and just trying to make the best memory
zeroing crate available.

## Security Warning

† NOTE: When we say "safe", we mean the caller doesn't need to use the
  `unsafe` keyword. 

This crate is presently **alpha quality**.

This crate makes use of `unsafe`, and furthermore, contains FFI bindings for
operating systems it hasn't been directly tested against. These usages have
not been expertly audited to ensure they are memory safe.

There is presently no automated testing in CI (e.g. ASAN) to ensure memory
safe operation.

Though the intrinsic wrappers in this crate are trivial, their current form
involves a certain degree of guesswork and their compatibility has not been
rigorously tested for memory safety on the platforms this crate claims to
support.

**USE AT YOUR OWN RISK!**

## License

**zeroize** is distributed under the terms of either the MIT license
or the Apache License (Version 2.0), at your option.

See [LICENSE] (Apache License, Version 2.0) file in the `iqlusioninc/crates`
toplevel directory of this repository or [LICENSE-MIT] for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be dual licensed as above,
without any additional terms or conditions.

[zeroize]: https://en.wikipedia.org/wiki/Zeroisation
[Documentation]: https://docs.rs/zeroize/
[Zeroing memory securely is hard]: http://www.daemonology.net/blog/2014-09-04-how-to-zero-a-buffer.html
[explicit_bzero()]: http://man7.org/linux/man-pages/man3/bzero.3.html
[explicit_memset()]: http://netbsd.gw.com/cgi-bin/man-cgi?explicit_memset+3.i386+NetBSD-8.0
[memset_s()]: https://www.unix.com/man-page/osx/3/memset_s/
[SecureZeroMemory()]: https://msdn.microsoft.com/en-us/library/windows/desktop/aa366877(v=vs.85).aspx
[volatile_set_memory()]: https://doc.rust-lang.org/std/intrinsics/fn.volatile_set_memory.html
[pin]: https://github.com/rust-lang/rfcs/blob/master/text/2349-pin.md
[good cryptographic hygiene]: https://cryptocoding.net/index.php/Coding_rules#Clean_memory_of_secret_data
[LICENSE]: https://github.com/iqlusioninc/crates/blob/master/LICENSE
[LICENSE-MIT]: https://github.com/iqlusioninc/crates/blob/master/zeroize/LICENSE-MIT
