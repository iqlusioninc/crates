//! BIP32 test vectors.
//!
//! Sourced from: <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#Test_Vectors>
//!
//! Note: Test vector 1 is omitted (for now) because the seed is smaller than
//! what we currently support.
// TODO(tarcieri): test `xpub`s

use bip32::{Seed, XPrv};
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

    let key_m = derive_xprv(&seed, "m");
    assert_eq!(key_m, XPrv::new(&seed).unwrap());
    assert_eq!(
        key_m,
        "xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U".parse().unwrap()
    );

    assert_eq!(
        derive_xprv(&seed, "m/0"),
        "xprv9vHkqa6EV4sPZHYqZznhT2NPtPCjKuDKGY38FBWLvgaDx45zo9WQRUT3dKYnjwih2yJD9mkrocEZXo1ex8G81dwSM1fwqWpWkeS3v86pgKt".parse().unwrap()
    );

    assert_eq!(
        derive_xprv(&seed, "m/0/2147483647'"),
        "xprv9wSp6B7kry3Vj9m1zSnLvN3xH8RdsPP1Mh7fAaR7aRLcQMKTR2vidYEeEg2mUCTAwCd6vnxVrcjfy2kRgVsFawNzmjuHc2YmYRmagcEPdU9".parse().unwrap()
    );

    assert_eq!(
        derive_xprv(&seed, "m/0/2147483647'/1"),
        "xprv9zFnWC6h2cLgpmSA46vutJzBcfJ8yaJGg8cX1e5StJh45BBciYTRXSd25UEPVuesF9yog62tGAQtHjXajPPdbRCHuWS6T8XA2ECKADdw4Ef".parse().unwrap()
    );

    assert_eq!(
        derive_xprv(&seed, "m/0/2147483647'/1/2147483646'"),
        "xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc".parse().unwrap()
    );

    assert_eq!(
        derive_xprv(&seed, "m/0/2147483647'/1/2147483646'/2"),
        "xprvA2nrNbFZABcdryreWet9Ea4LvTJcGsqrMzxHx98MMrotbir7yrKCEXw7nadnHM8Dq38EGfSh6dqA9QWTyefMLEcBYJUuekgW4BYPJcr9E7j".parse().unwrap()
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

    let key_m = derive_xprv(&seed, "m");
    assert_eq!(key_m, XPrv::new(&seed).unwrap());
    assert_eq!(
        key_m,
        "xprv9s21ZrQH143K25QhxbucbDDuQ4naNntJRi4KUfWT7xo4EKsHt2QJDu7KXp1A3u7Bi1j8ph3EGsZ9Xvz9dGuVrtHHs7pXeTzjuxBrCmmhgC6".parse().unwrap()
    );

    assert_eq!(
        derive_xprv(&seed, "m/0'"),
        "xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L".parse().unwrap()
    );
}
