use error::Error;

use algorithm::ED25519_ALG_ID;

/// Size of an Ed25519 public key
pub const ED25519_PUBKEY_SIZE: usize = 32;

/// Ed25519 public key (i.e. compressed Edwards-y coordinate)
pub struct Ed25519PublicKey(pub [u8; ED25519_PUBKEY_SIZE]);

impl Ed25519PublicKey {
    /// Create a new Ed25519 public key
    pub fn new(slice: &[u8]) -> Result<Self, Error> {
        if slice.len() != ED25519_PUBKEY_SIZE {
            fail!(
                ParseError,
                "bad Ed25519 public key length: {} (expected {})",
                slice.len(),
                ED25519_PUBKEY_SIZE
            );
        }

        let mut digest_bytes = [0u8; ED25519_PUBKEY_SIZE];
        digest_bytes.copy_from_slice(slice);

        Ok(Ed25519PublicKey(digest_bytes))
    }
}

impl AsRef<[u8]> for Ed25519PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl_encodable_public_key!(Ed25519PublicKey, ED25519_ALG_ID);
