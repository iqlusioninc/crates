//! `Box` types containing secrets

use alloc::boxed::Box;
use core::{
    any,
    fmt::{self, Debug},
};

use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{ExposeSecret, ExposeSecretMut};

/// Same as [`Secret`], but keeps the secret value in the heap instead of on the stack.
pub struct SecretBox<S: Zeroize> {
    inner_secret: Box<S>,
}

impl<S: Zeroize + Clone> SecretBox<S> {
    /// Create a secret value using the provided function as a constructor.
    ///
    /// The implementation makes an effort to zeroize the locally constructed value
    /// before it is copied to the heap, and constructing it inside the closure minimizes
    /// the possibility of it being accidentally copied by other code.
    pub fn new(ctr: impl FnOnce() -> S) -> Self {
        let mut data = ctr();
        let secret = Self {
            inner_secret: Box::new(data.clone()),
        };
        data.zeroize();
        secret
    }
}

impl<S: Zeroize + Default> Default for SecretBox<S> {
    fn default() -> Self {
        Self {
            inner_secret: Box::<S>::default(),
        }
    }
}

impl<S: Zeroize> Zeroize for SecretBox<S> {
    fn zeroize(&mut self) {
        self.inner_secret.as_mut().zeroize()
    }
}

impl<S: Zeroize> Drop for SecretBox<S> {
    fn drop(&mut self) {
        self.zeroize()
    }
}

impl<S: Zeroize> ZeroizeOnDrop for SecretBox<S> {}

impl<S> Debug for SecretBox<S>
where
    S: Zeroize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretBox(")?;
        f.write_str(any::type_name::<Self>())?;
        f.write_str(")")
    }
}

impl<S: Zeroize> ExposeSecret<S> for SecretBox<S> {
    fn expose_secret(&self) -> &S {
        self.inner_secret.as_ref()
    }
}

impl<S: Zeroize> ExposeSecretMut<S> for SecretBox<S> {
    fn expose_secret(&mut self) -> &mut S {
        self.inner_secret.as_mut()
    }
}
