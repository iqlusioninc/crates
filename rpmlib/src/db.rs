//! RPM database binding

use failure::Error;
use libc;
use rpmlib_sys::rpmlib;
use std::ffi::CString;
use std::ptr;
use streaming_iterator::StreamingIterator;

use header::Header;
use tag::Tag;
use ts::TransactionSet;

/// RPM database access: this type provides a handle to an RPM database valid
/// for the lifetime of a transaction.
///
/// The database used is whichever one is configured as the `_dbpath` in the
/// in the global macro context. By default this is unset: you will need to
/// call `rpmlib::read_config(None)` to read the default "rpmrc" configuration.
///
/// # Example
///
/// Finding the "rpm-devel" RPM in the database:
///
/// ```
/// use rpmlib::{Database, StreamingIterator, Txn, Tag, TagData};
///
/// rpmlib::read_config(None).unwrap();
///
/// let mut txn = Txn::create().unwrap();
/// let mut db = Database::open(&mut txn, false).unwrap();
/// let mut matches = db.find(Tag::NAME, "rpm-devel");
/// let package = matches.next().unwrap();
///
/// match package.get(Tag::NAME) {
///     TagData::Str(ref s) => println!("package name: {}", s),
///     _ => ()
/// }
///
/// match package.get(Tag::DESCRIPTION) {
///     TagData::Str(ref s) => println!("package description: {}", s),
///     _ => ()
/// }
/// ```
pub struct Database<'ts> {
    /// Borrows the `TransactionSet` mutably so we can control its database
    ts: &'ts mut TransactionSet,
}

impl<'ts> Database<'ts> {
    /// Open the default database
    pub fn open(ts: &'ts mut TransactionSet, writable: bool) -> Result<Self, Error> {
        let dbmode = if writable {
            libc::O_RDWR
        } else {
            libc::O_RDONLY
        };

        let ts_ptr = ts.as_ptr();
        if unsafe { ts.ffi().rpmtsOpenDB(ts_ptr, dbmode) } != 0 {
            // TODO: check the _dbpath macro and see if it's set?
            bail!("couldn't open _dbpath (writable: {})!", writable);
        }

        Ok(Self { ts })
    }

    /// Return an iterator over all of the packages in the database.
    pub fn packages<'db>(&'db mut self) -> MatchIterator<'db, 'ts> {
        let ts_ptr = self.ts.as_ptr();
        let iter_ptr = unsafe {
            self.ts
                .ffi()
                .rpmtsInitIterator(ts_ptr, rpmlib::rpmTag_e_RPMTAG_NAME, ptr::null(), 0)
        };

        MatchIterator::new(self, iter_ptr)
    }

    /// Find packages with a search key that exatcly matches the given tag.
    ///
    /// Panics if the glob contains null bytes.
    #[inline]
    pub fn find<'db, K>(&'db mut self, tag: Tag, key: K) -> MatchIterator<'db, 'ts>
    where
        K: AsRef<str>,
    {
        let mut iter = self.packages();
        iter.find(tag, key);
        iter
    }

    /// Find all packages with the given tag that match the given "glob"
    ///
    /// Panics if the glob contains null bytes.
    #[inline]
    pub fn find_glob<'db, G>(&'db mut self, tag: Tag, glob: &G) -> MatchIterator<'db, 'ts>
    where
        G: AsRef<str>,
    {
        let mut iter = self.packages();
        iter.glob(tag, glob);
        iter
    }

    /// Find all packages with the given tag that match the given regex
    ///
    /// Panics if the regex contains null bytes.
    #[inline]
    pub fn find_regex<'db, R>(&'db mut self, tag: Tag, regex: &R) -> MatchIterator<'db, 'ts>
    where
        R: AsRef<str>,
    {
        let mut iter = self.packages();
        iter.regex(tag, regex);
        iter
    }

    /// Obtain the TransactionSet this database was opened under
    #[inline]
    pub fn transaction_set(&mut self) -> &mut TransactionSet {
        &mut self.ts
    }
}

impl<'ts> Drop for Database<'ts> {
    fn drop(&mut self) {
        unsafe {
            let ts_ptr = self.ts.as_ptr();
            self.ts.ffi().rpmtsCloseDB(ts_ptr);
        }
    }
}

