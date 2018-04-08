//! txnset: transaction sets (a.k.a. rpmts)
//!
//! Nearly all access to rpmlib, including actions which don't necessarily
//! involve operations on the RPM database, require a transaction set.

#[cfg(feature = "rpmlib-sys")]
use rpmlib_sys::{rpmts, rpmtsCreate, rpmtsFree};

/// Transactions within rpmlib over the RPM database and other functionality
pub struct TxnSet {
    /// rpmlib's transaction set type (or more specifically, a mut pointer)
    #[cfg(feature = "rpmlib-sys")]
    ts: rpmts,
}

impl TxnSet {
    /// Create a new transaction
    #[inline]
    pub fn create() -> Self {
        #[cfg(not(feature = "rpmlib-sys"))]
        panic!("rpmlib not available");

        #[cfg(feature = "rpmlib-sys")]
        Self {
            ts: unsafe { rpmtsCreate() },
        }
    }
}

#[cfg(feature = "rpmlib-sys")]
impl Drop for TxnSet {
    fn drop(&mut self) {
        unsafe {
            rpmtsFree(self.ts);
        }
    }
}

#[cfg(all(test, feature = "rpmlib-sys"))]
mod tests {
    use super::TxnSet;

    #[test]
    fn create_test() {
        // Does create work at all?
        TxnSet::create();
    }
}
