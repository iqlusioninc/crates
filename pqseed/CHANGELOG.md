# Changelog

## 0.1.0 (Unreleased)

- Initial implementation of Project Eleven's BIP39/BIP32/BIP85 derivation
  scheme for ML-DSA-44 and SLH-DSA-SHA2-128s.
- Extend the scheme with ML-DSA-65 and ML-DSA-87 at crate-specific algorithm
  IDs 3 and 4, respectively.
- Add scoped derivation at `m/83696968'/505153'/coin_type'/algorithm'/account'/index'`
  under a `pqseed`-owned BIP85 application number, keyed by SLIP-44 coin type.
