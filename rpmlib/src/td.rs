use rpmlib_sys::rpmlib;
use std::ffi::CStr;
use std::{slice, str};

use byteorder::{BigEndian, ByteOrder};
use tag::TagType;

/// Data associated with a given header tag
#[derive(Debug)]
pub enum TagData<'h> {
    /// No data associated with this tag
    Null,

    /// Character
    Char(char),

    /// 8-bit integer
    Int8(i8),

    /// 16-bit integer
    Int16(i16),

    /// 32-bit integer
    Int32(i32),

    /// 64-bit integer
    Int64(i64),

    /// String
    Str(&'h str),

    /// String array
    StrArray(Vec<&'h str>),

    /// Internationalized string (UTF-8?)
    I18NStr(&'h str),

    /// Binary data
    Bin(&'h [u8]),
}

impl<'h> TagData<'h> {
    /// Convert an `rpmtd_s` into a `TagData::Char`
    pub(crate) unsafe fn char(td: &rpmlib::rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::CHAR as u32);
        assert_eq!(td.count, 1);
        TagData::Char((*(td.data as *const u8)).into())
    }

    /// Convert an `rpmtd_s` into an `TagData::Int8`
    pub(crate) unsafe fn int8(td: &rpmlib::rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::INT8 as u32);
        assert_eq!(td.count, 1);
        TagData::Int8(*(td.data as *const i8))
    }

    /// Convert an `rpmtd_s` int an `TagData::Int16`
    pub(crate) unsafe fn int16(td: &rpmlib::rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::INT16 as u32);
        TagData::Int16(BigEndian::read_i16(TagData::rpmtd_as_slice(td)))
    }

    /// Convert an `rpmtd_s` int an `TagData::Int32`
    pub(crate) unsafe fn int32(td: &rpmlib::rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::INT32 as u32);
        TagData::Int32(BigEndian::read_i32(TagData::rpmtd_as_slice(td)))
    }

    /// Convert an `rpmtd_s` int an `Int64`
    pub(crate) unsafe fn int64(td: &rpmlib::rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::INT64 as u32);
        TagData::Int64(BigEndian::read_i64(TagData::rpmtd_as_slice(td)))
    }

    /// Convert an `rpmtd_s` into a `Str`
    pub(crate) unsafe fn string(td: &rpmlib::rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::STRING as u32);
        let cstr = CStr::from_ptr(td.data as *const i8);

        // TODO: should we only try decoding RPM_STRING_TYPE as ASCII?
        TagData::Str(str::from_utf8(cstr.to_bytes()).unwrap_or_else(|e| {
            panic!(
                "failed to decode RPM_STRING_TYPE as UTF-8 (tag: {}): {}",
                td.tag, e
            );
        }))
    }

    /// Convert an `rpmtd_s` into a `StrArray`
    pub(crate) unsafe fn string_array(td: &rpmlib::rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::STRING_ARRAY as u32);
        panic!("RPM_STRING_ARRAY unimplemented! (tag: {})", td.tag);
    }

    /// Convert an `rpmtd_s` into an `I18NStr`
    pub(crate) unsafe fn i18n_string(td: &rpmlib::rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::I18NSTRING as u32);
        let cstr = CStr::from_ptr(td.data as *const i8);

        TagData::I18NStr(str::from_utf8(cstr.to_bytes()).unwrap_or_else(|e| {
            panic!(
                "failed to decode RPM_I18NSTRING_TYPE as UTF-8 (tag: {}): {}",
                td.tag, e
            );
        }))
    }

    /// Convert an `rpmtd_s` into a `Bin`
    pub(crate) unsafe fn bin(td: &rpmlib::rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::BIN as u32);
        TagData::Bin(TagData::rpmtd_as_slice(td))
    }

    /// Convert the data parts of an rpmtd_s into a byte slice
    pub(crate) unsafe fn rpmtd_as_slice(td: &rpmlib::rpmtd_s) -> &'h [u8] {
        // TODO: we should probably have more checks that this conversion
        // is correct and safe
        assert!(
            !td.data.is_null(),
            "rpmtd.data is NULL! (tag type: {})",
            td.tag
        );

        assert_ne!(
            td.type_,
            TagType::NULL as u32,
            "can't get slice of NULL data (tag type: {})",
            td.tag
        );

        slice::from_raw_parts(td.data as *const u8, td.count as usize)
    }
}
