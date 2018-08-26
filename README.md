# iqlusion crates <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-prod-web-assets/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a> <a href="https://crates.io">📦</a>

[![Apache 2.0 Licensed][license-image]][license-link]
[![Build Status][build-image]][build-link]
[![Appveyor Status][appveyor-image]][appveyor-link]

[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/master/LICENSE
[build-image]: https://circleci.com/gh/iqlusioninc/crates.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusioninc/crates
[appveyor-image]: https://ci.appveyor.com/api/projects/status/1ua33q2njho24e9h?svg=true
[appveyor-link]: https://ci.appveyor.com/project/tony-iqlusion/crates

This repository contains a set of Apache 2.0-licensed packages (a.k.a.  "crates")
for the [Rust](https://www.rust-lang.org/) programming language, contributed
to the community by [iqlusion](https://www.iqlusion.io).

If you are interested in contributing to this repository, please make sure to
read the [CONTRIBUTING.md] and [CODE_OF_CONDUCT.md] files first.

[CONTRIBUTING.md]: https://github.com/iqlusioninc/crates/blob/master/CONTRIBUTING.md
[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/crates/blob/master/CODE_OF_CONDUCT.md

## Crate Descriptions

This repository contains the following crates:

* [canonical-path:](https://github.com/iqlusioninc/crates/tree/master/canonical-path)
  `Path` and `PathBuf`-like types for representing canonical filesystem paths.
* [iq-bech32:](https://github.com/iqlusioninc/crates/tree/master/iq-bech32)
  Bech32 (BIP-173) human-friendly base32 encoding for binary data intended for
  use with cryptographic keys.
* [keyuri:](https://github.com/iqlusioninc/crates/tree/master/keyuri)
  URI-like serialization format for cryptographic keys.
* [tai64:](https://github.com/iqlusioninc/crates/tree/master/tai64)
  TAI64(N) timestamp format (Temps Atomique International)

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
