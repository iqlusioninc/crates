//! Keys-as-URIs (using bech32-encoded binary data with checksums)

#![crate_name = "keyuri"]
#![crate_type = "rlib"]
#![allow(unknown_lints, suspicious_arithmetic_impl)]
#![deny(
    warnings,
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#![doc(html_root_url = "https://docs.rs/keyuri/0.0.0")]

extern crate clear_on_drop;
#[macro_use]
extern crate failure;
extern crate iq_bech32;

use clear_on_drop::clear::Clear;
use iq_bech32::Bech32;

//
// Modules with macros
//

/// Error types (defined first so macros are available)
#[macro_use]
pub mod error;

/// String encoding configuration
#[macro_use]
mod encoding;

//
// Modules without macros
//

/// Cryptographic algorithm registry
pub mod algorithm;

/// Digest algorithms (i.e. hash functions) for key digests
pub mod digest;

/// Public key types
pub mod public_key;

/// Secret key types
pub mod secret_key;

/// Signature types
pub mod signature;

pub use encoding::Encodable;
pub use secret_key::AsSecretSlice;

use digest::Digest;
use encoding::{Encoding, DASHERIZED_ENCODING, URI_ENCODING};
use error::Error;
use public_key::PublicKey;
use secret_key::SecretKey;
use signature::Signature;

/// Keys-as-URIs: a serialization format for cryptographic keys and digests
pub struct KeyURI {
    /// Kind of KeyURI (e.g. secret key, public key, digests, signatures)
    kind: KeyURIKind,

    /// URI fragment (i.e. everything after `#`)
    fragment: Option<String>,
}

/// Kinds of KeyURIs
pub enum KeyURIKind {
    /// Digests (e.g. key digests symmetric secret keys, asymmetric public keys, or other data)
    Digest(Digest),

    /// Public keys (always asymmetric)
    PublicKey(PublicKey),

    /// Secret keys (symmetric or asymmetric)
    SecretKey(SecretKey),

    /// Digital signatures (always asymmetric)
    Signature(Signature),
}

impl KeyURI {
    /// Parse a KeyURI from a bech32 encoded string using the given encoding
    // TODO: parser generator rather than handrolling this?
    fn parse(uri: &str, encoding: &Encoding) -> Result<Self, Error> {
        let (prefix, mut data, fragment) = decode(uri, encoding)?;

        let kind = if prefix.starts_with(encoding.digest_scheme) {
            KeyURIKind::Digest(Digest::new(&prefix[encoding.digest_scheme.len()..], &data)?)
        } else if prefix.starts_with(encoding.public_key_scheme) {
            KeyURIKind::PublicKey(PublicKey::new(
                &prefix[encoding.public_key_scheme.len()..],
                &data,
            )?)
        } else if prefix.starts_with(encoding.secret_key_scheme) {
            KeyURIKind::SecretKey(SecretKey::new(
                &prefix[encoding.secret_key_scheme.len()..],
                &data,
            )?)
        } else if prefix.starts_with(encoding.signature_scheme) {
            KeyURIKind::Signature(Signature::new(
                &prefix[encoding.signature_scheme.len()..],
                &data,
            )?)
        } else {
            fail!(SchemeInvalid, "unknown KeyURI prefix: {}", prefix)
        };

        // Use `clear_on_drop` to wipe any potential secret data
        data.as_mut_slice().clear();

        Ok(Self { kind, fragment })
    }

    /// Parse a KeyURI
    pub fn parse_uri(uri: &str) -> Result<Self, Error> {
        Self::parse(uri, URI_ENCODING)
    }

    /// Parse a KeyURI in URI-embeddable (a.k.a. "dasherized") encoding
    pub fn parse_dasherized(token: &str) -> Result<Self, Error> {
        Self::parse(token, DASHERIZED_ENCODING)
    }

    /// Return the `KeyURIKind` for this URI
    pub fn kind(&self) -> &KeyURIKind {
        &self.kind
    }

    /// Return a `SecretKey` if the underlying URI is a `secret.key:`
    pub fn secret_key(&self) -> Option<&SecretKey> {
        match self.kind {
            KeyURIKind::SecretKey(ref key) => Some(key),
            _ => None,
        }
    }

    /// Is this KeyURI a `secret.key:`?
    pub fn is_secret_key(&self) -> bool {
        self.secret_key().is_some()
    }

    /// Return a `PublicKey` if the underlying URI is a `public.key:`
    pub fn public_key(&self) -> Option<&PublicKey> {
        match self.kind {
            KeyURIKind::PublicKey(ref key) => Some(key),
            _ => None,
        }
    }

    /// Is this KeyURI a `public.key:`?
    pub fn is_public_key(&self) -> bool {
        self.public_key().is_some()
    }

    /// Return a `Digest` if the underlying URI is a `public.digest:`
    pub fn digest(&self) -> Option<&Digest> {
        match self.kind {
            KeyURIKind::Digest(ref digest) => Some(digest),
            _ => None,
        }
    }

    /// Is this KeyURI a `public.digest:`?
    pub fn is_digest(&self) -> bool {
        self.digest().is_some()
    }

    /// Return a `Signature` if the underlying URI is a `public.signature:`
    pub fn signature(&self) -> Option<&Signature> {
        match self.kind {
            KeyURIKind::Signature(ref sig) => Some(sig),
            _ => None,
        }
    }

    /// Is this KeyURI a `public.signature:`?
    pub fn is_signature(&self) -> bool {
        self.signature().is_some()
    }

    /// Obtain the fragment for this URI (i.e. everything after `#`)
    pub fn fragment(&self) -> Option<&str> {
        self.fragment.as_ref().map(|f| f.as_ref())
    }
}

impl Encodable for KeyURI {
    /// Serialize this `KeyURI` as a URI-encoded `String`
    fn to_uri_string(&self) -> String {
        match self.kind {
            KeyURIKind::Digest(ref digest) => digest.to_uri_string(),
            KeyURIKind::PublicKey(ref pk) => pk.to_uri_string(),
            KeyURIKind::SecretKey(ref sk) => sk.to_uri_string(),
            KeyURIKind::Signature(ref sig) => sig.to_uri_string(),
        }
    }

    /// Serialize this `KeyURI` as a "dasherized" `String`
    fn to_dasherized_string(&self) -> String {
        match self.kind {
            KeyURIKind::Digest(ref digest) => digest.to_dasherized_string(),
            KeyURIKind::PublicKey(ref pk) => pk.to_dasherized_string(),
            KeyURIKind::SecretKey(ref sk) => sk.to_dasherized_string(),
            KeyURIKind::Signature(ref sig) => sig.to_dasherized_string(),
        }
    }
}

/// Decode a URI into its prefix (scheme + algorithm), data, and fragment
fn decode(uri: &str, encoding: &Encoding) -> Result<(String, Vec<u8>, Option<String>), Error> {
    // Extract the fragment if it exists. Note that fragment is not covered by the
    // bech32 checksum and can be modified (e.g. as a key description)
    if let Some(delimiter) = encoding.fragment_delimiter {
        if let Some(pos) = uri.find(delimiter) {
            let fragment = uri[(pos + 1)..].to_owned();
            let (prefix, data) = Bech32::new(encoding.sub_delimiter).decode(&uri[..pos])?;
            return Ok((prefix, data, Some(fragment)));
        }
    }

    let (prefix, data) = Bech32::new(encoding.sub_delimiter).decode(uri)?;
    Ok((prefix, data, None))
}
