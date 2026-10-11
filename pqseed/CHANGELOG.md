# Changelog

## 0.1.0 (Unreleased)

- Initial implementation of Project Eleven's BIP39/BIP32/BIP85 derivation
  scheme for ML-DSA-44 and SLH-DSA-SHA2-128s.
- Extend the scheme with ML-DSA-65 and ML-DSA-87 at crate-specific algorithm
  IDs 3 and 4, respectively.
- Add [pqhd] scoped derivation at
  `m/83696968'/505153'/coin_type'/scheme'/account'/index'`, keyed by SLIP-44
  coin type and expanded with BIP85-DRNG-SHAKE256.

[pqhd]: https://github.com/pqdh/pqhd
