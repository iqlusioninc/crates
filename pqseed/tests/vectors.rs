//! Cross-implementation vectors, not official Project Eleven test vectors.
//!
//! Generated independently with @scure/bip39 2.0.1, @scure/bip32 2.0.1,
//! @noble/hashes 2.0.1, and @noble/post-quantum 0.6.1. Mnemonic entropy is
//! 32 zero bytes for the first vector and bytes 0x00..0x1f for the others.
//! For each mnemonic/passphrase: mnemonicToSeedSync -> HDKey.fromMasterSeed
//! -> derive("m/83696968'/503131'/<id>'") -> HMAC-SHA512 with ASCII key
//! "bip-entropy-from-k" and raw child.privateKey -> first 32/48 bytes ->
//! ml_dsa44.keygen / slh_dsa_sha2_128s.keygen. Key hashes are SHA-256 of
//! the raw FIPS publicKey/secretKey returned by Noble (not DER or seed bytes).

use hex_literal::hex;
use pqseed::KeyDeriver;
#[cfg(any(feature = "ml-dsa", feature = "slh-dsa"))]
use sha2::{Digest, Sha256};

struct Vector {
    seed: [u8; 64],
    ml: [u8; 32],
    slh: [u8; 48],
    #[cfg(feature = "bip39")]
    phrase: &'static str,
    #[cfg(feature = "bip39")]
    passphrase: &'static str,
    #[cfg(feature = "ml-dsa")]
    ml_pk_hash: [u8; 32],
    #[cfg(feature = "slh-dsa")]
    slh_pk_hash: [u8; 32],
    #[cfg(feature = "slh-dsa")]
    slh_sk_hash: [u8; 32],
}

const VECTORS: &[Vector] = &[
    Vector {
        seed: hex!(
            "408b285c123836004f4b8842c89324c1f01382450c0d439af345ba7fc49acf70
                   5489c6fc77dbd4e3dc1dd8cc6bc9f043db8ada1e243c4a0eafb290d399480840"
        ),
        ml: hex!("e727e39395418f1af9ba73362ea790007d9f95512e8b28ace7f447240ccc313c"),
        slh: hex!(
            "7e76dfee07ba669b21ab0afc238896714463b45273b3d8ef8ebbba678814c454
                  f51157f51515d2df8b111290b776d662"
        ),
        #[cfg(feature = "bip39")]
        phrase: "abandon abandon abandon abandon abandon abandon abandon abandon \
                 abandon abandon abandon abandon abandon abandon abandon abandon \
                 abandon abandon abandon abandon abandon abandon abandon art",
        #[cfg(feature = "bip39")]
        passphrase: "",
        #[cfg(feature = "ml-dsa")]
        ml_pk_hash: hex!("611f4c7f6f45ed7cb1bd0d82b67232cee6627da1e57345fdc8a6f5e57f8cb158"),
        #[cfg(feature = "slh-dsa")]
        slh_pk_hash: hex!("0e69c10af69b0b8025b6e8fa34eadbf5b0cce847edbb32941aaed4d4d0c2c348"),
        #[cfg(feature = "slh-dsa")]
        slh_sk_hash: hex!("ecf13826a31217fa65761b7ea3d589d08aeb5227dfd2acaa59f74b8ff6be359e"),
    },
    Vector {
        seed: hex!(
            "e00fc08b7fa18c78eb9a943c2ce3903eb52d2590be5aadfe83b39465ac66c3a6
                   490a134828a61d311d62974b4ce0708d9a714c529f3e45b296640a9297c6f383"
        ),
        ml: hex!("8e14d7887d6cfed1e99f27150998d17b7253520ad69f58f446facc74aa783410"),
        slh: hex!(
            "7c63cca5e2c5278eda0c67bab12697595b63fcf7f7756e864e6646d1b03c5daa0
                  734eb901bf368f9f268177113427803"
        ),
        #[cfg(feature = "bip39")]
        phrase: "abandon amount liar amount expire adjust cage candy arch gather drum bullet \
                 absurd math era live bid rhythm alien crouch range attend journey unaware",
        #[cfg(feature = "bip39")]
        passphrase: "TREZOR",
        #[cfg(feature = "ml-dsa")]
        ml_pk_hash: hex!("9df6140c3d7c7a77162a26fca08a109bb5503d27926b288f24bffc4d6d9e4766"),
        #[cfg(feature = "slh-dsa")]
        slh_pk_hash: hex!("164c6b6828f37866965c40522ff2ecbe6c8d328cb066aae5fa7ff699428da920"),
        #[cfg(feature = "slh-dsa")]
        slh_sk_hash: hex!("cedaa94c149530c04b09e9c297ce080f956f6dce9707a8ffbf9901de99d2f200"),
    },
    Vector {
        seed: hex!(
            "de56ff0e9536e7185f43246276662657ad42281537ff332cb9e493897a477636
                   7be422b8fae0dcd752e3cd3d9ac5a533cbbe07ef83a24af90c8d0cb5d4bc4713"
        ),
        ml: hex!("d1264488c41b74c1f2bf0c207ac23c54afe721a6ee98d9deaeb9ea435e264807"),
        slh: hex!(
            "a85bcf5a6bfd71b314eef2c0f1b5f496626c5bd416c10f11109db415f30720166
                  5a2002cf10d10bb3c09aff069e358c8"
        ),
        #[cfg(feature = "bip39")]
        phrase: "abandon amount liar amount expire adjust cage candy arch gather drum bullet \
                 absurd math era live bid rhythm alien crouch range attend journey unaware",
        #[cfg(feature = "bip39")]
        passphrase: "café",
        #[cfg(feature = "ml-dsa")]
        ml_pk_hash: hex!("82a2e1758cb9de021936ba70712b3602e8a55a00df75b1b923d3e351991a127f"),
        #[cfg(feature = "slh-dsa")]
        slh_pk_hash: hex!("2bd13bfda8847c8d547b75f21c478321a2109fbfc00f8cf47c57c020d02288ea"),
        #[cfg(feature = "slh-dsa")]
        slh_sk_hash: hex!("c616b453c54dbd4c7d3f9f9d114d6975d460b552db090dcdb9e23b584f2d8d30"),
    },
];

