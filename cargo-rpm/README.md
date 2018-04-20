# cargo rpm

[![Crate][crate-image]][crate-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]

[crate-image]: https://img.shields.io/crates/v/cargo-rpm.svg
[crate-link]: https://crates.io/crates/cargo-rpm
[build-image]: https://circleci.com/gh/iqlusion-io/crates.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusion-io/crates
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusion-io/crates/blob/master/LICENSE

A [cargo subcommand] for building `.rpm` releases of Rust projects.

[cargo subcommand]: https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands 

## Installation

Install `cargo rpm` by running: `cargo install cargo-rpm`.

## Configuring a crate

To configure your crate for RPM releases, run `cargo rpm init`

This will create a `.rpm/YOURCRATENAME.spec` file which is passed to the
`rpmbuild` command. Though the generated spec should work out of the box,
it may need some customization if the resulting RPM has dependencies or
files other than target binaries.

For more information on spec files, see:
<http://ftp.rpm.org/max-rpm/s1-rpm-build-creating-spec-file.html>

## Building RPMs

Once your crate has been configured, run `cargo rpm build` to build release
targets for your project and package them into an RPM.

If you encounter errors, you may need to see more information about why
`rpmbuild` failed. Run `cargo rpm build -v` to enable verbose mode.

Finished `.rpm` files will be placed in `target/release/rpmbuild/RPMs/<arch>`

## License

The **cargo-rpm** crate is distributed under the terms of the Apache License
(Version 2.0).

See [LICENSE] file in the `iqlusion-io/crates` toplevel directory for more
information.

[LICENSE]: https://github.com/iqlusion-io/crates/blob/master/LICENSE
