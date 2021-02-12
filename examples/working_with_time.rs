use kdb::{Date, Timespan, Timestamp};
use std::{
    convert::TryFrom,
    time::{Duration, SystemTime},
};

fn main() {
    let t = Timestamp::from(SystemTime::now());
    println!("{} nanos from unix epoch", t.as_nanos_unix());
    println!("{} nanos from 1 Jan 2000 (KDB epoch)", t.as_raw());

    let ts = Timespan::from_nanos(60_000_000_000);
    let ts2 = Timespan::from_secs(60);
    let ts3 = Timespan::try_from(Duration::from_secs(60)).unwrap();
    assert_eq!(ts, ts2);
    assert_eq!(ts, ts3);

    let d = Date::new(2020, 2, 12);
    assert_eq!(d.as_raw(), 20 * 365 + 5 + 31 + 12 - 1); // Days from 1 Jan 2000
}
