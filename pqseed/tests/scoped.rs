//! Scoped derivation vectors for the `pqseed` coin-type namespace.
//!
//! Generated independently with a Python standard-library implementation of
//! hardened-only BIP32 and BIP85, which also reproduces the Noble-generated
//! vectors in `vectors.rs`. Path:
//! `m/83696968'/505153'/<coin_type>'/<alg>'/<account>'/<index>'`, then
//! HMAC-SHA512 with ASCII key "bip-entropy-from-k" over the raw child private
//! key, keeping the first 32 (ML-DSA) or 48 (SLH-DSA-SHA2-128s) bytes.
//! Seeds are the expanded BIP39 seeds from `vectors.rs`.

use hex_literal::hex;
use pqseed::{KeyDeriver, Scope};

const SEEDS: [[u8; 64]; 3] = [
    hex!(
        "408b285c123836004f4b8842c89324c1f01382450c0d439af345ba7fc49acf70
         5489c6fc77dbd4e3dc1dd8cc6bc9f043db8ada1e243c4a0eafb290d399480840"
    ),
    hex!(
        "e00fc08b7fa18c78eb9a943c2ce3903eb52d2590be5aadfe83b39465ac66c3a6
         490a134828a61d311d62974b4ce0708d9a714c529f3e45b296640a9297c6f383"
    ),
    hex!(
        "de56ff0e9536e7185f43246276662657ad42281537ff332cb9e493897a477636
         7be422b8fae0dcd752e3cd3d9ac5a533cbbe07ef83a24af90c8d0cb5d4bc4713"
    ),
];

const MAX: u32 = 0x7fff_ffff;

struct Vector {
    seed: usize,
    scope: Scope,
    ml44: [u8; 32],
    ml65: [u8; 32],
    ml87: [u8; 32],
    slh: [u8; 48],
}

const VECTORS: &[Vector] = &[
    Vector {
        seed: 0,
        scope: Scope::new(133, 0, 0),
        ml44: hex!("e698a4035cb08131a1f69809c167aaac27c40f90a6259a5318dd374583f2b90e"),
        ml65: hex!("14b0f8eaf217e75f06098c7e4a9655bde6d1af5b99293f5f1ee0bc270afb436e"),
        ml87: hex!("46d864dcfbf95b7b44d59832507d76e1c10edefd69ee810dce6d87b93bf4b941"),
        slh: hex!(
            "18196638f84052228cebea63a41ec571b7a1abf2c13295affbb437f2c0ff5e8b
             ca8c05a1807c2fe6a3938fbad7ad6f78"
        ),
    },
    Vector {
        seed: 0,
        scope: Scope::new(60, 1, 7),
        ml44: hex!("20ca0f46003ac874d4ab856a7ab0cf544bd29df4237d160601be634ad0f14272"),
        ml65: hex!("b89d7e462b757aa53c2b4c19521a4c69ecceee8eed5905c4dad2f22dd6e2a95d"),
        ml87: hex!("d58f26644c3f287d76db8b37bca8afbf01dc83a94850ff7461d34718e00b9ebc"),
        slh: hex!(
            "00061ea895b329410870c30978407e643c3e7903b57916f0610064aaa476c168
             c727bb988d18515804635244071bb55e"
        ),
    },
    Vector {
        seed: 0,
        scope: Scope::new(MAX, MAX, MAX),
        ml44: hex!("65921dd40d9f70730a698c5c21cf11642c45366d45ebe60bfabf5dd0493d5949"),
        ml65: hex!("f13b209d10a691d8eb09e092d349ba3bc1cee7231a5030aa57e37af3a0beb999"),
        ml87: hex!("df3e5afa4213b7c13209e628053ee17b3e47c66ebddca8182ab2d0505979bb12"),
        slh: hex!(
            "9b65e02d9f7fb83937cef3183ea87916eae98b2d08a6be0c9106c77aa3dbd28d
             6099af3ed7acab20deb5ca8189c40093"
        ),
    },
    Vector {
        seed: 1,
        scope: Scope::new(133, 0, 0),
        ml44: hex!("e4a569309dd650a3b1e29de4ee85006fc98b6f59b6c7fa39fa8813e0f0c06a27"),
        ml65: hex!("fa5ad3533fede660dbcd8a863c77b114181254286b237decf06eda21a5e2290b"),
        ml87: hex!("bc0c264c3019a8678a9caaf41b749e90d58991bd1107213160e4e97a9f8fce7e"),
        slh: hex!(
            "5559f37c91f8475b2bfd275781b6f7ef205cf7962ce0ec3ebcac5bf11a35af60
             ba9fa49c008698f78e7eddfcb3cc6f4f"
        ),
    },
    Vector {
        seed: 2,
        scope: Scope::new(133, 0, 0),
        ml44: hex!("17d67be31bd342bfd54ce359a0c406025a8dcb4c97cb854c246c709d5dd0fbd5"),
        ml65: hex!("e1de209412299ee47d12b079f3b0e8bdc636f84861deef8d75b4c28c854052f1"),
        ml87: hex!("54f395c1ea9ef603bcbf5b35e83e8f0a3e12d9be3dc5716aecaa2e104a1c3a05"),
        slh: hex!(
            "5c2e48880ec22d51630e7fb07cbd6ec2cf487c44aec66bdaf882f50d5a9ad5dc
             94c3ae2dbcb9c7918ed564fa0c639333"
        ),
    },
];

