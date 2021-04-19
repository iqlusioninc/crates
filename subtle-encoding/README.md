# subtle-encoding <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache 2.0/MIT Licensed][license-image]
![MSRV][rustc-image]
[![Safety Dance][safety-image]][safety-link]
[![Build Status][build-image]][build-link]

Rust crate for encoding/decoding binary data to/from **base64** and **hex**
encodings while avoiding data-dependent branching/table lookups, and therefore
providing "best effort" constant-time operation.

Also includes a non-constant-time Bech32 encoder/decoder gated under the
`bech32-preview` Cargo feature (with a goal of eventually making it
constant-time).

Useful for encoding/decoding secret values such as cryptographic keys.

[Documentation]

## Minimum Supported Rust Version

Rust **1.47** or newer.

In the future, we reserve the right to change MSRV (i.e. MSRV is out-of-scope
for this crate's SemVer guarantees), however when we do it will be accompanied by
a minor version bump.

## Security Notice

While this crate takes care to avoid data-dependent branching, that does not
actually make it "constant time", which is an architecture-dependent property.

This crate is a "best effort" attempt at providing a constant time encoding
library, however it presently provides no guarantees, nor has it been
independently audited for security vulnerabilities.

Use at your own risk.

## License

Copyright © 2018-2021 iqlusion

**subtle-encoding** is distributed under the terms of either the MIT license
or the Apache License (Version 2.0), at your option.

See [LICENSE] (Apache License, Version 2.0) file in the `iqlusioninc/crates`
toplevel directory of this repository or [LICENSE-MIT] for details.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/subtle-encoding.svg
[crate-link]: https://crates.io/crates/subtle-encoding
[docs-image]: https://docs.rs/subtle-encoding/badge.svg
[docs-link]: https://docs.rs/subtle-encoding/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.47+-blue.svg
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[build-image]: https://github.com/iqlusioninc/crates/actions/workflows/subtle-encoding.yml/badge.svg
[build-link]: https://github.com/iqlusioninc/crates/actions/workflows/subtle-encoding.yml

[//]: # (general links)

[Documentation]: https://docs.rs/subtle-encoding/
[LICENSE]: https://github.com/iqlusioninc/crates/blob/main/LICENSE
[LICENSE-MIT]: https://github.com/iqlusioninc/crates/blob/main/subtle-encoding/LICENSE-MIT
