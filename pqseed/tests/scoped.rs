//! pqhd conformance vectors for scoped derivation.
//!
//! Copied from the pqhd test vectors (https://github.com/pqdh/pqhd, files
//! `test-vectors/{0,2,3,4}.json`), produced by its standard-library Python
//! reference implementation. Path:
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
        ml44: hex!("60c0239a6bd0454bd70b300db40b3a4e35719e5ff71dc809e1f96a21622add84"),
        ml65: hex!("3e58d7dfc6ed8e373c640242dae281ce64eb8546cb3f549e69581e8ad6bc9b4f"),
        ml87: hex!("9bfa74d473da58b697190408bfe8b6fb7fe5a046990bd9ea94c9cb393b277db1"),
        slh: hex!(
            "07c1526a0ec9cf213c0cda3b667991ba418ec899e6030f20d0da034126de28a7
             384f930f912b9126d35b697fb9fc26b7"
        ),
        ml44_pk: hex!("90f20086f45fcf5629d26dd09fd1fdff4a00d0c756a1fb1badfb4b834305a422"),
        ml65_pk: hex!("6f60fbd47b1f9d05bb6f0faa61ee9628d0f14b6a0ed0f514e8c25725f672a08b"),
        ml87_pk: hex!("09d1696647ef3d16e130fc53be33225482c4b5ac30306dcb2e1a18ebea181b34"),
        slh_pk: hex!("d4a937ee12247f3ec26877221a721610834dfe399a1ed54ef0dc7741df78ded2"),
    },
    Vector {
        seed: 0,
        scope: Scope::new(133, 0, 0),
        ml44: hex!("0b3b36b08d98d4c702731451e75ee4576899d062b1986e132fd1094458f55358"),
        ml65: hex!("4df0b2ff4ad547381c11db85fde4c21682943e59b110faec4e98bbfe6e8297be"),
        ml87: hex!("33494dce3ec41bb82e9c716b50b3f09c3e59acd4c070d41c8c5a510a23dec4ad"),
        slh: hex!(
            "a67a7e9ae6ffa33b665325e93dbb9dc01ea171f0f09e1da3420d07f249bb0ede
             37bff572c5965fbee8bd4218f4b2bbe4"
        ),
        ml44_pk: hex!("08007e5f922b6aab1800091f94c5f280dab47bdf705d2e2fb6f056c0992bb614"),
        ml65_pk: hex!("a734ab4c01cd7f4e8a5285281597c8e1e03cef26eb46bca1c19a5351a06f3cee"),
        ml87_pk: hex!("89155a9fbea7a489fb0b6aa0ff53d02a57bafec16dab1f34fc172ba42e5364a9"),
        slh_pk: hex!("2847ce6c0cfb675d1489918ddf96dc2ca99b841e37aee0ded6d2699c700913ed"),
    },
    Vector {
        seed: 0,
        scope: Scope::new(60, 1, 7),
        ml44: hex!("0124c47b0bdbbe3f85383c26f43de6ddf0b404f1594f8ac2b7e978b418f8b322"),
        ml65: hex!("ffe4299f767f99661c92969f53db3fee55489d058483bc719da4685abe22eaa3"),
        ml87: hex!("b07d76b5f32428b301298699b7c39123ce8fc0dbcd92aafc7c6985826f88360f"),
        slh: hex!(
            "cb24a33fe1b029be7a92111850ccb1abc80aea54e1f3ef8bd285f37e4dc3f0bc
             6b2602c3752f98303e2038e1d2667065"
        ),
        ml44_pk: hex!("a0ce459dff30d0a18c1e253e8d6fe1da01076a08df27ad756e9df97eb81b575e"),
        ml65_pk: hex!("50f3a6f0de735818beefa607d331971f299909dc68e56294d44c4fe4c19a4710"),
        ml87_pk: hex!("9e57d3391a9ea489df0b9e9cf474be9ea2fc1241f2ee21294d1e28927a1b6fbe"),
        slh_pk: hex!("5a14b8817876686db3bb666603ff61e94baa9d0928387d63633423f6e5ec3279"),
    },
    Vector {
        seed: 1,
        scope: Scope::new(0, 0, 0),
        ml44: hex!("66471537bd4d1e8e0fe235b8b2195fa13b7b4faa4d52c094126e49a8a7d3c3e9"),
        ml65: hex!("a6585cad439cb60b51f526cb3cfe5143a7cf3f8ba8f9a45d3760cdeae3ab74d6"),
        ml87: hex!("507d26dba9f6617ec9a45a9df32ef062cf22010dc0fb3c3ef37f11768a622d94"),
        slh: hex!(
            "45dd0d98e0c1a57002639dfb809401a179e7dab2516608daa2e1208148eb83dc
             879684b695dba52ba4fedac14b6749c3"
        ),
        ml44_pk: hex!("3443c579916ad9c4f07c328cae03a2233ad02d540343c2bbd3f19d0b4272e620"),
        ml65_pk: hex!("d40867c9a372fa76b69a13e7494732883299d19fd1438b1971f6a85d715084e2"),
        ml87_pk: hex!("95669f4b0461fb14d17c508c42823fd99d6b178943bc13f881c38295eca2cf3b"),
        slh_pk: hex!("31543d8f46b614233d52dc16fc2609b928d352da6a1792e14793b47f102adaf5"),
    },
    Vector {
        seed: 1,
        scope: Scope::new(133, 0, 0),
        ml44: hex!("ecdd30c9ca640d41ec704eaecfe41e363513fe44260399b616bb9ce1404f22c9"),
        ml65: hex!("9400f718dc627f3cb767fef867adff2c1599a2b41976f273f140a55a58e69b88"),
        ml87: hex!("1a26b0d44590279d945d4253495b1cf6c77717a12b99ec7faa8089e2f52613bb"),
        slh: hex!(
            "cbd872e19b136f0d85972489fba7282a7011d15372f128f3163a1d329b871293
             72703a69a1183c37f82eba5b712586d9"
        ),
        ml44_pk: hex!("47258974a6010970f4a9473751c0abd2b33d52d0f6485a3b75485190933ed78b"),
        ml65_pk: hex!("920ff0eabcd9a6b16f89277efed841b92fd22f47c447e84460921cf873dcc2bb"),
        ml87_pk: hex!("8ef89261f29f92fc719263b05ffe497dd9aec7f50ba0c3bff28668f82e8032bb"),
        slh_pk: hex!("1cf972efe10bc1d930c2595c4c44eda330374fe38406f726da648e01387a3978"),
    },
    Vector {
        seed: 1,
        scope: Scope::new(60, 1, 7),
        ml44: hex!("42ce097429d964f63dd197d1bb909cb321cf7f2e09f948266cd1678367fcf896"),
        ml65: hex!("60b1db48c68c10e780b660802166e03f53fa559270969f53413176803af18bf0"),
        ml87: hex!("ef3f9ee0dd7e01727462fc37c8cece4ddf665ed3184c99957d3a4e6bcadd1983"),
        slh: hex!(
            "19eb55cddd9008389d1a13e2cb243d9e771b554096a05a714360e3eb722210ad
             6738ea5f53a265975b9b100568b1e687"
        ),
        ml44_pk: hex!("a97b40bb5d911561065866bc9941d6bd03e1885497b4a4d84193c192eccb821d"),
        ml65_pk: hex!("0daa86d9bf4c2b14855acb6a8d8eebcc0736cfd5f97b44bf28bdb0c11b602702"),
        ml87_pk: hex!("8a3d91f47e8e5d796ca230186574ff63953d18ab27f604c5011d4f222ac11e1d"),
        slh_pk: hex!("05d932f0516cdc4d64c46a12d82dd92fa9583059fbf47e085c3753a08eacd86f"),
    },
    Vector {
        seed: 2,
        scope: Scope::new(0, 0, 0),
        ml44: hex!("a7585415f1b466fcbdc564e5822747936cbaba4679ee4abd7273ed34d10fa648"),
        ml65: hex!("8dfb453102ee54bcaba9858e3e34ddabd075b4894da02eb294396f6cf5a7a916"),
        ml87: hex!("7ce97b639ec63f5718ca46d26375af7c6e6e346eb04be0dda62ecdfbb27a4c6e"),
        slh: hex!(
            "50c45022cd65483fd57b387884ff171a57a01e1b852bafb5f1403e074e98a317
             915c067077c2b3baff25e7ffb6ddeebb"
        ),
        ml44_pk: hex!("5cbf7d251740d3d04e20bb4a8bd31d1eca934dd910a8c78c165e876975d3f6e7"),
        ml65_pk: hex!("c4ba355be760b72668cc4dd99326f288dd8ddfdf11b52a6def329351e177cd33"),
        ml87_pk: hex!("aa51e230326bafe74dbe575cd996c1669322d287a172f09efdf7385b58bab761"),
        slh_pk: hex!("ed74ec4d9e70bce5e6715f937056f9b415964941bc7afa88471de60c39845429"),
    },
    Vector {
        seed: 2,
        scope: Scope::new(133, 0, 0),
        ml44: hex!("d6a51369e72a40c933b2b0355dc63f0972e9933e23691122eccca275d8fbb2a5"),
        ml65: hex!("912a6135e10ab8125d836662c8e734a3045fe8caf9d7e3993d0f4a44e72dd18f"),
        ml87: hex!("ead5d2b10fa8c043696fc5ba533331fc6367baf81c31d552d81a7dc9301cc8e0"),
        slh: hex!(
            "12a56c90212d80906745676107fe4fecf6f7fdd37c15d4c61f846c49be697108
             81837858b78a6117cfd15dd92742658d"
        ),
        ml44_pk: hex!("ce5c7a11a06feab89983ae9f1d4ed4182f6b79d0dca7660d54565805e2a18602"),
        ml65_pk: hex!("e1d8347d19007a8cd7d5d1438f839274cc1ba77b1c0c478f578176f79adaf88d"),
        ml87_pk: hex!("ce9007fffaa4fdfb2461e6f4cb4e82306bf68cfb9d1ab3d1e196857e9bac5e2b"),
        slh_pk: hex!("da4bb8c52ca4b3ea3500d1d1eb0357c878269d12a43dab187451c4a24c9b3500"),
    },
    Vector {
        seed: 2,
        scope: Scope::new(60, 1, 7),
        ml44: hex!("1b1149c95a21a8c5417bf1a2c54f74bd88b6c4c2cbfa7c515dd3ce08c5639fb3"),
        ml65: hex!("86eb8694b40e85f00f5ad17e482123a55c79657566329d94c0e510d69590f750"),
        ml87: hex!("3001cb3a7fb6521e41e04b43b52bc032f2a61dd24727f2ca4c3797d2d0282370"),
        slh: hex!(
            "a490e0a63b80a70e7c3f9d8e45d9d3888bbf205fe93dae6cd98cb474d50820ca
             5d4aad3f695914e144e91214a1ed659a"
        ),
        ml44_pk: hex!("137a2a27ab9509b9121bf72510752fb509123ec4a2a84543af4ee2879cb53fe1"),
        ml65_pk: hex!("accde93a48678c162569b0f787051b439f384506d6055a770fdf9a00ea2966f5"),
        ml87_pk: hex!("1257a69423270c8a8730cf227fbe1bf591ccc3c7af01f6ab8b664c0de9631596"),
        slh_pk: hex!("2c56ad81efd9dc58caf8bb4d6489cc69cf83304c0f48576aeb0f188dc7e94195"),
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
fn accept_maximum_scope_values() {
    let keys = KeyDeriver::from_seed(&SEEDS[0]);
    assert!(keys.ml_dsa44_seed_for(&Scope::new(MAX, MAX, MAX)).is_ok());
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