#[test]
fn scoped_seed_vectors() {
    for vector in VECTORS {
        let keys = KeyDeriver::from_seed(&SEEDS[vector.seed]);
        let scope = &vector.scope;
        assert_eq!(*keys.ml_dsa44_seed_for(scope).unwrap(), vector.ml44);
        assert_eq!(*keys.ml_dsa65_seed_for(scope).unwrap(), vector.ml65);
        assert_eq!(*keys.ml_dsa87_seed_for(scope).unwrap(), vector.ml87);
        assert_eq!(*keys.slh_dsa_sha2_128s_seed_for(scope).unwrap(), vector.slh);
    }
}

#[test]
fn scoped_seeds_differ_from_project_eleven_paths() {
    let keys = KeyDeriver::from_seed(&SEEDS[0]);
    let scope = Scope::new(0, 0, 0);
    assert_ne!(
        *keys.ml_dsa44_seed_for(&scope).unwrap(),
        *keys.ml_dsa44_seed().unwrap()
    );
    assert_ne!(
        *keys.slh_dsa_sha2_128s_seed_for(&scope).unwrap(),
        *keys.slh_dsa_sha2_128s_seed().unwrap()
    );
}

#[test]
fn reject_unhardenable_scope_values() {
    let keys = KeyDeriver::from_seed(&SEEDS[0]);
    for scope in [
        Scope::new(MAX + 1, 0, 0),
        Scope::new(0, MAX + 1, 0),
        Scope::new(0, 0, MAX + 1),
    ] {
        assert!(keys.ml_dsa44_seed_for(&scope).is_err());
        assert!(keys.slh_dsa_sha2_128s_seed_for(&scope).is_err());
    }
}

#[test]
#[cfg(feature = "ml-dsa")]
fn scoped_ml_dsa_keys_use_scoped_seeds() {
    use pqseed::ml_dsa::{Keypair, MlDsa44, MlDsa65, MlDsa87, SigningKey};

    let keys = KeyDeriver::from_seed(&SEEDS[0]);
    let scope = &VECTORS[0].scope;
    let expected = SigningKey::<MlDsa44>::from_seed((&VECTORS[0].ml44).into());
    assert_eq!(
        keys.ml_dsa44_for(scope).unwrap().verifying_key(),
        expected.verifying_key()
    );
    let expected = SigningKey::<MlDsa65>::from_seed((&VECTORS[0].ml65).into());
    assert_eq!(
        keys.ml_dsa65_for(scope).unwrap().verifying_key(),
        expected.verifying_key()
    );
    let expected = SigningKey::<MlDsa87>::from_seed((&VECTORS[0].ml87).into());
    assert_eq!(
        keys.ml_dsa87_for(scope).unwrap().verifying_key(),
        expected.verifying_key()
    );
}

#[test]
#[cfg(feature = "slh-dsa")]
fn scoped_slh_dsa_key_uses_scoped_seed() {
    use pqseed::slh_dsa::{Sha2_128s, SigningKey};

    let keys = KeyDeriver::from_seed(&SEEDS[0]);
    let seed = &VECTORS[0].slh;
    let expected =
        SigningKey::<Sha2_128s>::slh_keygen_internal(&seed[..16], &seed[16..32], &seed[32..]);
    assert_eq!(
        keys.slh_dsa_sha2_128s_for(&VECTORS[0].scope)
            .unwrap()
            .to_bytes(),
        expected.to_bytes()
    );
}
