//! TAI64(N) timestamp generation, parsing and calculation.

#![no_std]
#![doc(html_root_url = "https://docs.rs/tai64/3.0.0")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(feature = "chrono")]
use chrono::{DateTime, NaiveDateTime, Utc};
use core::{
    convert::{TryFrom, TryInto},
    fmt, ops,
    time::Duration,
};
#[cfg(feature = "serde")]
use serde::{de, ser, Deserialize, Serialize};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::time::{SystemTime, UNIX_EPOCH};

/// Unix epoch in TAI64: 1970-01-01 00:00:10 TAI.
pub const UNIX_EPOCH_TAI64: TAI64 = TAI64(10 + (1 << 62));

/// Unix EPOCH in TAI64N: 1970-01-01 00:00:10 TAI.
pub const UNIX_EPOCH_TAI64N: TAI64N = TAI64N(UNIX_EPOCH_TAI64, 0);

/// Length of serialized TAI64
const TAI64_LEN: usize = 8;

/// Length of serialized TAI64N
const TAI64N_LEN: usize = 12;

/// Number of nanoseconds in a second
const NANOS_PER_SECOND: u32 = 1_000_000_000;

/// A `TAI64` label.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TAI64(pub u64);

impl TAI64 {
    /// Get `TAI64N` timestamp according to system clock.
    #[cfg(feature = "std")]
    pub fn now() -> Self {
        TAI64N::now().into()
    }

    /// Parse TAI64 from a byte slice
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        slice.try_into()
    }

    /// Serialize TAI64 as bytes
    pub fn to_bytes(self) -> [u8; TAI64_LEN] {
        self.into()
    }

    /// Convert Unix timestamp to `TAI64`.
    pub fn from_unix(secs: i64) -> Self {
        TAI64((secs + 10 + (1 << 62)) as u64)
    }

    /// Convert `TAI64` to unix timestamp.
    pub fn to_unix(self) -> i64 {
        (self.0 as i64) - (10 + (1 << 62))
    }
}

impl From<TAI64N> for TAI64 {
    /// Remove the nanosecond component from a TAI64N value
    fn from(other: TAI64N) -> Self {
        other.0
    }
}

impl From<[u8; TAI64_LEN]> for TAI64 {
    /// Parse TAI64 from external representation
    fn from(bytes: [u8; TAI64_LEN]) -> Self {
        TAI64(u64::from_be_bytes(bytes))
    }
}

impl<'a> TryFrom<&'a [u8]> for TAI64 {
    type Error = Error;

    fn try_from(slice: &'a [u8]) -> Result<Self, Error> {
        let bytes: [u8; TAI64_LEN] = slice.try_into().map_err(|_| Error::LengthInvalid)?;
        Ok(bytes.into())
    }
}

impl From<TAI64> for [u8; 8] {
    /// Serialize TAI64 to external representation
    fn from(tai: TAI64) -> [u8; 8] {
        tai.0.to_be_bytes()
    }
}

impl ops::Add<u64> for TAI64 {
    type Output = Self;

    fn add(self, x: u64) -> Self {
        TAI64(self.0 + x)
    }
}

impl ops::Sub<u64> for TAI64 {
    type Output = Self;

    fn sub(self, x: u64) -> Self {
        TAI64(self.0 - x)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for TAI64 {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(<[u8; TAI64_LEN]>::deserialize(deserializer)?.into())
    }
}

#[cfg(feature = "serde")]
impl Serialize for TAI64 {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_bytes().serialize(serializer)
    }
}

/// A `TAI64N` timestamp.
///
/// Invariant: The nanosecond part <= 999999999.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TAI64N(pub TAI64, pub u32);

impl TAI64N {
    /// Get `TAI64N` timestamp according to system clock.
    #[cfg(feature = "std")]
    pub fn now() -> Self {
        TAI64N::from_system_time(&SystemTime::now())
    }

    /// Parse TAI64N from a byte slice
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        slice.try_into()
    }

    /// Serialize TAI64N as bytes
    pub fn to_bytes(self) -> [u8; TAI64N_LEN] {
        self.into()
    }

    /// Calculate how much time passes since the `other` timestamp.
    ///
    /// Returns `Ok(Duration)` if `other` is earlier than `self`,
    /// `Err(Duration)` otherwise.
    pub fn duration_since(&self, other: &Self) -> Result<Duration, Duration> {
        if self >= other {
            let (carry, n) = if self.1 >= other.1 {
                (0, self.1 - other.1)
            } else {
                (1, NANOS_PER_SECOND + self.1 - other.1)
            };

            let s = (self.0).0 - carry - (other.0).0;
            Ok(Duration::new(s, n))
        } else {
            Err(other.duration_since(self).unwrap())
        }
    }

    /// Convert `SystemTime` to `TAI64N`.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[cfg(feature = "std")]
    pub fn from_system_time(t: &SystemTime) -> Self {
        match t.duration_since(UNIX_EPOCH) {
            Ok(d) => UNIX_EPOCH_TAI64N + d,
            Err(e) => UNIX_EPOCH_TAI64N - e.duration(),
        }
    }

    /// Convert `TAI64N`to `SystemTime`.
    #[cfg(feature = "std")]
    pub fn to_system_time(&self) -> SystemTime {
        match self.duration_since(&UNIX_EPOCH_TAI64N) {
            Ok(d) => UNIX_EPOCH + d,
            Err(d) => UNIX_EPOCH - d,
        }
    }

    /// Convert `chrono::DateTime<Utc>` to `TAI64N`
    #[cfg(feature = "chrono")]
    pub fn from_datetime_utc(t: &DateTime<Utc>) -> Self {
        let unix_epoch: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc);

        let duration = t.signed_duration_since(unix_epoch);

        if duration.num_seconds() > 0 {
            UNIX_EPOCH_TAI64N + duration.to_std().unwrap()
        } else {
            UNIX_EPOCH_TAI64N - unix_epoch.signed_duration_since(*t).to_std().unwrap()
        }
    }

    /// Convert `TAI64N` to `chrono::DateTime<Utc>`
    #[cfg(feature = "chrono")]
    pub fn to_datetime_utc(&self) -> DateTime<Utc> {
        let (secs, nanos) = match self.to_system_time().duration_since(UNIX_EPOCH) {
            Ok(duration) => (duration.as_secs() as i64, duration.subsec_nanos()),
            Err(e) => (
                -(e.duration().as_secs() as i64),
                e.duration().subsec_nanos(),
            ),
        };

        DateTime::from_utc(NaiveDateTime::from_timestamp(secs, nanos), Utc)
    }
}

