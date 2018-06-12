use error::Error;

use algorithm::SHA256_ALG_ID;

/// Size of a SHA-256 digest
pub const SHA256_DIGEST_SIZE: usize = 32;

/// NIST SHA-256 digests
pub struct Sha256Digest(pub [u8; SHA256_DIGEST_SIZE]);

impl Sha256Digest {
    /// Create a new SHA-256 digest
    pub fn new(slice: &[u8]) -> Result<Self, Error> {
        if slice.len() != SHA256_DIGEST_SIZE {
            fail!(
                ParseError,
                "bad SHA-256 digest length: {} (expected {})",
                slice.len(),
                SHA256_DIGEST_SIZE
            );
        }

        let mut digest_bytes = [0u8; SHA256_DIGEST_SIZE];
        digest_bytes.copy_from_slice(slice);

        Ok(Sha256Digest(digest_bytes))
    }
}

impl AsRef<[u8]> for Sha256Digest {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl_encodable_digest!(Sha256Digest, SHA256_ALG_ID);
