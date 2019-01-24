use std::io;

use byteorder::{WriteBytesExt, NetworkEndian};

use crate::types::Uuid as UuidExt;


impl UuidExt for [u8; 16] {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i32::<NetworkEndian>(16)?;
        writer.write_all(self)
    }
}

#[cfg(feature = "with-uuid")]
mod with_uuid {
    use super::*;

    use uuid::Uuid;

    impl UuidExt for Uuid {
        fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
            writer.write_i32::<NetworkEndian>(16)?;
            writer.write_all(self.as_bytes())
        }
    }
}
