//! pqhd conformance vectors for scoped derivation.
//!
//! All nine cases from each of pqhd's `test-vectors/{200,201,202,300}.json`, pinned to
//! <https://github.com/pqdh/pqhd/tree/befdce14cecbb5d485d3239982bfa5b207cdb922/test-vectors>.
//! Derivation outputs are produced by its standard-library Python
//! reference implementation. The maximum-scope values below were additionally
//! computed with `drng(bip85_entropy(seed, path(MAX, scheme, MAX, MAX)), L)`
//! from the same revision's `reference/python/pqhd.py`. Path:
//! `m/83696968'/505153'/<coin_type>'/<scheme>'/<account>'/<index>'`, then
//! BIP-85 HMAC-SHA512 (key "bip-entropy-from-k") over the leaf private key,
//! then BIP-85-DRNG-SHAKE256: the first 32 (ML-DSA) or 48
//! (SLH-DSA-SHA2-128s) bytes of SHAKE256 over the 64-byte HMAC output.
//! Public-key hashes are SHA-256 of the raw FIPS-encoded public key. Seeds
//! are the expanded BIP39 seeds from `vectors.rs`.

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

#[allow(dead_code)]
struct Vector {
    seed: usize,
    scope: Scope,
    ml44: [u8; 32],
    ml65: [u8; 32],
    ml87: [u8; 32],
    slh: [u8; 48],
    ml44_pk: [u8; 32],
    ml65_pk: [u8; 32],
    ml87_pk: [u8; 32],
    slh_pk: [u8; 32],
}

