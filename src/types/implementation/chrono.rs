use chrono::{DateTime, Datelike, Timelike, NaiveDateTime, TimeZone};

use crate::types::{PgTimestamp, PgTimestampWithTimeZone, PgDate, PgTime};

// TODO: These constants are named terribly
// Microseconds amount difference UNIX epoch and PSQL epoch
const UNIX_PSQL_EPOCHS_US_DIFF: i64 = 946_684_800_000_000;

// Days amount difference between Day 1 (0000-01-01) and PSQL epoch
const DAY_1_PSQL_EPOCHS_DAY_DIFF: i32 = 730_120;

impl PgTimestamp for NaiveDateTime {
    fn to_timestamp(&self) -> i64 {
        (self.timestamp_nanos() / 1_000) - UNIX_PSQL_EPOCHS_US_DIFF
    }
}

impl<Tz> PgTimestampWithTimeZone for DateTime<Tz> where Tz: TimeZone {
    fn to_timestamp_with_time_zone(&self) -> i64 {
        self.naive_utc().to_timestamp()
    }
}

impl<T> PgDate for T where T: Datelike {
    fn to_date(&self) -> i32 {
        self.num_days_from_ce() - DAY_1_PSQL_EPOCHS_DAY_DIFF
    }
}

impl<T> PgTime for T where T: Timelike {
    fn to_time(&self) -> i64 {
        i64::from(self.num_seconds_from_midnight()) * 1_000 * 1_000 + i64::from(self.nanosecond() / 1_000)
    }
}
