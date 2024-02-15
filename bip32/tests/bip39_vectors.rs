//! BIP39 test vectors

#![cfg(all(feature = "bip39", feature = "secp256k1"))]

use bip32::{Mnemonic, Seed, XPrv};
use hex_literal::hex;

/// BIP39 test vector
struct TestVector {
    entropy: [u8; 32],
    phrase: &'static str,
    seed: [u8; 64],
    xprv: &'static str,
}

/// Password used on all test vectors
const TEST_VECTOR_PASSWORD: &str = "TREZOR";

/// From: https://github.com/trezor/python-mnemonic/blob/master/vectors.json
const TEST_VECTORS: &[TestVector] = &[
    TestVector {
        entropy: hex!("0000000000000000000000000000000000000000000000000000000000000000"),
        phrase: "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art",
        seed: hex!("bda85446c68413707090a52022edd26a1c9462295029f2e60cd7c4f2bbd3097170af7a4d73245cafa9c3cca8d561a7c3de6f5d4a10be8ed2a5e608d68f92fcc8"),
        xprv: "xprv9s21ZrQH143K32qBagUJAMU2LsHg3ka7jqMcV98Y7gVeVyNStwYS3U7yVVoDZ4btbRNf4h6ibWpY22iRmXq35qgLs79f312g2kj5539ebPM"
    },
    TestVector {
        entropy: hex!("7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"),
        phrase: "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth title",
        seed: hex!("bc09fca1804f7e69da93c2f2028eb238c227f2e9dda30cd63699232578480a4021b146ad717fbb7e451ce9eb835f43620bf5c514db0f8add49f5d121449d3e87"),
        xprv: "xprv9s21ZrQH143K3Y1sd2XVu9wtqxJRvybCfAetjUrMMco6r3v9qZTBeXiBZkS8JxWbcGJZyio8TrZtm6pkbzG8SYt1sxwNLh3Wx7to5pgiVFU"
    },
    TestVector {
        entropy: hex!("8080808080808080808080808080808080808080808080808080808080808080"),
        phrase: "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic bless",
        seed: hex!("c0c519bd0e91a2ed54357d9d1ebef6f5af218a153624cf4f2da911a0ed8f7a09e2ef61af0aca007096df430022f7a2b6fb91661a9589097069720d015e4e982f"),
        xprv: "xprv9s21ZrQH143K3CSnQNYC3MqAAqHwxeTLhDbhF43A4ss4ciWNmCY9zQGvAKUSqVUf2vPHBTSE1rB2pg4avopqSiLVzXEU8KziNnVPauTqLRo"
    },
    TestVector {
        entropy: hex!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"),
        phrase: "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote",
        seed: hex!("dd48c104698c30cfe2b6142103248622fb7bb0ff692eebb00089b32d22484e1613912f0a5b694407be899ffd31ed3992c456cdf60f5d4564b8ba3f05a69890ad"),
        xprv: "xprv9s21ZrQH143K2WFF16X85T2QCpndrGwx6GueB72Zf3AHwHJaknRXNF37ZmDrtHrrLSHvbuRejXcnYxoZKvRquTPyp2JiNG3XcjQyzSEgqCB"
    },
    TestVector {
        entropy: hex!("68a79eaca2324873eacc50cb9c6eca8cc68ea5d936f98787c60c7ebc74e6ce7c"),
        phrase: "hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay length",
        seed: hex!("64c87cde7e12ecf6704ab95bb1408bef047c22db4cc7491c4271d170a1b213d20b385bc1588d9c7b38f1b39d415665b8a9030c9ec653d75e65f847d8fc1fc440"),
        xprv: "xprv9s21ZrQH143K2XTAhys3pMNcGn261Fi5Ta2Pw8PwaVPhg3D8DWkzWQwjTJfskj8ofb81i9NP2cUNKxwjueJHHMQAnxtivTA75uUFqPFeWzk"
    },
    TestVector {
        entropy: hex!("9f6a2878b2520799a44ef18bc7df394e7061a224d2c33cd015b157d746869863"),
        phrase: "panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost inside",
        seed: hex!("72be8e052fc4919d2adf28d5306b5474b0069df35b02303de8c1729c9538dbb6fc2d731d5f832193cd9fb6aeecbc469594a70e3dd50811b5067f3b88b28c3e8d"),
        xprv: "xprv9s21ZrQH143K2WNnKmssvZYM96VAr47iHUQUTUyUXH3sAGNjhJANddnhw3i3y3pBbRAVk5M5qUGFr4rHbEWwXgX4qrvrceifCYQJbbFDems"
    },
    TestVector {
       entropy: hex!("066dca1a2bb7e8a1db2832148ce9933eea0f3ac9548d793112d9a95c9407efad"),
       phrase: "all hour make first leader extend hole alien behind guard gospel lava path output census museum junior mass reopen famous sing advance salt reform",
       seed: hex!("26e975ec644423f4a4c4f4215ef09b4bd7ef924e85d1d17c4cf3f136c2863cf6df0a475045652c57eb5fb41513ca2a2d67722b77e954b4b3fc11f7590449191d"),
       xprv: "xprv9s21ZrQH143K3rEfqSM4QZRVmiMuSWY9wugscmaCjYja3SbUD3KPEB1a7QXJoajyR2T1SiXU7rFVRXMV9XdYVSZe7JoUXdP4SRHTxsT1nzm"
    },
    TestVector {
        entropy: hex!("f585c11aec520db57dd353c69554b21a89b20fb0650966fa0a9d6f74fd989d8f"),
        phrase: "void come effort suffer camp survey warrior heavy shoot primary clutch crush open amazing screen patrol group space point ten exist slush involve unfold",
        seed: hex!("01f5bced59dec48e362f2c45b5de68b9fd6c92c6634f44d6d40aab69056506f0e35524a518034ddc1192e1dacd32c1ed3eaa3c3b131c88ed8e7e54c49a5d0998"),
        xprv: "xprv9s21ZrQH143K39rnQJknpH1WEPFJrzmAqqasiDcVrNuk926oizzJDDQkdiTvNPr2FYDYzWgiMiC63YmfPAa2oPyNB23r2g7d1yiK6WpqaQS"
    }
];


