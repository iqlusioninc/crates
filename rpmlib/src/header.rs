//! RPM package headers

use rpmlib_sys::rpmlib as ffi;
use std::mem;

use td::TagData;
use tag::Tag;

/// RPM package header
pub struct Header(*mut ffi::headerToken_s);

impl Header {
    /// Create a new header from an `ffi::Header`.
    pub(crate) fn new(ffi_header: ffi::Header) -> Self {
        // Increment rpmlib's internal reference count for this header
        unsafe {
            ffi::headerLink(ffi_header);
        }
        Header(ffi_header)
    }

    /// Get the data that corresponds to the given header tag.
    pub fn get(&self, tag: Tag) -> TagData {
        // Create a zeroed `rpmtd_s` and then immediately initialize it
        let mut td: ffi::rpmtd_s = unsafe { mem::zeroed() };
        unsafe {
            ffi::rpmtdReset(&mut td);
        }

        let rc = unsafe {
            ffi::headerGet(
                self.0,
                tag as i32,
                &mut td,
                ffi::headerGetFlags_e_HEADERGET_MINMEM,
            )
        };

        assert_ne!(rc, 0, "headerGet returned non-zero status: {}", rc);

        match td.type_ {
            ffi::rpmTagType_e_RPM_NULL_TYPE => TagData::Null,
            ffi::rpmTagType_e_RPM_CHAR_TYPE => unsafe { TagData::char(&td) },
            ffi::rpmTagType_e_RPM_INT8_TYPE => unsafe { TagData::int8(&td) },
            ffi::rpmTagType_e_RPM_INT16_TYPE => unsafe { TagData::int16(&td) },
            ffi::rpmTagType_e_RPM_INT32_TYPE => unsafe { TagData::int32(&td) },
            ffi::rpmTagType_e_RPM_INT64_TYPE => unsafe { TagData::int64(&td) },
            ffi::rpmTagType_e_RPM_STRING_TYPE => unsafe { TagData::string(&td) },
            ffi::rpmTagType_e_RPM_STRING_ARRAY_TYPE => unsafe { TagData::string_array(&td) },
            ffi::rpmTagType_e_RPM_I18NSTRING_TYPE => unsafe { TagData::i18n_string(&td) },
            ffi::rpmTagType_e_RPM_BIN_TYPE => unsafe { TagData::bin(&td) },
            other => panic!("unsupported rpmtd tag type: {}", other),
        }
    }

    /// Get the package's name from tag data
    pub fn name(&self) -> &str {
        self.get(Tag::NAME).as_str().unwrap()
    }

    /// Get the package's description from tag data
    pub fn description(&self) -> &str {
        self.get(Tag::DESCRIPTION).as_str().unwrap()
    }
}

impl Drop for Header {
    fn drop(&mut self) {
        // Decrement rpmlib's internal reference count for this header
        unsafe {
            ffi::headerFree(self.0);
        }
    }
}
