//! BIP32 test vectors.
//!
//! Sourced from: <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#Test_Vectors>
//!
//! Note: Test vector 1 is omitted (for now) because the seed is smaller than
//! what we currently support.
// TODO(tarcieri): test `xpub`s, add test vector 1, consolidate test vectors

#![cfg(feature = "secp256k1")]

use bip32::{Prefix, Seed, XPrv};
use hex_literal::hex;

/// Derive an [`XPrv`] for the given seed and derivation path.
///
/// Panics if anything goes wrong.
fn derive_xprv(seed: &Seed, path: &str) -> XPrv {
    XPrv::derive_child_from_path(&seed, &path.parse().unwrap()).unwrap()
}

/// BIP32 test vector 2
/// <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#test-vector-2>
#[test]
fn test_vector_2() {
    let seed = Seed::new(hex!(
        "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a2
         9f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542"
    ));

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
}

/// BIP32 test vector 3
/// <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#test-vector-3>
///
/// These vectors test for the retention of leading zeros. See:
/// - <https://github.com/bitpay/bitcore-lib/issues/47>
/// - <https://github.com/iancoleman/bip39/issues/58>
#[test]
fn test_vector_3() {
    let seed = Seed::new(hex!(
        "4b381541583be4423346c643850da4b320e46a87ae3d2a4e6da11eba819cd4ac
         ba45d239319ac14f863b8d5ab5a0d0c64d2e8a1e7d1457df2e5a3c51c73235be"
    ));

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
