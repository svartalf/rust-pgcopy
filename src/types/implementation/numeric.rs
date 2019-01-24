use std::io;

use byteorder::{WriteBytesExt, NetworkEndian};

use crate::types;

impl types::Type for i16 {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i32::<NetworkEndian>(2)?;
        writer.write_i16::<NetworkEndian>(*self)
    }
}
impl types::SmallInt for i16 {}

impl types::Type for i32 {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i32::<NetworkEndian>(4)?;
        writer.write_i32::<NetworkEndian>(*self)
    }
}
impl types::Integer for i32 {}

impl types::Type for i64 {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i32::<NetworkEndian>(8)?;
        writer.write_i64::<NetworkEndian>(*self)
    }
}
impl types::BigInt for i64 {}

impl types::Type for f32 {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i32::<NetworkEndian>(4)?;
        writer.write_f32::<NetworkEndian>(*self)
    }
}
impl types::Real for f32 {}

impl types::Type for f64 {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_i32::<NetworkEndian>(8)?;
        writer.write_f64::<NetworkEndian>(*self)
    }
}
impl types::Double for f64 {}
