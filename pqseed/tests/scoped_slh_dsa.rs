//! pqhd conformance vectors for the SLH-DSA parameter sets at scheme indices
//! 5-15 (SLH-DSA-SHA2-128s, index 2, is covered in `scoped.rs`).
//!
//! Copied from the pqhd test vectors (https://github.com/pqdh/pqhd, files
//! `test-vectors/{5..15}.json`, cases 0, 4, and 8: one per seed). `output` is
//! the first 3n bytes of BIP-85-DRNG-SHAKE256, split as
//! `SK.seed || SK.prf || PK.seed`. Public-key hashes are SHA-256 of the raw
//! FIPS 205 public key. Seeds are the expanded BIP39 seeds from `vectors.rs`.

use hex_literal::hex;
use pqseed::{KeyDeriver, Scope, SlhDsa};

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

#[allow(dead_code)]
struct Vector {
    set: SlhDsa,
    seed: usize,
    scope: Scope,
    output: &'static [u8],
    pk: [u8; 32],
}

const VECTORS: &[Vector] = &[
    Vector {
        set: SlhDsa::Shake128s,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "737f61170801b0ae98920a1c29351fec1ff6066e8d259b015085f4c8aeedeb2a
             3fe4c92fe2fb4a8f9596b311f3b16acf"
        ),
        pk: hex!("5a7eb609afe2741ab2ec09bf3cbfcce781204f72c250a5bcbef932062c567038"),
    },
    Vector {
        set: SlhDsa::Shake128s,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "bafb84486337ba53bedd99c27d1b010e690b9a47d1d0e35ccd1e1f5cb714c243
             59b89a07584f931367a2cfdcd56ee39f"
        ),
        pk: hex!("19dde6cbc2f9439958c905f88d8ccedaecc5dc3e182070b17679400505927056"),
    },
    Vector {
        set: SlhDsa::Shake128s,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "147073616b6991692d71ce1a01ec2cd119a3d013af889ccc7743d75cd3fe694d
             e6d89e077d760b64a571016a43d1f8b0"
        ),
        pk: hex!("5f19db0246ab5dac922753c791d0d996e1aaf2366a27badd8291ddcdbc6dbbba"),
    },
    Vector {
        set: SlhDsa::Sha2_128f,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "ffb645aa8279b69dd9ffa330bf3ddad1a6173aa25310a6c4bfa01f66c3254754
             890004faa2c6de7b91a059608c52e2a2"
        ),
        pk: hex!("6b48db22bcb6a9756f88cdd07eef1f37a269d65c259a9d0be190685b35c07e1e"),
    },
    Vector {
        set: SlhDsa::Sha2_128f,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "3d5d68e1e7f00db0f99dd46dcbbc5db2e78560878fedba744ffe26d89c7bab1f
             c4704a22ae817a5bf2715e7d8b6ece01"
        ),
        pk: hex!("ba79e46bea204f9f27ee933ef26def905c71782ebcc918f7e92bf6f4f5984add"),
    },
    Vector {
        set: SlhDsa::Sha2_128f,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "e1c42884e47227b735f54d3bda2540bf8f163b74df9b5bed0a189db5c9f62071
             9aaee544157f4f5ba08487d0f1645814"
        ),
        pk: hex!("6458fa112f72221936324cc931215c2eb017ccba06f23fac8d892d1e38ae0ea0"),
    },
    Vector {
        set: SlhDsa::Shake128f,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "6895b69ff6848081d5e8623c616a687500fb7f846079eb70490fcc153a4195ec
             7278803f15d1912c884bf3b20769a75a"
        ),
        pk: hex!("cdbf6da16e247aa251dafb51d132d356394779dfb3a72d63939f65b487d52e05"),
    },
    Vector {
        set: SlhDsa::Shake128f,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "6c88c800fe7615df3c75962dffa5e6b3f75b521118388856511fca7170ad166f
             222fbf70aa76a60d6d64c2099c6ee58b"
        ),
        pk: hex!("449c2fff0a98eb0c2b9b8894cfd9c8c47dd9f72c04e81da5ad96159d542f7d5e"),
    },
    Vector {
        set: SlhDsa::Shake128f,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "36b3a0c3fdfbccf21d843c3099a025075e7ed5acb24625ff73cca22462ebc63f
             410c256a713fc6f7c4ea71e225db4f0d"
        ),
        pk: hex!("10ff031d1594a2fd95cb0c4af802322f779d5c4b883ed56d8b356af4a0407dc6"),
    },
    Vector {
        set: SlhDsa::Sha2_192s,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "6d8635d59bbbd888d9b27e2f98331c3a423e1f0d989017d77e6172594c45cf7c
             b1d2212534c85bf792a776800d7a79b9ef041f4ef2e12414d8c251b0760e7ac5
             b3ed370b6ca32660"
        ),
        pk: hex!("58d81f27c3009044608997a33fc46d274b6bc8f8c899334ffd7afbbe2333cff5"),
    },
    Vector {
        set: SlhDsa::Sha2_192s,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "3723b221bed2fdab3c4c1175cb4ca1e3875a2dcdfd0052b8c588ea6bd1d4e246
             dbcb0da97e22fb0d54533ed2358a21bc7978e5813c0c163978f0be9d2fa2dddb
             355a4cda45f721ca"
        ),
        pk: hex!("4de8e6e0d386307b716eefef52fdb9123571c046f5e13985fb05b3b9449805ac"),
    },
    Vector {
        set: SlhDsa::Sha2_192s,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "d2aae9eda0424803b67f438d5621c6b67deca9e1e08c5cf06fc891f30271aa49
             a017f8f5d7169f58c94a558d1a7453b281584f004e227332854251d527544fb5
             dff70635ac61e2d5"
        ),
        pk: hex!("cfa9cc343afcb0d64f86362fdf00d7bd25ab427eb9bd2111d4445357e0ed40ec"),
    },
    Vector {
        set: SlhDsa::Shake192s,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "0acc5be0e5f670a3e4fc4ddbd6a675ce525d8129df856110e23752de2f55c64f
             6810946990aab36e971ae0b5ca5b68d989ff5a00e44ed262a1e8b8e38c4f3fcd
             b16369e6355c6aad"
        ),
        pk: hex!("e9e184e4538521d369093ed8c2519d404891a125fb38c65508e9642c8040a66a"),
    },
    Vector {
        set: SlhDsa::Shake192s,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "aa7f52b6824c9a737dfe722f99ed2e96cc95dba86815cea04c347c3e74879909
             d9b00c254049bb7e959ab7468b43005513eb2df9b18cefb4bd23ac773d4e16cb
             f226b493165b1c44"
        ),
        pk: hex!("853ee84dc5840b6e16e8a56e716950602b598499b9f4cae746065b45588aa04d"),
    },
    Vector {
        set: SlhDsa::Shake192s,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "ef005baea71a36c615c49a8d7b98965177d0fe8ef4bc6f3c03849309819d6026
             44403f4666c8d3c8c7aed4156f9caca8d9a3a0373b6b038bb87a22a8961d393c
             af7f3f37e859681c"
        ),
        pk: hex!("0bfefdb3a45871aca68c30d628d5ac1bf589d05a870cb45509d1a3fed5351911"),
    },
    Vector {
        set: SlhDsa::Sha2_192f,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "8ff821eb2c6f23c651bb000f95ccb59a063edf03a5699ce3e8dddc90c571dddd
             bc4ffaa67c09a126f54293c0f42fb0f05d3c58f6c565f1a552a62ccb8c2db31b
             7b2229e64e5760c7"
        ),
        pk: hex!("7b23386dd0d5ddbf21def7ae342cedfe18bead7b915c58478ecf9754e4cd28e4"),
    },
    Vector {
        set: SlhDsa::Sha2_192f,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "0b79534c6d611811bfa00b7b93a99efc2b539f090292d61ee51da2047d6aada6
             474b25e33789a9f67d1737da73c70db6e2f28002902795dcac94214b03702955
             2536bf0e8f248f4d"
        ),
        pk: hex!("ee148ae6f7c2c5e6a5fceef1910fa4900581e6de084f911851361d80c4891068"),
    },
    Vector {
        set: SlhDsa::Sha2_192f,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "904f474793c8dd1c8253329e932b85787b736dcb5d8168fa7c2a52ac01b19500
             e6163b8a6314b7a894fad1f0e18353dad959478837243922df3daa1979acdf01
             cd7d853415000754"
        ),
        pk: hex!("a7b4403e086688e606c8ed47cd9e175cd892527a37f5c18e339025f0efc42b4a"),
    },
    Vector {
        set: SlhDsa::Shake192f,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "0c8bdc8832970d95bc58e0b2eb20a8876b55f2fd718d5883af21a0eb28385152
             322ca74454d477001c9fefacb42be959f15e6496b932d9e60acf91364505e562
             95c98d389def5614"
        ),
        pk: hex!("820440b21b86ab9564009eeb575e44b22952beef66352016c5bc79d96210b3a8"),
    },
    Vector {
        set: SlhDsa::Shake192f,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "0a90690dd6a7989bf0106bc369e87bd7175b5b69e00d1cf025b8705b74519aed
             76736f0a91f6e99d420ef3be85d0f135af5c9cb221bbf79979c48a3aeaee0dd6
             cd7a3f48b223d7dd"
        ),
        pk: hex!("40869d1a41b2d066bcb40c04041a4ecbf3470a68afd3caa37fe60b4783631842"),
    },
    Vector {
        set: SlhDsa::Shake192f,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "e36757779f913e45185a659dd8c8da43434d177f7dd78afe63c7967fec72bd6f
             06d1a98874d368b33240087242cd6e56c76821983defa2a1a3a0d32d7cc26aec
             a06993cda94c8fdb"
        ),
        pk: hex!("a190aa004180d51dafa1a965fca438b176edd0cdff12adcc513eac25334880af"),
    },
    Vector {
        set: SlhDsa::Sha2_256s,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "13e88da8f53e5ce97856a94b28f998e7a287381dd7bf5823a2197fbda162a1ca
             e73e544f33fc4ed9f662ed5273234113398a43e3b5be39ccbe11343826056bac
             9f42c2ac303918296647ffc112ac52cc207f71cfd4fc67ab6a804a086017ab98"
        ),
        pk: hex!("4cacd8142c9b37aa33a996333ad4041dd3bec009d0b86a5e473e09efc2b39963"),
    },
    Vector {
        set: SlhDsa::Sha2_256s,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "a509e63fbaf38a94a271c92aee36855903c7ee8d1068ef35dd8b53b24a5b2f76
             e03be98fdb55a81e958174857c8f4756c87685212edd4a11a4aad8b0dc50fcba
             4b8b266812c4293974e3ecc7bb275feded842ca67b48fd63c4b71f044ee64573"
        ),
        pk: hex!("e1be50f620a1652d15bb0f549495378df912bd515ac80e708851651c72a2af09"),
    },
    Vector {
        set: SlhDsa::Sha2_256s,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "66908b1d759a7b809ce22ed77a9694fcc57c065bad4a5286deadf1d7d09513d4
             4ebad74bf04e58cd18cd8012296c491ebe4d2486985628a282e08426174d87f8
             e9541c690ebef150306bf4afd6465b6a5ddcb95f2b368cf817cf54d2bf26d286"
        ),
        pk: hex!("656f4135ba738be2b97fd25e369b0b1594af0017cb0a64d03957acaffc4c225d"),
    },
    Vector {
        set: SlhDsa::Shake256s,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "4338b3b7b07847635aa6b4345c545ba6041becf30f98c379990dd06463ea4f85
             e0d27a1752781ed89d93efd56a7d441521abe87eaa14963f22dc1874c83447d4
             edc01284ae541b8144fb09ff95650f67752290415006061a8c4ff5dd737abc4f"
        ),
        pk: hex!("2b5a13ae88d9f94c6001486d708b13cad2c36f1d2db23e07cfd27e29869db5a0"),
    },
    Vector {
        set: SlhDsa::Shake256s,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "4e8c7d1dd506e7fd47befc20b5ffef62b798b7bba0072d8f639e8593e36db051
             fdf63333c8f7d2faf28979231ef0824a8c9b1896a589066faec7d0e1c59feb7c
             6f888f398e73fd221e8ab47ef6d380282ae6e70527737b985687c9ea71eb8710"
        ),
        pk: hex!("17bb3e21e57b655ca51e549f1def496e5ad8d621fee5326e30535cf82b7cc96f"),
    },
    Vector {
        set: SlhDsa::Shake256s,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "0cbe241b7b59b71960488b3fb672048332cf316542a65a8328038c8bfcfd96d3
             5a3dfd39e654db196194e28c55d674db96a4bede1128e98a444180c7a8b9a106
             56a7afffe43957f0d9f2cd7b33cad834c0538567f77d84186092c9bd17d2a74c"
        ),
        pk: hex!("a06e4dad665eac944576a4b046c8891adf1fb09c057e912b998ff27176d29a5a"),
    },
    Vector {
        set: SlhDsa::Sha2_256f,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "36eab48ecffdbbb2062d82c2f31bfc8b22aceb1fb7836c31c94bde4f686ff344
             980fc9d6c7de718842d3b13d29465b6121c971f95eebdc0c05fea46b545019c7
             af7df4b74388246fd255742d517550c5f74058a28800955876d6313aa03255d8"
        ),
        pk: hex!("c05e89f3fb0d6ab5f077b79a00c2936c33896c7b5750944379ed694fc3813437"),
    },
    Vector {
        set: SlhDsa::Sha2_256f,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "cf47a237b0ee8a7bd49c5c8e85b77b427a5509403a97bec75c57d3af6f601e73
             69bb28131323323c3ac03b214aa252d4e23ee686eec82bc221f21537c485a63a
             b758a54c8e3e1a95dd287c70f2a81bc586f1e0f689fb59b9e1669354a2af91be"
        ),
        pk: hex!("e5ea12f7b73e2972c6c30706aa0dc82d640a983d2a51eeb22fa86e445c4f377e"),
    },
    Vector {
        set: SlhDsa::Sha2_256f,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "688b21f5f3be4b130bd61645d258f3aa0430a57a7f2266ee6ca6d025215dd449
             bc4d91f7c0cbc423a7e4f6df0ede8ff1936599f0c045d8181229dc48d81d6648
             1413cf883bb695129304ac9b170a1af90d1375f6715bffe02c84b798e06dca4c"
        ),
        pk: hex!("4857d764130f49eafc126381af23320a826e10b2cc3cd5b0c4166007946a698b"),
    },
    Vector {
        set: SlhDsa::Shake256f,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "21dec677c54c65516b8d14948ce26759b4d4275ecebebd6cbd7a4a6d871d3d27
             17ee54406e4db68d9572cb629b092d3fd6ac53fe7503bd2b73c41d1444ce31cc
             2f2cc2d55a62a51d8699566e77aa85dd7fa3869a7369e94cb72b23f76726e0d4"
        ),
        pk: hex!("7658d197c2659038443f0fa1530c78f53415cedb7e05e44312e071709251246f"),
    },
    Vector {
        set: SlhDsa::Shake256f,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "ec7ab7aaff97e749fab3ce991df64e56dc0614010868a01ced3ac2fdd1403065
             9f8757a2e41fef672e02a8e5f358f2a4d4a5c3692bff7fad94078ba932519e9c
             ab36dad8b6db12518f5c523156f987d2ce13610f1148e160a16d2c859ad54885"
        ),
        pk: hex!("38f4665dfaecd31f4b45e538bda9e8c0e1c9f3341ae84129eefd03371994dd78"),
    },
    Vector {
        set: SlhDsa::Shake256f,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "4dc04ef3b341413648ef5e1b2a450d50117c5be01151092dd0661ff5eea7559c
             061e426f1a4f6f9ff2ff7fd4ee67d5074f7016ea827c19ac0cbf202cd488ef8d
             d7b6756ed8f27676875344c07e0e26e1a7d35019560d887f582076abfaefd069"
        ),
        pk: hex!("6312c751757e55b9b38ad4a74bf932858a3e9bdf57fdce2d8bb6d128f8f0946c"),
    },
];

