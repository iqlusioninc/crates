use failure::Error;
use rpmlib_sys::rpmlib;
use std::ops::DerefMut;
use std::sync::MutexGuard;

use ffi::FFI;

/// rpmlib transactions, a.k.a. "transaction sets" (or "rpmts" in rpmlib)
///
/// Nearly all access to rpmlib, including actions which don't necessarily
/// involve operations on the RPM database, require a transaction set.
pub struct TransactionSet {
    /// Ensure we hold the FFI global lock throughout the transaction
    ffi: MutexGuard<'static, FFI>,

    /// Pointer to the underlying rpmlib transaction set
    ptr: rpmlib::rpmts,
}

impl TransactionSet {
    /// Create a transaction set (i.e. begin a transaction)
    #[inline]
    pub fn create() -> Result<Self, Error> {
        let mut ffi = FFI::try_lock()?;
        let ptr = unsafe { ffi.rpmtsCreate() };
        Ok(Self { ffi, ptr })
    }

    /// Borrow this transaction's lock on the FFI
    pub(crate) fn ffi(&mut self) -> &mut FFI {
        self.ffi.deref_mut()
    }

    /// Obtain the internal pointer to the transaction set
    pub(crate) fn as_ptr(&mut self) -> rpmlib::rpmts {
        self.ptr
    }
}

impl Drop for TransactionSet {
    fn drop(&mut self) {
        unsafe {
            self.ffi.rpmtsFree(self.ptr);
        }
    }
}
