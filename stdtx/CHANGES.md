# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.1 (2020-06-23)
### Changed
- Bump `prost-amino`/`prost-amino-derive` to v0.6 ([#451])

[#451]: https://github.com/iqlusioninc/crates/pull/451

## 0.2.0 (2020-06-18)
### Added
- `StdTx::new` method ([#446])
- Derive `Copy` on `Address` ([#445])
- `Msg::from_json_value` ([#443])
- Impl `serde::Deserialize` for `Address` ([#439])

### Changed
- Use serde for `StdFee` serialization ([#444])
- Have `stdtx::Builder` borrow signer ([#441])
- Update `ecdsa`, `sha2`, `signatory-secp256k1` dependencies  ([#435])
- Update `rust_decimal` to 1.6.0 ([#415])
- Update `anomaly` to 0.2.0 ([#354])

[#446]: https://github.com/iqlusioninc/crates/pull/446
[#445]: https://github.com/iqlusioninc/crates/pull/445
[#444]: https://github.com/iqlusioninc/crates/pull/444
[#443]: https://github.com/iqlusioninc/crates/pull/443
[#441]: https://github.com/iqlusioninc/crates/pull/441
[#439]: https://github.com/iqlusioninc/crates/pull/439
[#435]: https://github.com/iqlusioninc/crates/pull/435
[#415]: https://github.com/iqlusioninc/crates/pull/415
[#354]: https://github.com/iqlusioninc/crates/pull/354

## 0.1.0 (2020-01-27)
- Initial release
