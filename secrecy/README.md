# secrecy.rs 🤐 <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-prod-web-assets/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache 2.0/MIT Licensed][license-image]
![Rust 1.34+][rustc-image]
![forbid(unsafe_code)][unsafe-image]
[![Build Status][build-image]][build-link]

A simple secret-keeping library for Rust.

[Documentation][docs-link]

## About

**secrecy** is a *simple*, safe (i.e. `forbid(unsafe_code)` library which
provides wrapper types and traits for secret management in Rust, namely the
`Secret<T>` type for wrapping another value in a "secret cell" which attempts
to limit exposure (only available through a special `ExposeSecret` trait).

This helps to ensure secrets aren't accidentally copied, logged, or otherwise
exposed (as much as possible), and also ensures secrets are securely wiped
from memory when dropped.

### serde support

Optional `serde` support for parsing owned secret values is available, gated
under the `serde` cargo feature.

It uses the `Deserialize` and `DeserializeOwned` traits to implement
deserializing secret types which also impl these traits.

This doesn't guarantee `serde` (or code providing input to `serde`) won't
accidentally make additional copies of the secret, but does the best it can
with what it is given and tries to minimize risk of exposure as much as
possible.

## Requirements

- Rust 1.34+

## License

Copyright © 2019 iqlusion

**secrecy** is distributed under the terms of either the MIT license
or the Apache License (Version 2.0), at your option.

See [LICENSE] (Apache License, Version 2.0) file in the `iqlusioninc/crates`
toplevel directory of this repository or [LICENSE-MIT] for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be dual licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/secrecy.svg
[crate-link]: https://crates.io/crates/secrecy
[docs-image]: https://docs.rs/secrecy/badge.svg
[docs-link]: https://docs.rs/secrecy/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.34+-blue.svg
[unsafe-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[build-image]: https://circleci.com/gh/iqlusioninc/crates.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusioninc/crates

[//]: # (general links)

[LICENSE]: https://github.com/iqlusioninc/crates/blob/master/LICENSE
