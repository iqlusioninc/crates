# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.7.0 (2022-05-10)
### Added
- Impl for `std::error::Error` ([#932])

### Changed
- Rust 2021 edition upgrade ([#889])
- Bump `pbkdf2` to 0.11.0 ([#983])
- Bump `hmac` to v0.12 ([#994])
- Bump `sha2` to v0.10 ([#994])

[#889]: https://github.com/iqlusioninc/crates/pull/889
[#932]: https://github.com/iqlusioninc/crates/pull/932
[#983]: https://github.com/iqlusioninc/crates/pull/983
[#994]: https://github.com/iqlusioninc/crates/pull/994

## 0.6.0 (2021-06-17)
### Added
- `Seed::new` method ([#729])

### Changed
- `hmac` to v0.11 ([#704])
- `pbkdf2` to v0.8 ([#704])
- Rename `SEED_SIZE` to `Seed::SIZE` ([#728])
- MSRV 1.56 ([#755])
- Bump `rand_core` to v0.6 ([#759])

### Removed
- `Seed::derive_subkey` ([#748])
- `BIP39_BASE_DERIVATION_KEY` constant ([#748])

[#704]: https://github.com/iqlusioninc/crates/pull/704
[#728]: https://github.com/iqlusioninc/crates/pull/728
[#729]: https://github.com/iqlusioninc/crates/pull/729
[#748]: https://github.com/iqlusioninc/crates/pull/748
[#755]: https://github.com/iqlusioninc/crates/pull/755
[#759]: https://github.com/iqlusioninc/crates/pull/759

## 0.5.0 (2020-10-19)
- Replace `getrandom` with `rand_core` ([#540])
- Replace `lazy_static` with `once_cell` ([#539])
- Bump `hmac` to v0.10; pbkdf2 to v0.6 ([#538])
- MSRV 1.44+ ([#515])

[#540]: https://github.com/iqlusioninc/crates/pull/540
[#539]: https://github.com/iqlusioninc/crates/pull/539
[#538]: https://github.com/iqlusioninc/crates/pull/538
[#515]: https://github.com/iqlusioninc/crates/pull/515

## 0.4.0 (2020-06-17)
- Update `hmac` to v0.8 ([#435])
- Update `pbkdf2` to v0.4 ([#435])
- Update `sha2` to v0.9 ([#435])

[#435]: https://github.com/iqlusioninc/crates/pull/435

## 0.3.1 (2019-10-13)
- Upgrade to `subtle-encoding` v0.5.0 ([#283])

[#283]: https://github.com/iqlusioninc/crates/pull/283

## 0.3.0 (2019-10-13)
- Split out `bip39` cargo feature ([#280])
- Upgrade to `zeroize` v1.0.0 ([#279])

[#280]: https://github.com/iqlusioninc/crates/pull/280
[#279]: https://github.com/iqlusioninc/crates/pull/279

## 0.2.0 (2019-08-20)
- Vendor (simplified) BIP39 implementation from `tiny-bip39` ([#251])
- `subtle-encoding` v0.4.0 ([#249])
- `zeroize` v0.10.0 ([#248])

[#251]: https://github.com/iqlusioninc/crates/pull/251
[#249]: https://github.com/iqlusioninc/crates/pull/249
[#248]: https://github.com/iqlusioninc/crates/pull/248

## 0.1.2 (2019-07-23)
- Fix docs typo ([#235])

[#235]: https://github.com/iqlusioninc/crates/pull/235

## 0.1.1 (2019-07-21)
- Fix docs typo ([#232])

[#232]: https://github.com/iqlusioninc/crates/pull/232

## 0.1.0 (2019-07-21)
- Initial release
