//! BIP39 seed values

use crate::{KeyMaterial, Path, KEY_SIZE};
use hmac::{Hmac, Mac};
use sha2::Sha512;
use zeroize::Zeroize;

/// Base derivation secret for BIP39 keys
const BIP39_BASE_DERIVATION_KEY: [u8; 12] = [
    0x42, 0x69, 0x74, 0x63, 0x6f, 0x69, 0x6e, 0x20, 0x73, 0x65, 0x65, 0x64,
];

/// Number of bytes of PBKDF2 output to extract
pub const SEED_SIZE: usize = 64;

/// BIP39 seeds
pub struct Seed(pub(crate) [u8; SEED_SIZE]);

impl Seed {
    /// Get the inner secret byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Derive a BIP32 subkey from this seed
    pub fn derive_subkey(self, path: impl AsRef<Path>) -> KeyMaterial {
        let mut hmac = Hmac::<Sha512>::new_varkey(&BIP39_BASE_DERIVATION_KEY).unwrap();
        hmac.input(&self.0);

        // Use the chain code of the derived key as the root key
        let root_key = KeyMaterial::from_bytes(&hmac.result().code()[KEY_SIZE..]).unwrap();

        root_key.derive_subkey(path)
    }
}

impl Drop for Seed {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}
