//! `Box` types containing secrets

use super::{DebugSecret, ExposeSecret, Secret};
use alloc::boxed::Box;
use zeroize::Zeroize;

/// `Box` types containing a secret value
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub struct SecretBox<S: Zeroize>(Secret<Box<S>>);

impl<S: Zeroize> SecretBox<S> {
    /// Construct a `SecretBox` type containing a secret value
    pub fn new(secret: S) -> Self {
        SecretBox(Secret::new(Box::new(secret)))
    }
}

impl<S: Zeroize> ExposeSecret<S> for SecretBox<S> {
    fn expose_secret(&self) -> &S {
        &**self.0.expose_secret()
    }
}

impl<S: Zeroize> Zeroize for SecretBox<S> {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl<S: DebugSecret> DebugSecret for Box<S> {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_secret_box() {
        let secret = [42_u8; 32];
        let mut secret_box = SecretBox::new(secret);
        secret_box.zeroize();
        assert_eq!(secret_box.expose_secret(), &[0_u8; 32])
    }
}
