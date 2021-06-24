//! Elliptic Curve Digital Signature Algorithm (ECDSA) support.

#[cfg(feature = "secp256k1")]
pub mod secp256k1;

mod keyring;

pub use self::keyring::KeyRing;
pub use ecdsa::{elliptic_curve, Signature};

#[cfg(feature = "secp256k1")]
pub use k256::Secp256k1;
