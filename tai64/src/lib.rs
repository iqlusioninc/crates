//! TAI64(N) timestamp generation, parsing and calculation.
//!
//! # Limitations
//!
//! Does not handle leap seconds. But libtai does not either. So we
//! should interoperate just fine 😣.

#![crate_name = "tai64"]
#![crate_type = "rlib"]
#![deny(warnings, missing_docs, unsafe_code, unused_import_braces, unused_qualifications)]
#![doc(html_root_url = "https://docs.rs/tai64/0.1.0")]

extern crate byteorder;

use self::byteorder::{BigEndian, ByteOrder};

use std::ops::{Add, Sub};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// A `TAI64` label.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct TAI64(pub u64);

/// A `TAI64N` timestamp.
///
/// Invariant: The nanosecond part <= 999999999.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct TAI64N(pub TAI64, pub u32);

// To and from external representation.

impl TAI64 {
    /// Convert `TAI64` to external representation.
    pub fn to_external(&self) -> [u8; 8] {
        let mut result = [0u8; 8];
        BigEndian::write_u64(&mut result, self.0);
        result
    }

    /// Parse `TAI64` from external representation.
    pub fn from_external(ext: &[u8]) -> Option<Self> {
        if ext.len() != 8 {
            None
        } else {
            Some(TAI64(BigEndian::read_u64(ext)))
        }
    }
}

impl TAI64N {
    /// Convert `TAI64N` to external representation.
    pub fn to_external(&self) -> [u8; 12] {
        let mut result = [0u8; 12];
        result[..8].copy_from_slice(&self.0.to_external());
        BigEndian::write_u32(&mut result[8..], self.1);
        result
    }

    /// Parse `TAI64N` from external representation.
    pub fn from_external(ext: &[u8]) -> Option<Self> {
        if ext.len() != 12 {
            return None;
        }

        let s = TAI64::from_external(&ext[..8]).unwrap();
        let n = BigEndian::read_u32(&ext[8..]);

        if n <= 999_999_999 {
            Some(TAI64N(s, n))
        } else {
            None
        }
    }
}

impl TAI64N {
    /// Get `TAI64N` timestamp according to system clock.
    pub fn now() -> TAI64N {
        TAI64N::from_system_time(&SystemTime::now())
    }
}

// Operators.

const NANOSECONDS_PER_SECOND: u32 = 1_000_000_000;

impl Add<u64> for TAI64 {
    type Output = TAI64;

    fn add(self, x: u64) -> TAI64 {
        TAI64(self.0 + x)
    }
}

impl Sub<u64> for TAI64 {
    type Output = TAI64;

    fn sub(self, x: u64) -> TAI64 {
        TAI64(self.0 - x)
    }
}

impl Add<Duration> for TAI64N {
    type Output = TAI64N;

    fn add(self, d: Duration) -> TAI64N {
        let n = self.1 + d.subsec_nanos();
        let (carry, n) = if n >= NANOSECONDS_PER_SECOND {
            (1, n - NANOSECONDS_PER_SECOND)
        } else {
            (0, n)
        };
        TAI64N(self.0 + d.as_secs() + carry, n)
    }
}

impl Sub<Duration> for TAI64N {
    type Output = TAI64N;

    fn sub(self, d: Duration) -> TAI64N {
        let (carry, n) = if self.1 >= d.subsec_nanos() {
            (0, self.1 - d.subsec_nanos())
        } else {
            (1, NANOSECONDS_PER_SECOND + self.1 - d.subsec_nanos())
        };
        TAI64N(self.0 - carry - d.as_secs(), n)
    }
}

impl TAI64N {
    /// Calculate how much time passes since the `other` timestamp.
    ///
    /// Returns `Ok(Duration)` if `other` is ealier than `self`,
    /// `Err(Duration)` otherwise.
    pub fn duration_since(&self, other: &TAI64N) -> Result<Duration, Duration> {
        if self >= other {
            let (carry, n) = if self.1 >= other.1 {
                (0, self.1 - other.1)
            } else {
                (1, NANOSECONDS_PER_SECOND + self.1 - other.1)
            };
            let s = (self.0).0 - carry - (other.0).0;
            Ok(Duration::new(s, n))
        } else {
            Err(other.duration_since(self).unwrap())
        }
    }
}

// To and From unix timestamp.

