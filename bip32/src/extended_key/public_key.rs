//! Extended public keys

use crate::{
    ChildNumber, Error, ExtendedKey, ExtendedKeyAttrs, ExtendedPrivateKey, KeyFingerprint, Prefix,
    PrivateKey, PublicKey, PublicKeyBytes, Result,
};
use core::str::FromStr;

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};

/// Extended public secp256k1 ECDSA verification key.
#[cfg(feature = "secp256k1")]
pub type XPub = ExtendedPublicKey<k256::ecdsa::VerifyingKey>;

/// Extended public keys derived using BIP32.
///
/// Generic around a [`PublicKey`] type. When the `secp256k1` feature of this
/// crate is enabled, the [`XPub`] type provides a convenient alias for
/// extended ECDSA/secp256k1 public keys.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ExtendedPublicKey<K: PublicKey> {
    /// Derived public key
    public_key: K,

    /// Extended key attributes.
    attrs: ExtendedKeyAttrs,
}

impl<K> ExtendedPublicKey<K>
where
    K: PublicKey,
{
    /// Create a new extended public key from a public key and attributes.
    pub fn new(public_key: K, attrs: ExtendedKeyAttrs) -> Self {
        Self { public_key, attrs }
    }

    /// Obtain the non-extended public key value `K`.
    pub fn public_key(&self) -> &K {
        &self.public_key
    }

    /// Get attributes for this key such as depth, parent fingerprint,
    /// child number, and chain code.
    pub fn attrs(&self) -> &ExtendedKeyAttrs {
        &self.attrs
    }

    /// Compute a 4-byte key fingerprint for this extended public key.
    pub fn fingerprint(&self) -> KeyFingerprint {
        self.public_key().fingerprint()
    }

    /// Derive a child key for a particular [`ChildNumber`].
    pub fn derive_child(&self, child_number: ChildNumber) -> Result<Self> {
        let depth = self.attrs.depth.checked_add(1).ok_or(Error::Depth)?;
        let (tweak, chain_code) = self
            .public_key
            .derive_tweak(&self.attrs.chain_code, child_number)?;

        // We should technically loop here if the tweak is zero or overflows
        // the order of the underlying elliptic curve group, incrementing the
        // index, however per "Child key derivation (CKD) functions":
        // https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#child-key-derivation-ckd-functions
        //
        // > "Note: this has probability lower than 1 in 2^127."
        //
        // ...so instead, we simply return an error if this were ever to happen,
        // as the chances of it happening are vanishingly small.
        let public_key = self.public_key.derive_child(tweak)?;

        let attrs = ExtendedKeyAttrs {
            parent_fingerprint: self.public_key.fingerprint(),
            child_number,
            chain_code,
            depth,
        };

        Ok(ExtendedPublicKey { public_key, attrs })
    }

    /// Serialize the raw public key as a byte array (e.g. SEC1-encoded).
    pub fn to_bytes(&self) -> PublicKeyBytes {
        self.public_key.to_bytes()
    }

    /// Serialize this key as an [`ExtendedKey`].
    pub fn to_extended_key(&self, prefix: Prefix) -> ExtendedKey {
        ExtendedKey {
            prefix,
            attrs: self.attrs.clone(),
            key_bytes: self.to_bytes(),
        }
    }

    /// Serialize this key as a `String`.
    #[cfg(feature = "alloc")]
    pub fn to_string(&self, prefix: Prefix) -> String {
        self.to_extended_key(prefix).to_string()
    }
}

impl<K> From<&ExtendedPrivateKey<K>> for ExtendedPublicKey<K::PublicKey>
where
    K: PrivateKey,
{
    fn from(xprv: &ExtendedPrivateKey<K>) -> ExtendedPublicKey<K::PublicKey> {
        ExtendedPublicKey {
            public_key: xprv.private_key().public_key(),
            attrs: xprv.attrs().clone(),
        }
    }
}

impl<K> FromStr for ExtendedPublicKey<K>
where
    K: PublicKey,
{
    type Err = Error;

    fn from_str(xpub: &str) -> Result<Self> {
        ExtendedKey::from_str(xpub)?.try_into()
    }
}

