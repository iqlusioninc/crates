# iq-cli: Crate for making Cargo-like command-line interfaces

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]

[crate-image]: https://img.shields.io/crates/v/iq-cli.svg
[crate-link]: https://crates.io/crates/iq-cli
[docs-image]: https://docs.rs/iq-cli/badge.svg
[docs-link]: https://docs.rs/iq-cli/
[build-image]: https://circleci.com/gh/iqlusioninc/crates.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusioninc/crates
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/master/LICENSE

This crate contains reusable components for building Cargo-like command-line
interfaces which can selectively enable colored output when a TTY is available.

## License

The **iq-cli** crate is distributed under the terms of the
Apache License (Version 2.0).

Parts of this code were taken from the [Cargo](https://github.com/rust-lang/cargo)
project, which is copyright The Rust Project Developers, and dual licensed under
the MIT and Apache 2.0 licenses. However, at least for now we are only making
our codebase available under the Apache 2.0 license.

See [LICENSE] file in the `iqlusioninc/crates` toplevel directory for more
information.

[LICENSE]: https://github.com/iqlusioninc/crates/blob/master/LICENSE