// Unix epoch is 1970-01-01 00:00:10 TAI.

impl TAI64 {
    /// Convert unix timestamp to `TAI64`.
    pub fn from_unix(secs: i64) -> Self {
        TAI64(secs.checked_add(10 + (1 << 62)).unwrap() as u64)
    }

    /// Convert `TAI64` to unix timestamp.
    pub fn to_unix(&self) -> i64 {
        (self.0 as i64).checked_sub(10 + (1 << 62)).unwrap()
    }
}

// To and from SystemTime.

/// Unix EPOCH in TAI64N.
pub const UNIX_EPOCH_TAI64N: TAI64N = TAI64N(TAI64(10 + (1 << 62)), 0);

impl TAI64N {
    /// Convert `SystemTime` to `TAI64N`.
    pub fn from_system_time(t: &SystemTime) -> Self {
        match t.duration_since(UNIX_EPOCH) {
            Ok(d) => UNIX_EPOCH_TAI64N + d,
            Err(e) => UNIX_EPOCH_TAI64N - e.duration(),
        }
    }

    /// Convert `TAI64N`to `SystemTime`.
    pub fn to_system_time(&self) -> SystemTime {
        match self.duration_since(&UNIX_EPOCH_TAI64N) {
            Ok(d) => UNIX_EPOCH + d,
            Err(d) => UNIX_EPOCH - d,
        }
    }
}

impl From<SystemTime> for TAI64N {
    fn from(t: SystemTime) -> TAI64N {
        TAI64N::from_system_time(&t)
    }
}

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    extern crate chrono;

    use super::*;

    use std::time::{Duration, UNIX_EPOCH};

    use self::chrono::prelude::*;

    use quickcheck::{Arbitrary, Gen};

    #[test]
    fn known_anser() {
        // https://cr.yp.to/libtai/tai64.html:
        // The timestamp 1992-06-02 08:06:43 UTC should be TAI “40 00 00 00 2a 2b 2c 2d”.

        // There are 16 (positive) leap seconds between 1970-1-1 and
        // 1992-06-02. And chrono `NaiveDate` is in TAI scale. So add
        // 16 seconds.
        let t = NaiveDate::from_ymd(1992, 6, 2).and_hms(8, 6, 59);
        let unix_secs = t.timestamp();
        let tai64 = TAI64::from_unix(unix_secs);

        assert_eq!(tai64.0, 0x400000002a2b2c2d);
        assert_eq!(
            &tai64.to_external(),
            &[0x40, 0, 0, 0, 0x2a, 0x2b, 0x2c, 0x2d]
        );
    }

    #[test]
    fn before_epoch() {
        let t = UNIX_EPOCH - Duration::new(0, 1);
        let tai64n = TAI64N::from_system_time(&t);
        let t1 = tai64n.to_system_time();

        assert_eq!(t, t1);

        let t = UNIX_EPOCH - Duration::new(488294802189, 999999999);
        let tai64n = TAI64N::from_system_time(&t);
        let t1 = tai64n.to_system_time();

        assert_eq!(t, t1);

        let t = UNIX_EPOCH - Duration::new(73234, 68416841);
        let tai64n = TAI64N::from_system_time(&t);
        let t1 = tai64n.to_system_time();

        assert_eq!(t, t1);
    }

    impl Arbitrary for TAI64N {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let s = u64::arbitrary(g);
            let n = u32::arbitrary(g) % NANOSECONDS_PER_SECOND;
            TAI64N(TAI64(s), n)
        }
    }

    quickcheck!{
        // XXX: overflow?
        fn tai64n_add_sub(x: TAI64N, y: Duration) -> bool {
            x + y - y == x
        }

        fn duration_add_sub(x: TAI64N, y: TAI64N) -> bool {
            match x.duration_since(&y) {
                Ok(d) => {
                    assert_eq!(x, y + d);
                    assert_eq!(y, x - d);
                }
                Err(d) => {
                    assert_eq!(y, x + d);
                    assert_eq!(x, y - d);
                }
            }
            true
        }

        fn to_from_system_time(before_epoch: bool, d: Duration) -> bool {
            let st = if before_epoch {
                UNIX_EPOCH + d
            } else {
                UNIX_EPOCH - d
            };

            let st1 = TAI64N::from_system_time(&st).to_system_time();

            st == st1
        }
    }
}