struct TestVector12 {
    entropy: [u8; 16],
    phrase: &'static str,
    seed: [u8; 64],
    xprv: &'static str,
}

const TEST_VECTORS12: &[TestVector12] = &[

TestVector12 {
    entropy: hex!("00000000000000000000000000000000"),
    phrase:    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
    seed:    hex!("c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04"),
    xprv:    "xprv9s21ZrQH143K3h3fDYiay8mocZ3afhfULfb5GX8kCBdno77K4HiA15Tg23wpbeF1pLfs1c5SPmYHrEpTuuRhxMwvKDwqdKiGJS9XFKzUsAF"
 },
 
 TestVector12 {
    entropy: hex!("7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"),
    phrase: "legal winner thank year wave sausage worth useful legal winner thank yellow",
    seed:    hex!("2e8905819b8723fe2c1d161860e5ee1830318dbf49a83bd451cfb8440c28bd6fa457fe1296106559a3c80937a1c1069be3a3a5bd381ee6260e8d9739fce1f607"),
    xprv: "xprv9s21ZrQH143K2gA81bYFHqU68xz1cX2APaSq5tt6MFSLeXnCKV1RVUJt9FWNTbrrryem4ZckN8k4Ls1H6nwdvDTvnV7zEXs2HgPezuVccsq"
 },

 TestVector12 {
    entropy: hex!("80808080808080808080808080808080"),
    phrase: "letter advice cage absurd amount doctor acoustic avoid letter advice cage above",
    seed:    hex!("d71de856f81a8acc65e6fc851a38d4d7ec216fd0796d0a6827a3ad6ed5511a30fa280f12eb2e47ed2ac03b5c462a0358d18d69fe4f985ec81778c1b370b652a8"),
    xprv: "xprv9s21ZrQH143K2shfP28KM3nr5Ap1SXjz8gc2rAqqMEynmjt6o1qboCDpxckqXavCwdnYds6yBHZGKHv7ef2eTXy461PXUjBFQg6PrwY4Gzq"
 },
 TestVector12 {
    entropy: hex!("ffffffffffffffffffffffffffffffff"),
    phrase: "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong",
    seed:    hex!("ac27495480225222079d7be181583751e86f571027b0497b5b5d11218e0a8a13332572917f0f8e5a589620c6f15b11c61dee327651a14c34e18231052e48c069"),
    xprv: "xprv9s21ZrQH143K2V4oox4M8Zmhi2Fjx5XK4Lf7GKRvPSgydU3mjZuKGCTg7UPiBUD7ydVPvSLtg9hjp7MQTYsW67rZHAXeccqYqrsx8LcXnyd"
 },
 TestVector12 {
    entropy: hex!("9e885d952ad362caeb4efe34a8e91bd2"),
    phrase: "ozone drill grab fiber curtain grace pudding thank cruise elder eight picnic",
    seed:    hex!("274ddc525802f7c828d8ef7ddbcdc5304e87ac3535913611fbbfa986d0c9e5476c91689f9c8a54fd55bd38606aa6a8595ad213d4c9c9f9aca3fb217069a41028"),
    xprv: "xprv9s21ZrQH143K2oZ9stBYpoaZ2ktHj7jLz7iMqpgg1En8kKFTXJHsjxry1JbKH19YrDTicVwKPehFKTbmaxgVEc5TpHdS1aYhB2s9aFJBeJH"
 },

TestVector12 {
    entropy: hex!("23db8160a31d3e0dca3688ed941adbf3"),
    phrase: "cat swing flag economy stadium alone churn speed unique patch report train",
    seed:    hex!("deb5f45449e615feff5640f2e49f933ff51895de3b4381832b3139941c57b59205a42480c52175b6efcffaa58a2503887c1e8b363a707256bdd2b587b46541f5"),
    xprv:  "xprv9s21ZrQH143K4G28omGMogEoYgDQuigBo8AFHAGDaJdqQ99QKMQ5J6fYTMfANTJy6xBmhvsNZ1CJzRZ64PWbnTFUn6CDV2FxoMDLXdk95DQ"
},

TestVector12 {
    entropy: hex!("f30f8c1da665478f49b001d94c5fc452"),
    phrase: "vessel ladder alter error federal sibling chat ability sun glass valve picture",
    seed:    hex!("2aaa9242daafcee6aa9d7269f17d4efe271e1b9a529178d7dc139cd18747090bf9d60295d0ce74309a78852a9caadf0af48aae1c6253839624076224374bc63f"),
    xprv:"xprv9s21ZrQH143K2QWV9Wn8Vvs6jbqfF1YbTCdURQW9dLFKDovpKaKrqS3SEWsXCu6ZNky9PSAENg6c9AQYHcg4PjopRGGKmdD313ZHszymnps"
},

];



