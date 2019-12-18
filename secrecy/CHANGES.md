## [0.6.0] (2019-12-12)

- Impl `CloneableSecret` for `Secret<[T; N]>` where `T: Clone` ([#311])
- Impl `DebugSecret` for `[T; N]` where `N` <= 64 ([#310])
- Impl `FromStr` for `SecretString` ([#309])
- Upgrade to `bytes` v0.5 ([#301], [#308], [#312])

## [0.5.2] (2019-12-18)

- Backport Impl `FromStr` for `SecretString` ([#309])

## [0.5.1] (2019-11-30)

- Change default `DebugSecret` string to `[REDACTED]` ([#290])

## [0.5.0] (2019-10-13)

- Upgrade to `zeroize` v1.0.0 ([#279])

## [0.4.1] (2019-10-13)

- Upgrade to `zeroize` v1.0.0-pre ([#268])

## [0.4.0] (2019-09-03)

- Add `SerializableSecret` ([#262])
- Add (optional) concrete `SecretBytes` type ([#258], [#259], [#260], [#261])

## [0.3.1] (2019-08-26)

- Impl `CloneableSecret` for `String` ([#256])

## [0.3.0] (2019-08-20)

- Add support for `alloc` types ([#253])
- `zeroize` v0.10.0 ([#248])
- Add a default impl for `DebugSecret` trait ([#241])

## [0.2.2] (2019-06-28)

- README.md: add Gitter badges; update image links ([#221])

## [0.2.1] (2019-06-04)

- `zeroize` v0.9.0 ([#215])

## [0.2.0] (2019-05-29)

- Add `CloneableSecret` marker trait ([#210])

## 0.1.0 (2019-05-23)

- Initial release

[0.6.0]: https://github.com/iqlusioninc/crates/pull/313
[#312]: https://github.com/iqlusioninc/crates/pull/312
[#311]: https://github.com/iqlusioninc/crates/pull/311
[#310]: https://github.com/iqlusioninc/crates/pull/310
[#309]: https://github.com/iqlusioninc/crates/pull/309
[#308]: https://github.com/iqlusioninc/crates/pull/308
[#301]: https://github.com/iqlusioninc/crates/pull/301
[0.5.2]: https://github.com/iqlusioninc/crates/pull/316
[#309]: https://github.com/iqlusioninc/crates/pull/309
[0.5.1]: https://github.com/iqlusioninc/crates/pull/291
[#290]: https://github.com/iqlusioninc/crates/pull/290
[0.5.0]: https://github.com/iqlusioninc/crates/pull/282
[#279]: https://github.com/iqlusioninc/crates/pull/279
[0.4.1]: https://github.com/iqlusioninc/crates/pull/274
[#268]: https://github.com/iqlusioninc/crates/pull/268
[0.4.0]: https://github.com/iqlusioninc/crates/pull/263
[#262]: https://github.com/iqlusioninc/crates/pull/262
[#261]: https://github.com/iqlusioninc/crates/pull/261
[#260]: https://github.com/iqlusioninc/crates/pull/260
[#259]: https://github.com/iqlusioninc/crates/pull/259
[#258]: https://github.com/iqlusioninc/crates/pull/258
[0.3.1]: https://github.com/iqlusioninc/crates/pull/257
[#256]: https://github.com/iqlusioninc/crates/pull/256
[0.3.0]: https://github.com/iqlusioninc/crates/pull/254
[#253]: https://github.com/iqlusioninc/crates/pull/253
[#248]: https://github.com/iqlusioninc/crates/pull/248
[#241]: https://github.com/iqlusioninc/crates/pull/241
[0.2.2]: https://github.com/iqlusioninc/crates/pull/223
[#221]: https://github.com/iqlusioninc/crates/pull/221
[0.2.1]: https://github.com/iqlusioninc/crates/pull/216
[#215]: https://github.com/iqlusioninc/crates/pull/215
[0.2.0]: https://github.com/iqlusioninc/crates/pull/211
[#210]: https://github.com/iqlusioninc/crates/pull/210
