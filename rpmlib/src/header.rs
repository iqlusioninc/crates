//! RPM package headers

use rpmlib_sys::rpmlib;
use std::mem;

use td::TagData;
use tag::Tag;

/// RPM package header
pub struct Header(rpmlib::Header);

impl Header {
    /// Create a new header from an `rpmlib::Header`
    ///
    /// TODO: we rely on `MatchIterator` to not allow these to be borrowed after
    /// free. It seems like we should be able to codify that with lifetimes.
    pub(crate) fn new(rpmlib_header: rpmlib::Header) -> Self {
        Header(rpmlib_header)
    }

    /// Get the data that corresponds to the given header tag
    pub fn get(&self, tag: Tag) -> TagData {
        let mut td: rpmlib::rpmtd_s = unsafe { mem::zeroed() };

        let rc = unsafe {
            rpmlib::rpmtdReset(&mut td);
            rpmlib::headerGet(
                self.0,
                tag as i32,
                &mut td,
                rpmlib::headerGetFlags_e_HEADERGET_MINMEM,
            )
        };

        assert!(rc != 0, "headerGet returned non-zero status: {}", rc);

        // TODO: delete this when we're a bit more confident in all the tag data
        // types. Right now some of them may be bogus.
        // eprintln!("Header:");
        // eprintln!("- tag: {}", td.tag);
        // eprintln!("- type: {}", td.type_);
        // eprintln!("- count: {}", td.count);
        // eprintln!("- data: {:?}", td.data);
        // eprintln!("- flags: {}", td.flags);
        // eprintln!("- ix: {}", td.ix);

        match td.type_ {
            rpmlib::rpmTagType_e_RPM_NULL_TYPE => TagData::Null,
            rpmlib::rpmTagType_e_RPM_CHAR_TYPE => unsafe { TagData::char(&td) },
            rpmlib::rpmTagType_e_RPM_INT8_TYPE => unsafe { TagData::int8(&td) },
            rpmlib::rpmTagType_e_RPM_INT16_TYPE => unsafe { TagData::int16(&td) },
            rpmlib::rpmTagType_e_RPM_INT32_TYPE => unsafe { TagData::int32(&td) },
            rpmlib::rpmTagType_e_RPM_INT64_TYPE => unsafe { TagData::int64(&td) },
            rpmlib::rpmTagType_e_RPM_STRING_TYPE => unsafe { TagData::string(&td) },
            rpmlib::rpmTagType_e_RPM_STRING_ARRAY_TYPE => unsafe { TagData::string_array(&td) },
            rpmlib::rpmTagType_e_RPM_I18NSTRING_TYPE => unsafe { TagData::i18n_string(&td) },
            rpmlib::rpmTagType_e_RPM_BIN_TYPE => unsafe { TagData::bin(&td) },
            other => panic!("unsupported rpmtd tag type: {}", other),
        }
    }
}
