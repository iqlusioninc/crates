# iqlusion crates 📦 <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Apache 2.0 Licensed][license-image]][license-link]
[![dependency status][deps-image]][deps-link]

This repository contains a set of Apache 2.0-licensed packages (a.k.a.  "crates")
for the [Rust](https://www.rust-lang.org/) programming language, contributed
to the community by [iqlusion](https://www.iqlusion.io).

If you are interested in contributing to this repository, please make sure to
read the [CONTRIBUTING.md] and [CODE_OF_CONDUCT.md] files first.

## Crates

This repository contains the following crates:

| Name              | Version                    | Build                      | Description                            |
|-------------------|----------------------------|----------------------------|----------------------------------------|
| [bip32]           | ![][bip32-crate]           | ![][bip32-build]           | Hierarchical key derivation            |
| [canonical‑path]  | ![][canonical-path-crate]  | ![][canonical-path-build]  | Canonical filesystem path support      |
| [hkd32]           | ![][hkd32-crate]           | ![][hkd32-build]           | HMAC-based Hierarchical Key Derivation |
| [iqhttp]          | ![][iqhttp-crate]          | ![][iqhttp-build]          | HTTP client built on hyper             |
| [secrecy]         | ![][secrecy-crate]         | ![][secrecy-build]         | Simple secret-keeping library          |
| [signatory]       | ![][signatory-crate]       | ![][signatory-build]       | Signature library with ECDSA+Ed25519   |
| [subtle‑encoding] | ![][subtle-encoding-crate] | ![][subtle-encoding-build] | Constant-time hex/bech32/base64        |

## License

Copyright © 2018-2025 iqlusion

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

[//]: # (links)

[CONTRIBUTING.md]: https://github.com/iqlusioninc/crates/blob/main/CONTRIBUTING.md
[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/crates/blob/main/CODE_OF_CONDUCT.md

[//]: # (badges)

[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/main/LICENSE
[deps-image]: https://deps.rs/repo/github/iqlusioninc/crates/status.svg
[deps-link]: https://deps.rs/repo/github/iqlusioninc/crates

[//]: # (crates)

[bip32]: https://github.com/iqlusioninc/crates/tree/main/bip32
[bip32-crate]: https://img.shields.io/crates/v/bip32.svg
[canonical‑path]: https://github.com/iqlusioninc/crates/tree/main/canonical-path
[canonical-path-crate]: https://img.shields.io/crates/v/canonical-path.svg
[hkd32]: https://github.com/iqlusioninc/crates/tree/main/hkd32
[hkd32-crate]: https://img.shields.io/crates/v/hkd32.svg
[iqhttp]: https://github.com/iqlusioninc/crates/tree/main/iqhttp
[iqhttp-crate]: https://img.shields.io/crates/v/iqhttp.svg
[secrecy]: https://github.com/iqlusioninc/crates/tree/main/secrecy
[secrecy-crate]: https://img.shields.io/crates/v/secrecy.svg
[signatory]: https://github.com/iqlusioninc/crates/tree/main/signatory
[signatory-crate]: https://img.shields.io/crates/v/signatory.svg
[subtle‑encoding]: https://github.com/iqlusioninc/crates/tree/main/subtle-encoding
[subtle-encoding-crate]: https://img.shields.io/crates/v/subtle-encoding.svg

[//]: # (build)

[bip32-build]: https://github.com/iqlusioninc/crates/actions/workflows/bip32.yml/badge.svg
[canonical-path-build]: https://github.com/iqlusioninc/crates/actions/workflows/canonical-path.yml/badge.svg
[hkd32-build]: https://github.com/iqlusioninc/crates/actions/workflows/hkd32.yml/badge.svg
[iqhttp-build]: https://github.com/iqlusioninc/crates/actions/workflows/iqhttp.yml/badge.svg
[secrecy-build]: https://github.com/iqlusioninc/crates/actions/workflows/secrecy.yml/badge.svg
[signatory-build]: https://github.com/iqlusioninc/crates/actions/workflows/signatory.yml/badge.svg
[subtle-encoding-build]: https://github.com/iqlusioninc/crates/actions/workflows/subtle-encoding.yml/badge.svg
[tai64-build]: https://github.com/iqlusioninc/crates/actions/workflows/tai64.yml/badge.svg
