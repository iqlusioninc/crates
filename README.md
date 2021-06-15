# iqlusion crates 📦 <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][msrv-image]

This repository contains a set of Apache 2.0-licensed packages (a.k.a.  "crates")
for the [Rust](https://www.rust-lang.org/) programming language, contributed
to the community by [iqlusion](https://www.iqlusion.io).

If you are interested in contributing to this repository, please make sure to
read the [CONTRIBUTING.md] and [CODE_OF_CONDUCT.md] files first.

## Crates

This repository contains the following crates:

| Name              | Version                    | Build                      | Description                            |
|-------------------|----------------------------|----------------------------|----------------------------------------|
| [anomaly]         | ![][anomaly-crate]         | ![][anomaly-build]         | Error context library                  |
| [bip32]           | ![][bip32-crate]           | ![][bip32-build]           | Hierarchical key derivation            |
| [canonical‑path]  | ![][canonical-path-crate]  | ![][canonical-path-build]  | Canonical filesystem path support      |
| [datadog]         | ![][datadog-crate]         | ![][datadog-build]         | Datadog client library                 |
| [harp]            | ![][harp-crate]            | ![][harp-build]            | Minimalist HTTP library                |
| [hkd32]           | ![][hkd32-crate]           | ![][hkd32-build]           | HMAC-based Hierarchical Key Derivation |
| [secrecy]         | ![][secrecy-crate]         | ![][secrecy-build]         | Simple secret-keeping library          |
| [stdtx]           | ![][stdtx-crate]           | ![][stdtx-build]           | Cosmos StdTx builder/signer/serializer |
| [subtle‑encoding] | ![][subtle-encoding-crate] | ![][subtle-encoding-build] | Constant-time hex/bech32/base64        |
| [tai64]           | ![][tai64-crate]           | ![][tai64-build]           | TAI64(N) timestamp format              |
| [zeroize]         | ![][zeroize-crate]         | ![][zeroize-build]         | Securely zero memory                   |

## License

Copyright © 2018-2021 iqlusion

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
[msrv-image]: https://img.shields.io/badge/rustc-1.51+-blue.svg

[//]: # (crates)

[anomaly]: https://github.com/iqlusioninc/crates/tree/main/anomaly
[anomaly-crate]: https://img.shields.io/crates/v/anomaly.svg
[bip32]: https://github.com/iqlusioninc/crates/tree/main/bip32
[bip32-crate]: https://img.shields.io/crates/v/bip32.svg
[canonical‑path]: https://github.com/iqlusioninc/crates/tree/main/canonical-path
[canonical-path-crate]: https://img.shields.io/crates/v/canonical-path.svg
[datadog]: https://github.com/iqlusioninc/crates/tree/main/datadog
[datadog-crate]: https://img.shields.io/crates/v/datadog.svg
[harp]: https://github.com/iqlusioninc/crates/tree/main/harp
[harp-crate]: https://img.shields.io/crates/v/harp.svg
[hkd32]: https://github.com/iqlusioninc/crates/tree/main/hkd32
[hkd32-crate]: https://img.shields.io/crates/v/hkd32.svg
[secrecy]: https://github.com/iqlusioninc/crates/tree/main/secrecy
[secrecy-crate]: https://img.shields.io/crates/v/secrecy.svg
[stdtx]: https://github.com/iqlusioninc/crates/tree/main/stdtx
[stdtx-crate]: https://img.shields.io/crates/v/stdtx.svg
[subtle‑encoding]: https://github.com/iqlusioninc/crates/tree/main/subtle-encoding
[subtle-encoding-crate]: https://img.shields.io/crates/v/subtle-encoding.svg
[tai64]: https://github.com/iqlusioninc/crates/tree/main/tai64
[tai64-crate]: https://img.shields.io/crates/v/tai64.svg
[zeroize]: https://github.com/iqlusioninc/crates/tree/main/zeroize
[zeroize-crate]: https://img.shields.io/crates/v/zeroize.svg

[//]: # (build)

[anomaly-build]: https://github.com/iqlusioninc/crates/actions/workflows/anomaly.yml/badge.svg
[bip32-build]: https://github.com/iqlusioninc/crates/actions/workflows/bip32.yml/badge.svg
[canonical-path-build]: https://github.com/iqlusioninc/crates/actions/workflows/canonical-path.yml/badge.svg
[datadog-build]: https://github.com/iqlusioninc/crates/actions/workflows/datadog.yml/badge.svg
[harp-build]: https://github.com/iqlusioninc/crates/actions/workflows/harp.yml/badge.svg
[hkd32-build]: https://github.com/iqlusioninc/crates/actions/workflows/hkd32.yml/badge.svg
[secrecy-build]: https://github.com/iqlusioninc/crates/actions/workflows/secrecy.yml/badge.svg
[stdtx-build]: https://github.com/iqlusioninc/crates/actions/workflows/stdtx.yml/badge.svg
[subtle-encoding-build]: https://github.com/iqlusioninc/crates/actions/workflows/subtle-encoding.yml/badge.svg
[tai64-build]: https://github.com/iqlusioninc/crates/actions/workflows/tai64.yml/badge.svg
[zeroize-build]: https://github.com/iqlusioninc/crates/actions/workflows/zeroize.yml/badge.svg
