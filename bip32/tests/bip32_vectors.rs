//! BIP32 Test Vectors.
//!
//! Sourced from: <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#Test_Vectors>
//!
//! Note: Test vector 1 is omitted (for now) because the seed is smaller than
//! what we currently support.
// TODO(tarcieri): consolidate test vectors

#![cfg(all(feature = "alloc", feature = "secp256k1"))]

use bip32::{Prefix, XPrv};
use hex_literal::hex;

/// Derive an [`XPrv`] for the given seed and derivation path.
///
/// Panics if anything goes wrong.
fn derive_xprv(seed: &[u8], path: &str) -> XPrv {
    XPrv::derive_from_path(&seed, &path.parse().unwrap()).unwrap()
}

/// BIP32 Test Vector 1
/// <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#Test_vector_1>
#[test]
fn test_vector_1() {
    let seed = hex!("000102030405060708090a0b0c0d0e0f");

    // Chain m
    let key_m = derive_xprv(&seed, "m");
    assert_eq!(key_m, XPrv::new(&seed).unwrap());
    assert_eq!(
        &*key_m.to_string(Prefix::XPRV),
        "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi"
    );
    assert_eq!(
        key_m.public_key().to_string(Prefix::XPUB),
        "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8"
    );

    // Chain m/0'
    let key_m_0h = derive_xprv(&seed, "m/0'");
    assert_eq!(
        &*key_m_0h.to_string(Prefix::XPRV),
        "xprv9uHRZZhk6KAJC1avXpDAp4MDc3sQKNxDiPvvkX8Br5ngLNv1TxvUxt4cV1rGL5hj6KCesnDYUhd7oWgT11eZG7XnxHrnYeSvkzY7d2bhkJ7"
    );
    assert_eq!(
        key_m_0h.public_key().to_string(Prefix::XPUB),
        "xpub68Gmy5EdvgibQVfPdqkBBCHxA5htiqg55crXYuXoQRKfDBFA1WEjWgP6LHhwBZeNK1VTsfTFUHCdrfp1bgwQ9xv5ski8PX9rL2dZXvgGDnw"
    );

    // Chain m/0'/1
    let key_m_0h_1 = derive_xprv(&seed, "m/0'/1");
    assert_eq!(
        &*key_m_0h_1.to_string(Prefix::XPRV),
        "xprv9wTYmMFdV23N2TdNG573QoEsfRrWKQgWeibmLntzniatZvR9BmLnvSxqu53Kw1UmYPxLgboyZQaXwTCg8MSY3H2EU4pWcQDnRnrVA1xe8fs"
    );
    assert_eq!(
        key_m_0h_1.public_key().to_string(Prefix::XPUB),
        "xpub6ASuArnXKPbfEwhqN6e3mwBcDTgzisQN1wXN9BJcM47sSikHjJf3UFHKkNAWbWMiGj7Wf5uMash7SyYq527Hqck2AxYysAA7xmALppuCkwQ"
    );

    // Chain m/0'/1/2'
    let key_m_0h_1_2h = derive_xprv(&seed, "m/0'/1/2'");
    assert_eq!(
        &*key_m_0h_1_2h.to_string(Prefix::XPRV),
        "xprv9z4pot5VBttmtdRTWfWQmoH1taj2axGVzFqSb8C9xaxKymcFzXBDptWmT7FwuEzG3ryjH4ktypQSAewRiNMjANTtpgP4mLTj34bhnZX7UiM"
    );
    assert_eq!(
        key_m_0h_1_2h.public_key().to_string(Prefix::XPUB),
        "xpub6D4BDPcP2GT577Vvch3R8wDkScZWzQzMMUm3PWbmWvVJrZwQY4VUNgqFJPMM3No2dFDFGTsxxpG5uJh7n7epu4trkrX7x7DogT5Uv6fcLW5"
    );

    // Chain m/0'/1/2'/2
    let key_m_0h_1_2h_2 = derive_xprv(&seed, "m/0'/1/2'/2");
    assert_eq!(
        &*key_m_0h_1_2h_2.to_string(Prefix::XPRV),
        "xprvA2JDeKCSNNZky6uBCviVfJSKyQ1mDYahRjijr5idH2WwLsEd4Hsb2Tyh8RfQMuPh7f7RtyzTtdrbdqqsunu5Mm3wDvUAKRHSC34sJ7in334"
    );
    assert_eq!(
        key_m_0h_1_2h_2.public_key().to_string(Prefix::XPUB),
        "xpub6FHa3pjLCk84BayeJxFW2SP4XRrFd1JYnxeLeU8EqN3vDfZmbqBqaGJAyiLjTAwm6ZLRQUMv1ZACTj37sR62cfN7fe5JnJ7dh8zL4fiyLHV"
    );

    // Chain m/0'/1/2'/2/1000000000
    let key_m_0h_1_2h_2_1000000000 = derive_xprv(&seed, "m/0'/1/2'/2/1000000000");
    assert_eq!(
        &*key_m_0h_1_2h_2_1000000000.to_string(Prefix::XPRV),
        "xprvA41z7zogVVwxVSgdKUHDy1SKmdb533PjDz7J6N6mV6uS3ze1ai8FHa8kmHScGpWmj4WggLyQjgPie1rFSruoUihUZREPSL39UNdE3BBDu76"
    );
    assert_eq!(
        key_m_0h_1_2h_2_1000000000.public_key().to_string(Prefix::XPUB),
        "xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy"
    );

    // Test non-hardened derivation from an xpub
    let xpub = key_m_0h_1_2h_2.public_key();
    let xpub_child = xpub.derive_child(1000000000.into()).unwrap();
    assert_eq!(key_m_0h_1_2h_2_1000000000.public_key(), xpub_child);
}