const VECTORS: &[Vector] = &[
    Vector {
        seed: 0,
        scope: Scope::new(0, 0, 0),
        ml44: hex!("eb21bd409e47106547efe4d7b64e75753cdc448a2706a6517dbb7f07a59bd14b"),
        ml65: hex!("697e4b5b98e2895499e18d47cfa0b3c22d3511572d5cc2cb09a4a11a58aa0f3b"),
        ml87: hex!("7bc874d1fdae3e2050bb4d91c42bac4812703a777bb6f1f079d6574f6c765a8e"),
        slh: hex!(
            "6f0eae22c9d13357e532989dd3894c67e84cb9f007155f00cef02b44e9a52fb9
             c62f4e8ea8bbad35442f2f35a691d8a3"
        ),
        ml44_pk: hex!("679cebbbb832e65be33526a86c54237e16fafeef6619642485ecfe528d423c19"),
        ml65_pk: hex!("5081640742ee9dbd3d277b107a642979fb8d251e0ceb23419901c08a8bd0710e"),
        ml87_pk: hex!("ea351169156bf1574b31f28dd12429bde33fd88cdb0ac0c2a5f3a8886934713a"),
        slh_pk: hex!("a80be3c43e5e04855186c7e9ee3a08420b74c756c84e8f3851b3e23689c94244"),
    },
    Vector {
        seed: 0,
        scope: Scope::new(133, 0, 0),
        ml44: hex!("733ba3c92d6cc7fb33aae720de2335e108f691b8dc829047d48d92bee297187d"),
        ml65: hex!("6ec23681af7d459a2f32d163ca6cdb54796e778aa1fc529315c1c28283d13154"),
        ml87: hex!("a817d1442a8064b0fdc6cb147714871bbb702b9655568271f34d742ed1534836"),
        slh: hex!(
            "035101d0d0c8f741c669173acfad0b3f17bff0d25dfa4aa33a91b557ec94653e
             b6e543f0a977183d102ccb12b2dbc486"
        ),
        ml44_pk: hex!("888dc3c5331036e2bf58ad44c926e63e2d1f1520bdd60a8bd6ac745240b4cef2"),
        ml65_pk: hex!("481323fde2c4cf65f148856184ba915ee87b135b5c14dc02d1cfa3222f60a4b6"),
        ml87_pk: hex!("5713210d1ae437a7bb01f44d9c0cb0e618f1b8f81e3ea4fba3f1043dc7417c06"),
        slh_pk: hex!("04b1802eeaeb28e4a3a695d3aded03a4092a42d64e792094132bc7147d857255"),
    },
    Vector {
        seed: 0,
        scope: Scope::new(60, 1, 7),
        ml44: hex!("74e8248fe65f219c9451dbc784f9e971a1c235c275edf8607e618774b13ad61f"),
        ml65: hex!("e6e89f0dbd34647352164f8b6275f06219a78f27ff361b9ed6c3cc6ce61c5d44"),
        ml87: hex!("b2e7986e8a432755f245be9a3fcfd7ddc57fc7f0d44433e7d98c87206b51ee05"),
        slh: hex!(
            "e00b6b0b3934a8c7d98092c23b1baf7e04aff7db211066ab027d2bafb78ceb7e
             a90f52bd8f37047ff84c3d90b94c3415"
        ),
        ml44_pk: hex!("80fd47232381b247c1aa3a90af45113b151924096169a0ff6d1a921d732e97b8"),
        ml65_pk: hex!("e18897fc498f18b9ceb794f2a7a3b973a15c11875cb1f302defc8ad21b1fe2ca"),
        ml87_pk: hex!("4f42a5385e5acf658aa3a0d81408c6b0f12c65f4c331cea5b8b3436ef45b416c"),
        slh_pk: hex!("2d0ff465e1b63ab5c387608861be398d9afe4632f6df02257c65dfd84940366c"),
    },
    Vector {
        seed: 1,
        scope: Scope::new(0, 0, 0),
        ml44: hex!("8d5ebebabc9adf16199307f545ee6562b86451738fd828dafdeec48e3a43e792"),
        ml65: hex!("421fffa578cc9df36ca2c3f2207ae77a0640f278929a23b508ead6e0cb1eb945"),
        ml87: hex!("0eafe86c6efc6835d87e2f166205bb1bf1f8371a69d43720b171acb4cee235ab"),
        slh: hex!(
            "3a71e3bb582c949e3badb9b03292e20b519c0ba4fe7a19b206247eecd49bff07
             b38236a17622b66b10d89ead212e9635"
        ),
        ml44_pk: hex!("204df52b64be7980ffb5c4234328369bd9648ae6f43d6d4f644136425c1dc95a"),
        ml65_pk: hex!("a79ff03daf6e6a61413d96ad40f6699a7aa728857fb97b481565f7827a244112"),
        ml87_pk: hex!("023f3af859cb623a88c232101fa0f4c3c8dde692054a153821fb3960cfbd07d1"),
        slh_pk: hex!("a51074f1b718b83db2ee78c4d2b3971c1cf60a452f5d216b49ae2149cec555cc"),
    },
    Vector {
        seed: 1,
        scope: Scope::new(133, 0, 0),
        ml44: hex!("d10354ce75edcb911c5b81f9552c23162123bdbef6900b6dd7fc1ffe2ab8b0c9"),
        ml65: hex!("6080f388689258575a759958868580bc7f911eaa4f8253dfded17e4e07a6f56c"),
        ml87: hex!("03b212fa34651289a8dc040d5a0b5c95bb0b8f767f5e07276c7b25e94a9ac80f"),
        slh: hex!(
            "1a7014c10ecf93635b4e7cd8e86aea3c07aa363cbff0e2db01085927a268cefd
             6c6225e688bda30b6949fd52bbd9aa0f"
        ),
        ml44_pk: hex!("c56ec4fa2d5fe574e5d62c36dd559b4a6bca82d9035d28d421dbe707665828ed"),
        ml65_pk: hex!("63bfb7f23b161a83c380b19e6aa789567f56f4549c30001a2a4c364ad278a474"),
        ml87_pk: hex!("b5813795a6b9e4e048819a880296e938678cfec9900955093199424e67a58ce8"),
        slh_pk: hex!("e9ef4102c3f2aa355e017989e1ade779125e317860b4944c54cb6e8a3348a41a"),
    },
    Vector {
        seed: 1,
        scope: Scope::new(60, 1, 7),
        ml44: hex!("1ea0dc610dbac6e00f3f0cb96eaa4b15f1ea43825c13d86d6b5c0fc9bb2defca"),
        ml65: hex!("d685ea180d39eb8f1701290867c977b3971adedb95ff3642f34c5ecf55bda58f"),
        ml87: hex!("1eee84d9aa905646599abab41051fa39da5ecdf0bf3afb1edff950c315b10e6f"),
        slh: hex!(
            "05957fc62d13d9c00f25bebf6cdc5b3aefa2d2c361e3b4b7916481542b74da8e
             27c085d0948c5212d8b7eb63def4d7d0"
        ),
        ml44_pk: hex!("1f319586034e393b54b862b90d56e41abe267b4d1ffb4c3b31f419183c3ee918"),
        ml65_pk: hex!("cdfa38cc205b34dd8d2c4319157035b31eec9454634afb5e76ac4c732c6ea050"),
        ml87_pk: hex!("b188bb30edfcb8f69d14282a1a69b6cbd391ee9f05e590155d9fc7bc25b35f9b"),
        slh_pk: hex!("032e74cd7e4f19c85ffee109c1d10cb0f75317645385b0204f757adb63aa3360"),
    },
    Vector {
        seed: 2,
        scope: Scope::new(0, 0, 0),
        ml44: hex!("fa0810e3800db0899dec01852bafc16688e1cb293244fc2420555479c8fda1db"),
        ml65: hex!("1286d01164e55bae854a71755c3120fbafee9e6a5a67f218206368bca58fd58f"),
        ml87: hex!("1da93f73acc400b34c1f5c1972b1ca9585e927768fa48f44eadad723b9bd4b20"),
        slh: hex!(
            "950affbdb510a78ba60f0469de4fc6e96b4338668adb463e48c6058329031d31
             366dc55d58e768983a1ebc447a439144"
        ),
        ml44_pk: hex!("d9bbc9e871a17f78a891b0833139f86dd1db5a20fc41baada743cc5b478997f8"),
        ml65_pk: hex!("bb820376bd9a3673ad92b46fa475c8d0dac2bd6bae6773ba5d9d26e18d0e8126"),
        ml87_pk: hex!("82ea5427863b6ba773ffe11888c855679631f9a83adde77eafc3133d40c962c6"),
        slh_pk: hex!("d9353f109f58fd22900e992635217de23452ba1245aa25d68b71c713d974ac59"),
    },
    Vector {
        seed: 2,
        scope: Scope::new(133, 0, 0),
        ml44: hex!("53efdab36aa930c206b9fc9dc9073c97bd4c9527b109e6c57588d7b976f77f5d"),
        ml65: hex!("7be622c1a061122d52799d68824ff5ab32f810d890ab482ea1d0d6164cc3e3ac"),
        ml87: hex!("4844a010a0eb207976905be1adb8465632d7bb3ecc7f4d2008d9a077600e1b01"),
        slh: hex!(
            "c563254cd0eba4bf566af0fd2e6b7efe0711c4c101de69aadab4a57b3e013900
             c639ed696b73b3974922c082010d1105"
        ),
        ml44_pk: hex!("d041618d45ac62b01303cd6aae17f4034a8bfd89b377955269385a62af8f8146"),
        ml65_pk: hex!("cb5d2ea1f276fc7823a69b7fcbf81cd0aae0edfdb710de8532b2f47f00290b52"),
        ml87_pk: hex!("f2b89992ab748ffa9b0968ee9748a9b1367cf510e7141de7de7afa8b03ccb748"),
        slh_pk: hex!("aa1530bbf7bb55574094493d63dcc55200571f9ec6144afa955232fb97882e27"),
    },
    Vector {
        seed: 2,
        scope: Scope::new(60, 1, 7),
        ml44: hex!("492cfd2d6c6121be3580e4dd2ade9b16bbbdd7c6348d876ebed3be37dc9405fe"),
        ml65: hex!("773160f08f19df822b3bd24ceba7be090ab892e38f24c36efaf49353fa1c4c2c"),
        ml87: hex!("699d990fc93ea73be937111abb93e21e3f5dbb83b9119ece746e119ad0727f6d"),
        slh: hex!(
            "4500b13ee70c6ebb1d15f3fadae98981fc61929506ea2b8ba0787b69cb1c9737
             c2ec09a44dbe2e8da8aef3b337ef300b"
        ),
        ml44_pk: hex!("f0d899cb842e2c9ce4d30ccf56e50fbd52c425d8bd136e65e07b6d82553d9899"),
        ml65_pk: hex!("9153ccc293ba1816810512c02eabd51003b2b5a0d719f04cda39d746d5593d4b"),
        ml87_pk: hex!("ea15471636c783814227de19b0466ddfa329f99a10b870b78c91f479ac0f4fd3"),
        slh_pk: hex!("58f9df5eed387c147fbe77006382306ee2547eeff43a41507e4a043f424f3a88"),
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
#[cfg(feature = "bip39")]
fn scoped_mnemonic_normalization() {
    // The final upstream vector uses this mnemonic, composed "café", and
    // the asymmetric scope (60, 1, 7). Whitespace and NFKD must not change it.
    let phrase = "  abandon\u{3000}amount liar amount expire adjust\t cage candy arch gather drum bullet\n\
                  absurd math era live bid rhythm alien crouch range attend journey unaware  ";
    let keys = KeyDeriver::from_mnemonic(phrase, "cafe\u{301}").unwrap();
    let vector = &VECTORS[8];
    let scope = &vector.scope;
    assert_eq!(*keys.ml_dsa44_seed_for(scope).unwrap(), vector.ml44);
    assert_eq!(*keys.ml_dsa65_seed_for(scope).unwrap(), vector.ml65);
    assert_eq!(*keys.ml_dsa87_seed_for(scope).unwrap(), vector.ml87);
    assert_eq!(*keys.slh_dsa_sha2_128s_seed_for(scope).unwrap(), vector.slh);

    // Unlike mnemonic whitespace, passphrase whitespace is significant.
    // Expected output computed with the pinned upstream Python reference.
    let spaced = KeyDeriver::from_mnemonic(phrase, " cafe\u{301} ").unwrap();
    assert_eq!(
        *spaced.ml_dsa44_seed_for(scope).unwrap(),
        hex!("5ac0d1b9415e339895771990e9660ab0c386638d465e56d9022bf1ef973acbd8")
    );
}

#[test]
fn scoped_seeds_differ_from_project_eleven_paths() {
    let keys = KeyDeriver::from_seed(&SEEDS[0]);
    let scope = Scope::new(0, 0, 0);
    assert_ne!(
        *keys.ml_dsa44_seed_for(&scope).unwrap(),
        *keys.ml_dsa44_seed().unwrap()
    );
}

#[test]
fn accept_maximum_scope_values() {
    let keys = KeyDeriver::from_seed(&SEEDS[0]);
    let scope = Scope::new(MAX, MAX, MAX);
    assert_eq!(
        *keys.ml_dsa44_seed_for(&scope).unwrap(),
        hex!("55c7241ea28f5f01459fe01e2c012a1aa484965496371f4e3b8d5d11fd4dbec0")
    );
    assert_eq!(
        *keys.ml_dsa65_seed_for(&scope).unwrap(),
        hex!("aaffc11c254080e99800d97d658f406a7a641fa9ca7921d3e574f77a64cd588f")
    );
    assert_eq!(
        *keys.ml_dsa87_seed_for(&scope).unwrap(),
        hex!("59a9941f64a93a5e7a644c3cacccdfc9f4309fa0b667f00547ae9d9bc1ccf750")
    );
    assert_eq!(
        *keys.slh_dsa_sha2_128s_seed_for(&scope).unwrap(),
        hex!(
            "58f07fb8fb4e5f6ad1c00cb1a70ba9ebf403fd3a90c439c2e95a6e22811152fc
             9cbcf2264717092d70edc310e5d440ef"
        )
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
        assert!(keys.ml_dsa65_seed_for(&scope).is_err());
        assert!(keys.ml_dsa87_seed_for(&scope).is_err());
        assert!(keys.slh_dsa_sha2_128s_seed_for(&scope).is_err());
    }
}

#[test]
#[cfg(feature = "ml-dsa")]
fn scoped_ml_dsa_public_keys() {
    use pqseed::ml_dsa::Keypair;
    use sha2::{Digest, Sha256};

    for vector in VECTORS {
        let keys = KeyDeriver::from_seed(&SEEDS[vector.seed]);
        let scope = &vector.scope;
        let pk = keys.ml_dsa44_for(scope).unwrap().verifying_key().encode();
        assert_eq!(Sha256::digest(pk).as_slice(), vector.ml44_pk);
        let pk = keys.ml_dsa65_for(scope).unwrap().verifying_key().encode();
        assert_eq!(Sha256::digest(pk).as_slice(), vector.ml65_pk);
        let pk = keys.ml_dsa87_for(scope).unwrap().verifying_key().encode();
        assert_eq!(Sha256::digest(pk).as_slice(), vector.ml87_pk);
    }
}

#[test]
#[cfg(feature = "slh-dsa")]
fn scoped_slh_dsa_public_keys() {
    use pqseed::slh_dsa::signature::Keypair;
    use sha2::{Digest, Sha256};

    for vector in VECTORS {
        let keys = KeyDeriver::from_seed(&SEEDS[vector.seed]);
        let key = keys.slh_dsa_sha2_128s_for(&vector.scope).unwrap();
        let pk = key.verifying_key().to_bytes();
        assert_eq!(Sha256::digest(pk).as_slice(), vector.slh_pk);
    }
}
