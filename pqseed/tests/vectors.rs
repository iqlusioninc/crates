//! Cross-implementation vectors, not official Project Eleven test vectors.
//!
//! Generated independently with @scure/bip39 2.0.1, @scure/bip32 2.0.1,
//! @noble/hashes 2.0.1, and @noble/post-quantum 0.6.1. Mnemonic entropy is
//! 32 zero bytes for the first vector and bytes 0x00..0x1f for the others.
//! For each mnemonic/passphrase: mnemonicToSeedSync -> HDKey.fromMasterSeed
//! -> derive("m/83696968'/503131'/0'") -> HMAC-SHA512 with ASCII key
//! "bip-entropy-from-k" and raw child.privateKey -> first 32 bytes ->
//! ml_dsa44.keygen. Key hashes are SHA-256 of the raw FIPS publicKey
//! returned by Noble (not DER or seed bytes).

use hex_literal::hex;
use pqseed::KeyDeriver;
#[cfg(feature = "ml-dsa")]
use sha2::{Digest, Sha256};

struct Vector {
    seed: [u8; 64],
    ml: [u8; 32],
    #[cfg(feature = "bip39")]
    phrase: &'static str,
    #[cfg(feature = "bip39")]
    passphrase: &'static str,
    #[cfg(feature = "ml-dsa")]
    ml_pk_hash: [u8; 32],
}

const VECTORS: &[Vector] = &[
    Vector {
        seed: hex!(
            "408b285c123836004f4b8842c89324c1f01382450c0d439af345ba7fc49acf70
                   5489c6fc77dbd4e3dc1dd8cc6bc9f043db8ada1e243c4a0eafb290d399480840"
        ),
        ml: hex!("e727e39395418f1af9ba73362ea790007d9f95512e8b28ace7f447240ccc313c"),
        #[cfg(feature = "bip39")]
        phrase: "abandon abandon abandon abandon abandon abandon abandon abandon \
                 abandon abandon abandon abandon abandon abandon abandon abandon \
                 abandon abandon abandon abandon abandon abandon abandon art",
        #[cfg(feature = "bip39")]
        passphrase: "",
        #[cfg(feature = "ml-dsa")]
        ml_pk_hash: hex!("611f4c7f6f45ed7cb1bd0d82b67232cee6627da1e57345fdc8a6f5e57f8cb158"),
    },
    Vector {
        seed: hex!(
            "e00fc08b7fa18c78eb9a943c2ce3903eb52d2590be5aadfe83b39465ac66c3a6
                   490a134828a61d311d62974b4ce0708d9a714c529f3e45b296640a9297c6f383"
        ),
        ml: hex!("8e14d7887d6cfed1e99f27150998d17b7253520ad69f58f446facc74aa783410"),
        #[cfg(feature = "bip39")]
        phrase: "abandon amount liar amount expire adjust cage candy arch gather drum bullet \
                 absurd math era live bid rhythm alien crouch range attend journey unaware",
        #[cfg(feature = "bip39")]
        passphrase: "TREZOR",
        #[cfg(feature = "ml-dsa")]
        ml_pk_hash: hex!("9df6140c3d7c7a77162a26fca08a109bb5503d27926b288f24bffc4d6d9e4766"),
    },
    Vector {
        seed: hex!(
            "de56ff0e9536e7185f43246276662657ad42281537ff332cb9e493897a477636
                   7be422b8fae0dcd752e3cd3d9ac5a533cbbe07ef83a24af90c8d0cb5d4bc4713"
        ),
        ml: hex!("d1264488c41b74c1f2bf0c207ac23c54afe721a6ee98d9deaeb9ea435e264807"),
        #[cfg(feature = "bip39")]
        phrase: "abandon amount liar amount expire adjust cage candy arch gather drum bullet \
                 absurd math era live bid rhythm alien crouch range attend journey unaware",
        #[cfg(feature = "bip39")]
        passphrase: "café",
        #[cfg(feature = "ml-dsa")]
        ml_pk_hash: hex!("82a2e1758cb9de021936ba70712b3602e8a55a00df75b1b923d3e351991a127f"),
    },
];

#[test]
fn derived_seed_vectors() {
    for vector in VECTORS {
        let keys = KeyDeriver::from_seed(&vector.seed);
        assert_eq!(*keys.ml_dsa44_seed().unwrap(), vector.ml);
    }
}

#[test]
#[cfg(feature = "bip39")]
fn mnemonic_vectors_and_normalization() {
    for vector in VECTORS {
        let keys = KeyDeriver::from_mnemonic(vector.phrase, vector.passphrase).unwrap();
        assert_eq!(*keys.ml_dsa44_seed().unwrap(), vector.ml);
    }

    let vector = &VECTORS[2];
    let spaced = format!("  {}\n", vector.phrase.replace(' ', " \t "));
    let keys = KeyDeriver::from_mnemonic(&spaced, "cafe\u{301}").unwrap();
    assert_eq!(*keys.ml_dsa44_seed().unwrap(), vector.ml);
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
fn debug_is_redacted() {
    let keys = KeyDeriver::from_seed(&VECTORS[0].seed);
    assert_eq!(format!("{keys:?}"), "KeyDeriver { .. }");
}
