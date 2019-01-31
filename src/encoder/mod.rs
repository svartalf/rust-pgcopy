use std::i32;
use std::io::{Write, Result};

use byteorder::{WriteBytesExt, NetworkEndian};

use crate::types;


#[derive(Debug)]
pub struct Encoder<W: Write> {
    inner: W,
}

impl<W: Write> Encoder<W> {
    /// Create new encoder.
    pub fn new(writer: W) -> Encoder<W> {
        Encoder {
            inner: writer,
        }
    }

    /// Acquires a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Acquires a mutable reference to the underlying writer.
    ///
    /// Note that mutating the output/input state of the stream may corrupt this object,
    /// so care must be taken when using this method.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    /// Write binary format header.
    pub fn write_header(&mut self) -> Result<()> {
        self.inner.write_all(b"PGCOPY\n\xff\r\n\0")?;
        self.inner.write_i32::<NetworkEndian>(0)?;  // flags, empty for now
        self.inner.write_i32::<NetworkEndian>(0)?;  // extension area length

        Ok(())
    }

    /// Write binary format trailer.
    pub fn write_trailer(&mut self) -> Result<()> {
        self.inner.write_i16::<NetworkEndian>(-1)
    }

    /// Start a new tuple.
    ///
    /// Each tuple begins with a 16-bit integer count of the number of fields in the tuple.
    pub fn write_tuple(&mut self, fields: i16) -> Result<()> {
        self.inner.write_i16::<NetworkEndian>(fields)
    }

    pub fn write_null(&mut self) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(-1)
    }

    // Numeric types

    // Integer types
    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/int.c
    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/int8.c

    pub fn write_smallint(&mut self, value: i16) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(2)?;
        self.inner.write_i16::<NetworkEndian>(value.into())
    }

    pub fn write_int(&mut self, value: i32) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(4)?;
        self.inner.write_i32::<NetworkEndian>(value.into())
    }

    pub fn write_bigint(&mut self, value: i64) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(8)?;
        self.inner.write_i64::<NetworkEndian>(value.into())
    }

    // Arbitrary precision numbers

    pub fn write_numeric<T: types::Numeric>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    // Floating-point types

    pub fn write_real(&mut self, value: f32) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(4)?;
        self.inner.write_f32::<NetworkEndian>(value)
    }

    pub fn write_double(&mut self, value: f64) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(8)?;
        self.inner.write_f64::<NetworkEndian>(value)
    }

    // TODO: Monetary types

    // Character types
    pub fn write_str<T: AsRef<str>>(&mut self, value: T) -> Result<()> {
        self.write_bytea(value.as_ref().as_bytes())
    }

    // Binary Data types
    pub fn write_bytea<T: AsRef<[u8]>>(&mut self, value: T) -> Result<()> {
        let bytes = value.as_ref();
        debug_assert!(bytes.len() < i32::MAX as usize);

        self.inner.write_i32::<NetworkEndian>(bytes.len() as i32)?;
        self.inner.write_all(bytes)?;

        Ok(())
    }

    // Date/Time types
    pub fn write_timestamp<T: types::Timestamp>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    pub fn write_timestamp_with_time_zone<T: types::TimestampWithTimeZone>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    pub fn write_date<T: types::Date>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    pub fn write_time<T: types::Time>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    pub fn write_interval<T: types::Interval>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    // Boolean type
    pub fn write_bool<T: Into<bool>>(&mut self, value: T) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(1)?;
        self.inner.write_i8(value.into() as i8)
    }

    // TODO: Enumerated Types
    // TODO: Geometric Types

    // Network Address Types
    pub fn write_cidr<T: types::Cidr>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    pub fn write_inet<T: types::Inet>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    pub fn write_macaddr<T: types::MacAddr>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    // TODO: Bit String Types
    // TODO: Text Search Types

    // UUID Type
    pub fn write_uuid<T: types::Uuid>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    // XML Type
    pub fn write_xml<T: types::Xml>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    // JSON Types
    pub fn write_json<T: types::Json>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    pub fn write_jsonb<T: types::Jsonb>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    // TODO: Arrays
    // TODO: Composite Types
    // TODO: Range Types
}

#[macro_use]
#[cfg(test)]
mod tests;
