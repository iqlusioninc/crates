# <img alt="gaunt.rs" src="https://storage.googleapis.com/iqlusion-production-web/github/gaunt/gaunt-logo.svg" width=400>

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust 1.31+][rustc-image]
[![forbid(unsafe_code)][unsafe-image]][unsafe-link]
[![Build Status][build-image]][build-link]

High-level, (mostly) self-contained, minimalist HTTP toolkit (client-only).
Suitable for use in constrainted environments where `mio` and `tokio`
are not (yet) available.

[Documentation][docs-link]

## About

**gaunt.rs** is a minimalist alternative to `hyper` suitable for use in
environments where crates like `mio` and `tokio` aren't (yet) available,
such as Intel SGX or `#![no_std]` environments.

It's presently in an early stage of development and not ready for general
use, but has the following roadmap:

- Lightweight, `#![no_std]`-friendly core.
- `std`-based blocking socket I/O backend.
- `hyper`-based backend to leverage `mio`/`tokio` when available.
- Leverage `http` and `httparse` crates (`hyper` dependencies)
  as they are mature and well-tested.
- Add server support in addition to client support.

## License

Copyright © 2018 iqlusion

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/gaunt.svg
[crate-link]: https://crates.io/crates/gaunt
[docs-image]: https://docs.rs/gaunt/badge.svg
[docs-link]: https://docs.rs/gaunt/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/develop/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.31+-blue.svg
[unsafe-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[unsafe-link]: https://internals.rust-lang.org/t/disabling-unsafe-by-default/7988
[build-image]: https://travis-ci.com/iqlusioninc/crates.svg?branch=develop
[build-link]: https://travis-ci.com/iqlusioninc/crates/
