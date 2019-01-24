#[cfg(feature = "with-chrono")]
mod with_chrono {
    use std::io;

    use chrono::Datelike;
    use byteorder::{WriteBytesExt, NetworkEndian};

    use crate::types::Date;

    impl<T> Date for T where T: Datelike {
        fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
            // TODO: Move this big number to constant with an adequate name
            let days = self.num_days_from_ce() - 730_120;
            writer.write_i32::<NetworkEndian>(4)?;
            writer.write_i32::<NetworkEndian>(days)
        }
    }
}
