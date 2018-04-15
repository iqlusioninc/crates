//! rpmlib.rs integration tests

extern crate rpmlib;

use rpmlib::{config, db, Header, Tag};
use std::sync::{Once, ONCE_INIT};

/// The `.rpm` containing rpmlib itself
const TEST_PACKAGE: &str = "rpm-devel";

static CONFIGURE: Once = ONCE_INIT;

// Read the default config
// TODO: create a mock RPM database for testing
fn configure() {
    CONFIGURE.call_once(|| {
        config::read_file(None).unwrap();
    });
}

#[test]
fn db_find_test() {
    configure();

    let mut matches = db::find(Tag::NAME, TEST_PACKAGE);

    if let Some(package) = matches.next() {
        assert_eq!(package.name(), TEST_PACKAGE);
    } else {
        panic!("expected 1 result, got 0!");
    }

    assert!(matches.next().is_none(), "expected one result, got more!");
}

#[test]
fn header_collect_test() {
    configure();

    let matches: Vec<Header> = db::find(Tag::NAME, TEST_PACKAGE).collect();
    assert_eq!(
        matches.len(),
        1,
        "expected 1 result, got {}!",
        matches.len()
    );
    assert_eq!(matches[0].name(), TEST_PACKAGE);
}
