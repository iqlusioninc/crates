## [0.5.1] (2020-02-08)

- Remove Bech32 max length restriction ([#357])

## [0.5.0] (2019-10-13)

- Upgrade to `zeroize` v1.0.0 ([#279])

## [0.4.1] (2019-10-07)

- Upgrade to `zeroize` v1.0.0-pre ([#268])

## [0.4.0] (2019-08-19)

- Remove `failure` ([#245])
- Use `alloc` for heap allocations; MSRV 1.36+ ([#245])

## [0.3.7] (2019-05-19)

- `zeroize` v0.9.0 ([#215])

## [0.3.6] (2019-05-19)

- `zeroize` v0.8.0 ([#189])

## [0.3.5] (2019-05-19)

- `zeroize` v0.7.0 ([#186])

## [0.3.4] (2019-03-23)

- `zeroize` v0.6.0 ([#170])
- Make internals of Bech32 type private ([#165])

## [0.3.3] (2019-03-12)

- Return errors for undersize decode buffers and trailing whitespace ([#163])

## [0.3.2] (2018-12-25)

- Fix `#![no_std]` support ([#158])

## [0.3.1] (2018-12-25)

- Update to zeroize 0.5 ([#149])

## [0.3.0] (2018-11-25)

- Fix critical encode/decode bug in `release` builds ([#126])
- Non-constant-time Bech32 implementation via `bech32-preview` feature ([#113])

## 0.2.3 (2018-10-12)

- Bump zeroize dependency to 0.4

## 0.2.2 (2018-10-11)

- Bump zeroize dependency to 0.3

## 0.2.1 (2018-10-09)

- Re-export `IDENTITY` from the crate root

## 0.2.0 (2018-10-08)

- hex: support for encoding/decoding upper case

## 0.1.1 (2018-10-05)

- Fix build when using `--no-default-features`

## 0.1.0 (2018-10-03)

- Initial release

[0.5.1]: https://github.com/iqlusioninc/crates/pull/358
[#357]: https://github.com/iqlusioninc/crates/pull/357
[0.5.0]: https://github.com/iqlusioninc/crates/pull/283
[#279]: https://github.com/iqlusioninc/crates/pull/279
[0.4.1]: https://github.com/iqlusioninc/crates/pull/269
[#268]: https://github.com/iqlusioninc/crates/pull/268
[0.4.0]: https://github.com/iqlusioninc/crates/pull/249
[#215]: https://github.com/iqlusioninc/crates/pull/245
[0.3.7]: https://github.com/iqlusioninc/crates/pull/218
[#215]: https://github.com/iqlusioninc/crates/pull/215
[0.3.5]: https://github.com/iqlusioninc/crates/pull/187
[#186]: https://github.com/iqlusioninc/crates/pull/186
[0.3.4]: https://github.com/iqlusioninc/crates/pull/171
[#170]: https://github.com/iqlusioninc/crates/pull/170
[#165]: https://github.com/iqlusioninc/crates/pull/165
[0.3.3]: https://github.com/iqlusioninc/crates/pull/164
[#163]: https://github.com/iqlusioninc/crates/pull/163
[0.3.2]: https://github.com/iqlusioninc/crates/pull/160
[#158]: https://github.com/iqlusioninc/crates/pull/158
[0.3.1]: https://github.com/iqlusioninc/crates/pull/155
[#149]: https://github.com/iqlusioninc/crates/pull/149
[0.3.0]: https://github.com/iqlusioninc/crates/pull/129
[#126]: https://github.com/iqlusioninc/crates/pull/126
[#113]: https://github.com/iqlusioninc/crates/pull/113