#[test]
fn registry_indices_and_lengths() {
    let expected = [
        (SlhDsa::Sha2_128s, 2, 16),
        (SlhDsa::Shake128s, 5, 16),
        (SlhDsa::Sha2_128f, 6, 16),
        (SlhDsa::Shake128f, 7, 16),
        (SlhDsa::Sha2_192s, 8, 24),
        (SlhDsa::Shake192s, 9, 24),
        (SlhDsa::Sha2_192f, 10, 24),
        (SlhDsa::Shake192f, 11, 24),
        (SlhDsa::Sha2_256s, 12, 32),
        (SlhDsa::Shake256s, 13, 32),
        (SlhDsa::Sha2_256f, 14, 32),
        (SlhDsa::Shake256f, 15, 32),
    ];
    for (set, scheme, n) in expected {
        assert_eq!(set.scheme(), scheme, "{set:?}");
        assert_eq!(set.n(), n, "{set:?}");
    }
}

#[test]
fn slh_dsa_seed_vectors() {
    for vector in VECTORS {
        let keys = KeyDeriver::from_seed(&SEEDS[vector.seed]);
        let seed = keys.slh_dsa_seed_for(vector.set, &vector.scope).unwrap();
        let n = vector.set.n();
        assert_eq!(seed.as_bytes(), vector.output, "{:?}", vector.set);
        assert_eq!(seed.sk_seed(), &vector.output[..n]);
        assert_eq!(seed.sk_prf(), &vector.output[n..2 * n]);
        assert_eq!(seed.pk_seed(), &vector.output[2 * n..]);
    }
}

