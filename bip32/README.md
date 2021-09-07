# BIP32: HD Wallets

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]
[![Safety Dance][safety-image]][safety-link]
[![Build Status][build-image]][build-link]

BIP32 hierarchical key derivation implemented in a generic, `no_std`-friendly
manner. Supports deriving keys using the pure Rust `k256` crate or the
C library-backed `secp256k1` crate.

![Diagram](https://raw.githubusercontent.com/bitcoin/bips/4bc05ff903cb47eb18ce58a9836de1ac13ecf1b7/bip-0032/derivation.png)

[Documentation][docs-link]

## About

BIP32 is an algorithm for generating a hierarchy of elliptic curve keys,
a.k.a. "wallets", from a single seed value. A related algorithm also
implemented by this crate, BIP39, provides a way to derive the seed value
from a set of 24-words from a preset list, a.k.a. a "mnemonic".

## Minimum Supported Rust Version

Rust **1.51** or newer.

In the future, we reserve the right to change MSRV (i.e. MSRV is out-of-scope
for this crate's SemVer guarantees), however when we do it will be accompanied by
a minor version bump.

## License

Copyright © 2020-2021 iqlusion

**bip32.rs** is distributed under the terms of either the MIT license
or the Apache License (Version 2.0), at your option.

See [LICENSE-APACHE] (Apache License, Version 2.0) and [LICENSE-MIT] for
further details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be dual licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/bip32.svg
[crate-link]: https://crates.io/crates/bip32
[docs-image]: https://docs.rs/bip32/badge.svg
[docs-link]: https://docs.rs/bip32/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/main/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.51+-blue.svg
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[build-image]: https://github.com/iqlusioninc/crates/actions/workflows/bip32.yml/badge.svg
[build-link]: https://github.com/iqlusioninc/crates/actions/workflows/bip32.yml

[//]: # (links)

[bip32]: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
[libsecp256k1 C library]: https://github.com/bitcoin-core/secp256k1
[`secp256k1` Rust crate]: https://github.com/rust-bitcoin/rust-secp256k1/
[LICENSE-APACHE]: https://github.com/iqlusioninc/crates/blob/main/bip32/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/iqlusioninc/crates/blob/main/bip32/LICENSE-MIT
