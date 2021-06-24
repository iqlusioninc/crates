## 0.22.0 (2020-10-12)

- Remove `PublicKeyed` trait ([#64])
- Remove `signatory-dalek` references ([#62])
- MSRV 1.44+ ([#57])
- Bump `ecdsa` dependency to v0.8 ([#57])
- Bump `k256` dependency to v0.5 ([#57])
- Bump `p256` dependency to v0.5 ([#57])
- Bump `p384` dependency to v0.4 ([#57])
- Bump `secp256k1` dependency to v0.19 ([#57])

[#64]: https://github.com/iqlusioninc/signatory/pull/64
[#62]: https://github.com/iqlusioninc/signatory/pull/62
[#57]: https://github.com/iqlusioninc/signatory/pull/57

## 0.21.0 (2020-08-12)

- Bump `ecdsa` dependency to v0.7 ([#42])
- signatory-dalek: deprecate in favor of `ed25519-dalek` ([#40])

[#42]: https://github.com/iqlusioninc/signatory/pull/42
[#40]: https://github.com/iqlusioninc/signatory/pull/40

## 0.20.0 (2020-06-10)

- Bump `ecdsa`, `sha2`, `sha3`, and `signature`; MSRV 1.41+ ([#30])
- signatory-dalek: remove Ed25519ph (DigestSigner/DigestVerifier) ([#29])

[#30]: https://github.com/iqlusioninc/signatory/pull/30
[#29]: https://github.com/iqlusioninc/signatory/pull/29

## 0.19.0 (2020-04-19)

- Update `signature` crate requirement to v1.0.1 ([#14])
- Update `subtle-encoding` requirement to 0.5 ([#2])

[#14]: https://github.com/iqlusioninc/signatory/pull/14
[#2]: https://github.com/iqlusioninc/signatory/pull/2

## 0.18.1 (2020-03-02)

- Update links to point to new repository location
- signatory-secp256k1: update `secp256k1` requirement from 0.15 to 0.17
- signatory-secp256k1: support Ethereum's `Keccak256` hash function for signing

## 0.18.0 (2020-01-19)

- Upgrade `ecdsa` crate to v0.4

## 0.17.1 (2019-12-16)

- Fix Windows builds

## 0.17.0 (2019-12-11)

- Use `=` expression to lock all prerelease deps to specific versions
- Upgrade to `secp256k1` crate v0.17
- Upgrade to `ecdsa` crate v0.3

## 0.16.0 (2019-10-29)

- Use the `ecdsa` crate
- Upgrade to zeroize 1.0
- Upgrade to `signature` and `ed25519` crates v1.0.0-pre.1

## 0.15.0 (2019-10-11)

- signatory-dalek: Upgrade `ed25519-dalek` to 1.0.0-pre.2
- Upgrade to `signature` and `ed25519` crates v1.0.0-pre.0

## 0.14.0 (2019-10-10)

- Always use the `alloc` crate for `String`/`Vec`
- Upgrade to `signature` crate v0.3, `ed25519` crate v0.2; MSRV 1.36+

## 0.13.0 (2019-08-11)

- Update ring to v0.16; secp256k1 to v0.15
- Remove toplevel `signature` re-exports; add `encoding::Error`
- Use `ed25519::Signature` from the `ed25519` crate

## 0.12.0 (2019-06-07)

- Use the `signature` crate

## 0.11.5 (2019-06-04)

- Upgrade to `zeroize` 0.9

## 0.11.4 (2019-05-20)

- Support stable `alloc` API
- Upgrade to `zeroize` 0.8

## 0.11.3 (2019-03-13)

- Fix Missing TrailingWhitespace type-case in subtle-encoding error conversion

## 0.11.2 (2019-03-09)

- ecdsa: impl `PartialOrd` + `Ord` for PublicKeys
- ecdsa: Simplify trait bounds for Copy impl on curve point types

## 0.11.1 (2019-02-23)

- ecdsa: impl `Copy` + `Hash` for ECDSA curve points and public keys

## 0.11.0 (2019-02-12)

- signatory-yubihsm: Update `yubihsm` crate to v0.20
- signatory-dalek: Update `ed25519-dalek` crate to 1.0.0-pre.1
- signatory-ring: Update `ring` crate to 0.14
- signatory-sodiumoxide: Update `sodiumoxide` crate to 0.2
- signatory-secp256k1: Update `secp256k1` crate to 0.12
- Upgrade to Rust 2018 edition
- signatory-ledger-cosval: Upgrade ledger provider to validator app 0.2.1

## 0.10.1 (2018-11-27)

- Upgrade to `subtle-encoding` v0.3.0

## 0.10.0 (2018-10-16)

- Upgrade to `digest` 0.8, `generic-array` 0.12, and `yubihsm` 0.19
- Upgrade to `zeroize` 0.4

## 0.9.4 (2018-10-10)

- pkcs8: Properly gate `FILE_MODE` on Windows

## 0.9.3 (2018-10-09)

- Upgrade to `subtle-encoding` v0.2
- Fix unused import on Windows (closes #121)

## 0.9.2 (2018-10-08)

- More documentation fixups

## 0.9.1 (2018-10-08)

- Cargo.toml: Fix docs.rs build

## 0.9.0 (2018-10-08)

- Remove redundant "namespacing" from type names
- Move `curve` module (back) under `ecdsa`
- signatory-yubihsm: Upgrade to yubihsm 0.18
- Use `subtle-encoding` crate for constant-time encoding/decoding
- ECDSA `SecretKey` type and related traits (e.g. `GeneratePkcs8`)
- Properly handle leading zeroes in ASN.1 serialization/parsing
- signatory-yubihsm: Expose the yubihsm crate as a pub extern
- encoding: Use 0o600 file mode on Unix
- Eliminate `ed25519::FromSeed` trait
- yubihsm: NIST P-384 support
- ring: NIST P-384 support
- Add NIST P-384 elliptic curve type (closes #73)
- signatory-yubihsm: Fix ECDSA over secp256k1 signing (closes #87)
- `signatory-ledger-cosval` provider
- signatory-yubihsm: Normalize secp256k1 signatures to "low S" form
- signatory-secp256k1: Bump secp256k1 crate dependency to 0.11
- Unify verification API under the `Verifier` trait
- encoding: Add encoding module with hex and Base64 support
- Unify signing API under the `Signer` trait

## 0.8.0 (2018-08-19)

- Extract `from_pkcs8` into a trait
- signatory-yubihsm: Make ecdsa and ed25519 modules public

## 0.7.0 (2018-08-19)

- Factor providers into their own `signatory-*` crates
- Unify ECDSA traits across DER and fixed-sized signatures
- ECDSA DER signature parsing and serialization

## 0.6.1 (2018-07-31)

- Upgrade to `secp256k1` crate v0.10

## 0.6.0 (2018-07-31)

- Factor ECDSA PublicKey into compressed/uncompressed curve points
- ECDSA support for `yubihsm-provider`
- Upgrade to `yubihsm` crate 0.14
- Add rustdoc logo
- Audit project for security vulnerabilities with `cargo-audit`
- Update to `ed25519-dalek` 0.8
- Add ECDSA NIST P-256 support with *ring* provider
- Factor ECDSA traits apart into separate traits per method
- Upgrade to `sodiumoxide` 0.1
- Add `ed25519::Seed::from_keypair` method
- No default features
- Add `ed25519::Seed` type

## 0.5.2 (2018-05-19)

- Update to `yubihsm-rs` 0.9
- Fix benchmarks

## 0.5.1 (2018-04-13)

- Mark all Signers and Verifiers as Send safe

## 0.5.0 (2018-04-12)

- Upgrade to `yubihsm-rs` 0.8
- ECDSA verification support
- ECDSA support with secp256k1 provider
- Ed25519 FromSeed trait and miscellaneous cleanups
- Remove unnecessary direct dependency on `curve25519-dalek`

## 0.4.1 (2018-04-05)

- Add more bounds to the `Verifier` trait

## 0.4.0 (2018-04-05)

- Add an `ed25519` module to all providers
- `sodiumoxide` provider for Ed25519
- *ring* Ed25519 provider
- `ed25519::Verifier` trait

## 0.3.2 (2018-03-31)

- Upgrade `ed25519-dalek` to 0.6.2

## 0.3.1 (2018-03-27)

- Update to `yubihsm-rs` 0.7

## 0.3.0 (2018-03-20)

- Refactor providers + `yubihsm-rs` update + `Sync`-safe signers

## 0.2.0 (2018-03-13)

- Add `ed25519::Signer::public_key()`

## 0.1.0 (2018-03-12)

- Initial release