impl From<TAI64> for TAI64N {
    /// Remove the nanosecond component from a TAI64N value
    fn from(other: TAI64) -> Self {
        TAI64N(other, 0)
    }
}

impl TryFrom<[u8; TAI64N_LEN]> for TAI64N {
    type Error = Error;

    /// Parse TAI64 from external representation
    fn try_from(bytes: [u8; TAI64N_LEN]) -> Result<Self, Error> {
        let secs = TAI64::from_slice(&bytes[..TAI64_LEN])?;

        let mut nano_bytes = [0u8; 4];
        nano_bytes.copy_from_slice(&bytes[TAI64_LEN..]);
        let nanos = u32::from_be_bytes(nano_bytes);

        if nanos < NANOS_PER_SECOND {
            Ok(TAI64N(secs, nanos))
        } else {
            Err(Error::NanosInvalid)
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for TAI64N {
    type Error = Error;

    fn try_from(slice: &'a [u8]) -> Result<Self, Error> {
        let bytes: [u8; TAI64N_LEN] = slice.try_into().map_err(|_| Error::LengthInvalid)?;
        bytes.try_into()
    }
}

impl From<TAI64N> for [u8; TAI64N_LEN] {
    /// Serialize TAI64 to external representation
    fn from(tai: TAI64N) -> [u8; TAI64N_LEN] {
        let mut result = [0u8; TAI64N_LEN];
        result[..TAI64_LEN].copy_from_slice(&tai.0.to_bytes());
        result[TAI64_LEN..].copy_from_slice(&tai.1.to_be_bytes());
        result
    }
}

#[cfg(feature = "std")]
impl From<SystemTime> for TAI64N {
    fn from(t: SystemTime) -> Self {
        TAI64N::from_system_time(&t)
    }
}

#[cfg(feature = "chrono")]
impl From<DateTime<Utc>> for TAI64N {
    fn from(t: DateTime<Utc>) -> Self {
        TAI64N::from_datetime_utc(&t)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl ops::Add<Duration> for TAI64N {
    type Output = Self;

    fn add(self, d: Duration) -> Self {
        let n = self.1 + d.subsec_nanos();

        let (carry, n) = if n >= NANOS_PER_SECOND {
            (1, n - NANOS_PER_SECOND)
        } else {
            (0, n)
        };

        TAI64N(self.0 + d.as_secs() + carry, n)
    }
}

impl ops::Sub<Duration> for TAI64N {
    type Output = Self;

    fn sub(self, d: Duration) -> Self {
        let (carry, n) = if self.1 >= d.subsec_nanos() {
            (0, self.1 - d.subsec_nanos())
        } else {
            (1, NANOS_PER_SECOND + self.1 - d.subsec_nanos())
        };
        TAI64N(self.0 - carry - d.as_secs(), n)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for TAI64N {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        <[u8; TAI64N_LEN]>::deserialize(deserializer)?
            .try_into()
            .map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for TAI64N {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_bytes().serialize(serializer)
    }
}

/// TAI64 errors
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Invalid length
    LengthInvalid,

    /// Nanosecond part must be <= 999999999.
    NanosInvalid,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            Error::LengthInvalid => "length invalid",
            Error::NanosInvalid => "invalid number of nanoseconds",
        };

        write!(f, "{}", description)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    #[cfg(feature = "chrono")]
    use chrono::prelude::*;
    use quickcheck::{quickcheck, Arbitrary, Gen};

    #[cfg(feature = "chrono")]
    #[test]
    fn known_answer() {
        // https://cr.yp.to/libtai/tai64.html:
        // The timestamp 1992-06-02 08:06:43 UTC should be TAI “40 00 00 00 2a 2b 2c 2d”.

        // There are 16 (positive) leap seconds between 1970-1-1 and
        // 1992-06-02. And chrono `NaiveDate` is in TAI scale. So add
        // 16 seconds.
        let t = NaiveDate::from_ymd(1992, 6, 2).and_hms(8, 6, 59);
        let unix_secs = t.timestamp();
        let tai64 = TAI64::from_unix(unix_secs);

        assert_eq!(tai64.0, 0x400000002a2b2c2d);
        assert_eq!(&tai64.to_bytes(), &[0x40, 0, 0, 0, 0x2a, 0x2b, 0x2c, 0x2d]);
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
            let n = u32::arbitrary(g) % NANOS_PER_SECOND;
            TAI64N(TAI64(s), n)
        }
    }

    quickcheck! {
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
