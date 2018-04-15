//! Iterators for matches in the RPM database

use rpmlib_sys::rpmlib as ffi;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use {Header, Tag, ts::Txn};

/// Iterator over the matches from a database query
pub struct MatchIterator {
    /// Pointer to rpmlib's match iterator
    ptr: *mut ffi::rpmdbMatchIterator_s,

    /// Transaction in which we are reading the data. Held to ensure iterators
    /// always hold the global lock while iterating.
    #[allow(dead_code)]
    txn: Txn,
}

impl MatchIterator {
    /// Create a new `MatchIterator` for the current RPM database, searching
    /// by the (optionally) given search key.
    pub fn new(tag: Tag, key_opt: Option<&str>) -> Self {
        let mut txn = Txn::create();

        if let Some(key) = key_opt {
            if !key.is_empty() {
                let ptr = unsafe {
                    ffi::rpmtsInitIterator(
                        txn.as_mut_ptr(),
                        tag as ffi::rpm_tag_t,
                        key.as_ptr() as *const c_void,
                        key.len(),
                    )
                };
                return Self { ptr, txn };
            }
        }

        let ptr = unsafe {
            ffi::rpmtsInitIterator(txn.as_mut_ptr(), tag as ffi::rpm_tag_t, ptr::null(), 0)
        };

        Self { ptr, txn }
    }

    /// Find packages with a search key that exactly matches the given tag.
    ///
    /// Panics if the glob contains null bytes.
    pub fn find(&mut self, tag: Tag, key: &str) -> &mut MatchIterator {
        let cstr = CString::new(key).unwrap();

        unsafe {
            ffi::rpmdbSetIteratorRE(
                self.ptr,
                tag as ffi::rpm_tag_t,
                ffi::rpmMireMode_e_RPMMIRE_STRCMP,
                cstr.as_ptr(),
            );
        }

        self
    }

    /// Find all packages with the given tag that match the given "glob"
    ///
    /// Panics if the glob contains null bytes.
    pub fn glob(&mut self, tag: Tag, glob: &str) -> &mut MatchIterator {
        let cstr = CString::new(glob).unwrap();

        unsafe {
            ffi::rpmdbSetIteratorRE(
                self.ptr,
                tag as ffi::rpm_tag_t,
                ffi::rpmMireMode_e_RPMMIRE_GLOB,
                cstr.as_ptr(),
            );
        }

        self
    }

    /// Find all packages with the given tag that match the given regex
    ///
    /// Panics if the regex contains null bytes.
    pub fn regex(&mut self, tag: Tag, regex: &str) -> &mut MatchIterator {
        let cstr = CString::new(regex).unwrap();

        unsafe {
            ffi::rpmdbSetIteratorRE(
                self.ptr,
                tag as ffi::rpm_tag_t,
                ffi::rpmMireMode_e_RPMMIRE_REGEX,
                cstr.as_ptr(),
            );
        }

        self
    }
}

impl Iterator for MatchIterator {
    type Item = Header;

    /// Obtain the next header from the iterator
    fn next(&mut self) -> Option<Header> {
        let header_ptr = unsafe { ffi::rpmdbNextIterator(self.ptr) };

        if header_ptr.is_null() {
            None
        } else {
            Some(Header::new(header_ptr))
        }
    }
}

impl Drop for MatchIterator {
    fn drop(&mut self) {
        unsafe {
            ffi::rpmdbFreeIterator(self.ptr);
        }
    }
}
