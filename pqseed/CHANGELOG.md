# Changelog

## 0.1.0 (Unreleased)

- Initial implementation of Project Eleven's BIP39/BIP32/BIP85 derivation
  scheme for ML-DSA-44 and SLH-DSA-SHA2-128s.
- Extend the scheme with ML-DSA-65 and ML-DSA-87 at crate-specific algorithm
  IDs 3 and 4, respectively.
- Add [pqhd] scoped derivation at
  `m/83696968'/505153'/coin_type'/scheme'/account'/index'`, keyed by SLIP-44
  coin type and expanded with BIP85-DRNG-SHAKE256.
- Pin pqhd conformance to the draft specification in PR #1 and the registry
  in PR #2 for schemes 0, 2, 3, and 4; preserve unscoped outputs separately.
- Support every SLH-DSA parameter set for scoped derivation (pqhd schemes
  5–15, per PR #2) through `SlhDsa`, `slh_dsa_seed_for`, and
  `slh_dsa_for::<P>`.

[pqhd]: https://github.com/pqdh/pqhd/blob/b9674b302c0f320ee7f0b39317eb48ddbe67f00f/spec/README.md
