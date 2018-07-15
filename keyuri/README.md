# KeyURI

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]

[crate-image]: https://img.shields.io/crates/v/keyuri.svg
[crate-link]: https://crates.io/crates/keyuri
[docs-image]: https://docs.rs/keyuri/badge.svg
[docs-link]: https://docs.rs/keyuri/
[build-image]: https://circleci.com/gh/iqlusioninc/crates.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusioninc/crates
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/master/LICENSE

A format for serializing cryptographic keys based on the URI generic syntax.

KeyURI leverages the URI generic syntax defined in [RFC 3986] to provide simple
and succinct encodings of cryptographic keys, including public keys,
private/secret keys, encrypted secret keys with password-based key derivation,
digital signatures, key fingerprints, and other digests.

Binary data is serialized using the [Bech32] encoding format which helps
avoid human transcription errors. This format has been designed to use a
minimal set of alphanumeric characters which have been selected to avoid
accidental confusion, and also adds a checksum across the entire URI,
including the prefix. Keys which have been mis-transcribed will fail to
decode.

[RFC 3986]: https://tools.ietf.org/html/rfc3986
[Bech32]: https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki

## License

The **keyuri** crate is distributed under the terms of the Apache License
(Version 2.0).

See [LICENSE] file in the `iqlusioninc/crates` toplevel directory for more
information.

[LICENSE]: https://github.com/iqlusioninc/crates/blob/master/LICENSE