/// BIP32 Test Vector 2
/// <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#Test_vector_2>
#[test]
fn test_vector_2() {
    let seed = hex!(
        "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a2
         9f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542"
    );

    // Chain m
    let key_m = derive_xprv(&seed, "m");
    assert_eq!(key_m, XPrv::new(&seed).unwrap());
    assert_eq!(
        &*key_m.to_string(Prefix::XPRV),
        "xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U"
    );
    assert_eq!(
        key_m.public_key().to_string(Prefix::XPUB),
        "xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB"
    );

    // Chain m/0
    let key_m_0 = derive_xprv(&seed, "m/0");
    assert_eq!(
        &*key_m_0.to_string(Prefix::XPRV),
        "xprv9vHkqa6EV4sPZHYqZznhT2NPtPCjKuDKGY38FBWLvgaDx45zo9WQRUT3dKYnjwih2yJD9mkrocEZXo1ex8G81dwSM1fwqWpWkeS3v86pgKt"
    );
    assert_eq!(
        key_m_0.public_key().to_string(Prefix::XPUB),
        "xpub69H7F5d8KSRgmmdJg2KhpAK8SR3DjMwAdkxj3ZuxV27CprR9LgpeyGmXUbC6wb7ERfvrnKZjXoUmmDznezpbZb7ap6r1D3tgFxHmwMkQTPH"
    );

    // Chain m/0/2147483647'
    let key_m_0_2147483647h = derive_xprv(&seed, "m/0/2147483647'");
    assert_eq!(
        &*key_m_0_2147483647h.to_string(Prefix::XPRV),
        "xprv9wSp6B7kry3Vj9m1zSnLvN3xH8RdsPP1Mh7fAaR7aRLcQMKTR2vidYEeEg2mUCTAwCd6vnxVrcjfy2kRgVsFawNzmjuHc2YmYRmagcEPdU9"
    );
    assert_eq!(
        key_m_0_2147483647h.public_key().to_string(Prefix::XPUB),
        "xpub6ASAVgeehLbnwdqV6UKMHVzgqAG8Gr6riv3Fxxpj8ksbH9ebxaEyBLZ85ySDhKiLDBrQSARLq1uNRts8RuJiHjaDMBU4Zn9h8LZNnBC5y4a"
    );

    // Chain m/0/2147483647'/1
    let key_m_0_2147483647h_1 = derive_xprv(&seed, "m/0/2147483647'/1");
    assert_eq!(
        &*key_m_0_2147483647h_1.to_string(Prefix::XPRV),
        "xprv9zFnWC6h2cLgpmSA46vutJzBcfJ8yaJGg8cX1e5StJh45BBciYTRXSd25UEPVuesF9yog62tGAQtHjXajPPdbRCHuWS6T8XA2ECKADdw4Ef"
    );
    assert_eq!(
        key_m_0_2147483647h_1.public_key().to_string(Prefix::XPUB),
        "xpub6DF8uhdarytz3FWdA8TvFSvvAh8dP3283MY7p2V4SeE2wyWmG5mg5EwVvmdMVCQcoNJxGoWaU9DCWh89LojfZ537wTfunKau47EL2dhHKon"
    );

    // Chain m/0/2147483647'/1/2147483646'
    let key_m_0_2147483647h_1_2147483646h = derive_xprv(&seed, "m/0/2147483647'/1/2147483646'");
    assert_eq!(
        &*key_m_0_2147483647h_1_2147483646h.to_string(Prefix::XPRV),
        "xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc"
    );
    assert_eq!(
        key_m_0_2147483647h_1_2147483646h.public_key().to_string(Prefix::XPUB),
        "xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL"
    );

    // Chain m/0/2147483647'/1/2147483646'/2
    let key_m_0_2147483647h_1_2147483646h_2 = derive_xprv(&seed, "m/0/2147483647'/1/2147483646'/2");
    assert_eq!(
        &*key_m_0_2147483647h_1_2147483646h_2.to_string(Prefix::XPRV),
        "xprvA2nrNbFZABcdryreWet9Ea4LvTJcGsqrMzxHx98MMrotbir7yrKCEXw7nadnHM8Dq38EGfSh6dqA9QWTyefMLEcBYJUuekgW4BYPJcr9E7j"
    );
    assert_eq!(
        key_m_0_2147483647h_1_2147483646h_2.public_key().to_string(Prefix::XPUB),
        "xpub6FnCn6nSzZAw5Tw7cgR9bi15UV96gLZhjDstkXXxvCLsUXBGXPdSnLFbdpq8p9HmGsApME5hQTZ3emM2rnY5agb9rXpVGyy3bdW6EEgAtqt"
    );

    // Test non-hardened derivation from an xpub
    let xpub = key_m_0_2147483647h_1_2147483646h.public_key();
    let xpub_child = xpub.derive_child(2.into()).unwrap();
    assert_eq!(key_m_0_2147483647h_1_2147483646h_2.public_key(), xpub_child);
}