#[test]
fn derived_seed_vectors() {
    for vector in VECTORS {
        let keys = KeyDeriver::from_seed(&vector.seed);
        assert_eq!(*keys.ml_dsa44_seed().unwrap(), vector.ml);
        assert_eq!(*keys.slh_dsa_sha2_128s_seed().unwrap(), vector.slh);
    }
}

#[test]
#[cfg(feature = "bip39")]
fn mnemonic_vectors_and_normalization() {
    for vector in VECTORS {
        let keys = KeyDeriver::from_mnemonic(vector.phrase, vector.passphrase).unwrap();
        assert_eq!(*keys.ml_dsa44_seed().unwrap(), vector.ml);
        assert_eq!(*keys.slh_dsa_sha2_128s_seed().unwrap(), vector.slh);
    }

    let vector = &VECTORS[2];
    let spaced = format!("  {}\n", vector.phrase.replace(' ', " \t "));
    let keys = KeyDeriver::from_mnemonic(&spaced, "cafe\u{301}").unwrap();
    assert_eq!(*keys.ml_dsa44_seed().unwrap(), vector.ml);
    assert_eq!(*keys.slh_dsa_sha2_128s_seed().unwrap(), vector.slh);
}

#[test]
#[cfg(feature = "bip39")]
fn reject_invalid_mnemonics() {
    use pqseed::Error;

    // Valid BIP39, but insufficient entropy for the article's 24-word scheme.
    let short = "abandon abandon abandon abandon abandon abandon abandon abandon \
                 abandon abandon abandon about";
    assert!(matches!(
        KeyDeriver::from_mnemonic(short, ""),
        Err(Error::WordCount)
    ));

    // All valid words, exactly 24, but an invalid checksum.
    let checksum = ["abandon"; 24].join(" ");
    assert!(matches!(
        KeyDeriver::from_mnemonic(&checksum, ""),
        Err(Error::Bip39(bip39::Error::InvalidChecksum))
    ));

    let unknown = VECTORS[0].phrase.replace("art", "notaword");
    assert!(matches!(
        KeyDeriver::from_mnemonic(&unknown, ""),
        Err(Error::Bip39(bip39::Error::UnknownWord(_)))
    ));
    assert!(KeyDeriver::from_mnemonic("", "").is_err());
}

#[test]
#[cfg(feature = "ml-dsa")]
fn ml_dsa44_key_vectors_and_signatures() {
    use pqseed::ml_dsa::{Keypair, Signer, Verifier};

    for vector in VECTORS {
        let key = KeyDeriver::from_seed(&vector.seed).ml_dsa44().unwrap();
        let public = key.verifying_key();
        assert_eq!(
            Sha256::digest(public.encode()).as_slice(),
            vector.ml_pk_hash
        );
        let signature = key.sign(b"post-quantum key derivation");
        assert!(
            public
                .verify(b"post-quantum key derivation", &signature)
                .is_ok()
        );
        assert!(public.verify(b"modified message", &signature).is_err());
    }
}

#[test]
#[cfg(feature = "slh-dsa")]
fn slh_dsa_key_vectors_and_signatures() {
    use pqseed::slh_dsa::signature::{Keypair, Signer, Verifier};

    for vector in VECTORS {
        let key = KeyDeriver::from_seed(&vector.seed)
            .slh_dsa_sha2_128s()
            .unwrap();
        let public = key.verifying_key();
        assert_eq!(
            Sha256::digest(public.to_bytes()).as_slice(),
            vector.slh_pk_hash
        );
        // The public key does not depend on SK.prf: check the secret key too.
        assert_eq!(
            Sha256::digest(key.to_bytes()).as_slice(),
            vector.slh_sk_hash
        );
        let signature = key.sign(b"post-quantum key derivation");
        assert!(
            public
                .verify(b"post-quantum key derivation", &signature)
                .is_ok()
        );
        assert!(public.verify(b"modified message", &signature).is_err());
    }
}

#[test]
fn debug_is_redacted() {
    let keys = KeyDeriver::from_seed(&VECTORS[0].seed);
    assert_eq!(format!("{keys:?}"), "KeyDeriver { .. }");
}
