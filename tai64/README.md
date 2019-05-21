# TAI64 / TAI64N Timestamps for Rust

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust 1.34+][rustc-image]
[![Build Status][build-image]][build-link]

An implementation of the [TAI64(N)] (*Temps Atomique International*) timestamp
format in Rust.

Supports converting to/from Rust's built-in [SystemTime] type and optionally to
[chrono]'s [DateTime] type when the `"chrono"` feature is enabled.

[Documentation][docs-link]

## Requirements

- Rust 1.34+

## License

The **tai64** crate is distributed under the terms of the Apache License
(Version 2.0).

See [LICENSE] file in the `iqlusioninc/crates` toplevel directory for more
information.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/tai64.svg
[crate-link]: https://crates.io/crates/tai64
[docs-image]: https://docs.rs/tai64/badge.svg
[docs-link]: https://docs.rs/tai64/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/develop/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.34+-blue.svg
[build-image]: https://travis-ci.com/iqlusioninc/crates.svg?branch=develop
[build-link]: https://travis-ci.com/iqlusioninc/crates/

[//]: # (general links)

[TAI64(N)]: https://cr.yp.to/libtai/tai64.html
[SystemTime]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
[chrono]: https://github.com/chronotope/chrono
[DateTime]: https://docs.rs/chrono/0.4.0/chrono/struct.DateTime.html
[LICENSE]: https://github.com/iqlusioninc/crates/blob/develop/LICENSE
