use std::convert::TryFrom;
use std::fmt;
use std::time::{Duration, SystemTime};

pub const K_NANO_OFFSET: i64 = 946_684_800_000_000_000;
pub const K_SEC_OFFSET: i64 = K_NANO_OFFSET / 1_000_000_000;
pub const K_DAY_OFFSET: i32 = (K_SEC_OFFSET / 86_400) as i32;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Second(i32);
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

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Minute(i32);
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

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Date(i32);
impl From<i32> for Date {
    fn from(val: i32) -> Date {
        Date(val - K_DAY_OFFSET)
    }
}

impl From<Date> for i32 {
    fn from(val: Date) -> i32 {
        val.0 + K_DAY_OFFSET
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

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Month(i32);
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

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Time(i32);
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

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct DateTime(f64);

impl From<f64> for DateTime {
    fn from(val: f64) -> DateTime {
        DateTime(val - K_SEC_OFFSET as f64)
    }
}

impl From<DateTime> for f64 {
    fn from(val: DateTime) -> f64 {
        val.0 + K_SEC_OFFSET as f64
    }
}

impl From<DateTime> for SystemTime {
    fn from(date: DateTime) -> SystemTime {
        let secs = date.0 as i64 * 86400 + K_SEC_OFFSET;
        SystemTime::UNIX_EPOCH + Duration::from_secs(secs as u64)
    }
}

impl From<SystemTime> for DateTime {
    fn from(st: SystemTime) -> DateTime {
        let dur = st.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        DateTime((dur.as_secs_f64() / 86400.0) - K_DAY_OFFSET as f64)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn from_nanos_unix(n: u64) -> Timestamp {
        Timestamp(n as i64 - K_NANO_OFFSET)
    }

    pub fn to_nanos_unix(&self) -> u64 {
        (self.0 + K_NANO_OFFSET) as u64
    }

    pub fn as_raw(&self) -> i64 {
        self.0
    }

    pub fn from_raw(n: i64) -> Timestamp {
        Timestamp(n)
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

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Timespan(i64);

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
