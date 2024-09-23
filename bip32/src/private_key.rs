//! Trait for deriving child keys on a given type.

use crate::{ChainCode, ChildNumber, Error, HmacSha512, PublicKey, Result, KEY_SIZE};
use hmac::{KeyInit, Mac};

#[cfg(feature = "secp256k1")]
use crate::XPrv;

/// Bytes which represent a private key.
pub type PrivateKeyBytes = [u8; KEY_SIZE];

/// Trait for key types which can be derived using BIP32.
pub trait PrivateKey: Sized {
    /// Public key type which corresponds to this private key.
    type PublicKey: PublicKey;

    /// Initialize this key from bytes.
    fn from_bytes(bytes: &PrivateKeyBytes) -> Result<Self>;

    /// Serialize this key as bytes.
    fn to_bytes(&self) -> PrivateKeyBytes;

    /// Derive a child key from a parent key and the a provided tweak value,
    /// i.e. where `other` is referred to as "I sub L" in BIP32 and sourced
    /// from the left half of the HMAC-SHA-512 output.
    fn derive_child(&self, other: PrivateKeyBytes) -> Result<Self>;

    /// Get the [`Self::PublicKey`] that corresponds to this private key.
    fn public_key(&self) -> Self::PublicKey;

    /// Derive a tweak value that can be used to generate the child key (see [`derive_child`]).
    ///
    /// The `chain_code` is either a newly initialized one,
    /// or one obtained from the previous invocation of `derive_tweak()`
    /// (for a multi-level derivation).
    ///
    /// **Warning:** make sure that if you are creating a new `chain_code`, you are doing so
    /// in a cryptographically safe way.
    /// Normally this would be done according to BIP-39 (within [`ExtendedPrivateKey::new`]).
    fn derive_tweak(
        &self,
        chain_code: &ChainCode,
        child_number: ChildNumber,
    ) -> Result<(PrivateKeyBytes, ChainCode)> {
        let mut hmac = HmacSha512::new_from_slice(chain_code).map_err(|_| Error::Crypto)?;

        if child_number.is_hardened() {
            hmac.update(&[0]);
            hmac.update(&self.to_bytes());
        } else {
            hmac.update(&self.public_key().to_bytes());
        }

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
impl PrivateKey for k256::SecretKey {
    type PublicKey = k256::PublicKey;

    fn from_bytes(bytes: &PrivateKeyBytes) -> Result<Self> {
        Ok(k256::SecretKey::from_slice(bytes)?)
    }

    fn to_bytes(&self) -> PrivateKeyBytes {
        k256::SecretKey::to_bytes(self).into()
    }

    fn derive_child(&self, other: PrivateKeyBytes) -> Result<Self> {
        let child_scalar =
            Option::<k256::NonZeroScalar>::from(k256::NonZeroScalar::from_repr(other.into()))
                .ok_or(Error::Crypto)?;

        let derived_scalar = self.to_nonzero_scalar().as_ref() + child_scalar.as_ref();

        Option::<k256::NonZeroScalar>::from(k256::NonZeroScalar::new(derived_scalar))
            .map(Into::into)
            .ok_or(Error::Crypto)
    }

    fn public_key(&self) -> Self::PublicKey {
        k256::SecretKey::public_key(self)
    }
}

#[cfg(feature = "secp256k1")]
impl PrivateKey for k256::ecdsa::SigningKey {
    type PublicKey = k256::ecdsa::VerifyingKey;

    fn from_bytes(bytes: &PrivateKeyBytes) -> Result<Self> {
        Ok(k256::ecdsa::SigningKey::from_slice(bytes)?)
    }

    fn to_bytes(&self) -> PrivateKeyBytes {
        k256::ecdsa::SigningKey::to_bytes(self).into()
    }

    fn derive_child(&self, other: PrivateKeyBytes) -> Result<Self> {
        k256::SecretKey::from(self)
            .derive_child(other)
            .map(Into::into)
    }

    fn public_key(&self) -> Self::PublicKey {
        *self.verifying_key()
    }
}

#[cfg(feature = "secp256k1")]
impl From<XPrv> for k256::ecdsa::SigningKey {
    fn from(xprv: XPrv) -> k256::ecdsa::SigningKey {
        k256::ecdsa::SigningKey::from(&xprv)
    }
}

#[cfg(feature = "secp256k1")]
impl From<&XPrv> for k256::ecdsa::SigningKey {
    fn from(xprv: &XPrv) -> k256::ecdsa::SigningKey {
        xprv.private_key().clone()
    }
}

#[cfg(feature = "secp256k1-ffi")]
impl PrivateKey for secp256k1_ffi::SecretKey {
    type PublicKey = secp256k1_ffi::PublicKey;

    fn from_bytes(bytes: &PrivateKeyBytes) -> Result<Self> {
        Ok(secp256k1_ffi::SecretKey::from_slice(bytes)?)
    }

    fn to_bytes(&self) -> PrivateKeyBytes {
        *self.as_ref()
    }

    fn derive_child(&self, bytes: PrivateKeyBytes) -> Result<Self> {
        let scalar = secp256k1_ffi::Scalar::from_be_bytes(bytes)?;
        Ok(self.add_tweak(&scalar)?)
    }

    fn public_key(&self) -> Self::PublicKey {
        use secp256k1_ffi::{Secp256k1, SignOnly};
        let engine = Secp256k1::<SignOnly>::signing_only();
        secp256k1_ffi::PublicKey::from_secret_key(&engine, self)
    }
}

/// `secp256k1-ffi` smoke tests
#[cfg(all(test, feature = "bip39", feature = "secp256k1-ffi"))]
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
        let xprv = XPrv::derive_from_path(&seed, &path.parse().unwrap()).unwrap();

        assert_eq!(
            xprv,
            "xprvA2nrNbFZABcdryreWet9Ea4LvTJcGsqrMzxHx98MMrotbir7yrKCEXw7nadnHM8Dq38EGfSh6dqA9QWTyefMLEcBYJUuekgW4BYPJcr9E7j".parse().unwrap()
        );
    }
}
