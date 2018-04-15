//! Transaction sets: rpmlib's transaction API

use rpmlib_sys::rpmlib as ffi;
use std::sync::MutexGuard;
use std::sync::atomic::AtomicPtr;

use GlobalState;

/// rpmlib transactions, a.k.a. "transaction sets" (or "rpmts" in rpmlib)
///
/// Nearly all access to rpmlib, including actions which don't necessarily
/// involve operations on the RPM database, require a transaction set.
///
/// This library opens a single global transaction set on command, and all
/// operations which require one acquire it, use it, and then release it.
/// This allows us to keep them out of the public API.
pub(crate) struct TransactionSet(AtomicPtr<ffi::rpmts_s>);

impl TransactionSet {
    /// Create a transaction set (i.e. begin a transaction)
    ///
    /// This is not intended to be invoked directly, but instead obtained
    /// from `GlobalState`.
    pub(crate) fn create() -> Self {
        TransactionSet(AtomicPtr::new(unsafe { ffi::rpmtsCreate() }))
    }
}

impl Drop for TransactionSet {
    fn drop(&mut self) {
        unsafe {
            ffi::rpmtsFree(*self.0.get_mut());
        }
    }
}

impl TransactionSet {
    pub(crate) fn as_mut_ptr(&mut self) -> &mut *mut ffi::rpmts_s {
        self.0.get_mut()
    }
}

/// Crate-public wrapper for acquiring and releasing the global transaction set
/// which also cleans it prior to unlocking it.
pub(crate) struct Txn(MutexGuard<'static, GlobalState>);

impl Txn {
    /// Acquire the global state mutex, giving the current thread exclusive
    /// access to the global transaction set.
    pub fn create() -> Self {
        Txn(GlobalState::lock())
    }

    /// Obtain the internal pointer to the transaction set
    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffi::rpmts_s {
        // Since we're guaranteed to be holding the GlobalState mutex here,
        // we're free to deref the pointer.
        *self.0.ts.as_mut_ptr()
    }
}

/// Tidy up the shared global transaction set between uses
impl Drop for Txn {
    fn drop(&mut self) {
        unsafe {
            ffi::rpmtsClean(self.as_mut_ptr());
        }
    }
}
