//! Trait for deriving child keys on a given type.

use crate::{
    ChainCode, ChildNumber, Error, HmacSha512, KeyFingerprint, PrivateKeyBytes, Result, KEY_SIZE,
};
use hmac::{KeyInit, Mac};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

#[cfg(feature = "secp256k1")]
use {
    crate::XPub,
    k256::elliptic_curve::{group::prime::PrimeCurveAffine, sec1::ToEncodedPoint},
};

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

    /// Derive a child key from a parent key and a provided tweak value.
    fn derive_child(&self, other: PrivateKeyBytes) -> Result<Self>;

    /// Compute a 4-byte key fingerprint for this public key.
    ///
    /// Default implementation uses `RIPEMD160(SHA256(public_key))`.
    fn fingerprint(&self) -> KeyFingerprint {
        let digest = Ripemd160::digest(Sha256::digest(self.to_bytes()));
        digest[..4].try_into().expect("digest truncated")
    }

    /// Derive a tweak value that can be used to generate the child key (see [`derive_child`]).
    ///
    /// The `chain_code` is either a newly initialized one,
    /// or one obtained from the previous invocation of `derive_tweak()`
    /// (for a multi-level derivation).
    ///
    /// **Warning:** make sure that if you are creating a new `chain_code`, you are doing so
    /// in a cryptographically safe way.
    /// Normally this would be done according to BIP-39 (within [`ExtendedPrivateKey::new`]).
    ///
    /// **Note:** `child_number` cannot be a hardened one (will result in an error).
    fn derive_tweak(
        &self,
        chain_code: &ChainCode,
        child_number: ChildNumber,
    ) -> Result<(PrivateKeyBytes, ChainCode)> {
        if child_number.is_hardened() {
            // Cannot derive child public keys for hardened `ChildNumber`s
            return Err(Error::ChildNumber);
        }

        let mut hmac = HmacSha512::new_from_slice(chain_code).map_err(|_| Error::Crypto)?;

        hmac.update(&self.to_bytes());
        hmac.update(&child_number.to_bytes());

        let result = hmac.finalize().into_bytes();
        let (tweak_bytes, chain_code_bytes) = result.split_at(KEY_SIZE);

        // Note that at this point we are only asserting that `tweak_bytes` have the expected size.
        // Checking if it actually fits the curve scalar happens in `derive_child()`.
        let tweak = tweak_bytes.try_into()?;

        let chain_code = chain_code_bytes.try_into()?;

        Ok((tweak, chain_code))
    }
}

#[cfg(feature = "secp256k1")]
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

    fn derive_child(&self, other: PrivateKeyBytes) -> Result<Self> {
        let child_scalar =
            Option::<k256::NonZeroScalar>::from(k256::NonZeroScalar::from_repr(other.into()))
                .ok_or(Error::Crypto)?;

        let child_point = self.to_projective() + (k256::AffinePoint::generator() * *child_scalar);
        Self::from_affine(child_point.into()).map_err(|_| Error::Crypto)
    }
}

#[cfg(feature = "secp256k1")]
impl PublicKey for k256::ecdsa::VerifyingKey {
    fn from_bytes(bytes: PublicKeyBytes) -> Result<Self> {
        Ok(k256::ecdsa::VerifyingKey::from_sec1_bytes(&bytes)?)
    }

    fn to_bytes(&self) -> PublicKeyBytes {
        k256::CompressedPoint::from(self)
            .as_slice()
            .try_into()
            .expect("malformed key")
    }

    fn derive_child(&self, other: PrivateKeyBytes) -> Result<Self> {
        k256::PublicKey::from(self)
            .derive_child(other)
            .map(Into::into)
    }
}

#[cfg(feature = "secp256k1")]
impl From<XPub> for k256::ecdsa::VerifyingKey {
    fn from(xpub: XPub) -> k256::ecdsa::VerifyingKey {
        k256::ecdsa::VerifyingKey::from(&xpub)
    }
}

#[cfg(feature = "secp256k1")]
impl From<&XPub> for k256::ecdsa::VerifyingKey {
    fn from(xpub: &XPub) -> k256::ecdsa::VerifyingKey {
        *xpub.public_key()
    }
}

#[cfg(feature = "secp256k1-ffi")]
impl PublicKey for secp256k1_ffi::PublicKey {
    fn from_bytes(bytes: PublicKeyBytes) -> Result<Self> {
        Ok(secp256k1_ffi::PublicKey::from_slice(&bytes)?)
    }

    fn to_bytes(&self) -> PublicKeyBytes {
        self.serialize()
    }

    fn derive_child(&self, bytes: PrivateKeyBytes) -> Result<Self> {
        use secp256k1_ffi::{Secp256k1, VerifyOnly};

        let engine = Secp256k1::<VerifyOnly>::verification_only();
        let scalar = secp256k1_ffi::Scalar::from_be_bytes(bytes)?;
        Ok(self.add_exp_tweak(&engine, &scalar)?)
    }
}

/// `secp256k1-ffi` smoke tests
#[cfg(all(test, feature = "bip39", feature = "secp256k1-ffi"))]
mod tests {
    use hex_literal::hex;

    const SEED: [u8; 64] = hex!(
        "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a2
         9f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542"
    );

    type XPrv = crate::ExtendedPrivateKey<secp256k1_ffi::SecretKey>;

    #[test]
    fn secp256k1_ffi_xprv_derivation() {
        let path = "m/0/2147483647'/1/2147483646'/2";
        let xprv = XPrv::derive_from_path(&SEED, &path.parse().unwrap()).unwrap();

        assert_eq!(
            xprv.public_key(),
            "xpub6FnCn6nSzZAw5Tw7cgR9bi15UV96gLZhjDstkXXxvCLsUXBGXPdSnLFbdpq8p9HmGsApME5hQTZ3emM2rnY5agb9rXpVGyy3bdW6EEgAtqt".parse().unwrap()
        );
    }

    #[test]
    fn secp256k1_ffi_xpub_derivation() {
        let path = "m/0/2147483647'/1/2147483646'";
        let xprv = XPrv::derive_from_path(&SEED, &path.parse().unwrap()).unwrap();
        let xpub = xprv.public_key().derive_child(2.into()).unwrap();

        assert_eq!(
            xpub,
            "xpub6FnCn6nSzZAw5Tw7cgR9bi15UV96gLZhjDstkXXxvCLsUXBGXPdSnLFbdpq8p9HmGsApME5hQTZ3emM2rnY5agb9rXpVGyy3bdW6EEgAtqt".parse().unwrap()
        );
    }
}
