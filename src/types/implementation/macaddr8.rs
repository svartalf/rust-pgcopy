use std::io;

use byteorder::{WriteBytesExt, NetworkEndian};

use crate::types::MacAddr8;

impl MacAddr8 for [u8; 6] {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i32::<NetworkEndian>(8)?;

        writer.write_u8(self[0])?;
        writer.write_u8(self[1])?;
        writer.write_u8(self[2])?;

        writer.write_u8(0xff)?;
        writer.write_u8(0xfe)?;

        writer.write_u8(self[3])?;
        writer.write_u8(self[4])?;
        writer.write_u8(self[5])?;

        Ok(())
    }
}

impl MacAddr8 for [u8; 8] {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i32::<NetworkEndian>(8)?;
        for byte in self {
            writer.write_u8(*byte)?;
        }

        Ok(())
    }
}

#[cfg(feature = "with-eui48")]
mod with_eui48 {
    use std::io;

    use byteorder::{WriteBytesExt, NetworkEndian};
    use eui48::MacAddress;
    use crate::types::MacAddr8;

    impl MacAddr8 for MacAddress {
        fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
            writer.write_i32::<NetworkEndian>(8)?;

            let bytes = self.as_bytes();

            writer.write_u8(bytes[0])?;
            writer.write_u8(bytes[1])?;
            writer.write_u8(bytes[2])?;

            writer.write_u8(0xff)?;
            writer.write_u8(0xfe)?;

            writer.write_u8(bytes[3])?;
            writer.write_u8(bytes[4])?;
            writer.write_u8(bytes[5])?;

            Ok(())
        }
    }
}
