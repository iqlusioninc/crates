//! RPM database access
//!
//! The database used is whichever one is configured as the `_dbpath` in the
//! in the global macro context. By default this is unset: you will need to
//! call `rpmlib::config::read_file(None)` to read the default "rpmrc"
//! configuration.
//!
//! # Example
//!
//! Finding the "rpm-devel" RPM in the database:
//!
//! ```
//! use rpmlib::{self, Tag};
//!
//! rpmlib::config::read_file(None).unwrap();
//!
//! let mut matches = rpmlib::db::find(Tag::NAME, "rpm-devel");
//! let headers = matches.next().unwrap();
//!
//! println!("package name: {}", headers.name());
//! println!("package description: {}", headers.description());
//! ```

use {MatchIterator, Tag};

/// Find all packages in the RPM database
pub fn all_packages() -> MatchIterator {
    MatchIterator::new(Tag::NAME, None)
}

/// Find packages with a search key that exactly matches the given tag.
///
/// Panics if the glob contains null bytes.
pub fn find(tag: Tag, key: &str) -> MatchIterator {
    MatchIterator::new(tag, Some(key))
}

/// Find all packages with the given tag that match the given "glob"
///
/// Panics if the glob contains null bytes.
pub fn glob(tag: Tag, glob: &str) -> MatchIterator {
    let mut iter = all_packages();
    iter.glob(tag, glob);
    iter
}

/// Find all packages with the given tag that match the given regex
///
/// Panics if the regex contains null bytes.
pub fn regex(tag: Tag, regex: &str) -> MatchIterator {
    let mut iter = all_packages();
    iter.regex(tag, regex);
    iter
}
