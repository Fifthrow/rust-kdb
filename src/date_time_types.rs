use crate::kapi;
use std::convert::TryFrom;
use std::fmt;
use std::time::{Duration, SystemTime};

pub(crate) const K_NANO_OFFSET: i64 = 946_684_800_000_000_000;
pub(crate) const K_SEC_OFFSET: i64 = K_NANO_OFFSET / 1_000_000_000;
pub(crate) const K_DAY_OFFSET: i32 = (K_SEC_OFFSET / 86_400) as i32;

/// Represents the number of seconds since midnight (00:00)
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Second(i32);

impl Second {
    pub fn new(seconds_since_midnight: i32) -> Self {
        Second(seconds_since_midnight)
    }
}

impl From<i32> for Second {
    fn from(val: i32) -> Second {
        Second(val)
    }
}

impl From<Second> for i32 {
    fn from(val: Second) -> i32 {
        val.0
    }
}

impl fmt::Display for Second {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} seconds", self.0)
    }
}

impl fmt::Debug for Second {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Represents the number of minutes since midnight (00:00).
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Minute(i32);

impl Minute {
    pub fn new(minutes_since_midnight: i32) -> Self {
        Minute(minutes_since_midnight)
    }
}

impl From<i32> for Minute {
    fn from(val: i32) -> Minute {
        Minute(val)
    }
}

impl From<Minute> for i32 {
    fn from(val: Minute) -> i32 {
        val.0
    }
}

impl fmt::Display for Minute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} minutes", self.0)
    }
}

impl fmt::Debug for Minute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Represents the number of days since 1 Jan 2000.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Date(i32);

impl Date {
    pub fn new(year: i32, month: i32, day: i32) -> Self {
        Date(unsafe { kapi::ymd(year, month, day) })
    }
}

impl From<i32> for Date {
    fn from(val: i32) -> Date {
        Date(val)
    }
}

impl From<Date> for i32 {
    fn from(val: Date) -> i32 {
        val.0
    }
}

impl From<SystemTime> for Date {
    fn from(st: SystemTime) -> Date {
        let dur = st.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let dur_secs = (dur.as_secs() / 86400) as i64;
        Date(dur_secs as i32 - K_DAY_OFFSET)
    }
}

impl From<Date> for SystemTime {
    fn from(date: Date) -> SystemTime {
        let secs = date.0 as i64 * 86400 + K_SEC_OFFSET;
        SystemTime::UNIX_EPOCH + Duration::from_secs(secs as u64)
    }
}

/// The number of months since January 2000.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Month(i32);

impl Month {
    pub fn new(months_since_millenium: i32) -> Self {
        Month(months_since_millenium)
    }
}

impl From<i32> for Month {
    fn from(val: i32) -> Month {
        Month(val)
    }
}

impl From<Month> for i32 {
    fn from(val: Month) -> i32 {
        val.0
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} months", self.0)
    }
}

impl fmt::Debug for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// The number of milliseconds since midnight.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Time(i32);

impl Time {
    pub fn new(millis_since_midnight: i32) -> Self {
        Time(millis_since_midnight)
    }
}

impl From<i32> for Time {
    fn from(val: i32) -> Time {
        Time(val)
    }
}

impl From<Time> for i32 {
    fn from(val: Time) -> i32 {
        val.0
    }
}

/// Represents a date and time in KDB. Conversions between the
/// Unix Epoch and the KDB Epoch are done automatically.
///
/// Note that `Timestamp` is the preferred datatype for storing
/// high precision temporal data.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DateTime(f64);

impl DateTime {
    pub fn new(dt: f64) -> Self {
        DateTime(dt)
    }
}

impl From<f64> for DateTime {
    fn from(val: f64) -> DateTime {
        DateTime(val)
    }
}

impl From<DateTime> for f64 {
    fn from(val: DateTime) -> f64 {
        val.0
    }
}

/// Represents a timestamp in KDB. Conversions between the
/// Unix Epoch and the KDB Epoch are done automatically.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Timestamp(i64);

impl Timestamp {
    /// Creates a timestamp from a count of nanoseconds in the unix epoch
    pub fn from_nanos_unix(n: u64) -> Timestamp {
        Timestamp(n as i64 - K_NANO_OFFSET)
    }

    /// Converts the timestamp to the number of nanoseconds from the unix epoch and returns it
    pub fn to_nanos_unix(&self) -> u64 {
        (self.0 + K_NANO_OFFSET) as u64
    }

    /// Returns the raw timestamp stored as KDB values.
    pub fn as_raw(&self) -> i64 {
        self.0
    }

    /// Creates a timestamp based on a count of nanoseconds since 1 Jan 2000.
    pub fn from_raw(nanos_since_millenium: i64) -> Timestamp {
        Timestamp(nanos_since_millenium)
    }
}

impl From<Timestamp> for SystemTime {
    fn from(date: Timestamp) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_nanos(date.to_nanos_unix())
    }
}

impl From<SystemTime> for Timestamp {
    fn from(st: SystemTime) -> Timestamp {
        let dur = st.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        Timestamp::from_nanos_unix(dur.as_nanos() as u64)
    }
}

impl From<i64> for Timestamp {
    fn from(val: i64) -> Timestamp {
        Timestamp(val)
    }
}

impl From<Timestamp> for i64 {
    fn from(val: Timestamp) -> i64 {
        val.0
    }
}

/// Represents the number of nanoseconds since midnight
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Timespan(i64);

impl Timespan {
    pub fn new(nanos_since_midnight: i64) -> Self {
        Timespan(nanos_since_midnight)
    }
}

impl From<i64> for Timespan {
    fn from(val: i64) -> Timespan {
        Timespan(val)
    }
}

impl From<Timespan> for i64 {
    fn from(val: Timespan) -> i64 {
        val.0
    }
}

impl From<Timespan> for Duration {
    fn from(span: Timespan) -> Duration {
        Duration::from_nanos(span.0 as u64)
    }
}

impl TryFrom<Duration> for Timespan {
    type Error = crate::error::ConversionError;

    fn try_from(d: Duration) -> Result<Timespan, Self::Error> {
        let d = d.as_nanos();
        if d > std::i64::MAX as u128 {
            Err(crate::error::ConversionError::DurationTooLong)
        } else {
            Ok(Timespan(d as i64))
        }
    }
}
