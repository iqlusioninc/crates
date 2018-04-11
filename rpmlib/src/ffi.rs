//! Main entry point for all rpmlib FFI calls. If you are looking to add a call,
//! please add it here.
//!
//! This facade ensures single-threaded access to rpmlib, which has not been
//! designed with thread safety in mind.

#![allow(non_snake_case)]

use failure::Error;
use rpmlib_sys::rpmlib;
use std::os::raw::{c_char, c_int, c_void};
use std::sync::{Mutex, MutexGuard};

/// Main entry point into the rpmlib C library
///
/// rpmlib is not thread-safe (or at least, several things claim that it's not,
/// and I can find nothing which claims that has changed)
///
/// To ensure thread safety, we throw a global lock around the rpmlib API,
/// ensuring that only one thread is allowed to use it at a time (at least, so
/// long as they don't unsafely sidestep this facade).
///
/// This lock will be held throughout the lifetime of any TransactionSets.
pub(crate) struct FFI;

lazy_static! {
    static ref FFI_GLOBAL_LOCK: Mutex<FFI> = Mutex::new(FFI);
}

/// NOTE: these methods should be direct wrappers to the bindgen-generated FFI.
impl FFI {
    /// Attempt to acquire the global lock, immediately returning an error if
    /// the global lock is held by another thread
    pub fn try_lock() -> Result<MutexGuard<'static, FFI>, Error> {
        FFI_GLOBAL_LOCK
            .try_lock()
            .map_err(|e| format_err!("couldn't acquire rpmlib FFI lock: {}", e))
    }

    /// rpmReadConfigFiles()
    #[inline]
    pub unsafe fn rpmReadConfigFiles(
        &mut self,
        file: *const c_char,
        target: *const c_char,
    ) -> c_int {
        rpmlib::rpmReadConfigFiles(file, target)
    }

    /// rpmDefineMacro()
    pub unsafe fn rpmDefineMacro(
        &mut self,
        mc: rpmlib::rpmMacroContext,
        macro_: *const c_char,
        level: c_int,
    ) -> c_int {
        rpmlib::rpmDefineMacro(mc, macro_, level)
    }

    /// delMacro()
    pub unsafe fn delMacro(&mut self, mc: rpmlib::rpmMacroContext, n: *const c_char) {
        rpmlib::delMacro(mc, n)
    }

    /// rpmtsCreate()
    #[inline]
    pub unsafe fn rpmtsCreate(&mut self) -> rpmlib::rpmts {
        rpmlib::rpmtsCreate()
    }

    /// rpmtsFree()
    #[inline]
    pub unsafe fn rpmtsFree(&mut self, ts: rpmlib::rpmts) {
        rpmlib::rpmtsFree(ts);
    }

    /// rpmtsOpenDB()
    #[inline]
    pub unsafe fn rpmtsOpenDB(&mut self, ts: rpmlib::rpmts, dbmode: c_int) -> c_int {
        rpmlib::rpmtsOpenDB(ts, dbmode)
    }

    /// rpmtsCloseDB()
    #[inline]
    pub unsafe fn rpmtsCloseDB(&mut self, ts: rpmlib::rpmts) -> c_int {
        rpmlib::rpmtsCloseDB(ts)
    }

    /// rpmtsInitIterator()
    #[inline]
    pub unsafe fn rpmtsInitIterator(
        &mut self,
        ts: rpmlib::rpmts,
        rpmtag: rpmlib::rpmDbiTagVal,
        keyp: *const c_void,
        keylen: usize,
    ) -> rpmlib::rpmdbMatchIterator {
        rpmlib::rpmtsInitIterator(ts, rpmtag, keyp, keylen)
    }

    /// rpmdbSetIteratorRE()
    #[inline]
    pub unsafe fn rpmdbSetIteratorRE(
        &mut self,
        mi: rpmlib::rpmdbMatchIterator,
        tag: rpmlib::rpmTagVal,
        mode: rpmlib::rpmMireMode,
        pattern: *const c_char,
    ) -> c_int {
        rpmlib::rpmdbSetIteratorRE(mi, tag, mode, pattern)
    }

    /// rpmdbNextIterator()
    #[inline]
    pub unsafe fn rpmdbNextIterator(&mut self, mi: rpmlib::rpmdbMatchIterator) -> rpmlib::Header {
        rpmlib::rpmdbNextIterator(mi)
    }

    /// rpmdbFreeIterator()
    #[inline]
    pub unsafe fn rpmdbFreeIterator(
        &mut self,
        mi: rpmlib::rpmdbMatchIterator,
    ) -> rpmlib::rpmdbMatchIterator {
        rpmlib::rpmdbFreeIterator(mi)
    }
}
