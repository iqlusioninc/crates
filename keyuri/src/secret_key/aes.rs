use super::AsSecretSlice;
use algorithm::{AES128GCM_ALG_ID, AES256GCM_ALG_ID};

macro_rules! aes_key {
    ($name:ident, $size:expr, $docs:expr) => {
        #[doc=$docs]
        pub struct $name([u8; $size]);

        impl $name {
            /// Create a new AES key
            pub fn new(slice: &[u8]) -> Result<Self, ::error::Error> {
                if slice.len() != $size {
                    fail!(
                        ParseError,
                        "bad AES-{} key length: {} (expected {})",
                        $size * 8,
                        slice.len(),
                        $size
                    );
                }

                let mut key_bytes = [0u8; $size];
                key_bytes.copy_from_slice(slice);

                Ok($name(key_bytes))
            }
        }

        impl AsSecretSlice for $name {
            fn as_secret_slice(&self) -> &[u8] {
                self.0.as_ref()
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                use clear_on_drop::clear::Clear;
                self.0.as_mut().clear()
            }
        }
    };
}

aes_key!(Aes128GcmKey, 16, "AES-128 in Galois Counter Mode (GCM)");
impl_encodable_secret_key!(Aes128GcmKey, AES128GCM_ALG_ID);

aes_key!(Aes256GcmKey, 32, "AES-256 in Galois Counter Mode (GCM)");
impl_encodable_secret_key!(Aes256GcmKey, AES256GCM_ALG_ID);
