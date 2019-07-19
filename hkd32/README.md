# HMAC-based Hierarchical Key Derivation <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>


[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust 1.35+][rustc-image]
[![forbid(unsafe_code)][unsafe-image]][unsafe-link]
[![Build Status][build-image]][build-link]
[![Gitter Chat][gitter-image]][gitter-link]

`hkd32` is a Rust library which implements a hierarchical deterministic
symmetric key derivation construction inspired by
[BIP-0032: Hierarchical Deterministic Wallets][bip32].

It can be used to deterministically derive a hierarchy of symmetric keys
from initial keying material through repeated applications of the
Hash-based Message Authentication Code (HMAC).

This construction is specialized for deriving 32-byte (256-bit) keys from
an initial 32-bytes of input key material.

[Documentation][docs-link]

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

[crate-image]: https://img.shields.io/crates/v/hkd32.svg
[crate-link]: https://crates.io/crates/hkd32
[docs-image]: https://docs.rs/hkd32/badge.svg
[docs-link]: https://docs.rs/hkd32/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.35+-blue.svg
[unsafe-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[unsafe-link]: https://internals.rust-lang.org/t/disabling-unsafe-by-default/7988
[build-image]: https://travis-ci.com/iqlusioninc/crates.svg?branch=develop
[build-link]: https://travis-ci.com/iqlusioninc/crates/
[gitter-image]: https://badges.gitter.im/iqlusioninc/community.svg
[gitter-link]: https://gitter.im/iqlusioninc/community

[//]: # (general links)

[bip32]: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
