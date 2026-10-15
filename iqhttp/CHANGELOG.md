# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.0 (unreleased)
### Changed
- Upgrade to `hyper` v1, `hyper-util`, and `hyper-rustls` v0.27 with `rustls`
  v0.23.46+, fixing RUSTSEC-2026-0349 ([#1406])
- TLS uses the `ring` crypto provider explicitly and is HTTPS-only
- `HttpsClient::new` is now fallible (loading native root certificates)
- `HttpsClient::request` takes `Request<Full<Bytes>>` and responses are
  `Response<Incoming>`; these are re-exported from the new `body` module
- `proxy` feature uses `hyper-util`'s HTTP CONNECT tunnel instead of
  `hyper-proxy`

### Added
- `Error::{Client, Io}` variants

### Removed
- `Error::Proxy` variant

[#1406]: https://github.com/iqlusioninc/crates/issues/1406

## 0.1.1 (2022-04-23)
### Added
- `HttpsClient::{add_header, headers_mut}` methods ([#981])

### Changed
- Rust 2021 edition upgrade ([#889])

[#889]: https://github.com/iqlusioninc/crates/pull/889
[#981]: https://github.com/iqlusioninc/crates/pull/981

## 0.1.0 (2021-05-10)
- Initial release
