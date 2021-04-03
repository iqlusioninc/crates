# iqlusion crates 📦 <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][msrv-image]
[![Build Status][build-image]][build-link]
[![Gitter Chat][gitter-image]][gitter-link]

This repository contains a set of Apache 2.0-licensed packages (a.k.a.  "crates")
for the [Rust](https://www.rust-lang.org/) programming language, contributed
to the community by [iqlusion](https://www.iqlusion.io).

If you are interested in contributing to this repository, please make sure to
read the [CONTRIBUTING.md] and [CODE_OF_CONDUCT.md] files first.

[CONTRIBUTING.md]: https://github.com/iqlusioninc/crates/blob/main/CONTRIBUTING.md
[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/crates/blob/main/CODE_OF_CONDUCT.md

## Requirements

All crates require Rust 2018 edition and are tested on the following channels:

- `1.44.0` (minimum supported)
- `stable`

Crates may work on slightly earlier 2018 edition-supporting versions of Rust
(i.e. 1.31.0+) but are not tested on these releases or guaranteed to work.

All crates in CI with the above channels on the following operating systems:

- Linux
- macOS
- Windows

## Crates

This repository contains the following crates:

| Name              | Version                    | Description                                   |
|-------------------|----------------------------|-----------------------------------------------|
| [anomaly]         | ![][anomaly-crate]         | Error context library with sources/backtraces |
| [canonical-path]  | ![][canonical-path-crate]  | Canonical filesystem path support             |
| [harp]            | ![][harp-crate]            | Minimalist HTTP library                       |
| [hkd32]           | ![][hkd32-crate]           | HMAC-based Hierarchical Key Derivation        |
| [secrecy]         | ![][secrecy-crate]         | Simple secret-keeping library                 |
| [stdtx]           | ![][stdtx-crate]           | Cosmos StdTx builder/signer/serializer        |
| [subtle-encoding] | ![][subtle-encoding-crate] | Hex, Bech32, and Base64 in constant-time(ish) |
| [tai64]           | ![][tai64-crate]           | TAI64(N) timestamp format                     |
| [zeroize]         | ![][zeroize-crate]         | Securely zero memory                          |

## License

Copyright © 2018-2019 iqlusion

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
submitted for inclusion in the work by you shall be licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/main/LICENSE
[msrv-image]: https://img.shields.io/badge/rustc-1.44+-blue.svg
[build-image]: https://github.com/iqlusioninc/crates/workflows/Rust/badge.svg?branch=main&event=push
[build-link]: https://github.com/iqlusioninc/crates/actions
[gitter-image]: https://badges.gitter.im/iqlusioninc/community.svg
[gitter-link]: https://gitter.im/iqlusioninc/community

[//]: # (crates)

[anomaly]: https://github.com/iqlusioninc/crates/tree/main/anomaly
[anomaly-crate]: https://img.shields.io/crates/v/anomaly.svg
[canonical-path]: https://github.com/iqlusioninc/crates/tree/main/canonical-path
[canonical-path-crate]: https://img.shields.io/crates/v/canonical-path.svg
[harp]: https://github.com/iqlusioninc/crates/tree/main/harp
[harp-crate]: https://img.shields.io/crates/v/harp.svg
[hkd32]: https://github.com/iqlusioninc/crates/tree/main/hkd32
[hkd32-crate]: https://img.shields.io/crates/v/hkd32.svg
[secrecy]: https://github.com/iqlusioninc/crates/tree/main/secrecy
[secrecy-crate]: https://img.shields.io/crates/v/secrecy.svg
[stdtx]: https://github.com/iqlusioninc/crates/tree/main/stdtx
[stdtx-crate]: https://img.shields.io/crates/v/stdtx.svg
[subtle-encoding]: https://github.com/iqlusioninc/crates/tree/main/subtle-encoding
[subtle-encoding-crate]: https://img.shields.io/crates/v/subtle-encoding.svg
[tai64]: https://github.com/iqlusioninc/crates/tree/main/tai64
[tai64-crate]: https://img.shields.io/crates/v/tai64.svg
[zeroize]: https://github.com/iqlusioninc/crates/tree/main/zeroize
[zeroize-crate]: https://img.shields.io/crates/v/zeroize.svg
