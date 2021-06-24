# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0 (2021-06-23)
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
