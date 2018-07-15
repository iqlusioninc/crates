# iq-bech32

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]

[crate-image]: https://img.shields.io/crates/v/iq-bech32.svg
[crate-link]: https://crates.io/crates/iq-bech32
[docs-image]: https://docs.rs/iq-bech32/badge.svg
[docs-link]: https://docs.rs/iq-bech32/
[build-image]: https://circleci.com/gh/iqlusioninc/crates.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusioninc/crates
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/master/LICENSE

Rust crate for encoding/decoding Bech32 ([BIP-173]), a human-friendly base32
encoding for binary data intended for use with cryptographic keys.

This format has been designed to use a minimal set of alphanumeric characters
which have been selected to avoid accidental confusion, and also adds a checksum
across the encoded message, ensuring transcription errors are detected.

This crate in particular is designed with the following goals, which are aligned
with the general purpose of using this format for cryptographic secret keys:

- [X] Zero out (using [clear_on_drop]) all intermediate buffers and state
- [X] Support for Bech32 separator characters other than `1`
- [ ] TODO: Constant-time encoding/decoding to prevent side-channel leakage

[BIP-173]: https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki
[clear_on_drop]: https://github.com/cesarb/clear_on_drop

## License

The **iq-bech32** crate is distributed under the terms of the Apache License
(Version 2.0).

See [LICENSE] file in the `iqlusioninc/crates` toplevel directory for more
information.

[LICENSE]: https://github.com/iqlusioninc/crates/blob/master/LICENSE
