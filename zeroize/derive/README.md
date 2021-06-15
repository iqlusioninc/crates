# zeroize_derive 🄌 <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Crate][crate-image]][crate-link]
![Apache 2.0 Licensed/MIT][license-image]
![MSRV][rustc-image]
[![Build Status][build-image]][build-link]

Custom derive support for [zeroize]: a crate for securely zeroing memory
while avoiding compiler optimizations.

This crate isn't intended to be used directly.
See [zeroize] crate for documentation.

## Minimum Supported Rust Version

Rust **1.51** or newer.

In the future, we reserve the right to change MSRV (i.e. MSRV is out-of-scope
for this crate's SemVer guarantees), however when we do it will be accompanied by
a minor version bump.

## License

**zeroize_derive** is distributed under the terms of either the MIT license
or the Apache License (Version 2.0), at your option.

See [LICENSE] (Apache License, Version 2.0) file in the `iqlusioninc/crates`
toplevel directory of this repository or [LICENSE-MIT] for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be dual licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/zeroize_derive.svg
[crate-link]: https://crates.io/crates/zeroize_derive
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.51+-blue.svg
[build-image]: https://github.com/iqlusioninc/crates/workflows/Rust/badge.svg?branch=main&event=push
[build-link]: https://github.com/iqlusioninc/crates/actions

[//]: # (general links)

[zeroize]: https://github.com/iqlusioninc/crates/tree/main/zeroize
[LICENSE]: https://github.com/iqlusioninc/crates/blob/main/LICENSE
[LICENSE-MIT]: https://github.com/iqlusioninc/crates/blob/main/zeroize/LICENSE-MIT
