# rpmlib.rs: RPM Package Manager binding for Rust

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]

[crate-image]: https://img.shields.io/crates/v/rpmlib.svg
[crate-link]: https://crates.io/crates/rpmlib
[docs-image]: https://docs.rs/rpmlib/badge.svg
[docs-link]: https://docs.rs/rpmlib/
[build-image]: https://circleci.com/gh/iqlusion-io/crates.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusion-io/crates
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusion-io/crates/blob/master/LICENSE

The [rpmlib] C library (available in the `rpm-devel` RPM package) exposes a
programmatic interface to the [RPM Package Manager], and this crate aims to
provide a safe, idiomatic Rust wrapper.

[Documentation](https://docs.rs/rpmlib/)

[rpmlib]: https://docs.fedoraproject.org/en-US/Fedora_Draft_Documentation/0.1/html/RPM_Guide/ch-programming-c.html
[RPM Package Manager]: http://rpm.org/

## Status

- [X] Search and query RPM database by tag with exact match, glob, and regex [#13](https://github.com/iqlusion-io/crates/issues/13)
- [ ] RPM database management: create database, delete database [#25](https://github.com/iqlusion-io/crates/issues/25)
- [ ] Install and upgrade packages [#16](https://github.com/iqlusion-io/crates/issues/16)
- [ ] Version comparison support (i.e. dependency sets) [#15](https://github.com/iqlusion-io/crates/issues/15)
- [ ] RPM reader API (i.e. for `.rpm` files) [#14](https://github.com/iqlusion-io/crates/issues/14)
- [ ] RPM builder API (i.e. `librpmbuild`) [#18](https://github.com/iqlusion-io/crates/issues/18)
- [ ] RPM signing API (i.e. `librpmsign`) [#18](https://github.com/iqlusion-io/crates/issues/18)

## License

The **rpmlib** crate is distributed under the terms of the Apache License
(Version 2.0).

See [LICENSE] file in the `iqlusion-io/crates` toplevel directory for more
information.

[LICENSE]: https://github.com/iqlusion-io/crates/blob/master/LICENSE
