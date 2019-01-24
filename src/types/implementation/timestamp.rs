#[cfg(feature = "with-chrono")]
mod with_chrono {
    use std::io;

    use chrono::{NaiveDateTime, DateTime, TimeZone};
    use byteorder::{WriteBytesExt, NetworkEndian};

    use crate::types::{Timestamp, TimestampWithTimeZone};

    impl Timestamp for NaiveDateTime {
        fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
            // TODO: Move this big number to constant with an adequate name
            let us = (self.timestamp_nanos() / 1_000) - 946_684_800_000_000;

            writer.write_i32::<NetworkEndian>(8)?;
            writer.write_i64::<NetworkEndian>(us)
        }
    }

    impl<Tz: TimeZone> TimestampWithTimeZone for DateTime<Tz> {
        fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
            // TODO: Move this big number to constant with an adequate name
            let us = (self.naive_utc().timestamp_nanos() / 1_000) - 946_684_800_000_000;

            writer.write_i32::<NetworkEndian>(8)?;
            writer.write_i64::<NetworkEndian>(us)
        }

    }
}
