use std::io;

use byteorder::{WriteBytesExt, NetworkEndian};

use crate::types::MacAddr;

impl MacAddr for [u8; 6] {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i32::<NetworkEndian>(6)?;
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
    use crate::types::MacAddr;

    impl MacAddr for MacAddress {
        fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
            writer.write_i32::<NetworkEndian>(6)?;
            for byte in self.as_bytes() {
                writer.write_u8(*byte)?;
            }

            Ok(())
        }
    }
}
