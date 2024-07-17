# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.5.2 (2023-07-17)
### Added
- `PrivateKey::derive_tweak()` and `PublicKey::derive_tweak()` ([#1186])

[#1186]: https://github.com/iqlusioninc/crates/pull/1186

## 0.5.1 (2023-05-29)
### Added
- `ExtendedPublicKey::new` ([#1136])

### Changed
- Bump `bs58` to v0.5 ([#1139])

[#1136]: https://github.com/iqlusioninc/crates/pull/1136
[#1139]: https://github.com/iqlusioninc/crates/pull/1139

## 0.5.0 (2023-03-28)
### Added
- Support for private `ExtendedKey` conversion to `ExtendedPublicKey` ([#1021])

### Changed
- Upgrade elliptic curve crates; MSRV 1.65 ([#1105])
  - `ecdsa` v0.16
  - `ed25519-dalek` v2.0.0-pre.0
  - `k256` v0.13
  - `p256` v0.13
  - `p384` v0.13
- Bump `secp256k1` crate dependency to v0.27 ([#1115])

[#1021]: https://github.com/iqlusioninc/crates/pull/1021
[#1105]: https://github.com/iqlusioninc/crates/pull/1105
[#1115]: https://github.com/iqlusioninc/crates/pull/1115

## 0.4.0 (2022-05-10)
### Changed
- Bump `pbkdf2` to 0.11.0 ([#983])
- Bump `hmac` to v0.12 ([#994])
- Bump `k256` to v0.11 ([#994])
- Bump `p256` to v0.11 ([#994])
- Bump `sha2` to v0.10 ([#994])
- Replace `ripemd160` dependency with `ripemd` ([#994])
- MSRV 1.57 ([#994], [#995])
- Use const panic for `Prefix::from_parts_unchecked` ([#995])

[#983]: https://github.com/iqlusioninc/crates/pull/983
[#994]: https://github.com/iqlusioninc/crates/pull/994
[#995]: https://github.com/iqlusioninc/crates/pull/995

## 0.3.0 (2022-01-05)
### Changed
- Rust 2021 edition upgrade ([#889])
- Decouple from `hkd32` ([#907])
- Bump `k256` dependency to v0.10 ([#938])
- Bump `secp256k1` (FFI) dependency to v0.21 ([#942])

[#889]: https://github.com/iqlusioninc/crates/pull/889
[#907]: https://github.com/iqlusioninc/crates/pull/907
[#938]: https://github.com/iqlusioninc/crates/pull/938
[#942]: https://github.com/iqlusioninc/crates/pull/942

## 0.2.2 (2021-09-07)
### Changed
- Avoid `AsRef` ambiguity with `generic-array` ([#859])

[#859]: https://github.com/iqlusioninc/crates/pull/859

## 0.2.1 (2021-06-23)
### Added
- `From` conversions to `k256::ecdsa::*Key` ([#777])

[#777]: https://github.com/iqlusioninc/crates/pull/777

## 0.2.0 (2021-06-23) [YANKED]
### Added
- Non-hardened derivation support with `XPub::derive_child` ([#772])

### Changed
- Rename `XPrv::derive_child_from_seed` => `XPrv::derive_from_path` ([#773])

[#772]: https://github.com/iqlusioninc/crates/pull/772
[#773]: https://github.com/iqlusioninc/crates/pull/773

## 0.1.1 (2021-06-18)
### Added
- Documentation improvements and usage example ([#764])

[#764]: https://github.com/iqlusioninc/crates/pull/764

## 0.1.0 (2021-06-17)
- Initial release
