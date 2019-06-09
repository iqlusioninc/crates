# zeroize_derive 🄌 <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Crate][crate-image]][crate-link]
![Apache 2.0 Licensed/MIT][license-image]
![Rust 1.35+][rustc-image]
[![Build Status][build-image]][build-link]

Custom derive support for [zeroize]: a crate for securely zeroing memory
while avoiding compiler optimizations.

This crate isn't intended to be used directly.
See [zeroize] crate for documentation.

## Requirements

- Rust 1.35+

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
[rustc-image]: https://img.shields.io/badge/rustc-1.35+-blue.svg
[build-image]: https://travis-ci.com/iqlusioninc/crates.svg?branch=develop
[build-link]: https://travis-ci.com/iqlusioninc/crates/

[//]: # (general links)

[zeroize]: https://github.com/iqlusioninc/crates/tree/develop/zeroize
[LICENSE]: https://github.com/iqlusioninc/crates/blob/develop/LICENSE
[LICENSE-MIT]: https://github.com/iqlusioninc/crates/blob/develop/zeroize/LICENSE-MIT
