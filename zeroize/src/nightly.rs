//! nightly: use the `volatile_set_memory` intrinsic

use core::intrinsics::volatile_set_memory;

/// Zero out memory using `core::intrinsics::volatile_set_memory`
///
/// The volatile parameter is set to true, so it will not be optimized out
/// unless size is equal to zero.
pub(crate) fn secure_zero_memory(bytes: &mut [u8]) {
    unsafe { volatile_set_memory(bytes.as_mut_ptr(), 0, bytes.len()) }
}
