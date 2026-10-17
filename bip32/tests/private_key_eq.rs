//! Equality tests for `ExtendedPrivateKey`.

#![cfg(feature = "secp256k1")]

use bip32::{ExtendedKey, XPrv};
use subtle::ConstantTimeEq;

/// BIP-32 test vector 1 master key; only its public attributes are reused.
const XPRV: &str = "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi";

/// Build an `XPrv` with `base`'s attributes and a secret scalar of `n`.
fn xprv_with_secret(base: &ExtendedKey, n: u8) -> XPrv {
    let mut ek = base.clone();
    ek.key_bytes = [0u8; 33];
    ek.key_bytes[32] = n;
    XPrv::try_from(ek).unwrap()
}

#[test]
fn different_secret_keys_are_not_equal() {
    let base: ExtendedKey = XPRV.parse().unwrap();
    let a = xprv_with_secret(&base, 1);
    let b = xprv_with_secret(&base, 2);

    assert_eq!(a.attrs(), b.attrs());
    assert!(!bool::from(a.ct_eq(&b)));
    assert!(!bool::from(b.ct_eq(&a)));
    assert_ne!(a, b);
}

#[test]
fn identical_keys_are_equal() {
    let base: ExtendedKey = XPRV.parse().unwrap();
    let a = xprv_with_secret(&base, 1);
    let b = xprv_with_secret(&base, 1);

    assert!(bool::from(a.ct_eq(&b)));
    assert_eq!(a, b);
}
