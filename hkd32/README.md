# HMAC-based Hierarchical Key Derivation <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]
[![Build Status][build-image]][build-link]

`hkd32` is a Rust library which implements a hierarchical deterministic
symmetric key derivation construction inspired by
[BIP-0032: Hierarchical Deterministic Wallets][bip32].

It can be used to deterministically derive a hierarchy of symmetric keys
from initial keying material (or when the `mnemonic` feature is enabled,
through a 24-word [BIP39] passphrase) by repeatedly applying the
Hash-based Message Authentication Code (HMAC).

This construction is specialized for deriving 32-byte (256-bit) keys from
an initial 32-bytes of input key material.

[Documentation][docs-link]

## Minimum Supported Rust Version

- Rust **1.47**

## License

Copyright © 2019-2021 iqlusion

Includes code from the `bip39` crate. Copyright © 2017-2018 Stephen Oliver,
with contributions by Maciej Hirsz.

**hkd32** is distributed under the terms of either the MIT license
or the Apache License (Version 2.0), at your option.

See [LICENSE] (Apache License, Version 2.0) file in the `iqlusioninc/crates`
toplevel directory of this repository or [LICENSE-MIT] for details.

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
[license-link]: https://github.com/iqlusioninc/crates/blob/main/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.47+-blue.svg
[build-image]: https://github.com/iqlusioninc/crates/actions/workflows/hkd32.yml/badge.svg
[build-link]: https://github.com/iqlusioninc/crates/actions/workflows/hkd32.yml

[//]: # (general links)

[bip32]: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
[bip39]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
[LICENSE]: https://github.com/iqlusioninc/crates/blob/main/LICENSE
[LICENSE-MIT]: https://github.com/iqlusioninc/crates/blob/main/hkd32/LICENSE-MIT
