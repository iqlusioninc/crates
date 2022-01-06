# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.6.0 (2022-01-06)
### Changed
- Rust 2021 edition upgrade ([#889])
- Bump `k256` dependency to v0.10 ([#938])

[#889]: https://github.com/iqlusioninc/crates/pull/889
[#938]: https://github.com/iqlusioninc/crates/pull/938

## 0.5.0 (2021-06-23)
### Changed
- MSRV 1.51+ ([#755])
- Bump `k256` to v0.9 ([#759])
- `rand_core` to v0.6 ([#759])

[#755]: https://github.com/iqlusioninc/crates/pull/755
[#759]: https://github.com/iqlusioninc/crates/pull/759

## 0.4.0 (2020-12-21)
### Changed
- MSRV 1.46+ ([#588])
- Bump `ecdsa` dependency to v0.10 ([#588])
- Bump `k256` dependency to v0.7 ([#588])
- Replace `anomaly` with `eyre` ([#555])

[#588]: https://github.com/iqlusioninc/crates/pull/588
[#555]: https://github.com/iqlusioninc/crates/pull/555

## 0.3.0 (2020-10-12)
### Changed
- MSRV 1.44+ ([#515])
- Replace `signatory-secp256k1` with `k256` ([#514])

[#515]: https://github.com/iqlusioninc/crates/pull/515
[#514]: https://github.com/iqlusioninc/crates/pull/514

## 0.2.4 (2020-10-06)
### Fixed
- Hex-encode `bytes` fields ([#521])

[#521]: https://github.com/iqlusioninc/crates/pull/521

## 0.2.3 (2020-10-05)
### Added
- `msg::Builder::bytes` method ([#520])

[#520]: https://github.com/iqlusioninc/crates/pull/520

## 0.2.2 (2020-10-05)
### Added
- `bytes` fields ([#519])

[#519]: https://github.com/iqlusioninc/crates/pull/519

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
