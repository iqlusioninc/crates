# iq-cli: Command-line app microframework

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Appveyor Status][appveyor-image]][appveyor-link]
[![Apache 2.0 Licensed][license-image]][license-link]

[crate-image]: https://img.shields.io/crates/v/iq-cli.svg
[crate-link]: https://crates.io/crates/iq-cli
[docs-image]: https://docs.rs/iq-cli/badge.svg
[docs-link]: https://docs.rs/iq-cli/
[build-image]: https://circleci.com/gh/iqlusioninc/crates.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusioninc/crates
[appveyor-image]: https://ci.appveyor.com/api/projects/status/1ua33q2njho24e9h?svg=true
[appveyor-link]: https://ci.appveyor.com/project/tony-iqlusion/crates
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/master/LICENSE

This crate provides a set of largely self-contained, opinionated components
for building command-line applications in Rust. It provides the following
features:

* Cargo-like status messages with easy-to-use macros
* Colored terminal output (with autodetection of color support)

## License

The **iq-cli** crate is distributed under the terms of the
Apache License (Version 2.0).

Parts of this code were taken from the following projects, which have agreed
to license their code under the Apache License (Version 2.0):

* [Cargo](https://github.com/rust-lang/cargo)
* [gumdrop](https://github.com/murarth/gumdrop)
* [isatty](https://github.com/dtolnay/isatty)

See [LICENSE] file in the `iqlusioninc/crates` toplevel directory for more
information.

[LICENSE]: https://github.com/iqlusioninc/crates/blob/master/LICENSE
