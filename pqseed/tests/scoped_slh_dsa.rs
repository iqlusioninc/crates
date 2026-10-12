//! pqhd conformance vectors for the SLH-DSA parameter sets at scheme indices
//! 301-311 (SLH-DSA-SHA2-128s, index 300, is covered in `scoped.rs`).
//!
//! Copied from the pqhd test vectors pinned to
//! <https://github.com/pqdh/pqhd/tree/befdce14cecbb5d485d3239982bfa5b207cdb922/test-vectors>,
//! files `test-vectors/{301..311}.json`, cases 0, 4, and 8: one per seed. `output` is
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
            "c21fbe627f7d39a682eaff9bec2954a8bd3f4883bf036742a907cfb7d2cd0e81
             8f7845154a423d26e01176d88685f498"
        ),
        pk: hex!("da17829262c5bef301b78268b82ddf89f13cd514a4ccccd19dd7a6c5dca47f94"),
    },
    Vector {
        set: SlhDsa::Shake128s,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "a844e41e58944d02a24b8d832694bc4eab06021e306ff05b3aff053ceb510147
             902ac0c62e636f699a2d248148246693"
        ),
        pk: hex!("9d7f051d8410d97e0669e3d89f58721c9c3370dbf1eb299bf2709e07c29a1f29"),
    },
    Vector {
        set: SlhDsa::Shake128s,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "bed36e7e9bf9f6e4ced6de3adf44e986ab900d7c7a47838b8397ffffda61d5dc
             233b4b6e0cfb414b0aa5c05bf22adf2c"
        ),
        pk: hex!("a45a740a9569fa4329244c6ec3cb66362774268f99ccc6c2aa552853cc723681"),
    },
    Vector {
        set: SlhDsa::Sha2_128f,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "fb0dbc4160712b352ee08f039cb5d81051fe2077b49db8f93f3c7919104a33a2
             503ea16166aa81cfc5dcbd0483879afa"
        ),
        pk: hex!("4c866df7313f7eb20a4560162db41dd94448c843b99551be5f96d6a3bbe1f5bf"),
    },
    Vector {
        set: SlhDsa::Sha2_128f,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "a1f527e63bfc491232b1eabc0dd26e3ab2fc385e5119d14339e3ec4b712013e8
             e54501481aa903b6e66c7fb8c893f099"
        ),
        pk: hex!("9463a38171090325a130dd46510cdcd96066881d6058a75c195002d0123b3b34"),
    },
    Vector {
        set: SlhDsa::Sha2_128f,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "3f1cd88fbc293fff4dcf82ab8d827ed8d3f74931ac7c2cfca0e02fcc3f14130b
             37ff84adc1818d13608cc71f3d36f117"
        ),
        pk: hex!("68057de5b99de80dc40333bdb80ced89626df44e43b9a532b6a8e4450a231f1c"),
    },
    Vector {
        set: SlhDsa::Shake128f,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "05b3870e2cee4c859f5f6aadc1006f233c6b72d398f18fdf27ffb104eb406bce
             1796eb6a6e9683017052e81bdcd0199a"
        ),
        pk: hex!("9bceba27c9656cb55c14673d8be03fa29b5e97960e09bcb4cd916e7baa4c1efc"),
    },
    Vector {
        set: SlhDsa::Shake128f,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "ae8b165eef8d2c6e9e2273794180c8e58b30578b53b194ac67ee046b6447ec97
             5c88bed7d93616bc6f6b1097898ac8aa"
        ),
        pk: hex!("1c11d6ff6d7a04a7a2af8b2d30d1a1df1d7543bf2b0a29f119a2f2b19706fc40"),
    },
    Vector {
        set: SlhDsa::Shake128f,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "df749ff33726fcd83cfd25a431a8ed53130c45d401700ac0c86ace65e0825334
             21b512a7485de8c51c19a2b9f91527ca"
        ),
        pk: hex!("85758352f24780aaab472d668611b1d3b873c995390d695639c649baa768a6be"),
    },
    Vector {
        set: SlhDsa::Sha2_192s,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "b28eeb262e3171ea845ddadb0789bcf10dcc4b036e5202d7fb3500791a0ab63a
             cc078343070707bcc6b39245ef5f4193f75fd74dd930ede6006d1667519dcd96
             f19e148e61f0b4b6"
        ),
        pk: hex!("d80aadd5a1f2c9d3c7b23bac8e30fa3cb6ba661b83c3926ad5b007712ec260b5"),
    },
    Vector {
        set: SlhDsa::Sha2_192s,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "cf5d8aeb28b3ee4333be82022d6afe1abb32456149869509005ceeb213d8c55e
             5b124bee1ff9598059c535b51feba91762ebede2fd8b2ae02404e095c2a1f504
             f090ab68409cf804"
        ),
        pk: hex!("02cd62c24bebf2b58799fc96bb16df483a1654d0e82a50df610db83e389b7d12"),
    },
    Vector {
        set: SlhDsa::Sha2_192s,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "b24625acabb3e06a5c8d66c2320646a8a595cd525034063775043701f6ca5fc5
             c3fc2a09c990c177ec0a1ddb9e46164e5d792563a25e2ec4b12f459fa742a896
             85d722900b9a9d36"
        ),
        pk: hex!("579cb21b06b954305804fc4cf34bb82a29a94d6bac6175c274abf5a977a997cd"),
    },
    Vector {
        set: SlhDsa::Shake192s,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "511b9d8819c824d7ebd458a798aba6ee9007a814b0c9cba4c46f07cc29dfc451
             1f97cd66af9d0acf5222781f66e7721d2f209e1e825be7833870d13667b2ec1a
             eea8e663e04b31c1"
        ),
        pk: hex!("205eb25e4386463b3dbcbf55bbdbc333e78817a4b5beebda8c467632c66de484"),
    },
    Vector {
        set: SlhDsa::Shake192s,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "c3ecffaac7aaa1a3d768372a8edb0f9e8680984710f0614a5a508e6eaad6c7f4
             fcb4547591c196cf79787a316f7f080a01d330cdb2325aef94656048aeb5f0c1
             6d3b1472dcbe05af"
        ),
        pk: hex!("3d0b9ced23c50ba6a3bec3a49825fbdc1867ef101f5de336e930df2f8c3d44d2"),
    },
    Vector {
        set: SlhDsa::Shake192s,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "1e13a04f3afc634b10c9d6575e2cebe86d6a861d8d17a3aab3dc72d4c32c4826
             2521da31d0df26f4611d14fb14f3fed038e12acd3dc2b8dc2aaa80ef26595022
             56b9faab5173f5d2"
        ),
        pk: hex!("19faed5f1f467e7a6edebed36dde6c6dfa34ae8462166e0a6aabd91bb17c4cdc"),
    },
    Vector {
        set: SlhDsa::Sha2_192f,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "255bab80236d65c6b475aa9f454ceee7f2ad7322e594e4e4ee66a91ead89c83b
             f0e753c9867cc542c7a63868a9cdda7cceff7c1dad306e17414e1feffe93e431
             9bd13f3d29677eb1"
        ),
        pk: hex!("6ecabd0bf35ffb4c2c940edd2ca9876ed4e4ad3968d3322aa071a3dbccc88743"),
    },
    Vector {
        set: SlhDsa::Sha2_192f,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "59987c531d9b84c9fd2dd8e13613be7ca2051ab8f91d1516d8de6a271e3c10a4
             3ccd513a683b4974c0f7a0c7737f8390c29868fd29e3ef77c38f2a4b2152c04f
             099895ba0960bb8d"
        ),
        pk: hex!("9c02ca6e3202aedf2ea23137e1f1ae382af43ad91a88d1da282fb644358072ff"),
    },
    Vector {
        set: SlhDsa::Sha2_192f,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "cc39c05adc35db5b692c64b74a7898b5189a2f87da44a7c1d04d214ec267ac80
             4c7157f5ea14a149befd847edf36df4d9c9db916bd598186fb6afc2c57aa5090
             423cf5cc49f5d9f2"
        ),
        pk: hex!("feb90da6d37b3167bfb542de317d9fbba9becc2eb0ff7884260e4b5aadd7e446"),
    },
    Vector {
        set: SlhDsa::Shake192f,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "86f0f65ff96005d8f37e30495bca2577c73688ee1b0c12e9a3acfb47f69bbe25
             181230d2e12d77b06dd3e34d189ea48124532c0d558920e68e7b91bb54e06963
             03396ffc6b7d8cd7"
        ),
        pk: hex!("d2c799a33f653b1904486985097ce4a3f8029d889c9be00441730af43db8b552"),
    },
    Vector {
        set: SlhDsa::Shake192f,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "41f8680bb6c7ad20d8ebc99bf0ebca7824bef20cb6b318651f471733fa6d958d
             c12eb21ac1ead56033408673f92ca9d18e692cc316760a871b1a1d44aa267931
             fb68c14a980c8747"
        ),
        pk: hex!("72b1f066695e3aa66cb30333941751d3fa434c7ee10d6df1378d840e25b97adb"),
    },
    Vector {
        set: SlhDsa::Shake192f,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "39073c80512da859ad2ca0bdc2677c1b32e21b5c3fce124593176c16cd8988cf
             ba0b9c439e7c7307b47908d7d3000006774ac141f225e43eb167c098b3996546
             6885bb0f2ebf73f5"
        ),
        pk: hex!("f94be8bd400cccee96893976d4ff6007986bf5b28d9e7e673ebef794f052cf21"),
    },
    Vector {
        set: SlhDsa::Sha2_256s,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "8f9d4d26fc00e94baa62da77c1d55d894734176ce817e133bbba9b526148e9b6
             fad53ad11fdc14af14610a9b87815df24f80ec8f81108f4d155fadfbcc3fd863
             d007fc4e1d008a6db143261420bf58b9a92a5328e732106d411f918949db3a4f"
        ),
        pk: hex!("2749baef479b3027ec390fe35860142798e26af611f47563b8d5ae7eb0fc112a"),
    },
    Vector {
        set: SlhDsa::Sha2_256s,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "53064ed708fdd5e803ed0e90c366562a80937d75c339f7598a652f914c1f1419
             8b101b47ccad2f3af65d2a48d42b05933f50c06aa31999dcc80a021f338dccd6
             4af8e3c626a15fd70e79536935ae5a99d6d46cfe95707dd13068b4e8d12bc177"
        ),
        pk: hex!("e3622e9ff32406e2925db196ffce5018aa2eb6238f58e97a0d676ea521baa429"),
    },
    Vector {
        set: SlhDsa::Sha2_256s,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "6862ec9d2f7bb882079341ba7ecb7cb3b39401ba674ad1ee6f19ecc0f89e2d20
             eaee2f7be21287a57154ae5945d3e439efadaea13fee7d428d02fcbc9e1363d8
             e2d9c6d505c354cdee7903029486fd9ba49d3d64ab371318c16ba1ec34019419"
        ),
        pk: hex!("f8ffaf0c7b89028800daf169e189248c176173b024ccaef6f606c88122757fd1"),
    },
    Vector {
        set: SlhDsa::Shake256s,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "33ff74930e811b59c0e254cbc967be35ffcf9122bb112b0aed163a8d8e11ab45
             894027e51550e840412a15a0959182482ab3f6c501a86b9b39f1ec644eb3f50c
             669386f535c530669c81b5426fc822ed532aa64da31c000c482673d1c7ea318a"
        ),
        pk: hex!("180b498ec87460a9a2a42fb8a0a7ecca5a4c38c01542e34110d62a1fa335fbc9"),
    },
    Vector {
        set: SlhDsa::Shake256s,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "38d2265aaf905bca4fcfceff08c30052f12edafd57024c222c6ecadd88545b1e
             ce806a39113c8b84191c6ffa4fa0898a5673501d6da457b856e41a738446e3d6
             878570ec9f5e5a04c32d6452b1dd8757ac8f31df88ef6c57e3c9b18924268e9c"
        ),
        pk: hex!("5d5c32213afa68b9d6b24562f245e13944b89a8ac7d0cf29c198bdd777ca8022"),
    },
    Vector {
        set: SlhDsa::Shake256s,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "3df99f02481945f2c8488d19505b2a9883ba4d78243ae805c0c85f613fee3ca4
             40837263df1f8efe866af307a705a2c7cf561bab04829ee2baa612e7d1b6091e
             836ca1627f9997f19202e577939725f20af1b7b96a7c53b795bafa2d05b861b9"
        ),
        pk: hex!("56b086270542b84405d600eb942702a346e613922b2fed2b4dcc576fe2bb0387"),
    },
    Vector {
        set: SlhDsa::Sha2_256f,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "274c0667cc4f8241e6e087b6687d19418ee431ca2c65074b69efeb0d24dfb43e
             f7cfeb4ae0fd295dd0fdfd90aba20337a6bfb26477871facd86a2694f32f4b08
             c8f81c565b9aa04376f7b8ae4a997d316cc82ab984e0598aed09eb581d520c75"
        ),
        pk: hex!("67d50fcbb276fbd938d5595edbb2f1bbf465870f4fa3f76a39ffe3e038372d4a"),
    },
    Vector {
        set: SlhDsa::Sha2_256f,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "359149ee20e546200f4e09ea441629527cc5056d00002724dc914d5fc2ce61bb
             e18bbb2dc01ac44c0a5e5a8538e15110ae228e8e293b1670eaa5efb72d5e7e1e
             1b63d3e97c3d3d26b5c33a25d484054000543899cd5f4fb0d480bd3aaef969dd"
        ),
        pk: hex!("43a65db177dee0ce5e40815efa0804fef5029bb6301d002bf7307c8e3717794e"),
    },
    Vector {
        set: SlhDsa::Sha2_256f,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "dd5f714d704351ba5d32781387806e1f922cd00740c4cdd076a3dd42456ac3fc
             7a7d5aab7e1fe455d8dbad5b56beb35bee2bf1596f625b771b330ed0daa62dd7
             9c352b27be15252aedc0cae56832358fd17b954e6c297af03d6109df9db17c1a"
        ),
        pk: hex!("b2310601e87f4fa4f912f8c9bb599f9dcc34a440a7ea8f6254f6323c693ba68e"),
    },
    Vector {
        set: SlhDsa::Shake256f,
        seed: 0,
        scope: Scope::new(0, 0, 0),
        output: &hex!(
            "7ff6f593b6c2727cfa7be415d1c062d286c1cb5efbda44ea92ab0ba287daae50
             b29aad51d9859e1ca79a1cc7c40ac15a7a780df92d7023ebe8c0cdc0eadffb6d
             e89f1126404c18fdc6a4e8895a7a3f9de58e69fca596d36aaf3069753973fad8"
        ),
        pk: hex!("d902485407f278007bfc040efb4e1fd5f1bfbdfed43fcd37a96ede621d8eae94"),
    },
    Vector {
        set: SlhDsa::Shake256f,
        seed: 1,
        scope: Scope::new(133, 0, 0),
        output: &hex!(
            "191cab1698c5ca4ed2520f9cf16c99d2de213fa19e374a7f5cdc4c0bfb51a1d7
             98b42eb8c9c7603b9c13626d640180a5209d1f6766e18b6618eed9957be0fe7c
             ded39d2436a5dd48c25ee7cb3e9f7abd3508e20efae6a4613178999d066a80a2"
        ),
        pk: hex!("a34485a6cba51ae34b0b046ccdfd458efb60229eadbe313e6eb0fb4e8917d051"),
    },
    Vector {
        set: SlhDsa::Shake256f,
        seed: 2,
        scope: Scope::new(60, 1, 7),
        output: &hex!(
            "136d3bf3933f60f93785939b3e214ff2cbfa87713ddaee383ee1a8563c3c4ec0
             c4febbdd3ab14c856d93d8be09358791b0c64d07b9086300c681df63cf5bf99c
             6e457dfb3020cee2d71b48ce6bea43a230157fac172b1485472d617b95797dbf"
        ),
        pk: hex!("2dd10da16b94cf18f7ed04307bca0e3af744ccbac38e44a7b70dd538e57afa98"),
    },
];

#[test]
fn registry_indices_and_lengths() {
    let expected = [
        (SlhDsa::Sha2_128s, 300, 16),
        (SlhDsa::Shake128s, 306, 16),
        (SlhDsa::Sha2_128f, 301, 16),
        (SlhDsa::Shake128f, 307, 16),
        (SlhDsa::Sha2_192s, 302, 24),
        (SlhDsa::Shake192s, 308, 24),
        (SlhDsa::Sha2_192f, 303, 24),
        (SlhDsa::Shake192f, 309, 24),
        (SlhDsa::Sha2_256s, 304, 32),
        (SlhDsa::Shake256s, 310, 32),
        (SlhDsa::Sha2_256f, 305, 32),
        (SlhDsa::Shake256f, 311, 32),
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
