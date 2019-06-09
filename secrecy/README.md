# secrecy.rs 🤐 <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache 2.0/MIT Licensed][license-image]
![Rust 1.34+][rustc-image]
[![forbid(unsafe_code)][unsafe-image]][unsafe-link]
[![Build Status][build-image]][build-link]
[![Gitter Chat][gitter-image]][gitter-link]

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

## Requirements

- Rust 1.35+

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

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

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
[unsafe-link]: https://internals.rust-lang.org/t/disabling-unsafe-by-default/7988
[build-image]: https://travis-ci.com/iqlusioninc/crates.svg?branch=develop
[build-link]: https://travis-ci.com/iqlusioninc/crates/
[gitter-image]: https://badges.gitter.im/iqlusioninc/community.svg
[gitter-link]: https://gitter.im/iqlusioninc/community

[//]: # (general links)

[LICENSE]: https://github.com/iqlusioninc/crates/blob/master/LICENSE
