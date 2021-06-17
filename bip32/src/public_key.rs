//! Trait for deriving child keys on a given type.

use crate::{KeyFingerprint, Result, KEY_SIZE};
use core::convert::TryInto;
use ripemd160::Ripemd160;
use sha2::{Digest, Sha256};

#[cfg(feature = "secp256k1")]
use k256::elliptic_curve::sec1::ToEncodedPoint;

/// Bytes which represent a public key.
///
/// Includes an extra byte for an SEC1 tag.
pub type PublicKeyBytes = [u8; KEY_SIZE + 1];

/// Trait for key types which can be derived using BIP32.
pub trait PublicKey: Sized {
    /// Initialize this key from bytes.
    fn from_bytes(bytes: PublicKeyBytes) -> Result<Self>;

    /// Serialize this key as bytes.
    fn to_bytes(&self) -> PublicKeyBytes;

    /// Compute a 4-byte key fingerprint for this public key.
    ///
    /// Default implementation uses `RIPEMD160(SHA256(public_key))`.
    fn fingerprint(&self) -> KeyFingerprint {
        let digest = Ripemd160::digest(&Sha256::digest(&self.to_bytes()));
        digest[..4].try_into().expect("digest truncated")
    }
}

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
impl PublicKey for k256::PublicKey {
    fn from_bytes(bytes: PublicKeyBytes) -> Result<Self> {
        Ok(k256::PublicKey::from_sec1_bytes(&bytes)?)
    }

    fn to_bytes(&self) -> PublicKeyBytes {
        self.to_encoded_point(true)
            .as_bytes()
            .try_into()
            .expect("malformed public key")
    }
}

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
impl PublicKey for k256::ecdsa::VerifyingKey {
    fn from_bytes(bytes: PublicKeyBytes) -> Result<Self> {
        Ok(k256::ecdsa::VerifyingKey::from_sec1_bytes(&bytes)?)
    }

    fn to_bytes(&self) -> PublicKeyBytes {
        self.to_bytes().as_ref().try_into().expect("malformed key")
    }
}

#[cfg(feature = "secp256k1-ffi")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1-ffi")))]
impl PublicKey for secp256k1_ffi::PublicKey {
    fn from_bytes(bytes: PublicKeyBytes) -> Result<Self> {
        Ok(secp256k1_ffi::PublicKey::from_slice(&bytes)?)
    }

    fn to_bytes(&self) -> PublicKeyBytes {
        self.serialize()
    }
}

/// `secp256k1-ffi` smoke tests
#[cfg(all(test, feature = "secp256k1-ffi"))]
mod tests {
    use hex_literal::hex;

    type XPrv = crate::ExtendedPrivateKey<secp256k1_ffi::SecretKey>;

    #[test]
    fn secp256k1_ffi_derivation() {
        let seed = hex!(
            "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a2
             9f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542"
        );

        let path = "m/0/2147483647'/1/2147483646'/2";
        let xprv = XPrv::derive_child_from_seed(&seed, &path.parse().unwrap()).unwrap();

        assert_eq!(
            xprv.public_key(),
            "xpub6FnCn6nSzZAw5Tw7cgR9bi15UV96gLZhjDstkXXxvCLsUXBGXPdSnLFbdpq8p9HmGsApME5hQTZ3emM2rnY5agb9rXpVGyy3bdW6EEgAtqt".parse().unwrap()
        );
    }
}
