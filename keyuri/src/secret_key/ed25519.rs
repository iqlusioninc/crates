use clear_on_drop::clear::Clear;

use super::AsSecretSlice;
use algorithm::ED25519_ALG_ID;
use error::Error;

/// Size of an Ed25519 secret key
pub const ED25519_SECKEY_SIZE: usize = 32;

/// Ed25519 secret key (i.e. compressed Edwards-y coordinate)
pub struct Ed25519SecretKey([u8; ED25519_SECKEY_SIZE]);

impl Ed25519SecretKey {
    /// Create a new Ed25519 secret key
    pub fn new(slice: &[u8]) -> Result<Self, Error> {
        if slice.len() != ED25519_SECKEY_SIZE {
            fail!(
                ParseError,
                "bad Ed25519 secret key length: {} (expected {})",
                slice.len(),
                ED25519_SECKEY_SIZE
            );
        }

        let mut digest_bytes = [0u8; ED25519_SECKEY_SIZE];
        digest_bytes.copy_from_slice(slice);

        Ok(Ed25519SecretKey(digest_bytes))
    }
}

impl AsSecretSlice for Ed25519SecretKey {
    fn as_secret_slice(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Drop for Ed25519SecretKey {
    fn drop(&mut self) {
        self.0.as_mut().clear()
    }
}

impl_encodable_secret_key!(Ed25519SecretKey, ED25519_ALG_ID);