/// BIP32 Test Vector 3
/// <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#Test_vector_3>
///
/// These vectors test for the retention of leading zeros. See:
/// - <https://github.com/bitpay/bitcore-lib/issues/47>
/// - <https://github.com/iancoleman/bip39/issues/58>
#[test]
fn test_vector_3() {
    let seed = hex!(
        "4b381541583be4423346c643850da4b320e46a87ae3d2a4e6da11eba819cd4ac
         ba45d239319ac14f863b8d5ab5a0d0c64d2e8a1e7d1457df2e5a3c51c73235be"
    );

    // Chain m
    let key_m = derive_xprv(&seed, "m");
    assert_eq!(key_m, XPrv::new(&seed).unwrap());
    assert_eq!(
        &*key_m.to_string(Prefix::XPRV),
        "xprv9s21ZrQH143K25QhxbucbDDuQ4naNntJRi4KUfWT7xo4EKsHt2QJDu7KXp1A3u7Bi1j8ph3EGsZ9Xvz9dGuVrtHHs7pXeTzjuxBrCmmhgC6"
    );
    assert_eq!(
        key_m.public_key().to_string(Prefix::XPUB),
        "xpub661MyMwAqRbcEZVB4dScxMAdx6d4nFc9nvyvH3v4gJL378CSRZiYmhRoP7mBy6gSPSCYk6SzXPTf3ND1cZAceL7SfJ1Z3GC8vBgp2epUt13"
    );

    // Chain m/0'
    let key_m_0h = derive_xprv(&seed, "m/0'");
    assert_eq!(
        &*key_m_0h.to_string(Prefix::XPRV),
        "xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L"
    );
    assert_eq!(
        key_m_0h.public_key().to_string(Prefix::XPUB),
        "xpub68NZiKmJWnxxS6aaHmn81bvJeTESw724CRDs6HbuccFQN9Ku14VQrADWgqbhhTHBaohPX4CjNLf9fq9MYo6oDaPPLPxSb7gwQN3ih19Zm4Y"
    );
}

