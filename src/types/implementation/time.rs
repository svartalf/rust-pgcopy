#[cfg(feature = "with-chrono")]
mod with_chrono {
    use std::io;

    use chrono::Timelike;
    use byteorder::{WriteBytesExt, NetworkEndian};

    use crate::types::Time;

    impl<T> Time for T where T: Timelike {
        fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
            let us = i64::from(self.num_seconds_from_midnight()) * 1_000 * 1_000 + i64::from(self.nanosecond() / 1_000);
            writer.write_i32::<NetworkEndian>(8)?;
            writer.write_i64::<NetworkEndian>(us)
        }
    }
}
