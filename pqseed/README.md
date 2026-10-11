# Post-quantum seed derivation

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0/MIT Licensed][license-image]][license-link]
![MSRV][rustc-image]
[![Build Status][build-image]][build-link]

`pqseed` implements [Project Eleven's seed derivation scheme][scheme]: a
24-word BIP39 mnemonic expands to a BIP32 tree, BIP85 extracts deterministic
entropy, and that entropy generates post-quantum signature keys.

| Algorithm | Hardened BIP32 path | BIP85 output used |
|-----------|---------------------|-------------------|
| ML-DSA-44 | `m/83696968'/503131'/0'` | First 32 bytes |
| SLH-DSA-SHA2-128s | `m/83696968'/503131'/2'` | First 48 bytes |
| ML-DSA-65 (extension) | `m/83696968'/503131'/3'` | First 32 bytes |
| ML-DSA-87 (extension) | `m/83696968'/503131'/4'` | First 32 bytes |

For each path, BIP85 computes HMAC-SHA-512 with key `bip-entropy-from-k`
and the raw 32-byte child private key as the message. SLH-DSA receives
`SK.seed || SK.prf || PK.seed` in three consecutive 16-byte chunks. Algorithm
ID 1 is reserved by the article. IDs 3 and 4 are **`pqseed` extensions**, not
assignments from Project Eleven or a BIP standard. Interoperability for these
variants requires using these exact IDs; future external assignments may differ.
Each ML-DSA variant uses a separate path rather than reusing another variant's
seed. These article-compatible paths have no coin type, account, or index:
every chain using them shares one key per algorithm per mnemonic.

## Scoped derivation for chains (pqhd)

A chain adopting post-quantum keys needs every wallet implementation to
derive the same key from the same mnemonic, coin type, and scheme, without
sharing keys with other chains. The `*_for` methods implement [pqhd], a
standard and scheme registry for this. pqhd uses its own BIP85 application
number, `505153'` (the hex bytes of ASCII "PQS", following the convention
behind Project Eleven's `503131'` for "P11"), so it cannot collide with future
Project Eleven assignments.

```text
m / 83696968' / 505153' / coin_type' / scheme' / account' / index'
```

1. Expand the BIP39 mnemonic and passphrase to the 64-byte seed.
2. Derive the BIP32 (secp256k1) master key and follow the path above. Every
   level is hardened; each value must be below 2<sup>31</sup>.
3. Compute HMAC-SHA-512 with key `bip-entropy-from-k` over the raw 32-byte
   leaf private key, as in BIP85.
4. Read `L` bytes from BIP85-DRNG-SHAKE256 (SHAKE256 over the 64-byte HMAC
   output) and pass them to the scheme's key generation.

Unlike Project Eleven's paths, which keep the first `L` bytes of the HMAC
output, pqhd always uses the DRNG. This crate supports these [pqhd registry]
entries:

| ID | Scheme | `L` | Key generation |
|----|--------|-----|----------------|
| 0 | ML-DSA-44 | 32 | FIPS 204 `ML-DSA.KeyGen_internal(ξ)` |
| 2 | SLH-DSA-SHA2-128s | 48 | FIPS 205 `slh_keygen_internal(SK.seed, SK.prf, PK.seed)`, 16 bytes each, in that order |
| 3 | ML-DSA-65 | 32 | FIPS 204 `ML-DSA.KeyGen_internal(ξ)` |
| 4 | ML-DSA-87 | 32 | FIPS 204 `ML-DSA.KeyGen_internal(ξ)` |

`coin_type` is the chain's [SLIP-44] registered coin type, for example `133'`
for Zcash or `1'` for testnets. Coin type comes first, as in BIP44, so each
chain owns a subtree and picks its algorithm and account layout within it.
Non-hardened BIP44 levels such as `change` are not used: post-quantum keys
have no public derivation, and exposing a secp256k1 extended public key is
not quantum-safe. Watch-only wallets need exported public keys instead.

```rust
# #[cfg(feature = "ml-dsa")]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let seed = [0u8; 64];
use pqseed::{KeyDeriver, Scope, ml_dsa::Keypair};

let keys = KeyDeriver::from_seed(&seed);
// Zcash (SLIP-44 coin type 133), account 0, key 0.
let signing_key = keys.ml_dsa44_for(&Scope::new(133, 0, 0))?;
let verifying_key = signing_key.verifying_key();
# Ok(())
# }
# #[cfg(not(feature = "ml-dsa"))]
# fn main() {}
```

Scoped keys at `(coin_type, account, index)` are unrelated to the unscoped
article keys, including at `(0, 0, 0)`. [`tests/scoped.rs`] checks seeds and
public keys against the pqhd test vectors.

## Usage

```rust
# #[cfg(all(feature = "bip39", feature = "ml-dsa"))]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use pqseed::{KeyDeriver, ml_dsa::{Keypair, Signer, Verifier}};

// Public test mnemonic. Never use it to protect real assets.
let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon \
              abandon abandon abandon abandon abandon abandon abandon abandon \
              abandon abandon abandon abandon abandon abandon abandon art";
let keys = KeyDeriver::from_mnemonic(phrase, "")?;
let signing_key = keys.ml_dsa44()?;
let verifying_key = signing_key.verifying_key();
let signature = signing_key.sign(b"hello post-quantum world");
verifying_key.verify(b"hello post-quantum world", &signature)?;
# Ok(())
# }
# #[cfg(not(all(feature = "bip39", feature = "ml-dsa")))]
# fn main() {}
```

Use `keys.ml_dsa65()` or `keys.ml_dsa87()` for the other ML-DSA parameter sets,
with the same signing and verification traits. Use `keys.slh_dsa_sha2_128s()`
for SLH-DSA; its public key is available through
`pqseed::slh_dsa::signature::Keypair`. The RustCrypto crates are re-exported so
callers can use their native signing and verification APIs. All signing-key
types erase secret key material on drop.

`from_mnemonic` checks the English word list, checksum, and exactly 24 words.
It supports BIP39 passphrase normalization; an empty passphrase matches the
article. Any other passphrase produces different keys, with no way to detect
a mistyped passphrase. Generate new mnemonics using the `bip39` crate and a
cryptographically secure RNG, rather than choosing words yourself.

## Features and embedded use

This crate is `no_std`. Default features enable `bip39`, `ml-dsa`, and `slh-dsa`.
Each can be enabled independently:

- `bip39`: mnemonic parsing and Unicode NFKD normalization (requires `alloc`).
- `ml-dsa`: RustCrypto ML-DSA-44, ML-DSA-65, and ML-DSA-87 key generation.
- `slh-dsa`: RustCrypto SLH-DSA-SHA2-128s key generation. Currently uses a
  pinned prerelease and its low-level deterministic key-generation API.

Without default features, `KeyDeriver::from_seed` accepts an already expanded
64-byte BIP39 seed. `ml_dsa44_seed`, `ml_dsa65_seed`, `ml_dsa87_seed`, and
`slh_dsa_sha2_128s_seed` remain available without either signature backend or a
heap allocator. They return secret, zeroizing byte arrays suitable for other
compatible implementations.

## Security

This is a custom derivation scheme, **not an audited implementation** or a
new BIP standard. The article's audit does not cover this crate. The underlying
RustCrypto signature implementations also warn that they have not been audited.

The output signatures are post-quantum, but BIP32 still uses secp256k1. Do not
publish intermediate extended public keys: a sufficiently capable quantum
attacker can recover their corresponding private scalars. This crate exposes
no intermediate keys, but reusing the mnemonic in a classical wallet can expose
related material and requires its own security analysis. Do not infer a formal
post-quantum security guarantee for the whole construction from its signatures.

Use a securely generated 24-word mnemonic (256 bits of entropy). The raw seed
constructor cannot establish its provenance. Losing the mnemonic or BIP39
passphrase loses all keys; exposing both compromises all derived keys.

Stored BIP39 seeds, copied leaf scalars, BIP85 digests, and returned seeds are
erased after use or on drop. This is best-effort memory hygiene, not a guarantee
that all compiler temporaries or dependency internals are erased. Callers own
their input strings and raw seed copies and must protect and erase them.
Returned seed buffers expose their bytes, including through `Debug`; never
log them. `KeyDeriver` itself has a redacted `Debug` implementation.

## Minimum Supported Rust Version

- Rust **1.85**

## License

Copyright © 2026 iqlusion

**pqseed** is distributed under the terms of either the MIT license or the
Apache License (Version 2.0), at your option. See [LICENSE-APACHE] and
[LICENSE-MIT] for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be dual licensed as above,
without any additional terms or conditions.

[scheme]: https://www.projecteleven.com/blog/generating-post-quantum-keypairs-from-a-single-24word-seed-phrase
[crate-image]: https://img.shields.io/crates/v/pqseed.svg?logo=rust
[crate-link]: https://crates.io/crates/pqseed
[docs-image]: https://docs.rs/pqseed/badge.svg
[docs-link]: https://docs.rs/pqseed/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/main/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.85+-blue.svg
[build-image]: https://github.com/iqlusioninc/crates/actions/workflows/pqseed.yml/badge.svg
[build-link]: https://github.com/iqlusioninc/crates/actions/workflows/pqseed.yml
[LICENSE-APACHE]: https://github.com/iqlusioninc/crates/blob/main/pqseed/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/iqlusioninc/crates/blob/main/pqseed/LICENSE-MIT
[SLIP-44]: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
[pqhd]: https://github.com/pqdh/pqhd
[pqhd registry]: https://github.com/pqdh/pqhd/blob/main/registry/README.md
[`tests/scoped.rs`]: https://github.com/iqlusioninc/crates/blob/main/pqseed/tests/scoped.rs