impl<K> TryFrom<ExtendedKey> for ExtendedPublicKey<K>
where
    K: PublicKey,
{
    type Error = Error;

    #[cfg(any(feature = "secp256k1", feature = "secp256k1-ffi"))]
    fn try_from(extended_key: ExtendedKey) -> Result<ExtendedPublicKey<K>> {
        if extended_key.prefix.is_public() {
            Ok(ExtendedPublicKey {
                public_key: PublicKey::from_bytes(extended_key.key_bytes)?,
                attrs: extended_key.attrs.clone(),
            })
        } else if extended_key.prefix.is_private() {
            #[cfg(feature = "secp256k1")]
            let private_key = crate::XPrv::try_from(extended_key)?;
            #[cfg(all(feature = "secp256k1-ffi", not(feature = "secp256k1")))]
            let private_key =
                ExtendedPrivateKey::<secp256k1_ffi::SecretKey>::try_from(extended_key)?;
            let pubkey_bytes = private_key.public_key().to_bytes();
            Ok(ExtendedPublicKey {
                public_key: PublicKey::from_bytes(pubkey_bytes)?,
                attrs: private_key.attrs().clone(),
            })
        } else {
            Err(Error::Crypto)
        }
    }

    #[cfg(not(any(feature = "secp256k1", feature = "secp256k1-ffi")))]
    fn try_from(extended_key: ExtendedKey) -> Result<ExtendedPublicKey<K>> {
        if extended_key.prefix.is_public() {
            Ok(ExtendedPublicKey {
                public_key: PublicKey::from_bytes(extended_key.key_bytes)?,
                attrs: extended_key.attrs.clone(),
            })
        } else {
            Err(Error::Crypto)
        }
    }
}

#[cfg(any(feature = "secp256k1", feature = "secp256k1-ffi"))]
#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    const CHAIN_CODE: [u8; 32] =
        hex!("873DFF81C02F525623FD1FE5167EAC3A55A049DE3D314BB42EE227FFED37D508");
    #[cfg(feature = "secp256k1")]
    const XPUB_BASE58: &'static str = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhe\
                                       PY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";
    const XPRV_BASE58: &'static str = "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPP\
                                       qjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi";
    const PUB_KEY_BYTES: PublicKeyBytes =
        hex!("0339A36013301597DAEF41FBE593A02CC513D0B55527EC2DF1050E2E8FF49C85C2");

    #[cfg(feature = "secp256k1")]
    #[test]
    fn extendedpub_from_pub_xkey() {
        let xkey: ExtendedKey = XPUB_BASE58.parse().unwrap();
        let xpub = XPub::try_from(xkey).unwrap();

        assert_eq!(xpub.attrs.depth, 0);
        assert_eq!(xpub.attrs.parent_fingerprint, [0u8; 4]);
        assert_eq!(xpub.attrs.child_number.0, 0);
        assert_eq!(xpub.attrs.chain_code, CHAIN_CODE);
        assert_eq!(xpub.to_bytes(), PUB_KEY_BYTES);
    }

    #[cfg(feature = "secp256k1")]
    #[test]
    fn extendedpub_from_priv_xkey() {
        let xkey: ExtendedKey = XPRV_BASE58.parse().unwrap();
        let xpub = XPub::try_from(xkey).unwrap();

        assert_eq!(xpub.attrs.depth, 0);
        assert_eq!(xpub.attrs.parent_fingerprint, [0u8; 4]);
        assert_eq!(xpub.attrs.child_number.0, 0);
        assert_eq!(xpub.attrs.chain_code, CHAIN_CODE);
        assert_eq!(xpub.to_bytes(), PUB_KEY_BYTES);
    }

    #[cfg(feature = "secp256k1-ffi")]
    #[test]
    fn extendedpub_from_priv_xkey_secp() {
        use secp256k1_ffi::PublicKey;

        let xkey: ExtendedKey = XPRV_BASE58.parse().unwrap();
        let xpub = ExtendedPublicKey::<PublicKey>::try_from(xkey).unwrap();

        assert_eq!(xpub.attrs.depth, 0);
        assert_eq!(xpub.attrs.parent_fingerprint, [0u8; 4]);
        assert_eq!(xpub.attrs.child_number.0, 0);
        assert_eq!(xpub.attrs.chain_code, CHAIN_CODE);
        assert_eq!(xpub.to_bytes(), PUB_KEY_BYTES);
    }
}
