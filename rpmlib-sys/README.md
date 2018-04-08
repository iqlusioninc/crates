# rpmlib-sys: bindgen wrapper for rpmlib (RedHat Package Manager library)

[![Crate][crate-image]][crate-link] [![Build Status][build-image]][build-link] [![Apache 2.0 Licensed][license-image]][license-link]

[crate-image]: https://img.shields.io/crates/v/rpmlib-sys.svg
[crate-link]: https://crates.io/crates/rpmlib-sys
[build-image]: https://circleci.com/gh/iqlusion-io/crates.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusion-io/crates
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusion-io/crates/blob/master/LICENSE

This crate uses bindgen to generate an unsafe FFI wrapper for the
[rpmlib C library], which provides a low-level API for interacting with the
[RedHat Package Manager (RPM)] and **.rpm** files.

This crate isn't intended to be used directly, but instead provides an unsafe,
low-level binding used by the higher level **rpmlib** crate, which aims to
provide a safe, idiomatic, high-level binding to the C library:

* **rpmlib crate**: https://docs.rs/crate/rpmlib/

[rpmlib C library]: https://docs.fedoraproject.org/en-US/Fedora_Draft_Documentation/0.1/html/RPM_Guide/ch-programming-c.html
[RedHat Package Manager (RPM)]: http://rpm.org/

## License

The **rpmlib-sys** crate is distributed under the terms of the Apache License
(Version 2.0).

See [LICENSE] file in the `iqlusion-io/crates` toplevel directory for more
information.

[LICENSE]: https://github.com/iqlusion-io/crates/blob/master/LICENSE
