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

//    // Integer types
//    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/int.c
//    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/int8.c
//
//    pub fn write_smallint<T: types::SmallInt>(&mut self, value: T) -> Result<()> {
//        value.to_writer(&mut self.inner)
//    }
//
//    pub fn write_integer<T: types::Integer>(&mut self, value: T) -> Result<()> {
//        value.to_writer(&mut self.inner)
//    }
//
//    pub fn write_bigint<T: types::BigInt>(&mut self, value: T) -> Result<()> {
//        value.to_writer(&mut self.inner)
//    }
//
//    pub fn write_numeric<T: types::Numeric>(&mut self, value: T) -> Result<()> {
//        value.to_writer(&mut self.inner)
//    }
//
//    // Floating point types
//    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/float.c
//
//    pub fn write_real<T: types::Real>(&mut self, value: T) -> Result<()> {
//        value.to_writer(&mut self.inner)
//    }
//
//    pub fn write_double<T: types::Double>(&mut self, value: T) -> Result<()> {
//        value.to_writer(&mut self.inner)
//    }
//
//    // TODO: Serial types
//
//    // Character types
//    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/varchar.c
//    pub fn write_str<T: AsRef<str>>(&mut self, value: T) -> Result<()> {
//        self.write_bytes(value.as_ref().as_bytes())
//    }
//
//    // Binary data types
//    pub fn write_bytes<T: AsRef<[u8]>>(&mut self, value: T) -> Result<()> {
//        let bytes = value.as_ref();
//        // TODO: Should panic probably
//        debug_assert!(bytes.len() < i32::MAX as usize);
//
//        self.inner.write_i32::<NetworkEndian>(bytes.len() as i32)?;
//        self.inner.write_all(bytes)?;
//
//        Ok(())
//    }

    // Date/time types

    // Date and time (no time zone)
//    pub fn write_timestamp<T: types::PgTimestamp>(&mut self, value: T) -> Result<()> {
//        self.inner.write_i32::<NetworkEndian>(8)?;
//        self.inner.write_i64::<NetworkEndian>(value.to_timestamp())
//    }
//
//    // Date and time (with time zone)
//    pub fn write_timestamp_with_time_zone<T: types::PgTimestampWithTimeZone>(&mut self, value: T) -> Result<()> {
//        self.inner.write_i32::<NetworkEndian>(8)?;
//        self.inner.write_i64::<NetworkEndian>(value.to_timestamp_with_time_zone())
//    }
//
//    pub fn write_date<T: types::PgDate>(&mut self, value: T) -> Result<()> {
//        self.inner.write_i32::<NetworkEndian>(4)?;
//        self.inner.write_i32::<NetworkEndian>(value.to_date())
//    }
//
//    pub fn write_time<T: types::PgTime>(&mut self, value: T) -> Result<()> {
//        self.inner.write_i32::<NetworkEndian>(8)?;
//        self.inner.write_i64::<NetworkEndian>(value.to_time())
//    }

    // Boolean type
    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/bool.c

//    pub fn write_bool(&mut self, value: bool) -> Result<()> {
//        self.inner.write_i32::<NetworkEndian>(1)?;
//        self.inner.write_i8(value as i8)
//    }
//
//    // TODO: Enumerated types
//    // TODO: Geometric types
//    // TODO: Network address types
//    // TODO: Bit String types
//
//    // UUID type
//    pub fn write_uuid<T: types::PgUuid>(&mut self, value: T) -> Result<()> {
//        self.inner.write_i32::<NetworkEndian>(16)?;
//        self.inner.write_all(value.to_uuid())?;
//
//        Ok(())
//    }

    // TODO: XML type
    // TODO: JSON types
//    pub fn write_json<T: types::PgJson>(&mut self, value: T) -> Result<()> {
//        unimplemented!()
//    }
    // TODO: Arrays
    // TODO: Composite types
    // TODO: Range types
}


// https://github.com/uwescience/myria/blob/master/src/edu/washington/escience/myria/PostgresBinaryTupleWriter.java#L79
// https://github.com/uwescience/myria/blob/master/test/edu/washington/escience/myria/PostgresBinaryTupleWriterTest.java
