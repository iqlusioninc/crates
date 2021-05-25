//! BIP39 seed values

use crate::{KeyMaterial, Path, KEY_SIZE};
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha512;
use zeroize::Zeroize;

/// Base derivation secret for BIP39 keys.
#[cfg_attr(docsrs, doc(cfg(feature = "bip39")))]
pub const BIP39_DOMAIN_SEPARATOR: [u8; 12] = [
    0x42, 0x69, 0x74, 0x63, 0x6f, 0x69, 0x6e, 0x20, 0x73, 0x65, 0x65, 0x64,
];

/// BIP39 seeds.
#[cfg_attr(docsrs, doc(cfg(feature = "bip39")))]
pub struct Seed(pub(crate) [u8; Seed::SIZE]);

impl Seed {
    /// Number of bytes of PBKDF2 output to extract.
    pub const SIZE: usize = 64;

    /// Create a new seed from the given bytes.
    pub fn new(bytes: [u8; Seed::SIZE]) -> Self {
        Seed(bytes)
    }

    /// Get the inner secret byte slice
    pub fn as_bytes(&self) -> &[u8; Seed::SIZE] {
        &self.0
    }

    /// Derive a BIP32 subkey from this seed
    pub fn derive_subkey(self, path: impl AsRef<Path>) -> KeyMaterial {
        let mut hmac = Hmac::<Sha512>::new_from_slice(&BIP39_DOMAIN_SEPARATOR)
            .expect("HMAC key size incorrect");
        hmac.update(&self.0);

        // Use the chain code of the derived key as the root key
        let root_key = KeyMaterial::from_bytes(&hmac.finalize().into_bytes()[KEY_SIZE..]).unwrap();

        root_key.derive_subkey(path)
    }
}

impl AsRef<[u8; Seed::SIZE]> for Seed {
    fn as_ref(&self) -> &[u8; Seed::SIZE] {
        self.as_bytes()
    }
}

impl Drop for Seed {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}
