# Changelog

## 0.1.0 (Unreleased)

- Initial implementation of Project Eleven's BIP39/BIP32/BIP85 derivation
  scheme for ML-DSA-44 at unscoped ID 0.
- Remove legacy unscoped IDs 2–4 and their key/seed APIs. ID 1 was never
  supported. ML-DSA-65/87 and SLH-DSA remain available through scoped pqhd APIs.
- Add [pqhd] scoped derivation at
  `m/83696968'/505153'/coin_type'/scheme'/account'/index'`, keyed by SLIP-44
  coin type and expanded with BIP85-DRNG-SHAKE256.
- Support every SLH-DSA parameter set for scoped derivation through `SlhDsa`,
  `slh_dsa_seed_for`, and `slh_dsa_for::<P>`.
- **Breaking draft update:** align with the merged pqhd family-block registry:
  ML-DSA uses schemes 200–202 and SLH-DSA uses 300–311. All scoped outputs
  change from the earlier draft IDs 0 and 2–15; unscoped ML-DSA-44
  output remains unchanged. Pin documentation and refreshed vectors to the
  merged revision, including mnemonic/passphrase whitespace semantics.

[pqhd]: https://github.com/pqdh/pqhd/blob/befdce14cecbb5d485d3239982bfa5b207cdb922/spec/README.md