/// Iterator over the matches from a database query
pub struct MatchIterator<'db, 'ts: 'db> {
    // TODO: allow Header to borrow mut borrow the database for a safer memory
    // model. See notes in `header.rs`
    #[allow(dead_code)]
    db: &'db mut Database<'ts>,
    ptr: rpmlib::rpmdbMatchIterator,
    next_item: Option<Header>,
}

impl<'db, 'ts> MatchIterator<'db, 'ts> {
    pub(crate) fn new(db: &'db mut Database<'ts>, ptr: rpmlib::rpmdbMatchIterator) -> Self {
        assert!(!ptr.is_null(), "iterator pointer is NULL!");
        Self {
            db,
            ptr,
            next_item: None,
        }
    }

    /// Find packages with a search key that exatcly matches the given tag.
    ///
    /// Panics if the glob contains null bytes.
    pub fn find<K>(&mut self, tag: Tag, key: K) -> &mut Self
    where
        K: AsRef<str>,
    {
        let key_cstr = CString::new(key.as_ref()).unwrap();

        unsafe {
            self.db.ts.ffi().rpmdbSetIteratorRE(
                self.ptr,
                tag as rpmlib::rpm_tag_t,
                rpmlib::rpmMireMode_e_RPMMIRE_STRCMP,
                key_cstr.as_ptr(),
            );
        }

        self
    }

    /// Find all packages with the given tag that match the given "glob"
    ///
    /// Panics if the glob contains null bytes.
    pub fn glob<G>(&mut self, tag: Tag, glob: &G) -> &mut Self
    where
        G: AsRef<str>,
    {
        let glob_cstr = CString::new(glob.as_ref()).unwrap();

        unsafe {
            self.db.ts.ffi().rpmdbSetIteratorRE(
                self.ptr,
                tag as rpmlib::rpm_tag_t,
                rpmlib::rpmMireMode_e_RPMMIRE_GLOB,
                glob_cstr.as_ptr(),
            );
        }

        self
    }

    /// Find all packages with the given tag that match the given regex
    ///
    /// Panics if the regex contains null bytes.
    pub fn regex<R>(&mut self, tag: Tag, regex: &R) -> &mut Self
    where
        R: AsRef<str>,
    {
        let regex_cstr = CString::new(regex.as_ref()).unwrap();

        unsafe {
            self.db.ts.ffi().rpmdbSetIteratorRE(
                self.ptr,
                tag as rpmlib::rpm_tag_t,
                rpmlib::rpmMireMode_e_RPMMIRE_REGEX,
                regex_cstr.as_ptr(),
            );
        }

        self
    }
}

impl<'db, 'ts> StreamingIterator for MatchIterator<'db, 'ts> {
    type Item = Header;

    fn advance(&mut self) {
        let header_ptr = unsafe { self.db.ts.ffi().rpmdbNextIterator(self.ptr) };

        if header_ptr.is_null() {
            self.next_item = None
        } else {
            self.next_item = Some(Header::new(header_ptr))
        }
    }

    fn get(&self) -> Option<&Header> {
        self.next_item.as_ref()
    }
}

impl<'db, 'ts> Drop for MatchIterator<'db, 'ts> {
    fn drop(&mut self) {
        unsafe {
            self.db.ts.ffi().rpmdbFreeIterator(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use {read_config, Database, Tag, TagData, Txn};
    use streaming_iterator::StreamingIterator;

    /// The `.rpm` containing rpmlib itself
    const TEST_PACKAGE: &str = "rpm-devel";

    #[test]
    fn test_package_lookup() {
        // Read the default config
        // TODO: create a mock RPM database for testing
        read_config(None).unwrap();

        let mut txn = Txn::create().unwrap();
        let mut db = Database::open(&mut txn, false).unwrap();
        let mut matches = db.find(Tag::NAME, TEST_PACKAGE);

        if let Some(package) = matches.next() {
            match package.get(Tag::NAME) {
                TagData::Str(ref s) => assert_eq!(s, &TEST_PACKAGE),
                _ => panic!("unexpected result for package.get()!"),
            }
        } else {
            panic!("expected 1 result, got 0!");
        }

        assert!(matches.next().is_none(), "expected one result, got more!");
    }
}