#[test]
fn slh_dsa_seed_debug_is_redacted() {
    let keys = KeyDeriver::from_seed(&SEEDS[0]);
    let seed = keys
        .slh_dsa_seed_for(SlhDsa::Shake256f, &Scope::new(0, 0, 0))
        .unwrap();
    assert_eq!(format!("{seed:?}"), "SlhDsaSeed { set: Shake256f, .. }");
}

#[test]
fn generic_sha2_128s_matches_dedicated_methods() {
    let keys = KeyDeriver::from_seed(&SEEDS[1]);
    let scope = Scope::new(133, 0, 0);
    let seed = keys.slh_dsa_seed_for(SlhDsa::Sha2_128s, &scope).unwrap();
    assert_eq!(
        seed.as_bytes(),
        &keys.slh_dsa_sha2_128s_seed_for(&scope).unwrap()[..]
    );
}

#[test]
#[cfg(feature = "slh-dsa")]
fn slh_dsa_public_keys() {
    use pqseed::slh_dsa::{self, signature::Keypair};
    use sha2::{Digest, Sha256};

    fn pk_hash<P: pqseed::SlhDsaParams>(keys: &KeyDeriver, scope: &Scope) -> [u8; 32] {
        let key = keys.slh_dsa_for::<P>(scope).unwrap();
        Sha256::digest(key.verifying_key().to_bytes()).into()
    }

    for vector in VECTORS {
        let keys = KeyDeriver::from_seed(&SEEDS[vector.seed]);
        let scope = &vector.scope;
        let hash = match vector.set {
            SlhDsa::Shake128s => pk_hash::<slh_dsa::Shake128s>(&keys, scope),
            SlhDsa::Sha2_128f => pk_hash::<slh_dsa::Sha2_128f>(&keys, scope),
            SlhDsa::Shake128f => pk_hash::<slh_dsa::Shake128f>(&keys, scope),
            SlhDsa::Sha2_192s => pk_hash::<slh_dsa::Sha2_192s>(&keys, scope),
            SlhDsa::Shake192s => pk_hash::<slh_dsa::Shake192s>(&keys, scope),
            SlhDsa::Sha2_192f => pk_hash::<slh_dsa::Sha2_192f>(&keys, scope),
            SlhDsa::Shake192f => pk_hash::<slh_dsa::Shake192f>(&keys, scope),
            SlhDsa::Sha2_256s => pk_hash::<slh_dsa::Sha2_256s>(&keys, scope),
            SlhDsa::Shake256s => pk_hash::<slh_dsa::Shake256s>(&keys, scope),
            SlhDsa::Sha2_256f => pk_hash::<slh_dsa::Sha2_256f>(&keys, scope),
            SlhDsa::Shake256f => pk_hash::<slh_dsa::Shake256f>(&keys, scope),
            other => panic!("unexpected set {other:?}"),
        };
        assert_eq!(hash, vector.pk, "{:?}", vector.set);
    }
}

#[test]
#[cfg(feature = "slh-dsa")]
fn generic_sha2_128s_key_matches_dedicated_method() {
    use pqseed::slh_dsa;

    let keys = KeyDeriver::from_seed(&SEEDS[2]);
    let scope = Scope::new(60, 1, 7);
    assert_eq!(
        keys.slh_dsa_for::<slh_dsa::Sha2_128s>(&scope)
            .unwrap()
            .to_bytes(),
        keys.slh_dsa_sha2_128s_for(&scope).unwrap().to_bytes()
    );
}
