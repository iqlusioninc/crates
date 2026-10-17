# Post-quantum seed derivation

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0/MIT Licensed][license-image]][license-link]
![MSRV][rustc-image]
[![Build Status][build-image]][build-link]

`pqseed` derives post-quantum signature keys from 24-word BIP39 mnemonics.
Scoped `*_for` methods implement the [pqhd draft specification][pqhd spec]
and its [scheme registry][pqhd registry]. Unscoped ML-DSA-44 methods retain
ID 0 from [Project Eleven's seed derivation scheme][scheme]. These are
distinct derivations and produce different keys.

**pqhd is draft v0.1.0, not a finalized standard. Do not protect real assets
with keys derived under this draft.** This implementation targets the
specification and registry merged into pqhd `main` at the [pinned revision].
The application number `505153'` is not yet registered with BIP85, and the
draft may change before v1.0.

**Breaking draft update:** scoped methods now use the family-block registry:
ML-DSA IDs 200–202 and SLH-DSA IDs 300–311. All scoped keys differ from the
earlier draft using IDs 0 and 2–15; those IDs are not aliases or deprecated
recovery entries in the current registry. Recovering earlier draft keys
requires the earlier implementation. Unscoped ML-DSA-44 keys are unchanged;
legacy unscoped IDs 2–4 have been removed, and ID 1 was never supported.

## Unscoped Project Eleven derivation

| Algorithm | Hardened BIP32 path | BIP85 output used |
|-----------|---------------------|-------------------|
| ML-DSA-44 | `m/83696968'/503131'/0'` | First 32 bytes |

BIP85 computes HMAC-SHA-512 with key `bip-entropy-from-k` and the raw 32-byte
child private key as the message. This article-compatible path has no coin
type, account, or index: every chain using it shares one ML-DSA-44 key per
mnemonic. All other parameter sets are available only through scoped pqhd
derivation; do not reuse an ML-DSA-44 seed for another scheme.

## Scoped derivation for chains (pqhd)

A chain adopting post-quantum keys needs every wallet implementation to
derive the same key from the same mnemonic, coin type, and scheme, without
sharing keys with other chains. The `*_for` methods implement the [pqhd draft
specification][pqhd spec]. Its proposed BIP85 application number is `505153'`
(the hex bytes of ASCII "PQS", following the convention behind Project Eleven's
`503131'` for "P11"). This separates pqhd paths from the Project Eleven subtree.

```text
m / 83696968' / 505153' / coin_type' / scheme' / account' / index'
```

1. NFKD-normalize the mnemonic, split on whitespace, validate the English
   BIP39 wordlist, checksum, and exactly 24 words, then join words with single
   ASCII spaces. NFKD-normalize the passphrase without trimming or collapsing
   its whitespace. Expand with BIP39 PBKDF2-HMAC-SHA512 (2048 iterations) to
   the 64-byte seed.
2. Derive the BIP32 (secp256k1) master key and follow the path above. Every
   level is hardened; each value must be below 2<sup>31</sup>. Invalid master
   or child keys fail derivation; indices are never silently incremented.
3. Compute HMAC-SHA-512 with key `bip-entropy-from-k` over the raw 32-byte
   leaf private key, as in BIP85.
4. Read `L` bytes from BIP85-DRNG-SHAKE256 (SHAKE256 over the 64-byte HMAC
   output) and pass them to the scheme's key generation.

Unlike the Project Eleven path, which keeps the first 32 bytes of the HMAC
output, pqhd always uses the DRNG. This crate supports these [pqhd registry]
entries:

| ID | Scheme | `L` | Key generation |
|----|--------|-----|----------------|
| 200 | ML-DSA-44 | 32 | FIPS 204 `ML-DSA.KeyGen_internal(ξ)` |
| 201 | ML-DSA-65 | 32 | FIPS 204 `ML-DSA.KeyGen_internal(ξ)` |
| 202 | ML-DSA-87 | 32 | FIPS 204 `ML-DSA.KeyGen_internal(ξ)` |
| 300–305 | SLH-DSA-SHA2: 128s, 128f, 192s, 192f, 256s, 256f, respectively | 48, 72, or 96 | FIPS 205 `slh_keygen_internal(SK.seed, SK.prf, PK.seed)`, `n` bytes each, in that order |
| 306–311 | SLH-DSA-SHAKE: 128s, 128f, 192s, 192f, 256s, 256f, respectively | 48, 72, or 96 | FIPS 205 `slh_keygen_internal(SK.seed, SK.prf, PK.seed)`, `n` bytes each, in that order |

These entries are `active` in the draft registry. Each family owns a block
of 100 indices; block 0 is never assigned. ML-KEM (IDs 100–102) is not
implemented by this crate. Reserved FN-DSA entries 400 and 401 are not
derivable. The registry requires deprecated entries to remain derivable for
recovery, but currently none are deprecated. The API exposes only the
supported schemes, not arbitrary numeric scheme IDs: [`SlhDsa`] lists every
SLH-DSA parameter set with its scheme index.

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
public keys against all 36 [pqhd test vectors] for schemes 200–202 and 300;
[`tests/scoped_slh_dsa.rs`] checks three vectors per SLH-DSA set 301–311, one for
each seed. Use `ml_dsa65_for`, `ml_dsa87_for`, `slh_dsa_sha2_128s_for`, or
`slh_dsa_for::<P>` (for example `P = pqseed::slh_dsa::Shake256f`) for the other
scoped keys. The corresponding `*_seed_for` methods, including
`slh_dsa_seed_for`, return the registry's key-generation inputs without
requiring a signature backend.

## Restoring keys from a mnemonic

Neither a public key nor an address records which derivation produced it, so
a wallet restoring from a mnemonic must try each derivation it supports and
match the results against keys known on chain:

1. For each pqhd scheme the chain uses, scan scoped keys at the chain's
   `coin_type`, incrementing `index` (and then `account`) up to a gap limit, as
   BIP44 wallets do. Each scheme has its own path, so a match identifies the
   scheme.
2. Check the unscoped ML-DSA-44 key at `m/83696968'/503131'/0'` once. It has
   no coin type, account, or index, so there is exactly one per mnemonic.

The BIP39 passphrase is part of the input: a wrong passphrase derives valid
but different keys rather than failing. Keys derived with earlier draft
versions of this crate (scoped IDs 0 and 2–15, or unscoped IDs 2–4) cannot be
derived by this version.

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

Use `keys.ml_dsa65_for(&scope)` or `keys.ml_dsa87_for(&scope)` for the other
ML-DSA parameter sets, with the same signing and verification traits. Use
`keys.slh_dsa_sha2_128s_for(&scope)` for SLH-DSA; its public key is available through
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
- `slh-dsa`: RustCrypto key generation for all SLH-DSA parameter sets. Uses a
  pinned prerelease and its low-level deterministic key-generation API.

Without default features, `KeyDeriver::from_seed` accepts an already expanded
64-byte BIP39 seed. `ml_dsa44_seed` and all scoped `*_seed_for` methods remain
available without either signature backend or a heap allocator. They return
secret, zeroizing seed buffers suitable for other compatible implementations.

## Security

This is a custom derivation scheme, **not an audited implementation** or a
new BIP standard. The article's audit does not cover this crate. The underlying
RustCrypto signature implementations also warn that they have not been audited.

The output signatures are post-quantum, but BIP32 still uses secp256k1. Do not
export intermediate extended public or private keys. A sufficiently capable
quantum attacker can recover private scalars from extended public keys. This
crate exposes no intermediate keys, but reusing the mnemonic in a classical
wallet can expose related material and requires its own security analysis.
In particular, exposing an xpub at or above `m/83696968'` exposes pqhd keys to
a quantum attacker. Watch-only wallets must use the derived PQ public keys.
Do not infer a formal post-quantum security guarantee for the whole
construction from its signatures.

Use a securely generated 24-word mnemonic (256 bits of entropy). The raw seed
constructor cannot establish its provenance. Losing the mnemonic or BIP39
passphrase loses all keys; exposing both compromises all derived keys.
All derived keys are bounded by that entropy and by the 256-bit BIP32 scalars:
roughly 128 bits against Grover's algorithm, even for NIST category 3 or 5
schemes. A key derived for one scheme must not be reused for another scheme.

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
[pinned revision]: https://github.com/pqdh/pqhd/commit/befdce14cecbb5d485d3239982bfa5b207cdb922
[pqhd spec]: https://github.com/pqdh/pqhd/blob/befdce14cecbb5d485d3239982bfa5b207cdb922/spec/README.md
[pqhd registry]: https://github.com/pqdh/pqhd/blob/befdce14cecbb5d485d3239982bfa5b207cdb922/registry/README.md
[pqhd test vectors]: https://github.com/pqdh/pqhd/tree/befdce14cecbb5d485d3239982bfa5b207cdb922/test-vectors
[`tests/scoped.rs`]: https://github.com/iqlusioninc/crates/blob/main/pqseed/tests/scoped.rs
[`tests/scoped_slh_dsa.rs`]: https://github.com/iqlusioninc/crates/blob/main/pqseed/tests/scoped_slh_dsa.rs
[`SlhDsa`]: https://docs.rs/pqseed/latest/pqseed/enum.SlhDsa.html
