# stdtx.rs 🌌 <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][msrv-image]
[![Gitter Chat][gitter-image]][gitter-link]

Extensible schema-driven [Cosmos] [StdTx] builder and [Amino] serializer.

[Documentation][docs-link]

## About

**stdtx.rs** is a Rust library for composing transactions in the [StdTx]
format used by several [Tendermint]-based networks.

It includes support for cryptographically signing transactions and serializing
them in the [Amino] encoding format.

Definitions of transaction types are easily extensible, and can be defined at
runtime by loading them from a TOML definition file. This allows
**stdtx.rs** to be used with any [Tendermint]-based software which
uses the [StdTx] format without requiring upstream modifications.

## Minimum Supported Rust Version

- Rust **1.39+**

## License

Copyright © 2020 Tony Arcieri

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

[crate-image]: https://img.shields.io/crates/v/stdtx.svg
[crate-link]: https://crates.io/crates/stdtx
[docs-image]: https://docs.rs/stdtx/badge.svg
[docs-link]: https://docs.rs/stdtx/
[build-image]: https://github.com/iqlusioninc/crates/workflows/Rust/badge.svg?branch=develop&event=push
[build-link]: https://github.com/iqlusioninc/crates/actions
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/develop/LICENSE
[msrv-image]: https://img.shields.io/badge/rustc-1.39+-blue.svg
[gitter-image]: https://badges.gitter.im/iqlusioninc/community.svg
[gitter-link]: https://gitter.im/iqlusioninc/community

[//]: # (general links)

[Cosmos]: https://cosmos.network/
[StdTx]: https://godoc.org/github.com/cosmos/cosmos-sdk/x/auth/types#StdTx
[Tendermint]: https://tendermint.com/
[Amino]: https://github.com/tendermint/go-amino
