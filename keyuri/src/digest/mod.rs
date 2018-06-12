use encoding::Encodable;
use error::Error;

/// NIST SHA-2 family of hash functions
mod sha2;

pub use self::sha2::Sha256Digest;
use algorithm::SHA256_ALG_ID;

/// Digest (i.e. hash) algorithms
pub enum Digest {
    /// NIST SHA-2 with a 256-bit digest
    Sha256(Sha256Digest),
}

impl Digest {
    /// Create a new `Digest` for the given algorithm
    pub fn new(alg: &str, bytes: &[u8]) -> Result<Self, Error> {
        let result = match alg {
            SHA256_ALG_ID => Digest::Sha256(Sha256Digest::new(bytes)?),
            _ => fail!(AlgorithmInvalid, "{}", alg),
        };

        Ok(result)
    }

    /// Return a `Sha256Digest` if the underlying digest is SHA-256
    pub fn sha256_digest(&self) -> Option<&Sha256Digest> {
        match self {
            Digest::Sha256(ref digest) => Some(digest),
        }
    }

    /// Is this Digest a SHA-256 digest?
    pub fn is_sha256_digest(&self) -> bool {
        self.sha256_digest().is_some()
    }
}

impl Encodable for Digest {
    /// Serialize this `Digest` as a URI-encoded `String`
    fn to_uri_string(&self) -> String {
        match self {
            Digest::Sha256(ref digest) => digest.to_uri_string(),
        }
    }

    /// Serialize this `Digest` as a "dasherized" `String`
    fn to_dasherized_string(&self) -> String {
        match self {
            Digest::Sha256(ref digest) => digest.to_dasherized_string(),
        }
    }
}