/// BIP32 Test Vector 4
/// <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#test-vector-4>
///
/// These vectors test for the retention of leading zeros. See:
/// <https://github.com/btcsuite/btcutil/issues/172>
#[test]
fn test_vector_4() {
    let seed = hex!("3ddd5602285899a946114506157c7997e5444528f3003f6134712147db19b678");

    // Chain m
    let key_m = derive_xprv(&seed, "m");
    assert_eq!(key_m, XPrv::new(&seed).unwrap());
    assert_eq!(
        &*key_m.to_string(Prefix::XPRV),
        "xprv9s21ZrQH143K48vGoLGRPxgo2JNkJ3J3fqkirQC2zVdk5Dgd5w14S7fRDyHH4dWNHUgkvsvNDCkvAwcSHNAQwhwgNMgZhLtQC63zxwhQmRv"
    );
    assert_eq!(
        key_m.public_key().to_string(Prefix::XPUB),
        "xpub661MyMwAqRbcGczjuMoRm6dXaLDEhW1u34gKenbeYqAix21mdUKJyuyu5F1rzYGVxyL6tmgBUAEPrEz92mBXjByMRiJdba9wpnN37RLLAXa"
    );

    // Chain m/0'
    let key_m_0h = derive_xprv(&seed, "m/0'");
    assert_eq!(
        &*key_m_0h.to_string(Prefix::XPRV),
        "xprv9vB7xEWwNp9kh1wQRfCCQMnZUEG21LpbR9NPCNN1dwhiZkjjeGRnaALmPXCX7SgjFTiCTT6bXes17boXtjq3xLpcDjzEuGLQBM5ohqkao9G"
    );
    assert_eq!(
        key_m_0h.public_key().to_string(Prefix::XPUB),
        "xpub69AUMk3qDBi3uW1sXgjCmVjJ2G6WQoYSnNHyzkmdCHEhSZ4tBok37xfFEqHd2AddP56Tqp4o56AePAgCjYdvpW2PU2jbUPFKsav5ut6Ch1m"
    );

    // Chain m/0'/1'
    let key_m_0h_1h = derive_xprv(&seed, "m/0'/1'");
    assert_eq!(
        &*key_m_0h_1h.to_string(Prefix::XPRV),
        "xprv9xJocDuwtYCMNAo3Zw76WENQeAS6WGXQ55RCy7tDJ8oALr4FWkuVoHJeHVAcAqiZLE7Je3vZJHxspZdFHfnBEjHqU5hG1Jaj32dVoS6XLT1"
    );
    assert_eq!(
        key_m_0h_1h.public_key().to_string(Prefix::XPUB),
        "xpub6BJA1jSqiukeaesWfxe6sNK9CCGaujFFSJLomWHprUL9DePQ4JDkM5d88n49sMGJxrhpjazuXYWdMf17C9T5XnxkopaeS7jGk1GyyVziaMt"
    );
}
