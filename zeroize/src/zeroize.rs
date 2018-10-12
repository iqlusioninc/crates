// TODO: support for other types (e.g. integers)

use super::secure_zero_memory;

#[cfg(feature = "std")]
use std::prelude::v1::*;

/// Trait for securely erasing types from memory
pub trait Zeroize {
    /// Zero out this object from memory (using Rust or OS intrinsics which
    /// ensure the zeroization operation is not "optimized away")
    fn zeroize(&mut self);
}

/// Byte slices are the core type we can zeroize
impl<'a> Zeroize for &'a mut [u8] {
    fn zeroize(&mut self) {
        secure_zero_memory(self);
    }
}

/// Zeroize `Vec<u8>`s by zeroizing their memory and then truncating them,
/// for consistency with the `String` behavior below, and also as an indication
/// the underlying memory has been wiped.
// TODO: other vector types? Generic vector impl?
#[cfg(feature = "std")]
impl Zeroize for Vec<u8> {
    fn zeroize(&mut self) {
        self.as_mut_slice().zeroize();
        self.clear();
    }
}

/// Zeroize `String`s by zeroizing their backing memory then truncating them,
/// to zero length (ensuring valid UTF-8, since they're empty)
#[cfg(feature = "std")]
impl Zeroize for String {
    fn zeroize(&mut self) {
        unsafe {
            self.as_bytes_mut().zeroize();
        }
        self.clear();
    }
}

macro_rules! impl_zeroize_for_as_mut {
    ($as_mut:ty) => {
        impl Zeroize for $as_mut {
            fn zeroize(&mut self) {
                self.as_mut().zeroize();
            }
        }
    };
}

impl_zeroize_for_as_mut!([u8; 1]);
impl_zeroize_for_as_mut!([u8; 2]);
impl_zeroize_for_as_mut!([u8; 3]);
impl_zeroize_for_as_mut!([u8; 4]);
impl_zeroize_for_as_mut!([u8; 5]);
impl_zeroize_for_as_mut!([u8; 6]);
impl_zeroize_for_as_mut!([u8; 7]);
impl_zeroize_for_as_mut!([u8; 8]);
impl_zeroize_for_as_mut!([u8; 9]);
impl_zeroize_for_as_mut!([u8; 10]);
impl_zeroize_for_as_mut!([u8; 11]);
impl_zeroize_for_as_mut!([u8; 12]);
impl_zeroize_for_as_mut!([u8; 13]);
impl_zeroize_for_as_mut!([u8; 14]);
impl_zeroize_for_as_mut!([u8; 15]);
impl_zeroize_for_as_mut!([u8; 16]);
impl_zeroize_for_as_mut!([u8; 17]);
impl_zeroize_for_as_mut!([u8; 18]);
impl_zeroize_for_as_mut!([u8; 19]);
impl_zeroize_for_as_mut!([u8; 20]);
impl_zeroize_for_as_mut!([u8; 21]);
impl_zeroize_for_as_mut!([u8; 22]);
impl_zeroize_for_as_mut!([u8; 23]);
impl_zeroize_for_as_mut!([u8; 24]);
impl_zeroize_for_as_mut!([u8; 25]);
impl_zeroize_for_as_mut!([u8; 26]);
impl_zeroize_for_as_mut!([u8; 27]);
impl_zeroize_for_as_mut!([u8; 28]);
impl_zeroize_for_as_mut!([u8; 29]);
impl_zeroize_for_as_mut!([u8; 30]);
impl_zeroize_for_as_mut!([u8; 31]);
impl_zeroize_for_as_mut!([u8; 32]);
impl_zeroize_for_as_mut!([u8; 33]);
impl_zeroize_for_as_mut!([u8; 34]);
impl_zeroize_for_as_mut!([u8; 35]);
impl_zeroize_for_as_mut!([u8; 36]);
impl_zeroize_for_as_mut!([u8; 37]);
impl_zeroize_for_as_mut!([u8; 38]);
impl_zeroize_for_as_mut!([u8; 39]);
impl_zeroize_for_as_mut!([u8; 40]);
impl_zeroize_for_as_mut!([u8; 41]);
impl_zeroize_for_as_mut!([u8; 42]);
impl_zeroize_for_as_mut!([u8; 43]);
impl_zeroize_for_as_mut!([u8; 44]);
impl_zeroize_for_as_mut!([u8; 45]);
impl_zeroize_for_as_mut!([u8; 46]);
impl_zeroize_for_as_mut!([u8; 47]);
impl_zeroize_for_as_mut!([u8; 48]);
impl_zeroize_for_as_mut!([u8; 49]);
impl_zeroize_for_as_mut!([u8; 50]);
impl_zeroize_for_as_mut!([u8; 51]);
impl_zeroize_for_as_mut!([u8; 52]);
impl_zeroize_for_as_mut!([u8; 53]);
impl_zeroize_for_as_mut!([u8; 54]);
impl_zeroize_for_as_mut!([u8; 55]);
impl_zeroize_for_as_mut!([u8; 56]);
impl_zeroize_for_as_mut!([u8; 57]);
impl_zeroize_for_as_mut!([u8; 58]);
impl_zeroize_for_as_mut!([u8; 59]);
impl_zeroize_for_as_mut!([u8; 60]);
impl_zeroize_for_as_mut!([u8; 61]);
impl_zeroize_for_as_mut!([u8; 62]);
impl_zeroize_for_as_mut!([u8; 63]);
impl_zeroize_for_as_mut!([u8; 64]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroize_slice() {
        let mut arr = [42; 3];
        arr.zeroize();
        assert_eq!(arr, [0, 0, 0]);
    }

    #[test]
    fn zeroize_vec() {
        let mut vec = vec![42; 3];
        vec.zeroize();
        assert!(vec.is_empty());
    }

    #[test]
    fn zeroize_string() {
        let mut string = String::from("Hello, world!");
        string.zeroize();
        assert!(string.is_empty());
    }

    #[test]
    fn zeroize_box() {
        let mut boxed_arr = Box::new([42; 3]);
        boxed_arr.zeroize();
        assert_eq!(boxed_arr.as_ref(), &[0u8; 3]);
    }
}