#[test]
fn test_mnemonic() {
    for vector in TEST_VECTORS {
        let mnemonic = Mnemonic::from_entropy(vector.entropy.to_vec(), Default::default());
        assert_eq!(mnemonic.is_ok(),true);
        assert_eq!(mnemonic.unwrap().phrase(), vector.phrase);
    }
    for vector in TEST_VECTORS12 {
        let mnemonic = Mnemonic::from_entropy(vector.entropy.to_vec(), Default::default());
        assert_eq!(mnemonic.is_ok(),true);
        assert_eq!(mnemonic.unwrap().phrase(), vector.phrase);
    }
}

#[test]
fn test_seed() {
    for vector in TEST_VECTORS {
        let mnemonic = Mnemonic::new(vector.phrase, Default::default()).unwrap();
        assert_eq!(
            &vector.seed,
            mnemonic.to_seed(TEST_VECTOR_PASSWORD).as_bytes()
        );
    }
    for vector in TEST_VECTORS12 {
        let mnemonic = Mnemonic::new(vector.phrase, Default::default()).unwrap();
        assert_eq!(
            &vector.seed,
            mnemonic.to_seed(TEST_VECTOR_PASSWORD).as_bytes()
        );
    }
}

#[test]
fn test_xprv() {
    for vector in TEST_VECTORS {
        let seed = Seed::new(vector.seed);
        let expected_xprv = vector.xprv.parse::<XPrv>().unwrap();
        let derived_xprv = XPrv::new(&seed).unwrap();
        assert_eq!(expected_xprv, derived_xprv);
    }

    for vector in TEST_VECTORS12 {
        let seed = Seed::new(vector.seed);
        let expected_xprv = vector.xprv.parse::<XPrv>().unwrap();
        let derived_xprv = XPrv::new(&seed).unwrap();
        assert_eq!(expected_xprv, derived_xprv);
    }
}
