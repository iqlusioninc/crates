use algorithm::ED25519_ALG_ID;
use encoding::Encodable;
use error::Error;

/// Ed25519 elliptic curve digital signature algorithm (RFC 8032)
mod ed25519;

pub use self::ed25519::Ed25519PublicKey;

/// Public key algorithms
pub enum PublicKey {
    /// Ed25519 (RFC 8032) public key
    Ed25519(Ed25519PublicKey),
}

impl PublicKey {
    /// Create a new `PublicKey` for the given algorithm
    pub fn new(alg: &str, bytes: &[u8]) -> Result<Self, Error> {
        let result = match alg {
            ED25519_ALG_ID => PublicKey::Ed25519(Ed25519PublicKey::new(bytes)?),
            _ => fail!(AlgorithmInvalid, "{}", alg),
        };

        Ok(result)
    }

    /// Return an `Ed25519PublicKey` if the underlying public key is Ed25519
    pub fn ed25519_key(&self) -> Option<&Ed25519PublicKey> {
        match self {
            PublicKey::Ed25519(ref digest) => Some(digest),
        }
    }

    /// Is this `PublicKey` an Ed25519 public key?
    pub fn is_ed25519_key(&self) -> bool {
        self.ed25519_key().is_some()
    }
}

impl Encodable for PublicKey {
    /// Serialize this `PublicKey` as a URI-encoded `String`
    fn to_uri_string(&self) -> String {
        match self {
            PublicKey::Ed25519(ref key) => key.to_uri_string(),
        }
    }

    /// Serialize this `PublicKey` as a "dasherized" `String`
    fn to_dasherized_string(&self) -> String {
        match self {
            PublicKey::Ed25519(ref key) => key.to_dasherized_string(),
        }
    }
}
