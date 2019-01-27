use std::i32;
use std::io::{Write, Result};

use byteorder::{WriteBytesExt, NetworkEndian};
use chrono::{NaiveDateTime, DateTime, TimeZone, Datelike, Timelike};
use uuid::Uuid;


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

    // Integer types
    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/int.c
    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/int8.c

    pub fn write_i16(&mut self, value: i16) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(2)?;
        self.inner.write_i16::<NetworkEndian>(value)
    }

    pub fn write_i32(&mut self, value: i32) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(4)?;
        self.inner.write_i32::<NetworkEndian>(value)
    }

    pub fn write_i64(&mut self, value: i64) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(8)?;
        self.inner.write_i64::<NetworkEndian>(value)
    }

    // TODO: Arbitrary precision numbers

    // Floating point types
    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/float.c

    pub fn write_f32(&mut self, value: f32) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(4)?;
        self.inner.write_f32::<NetworkEndian>(value)
    }

    pub fn write_f64(&mut self, value: f64) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(8)?;
        self.inner.write_f64::<NetworkEndian>(value)
    }

    // TODO: Serial types

    // Character types
    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/varchar.c
    pub fn write_str<T: AsRef<str>>(&mut self, value: T) -> Result<()> {
        let str = value.as_ref();
        debug_assert!(str.len() < i32::MAX as usize);

        self.inner.write_i32::<NetworkEndian>(str.len() as i32)?;  // TODO: Possible value truncation
        self.inner.write_all(str.as_bytes())?;

        Ok(())
    }

    // Binary data types
    pub fn write_bytes<T: AsRef<[u8]>>(&mut self, value: T) -> Result<()> {
        let bytes = value.as_ref();
        debug_assert!(bytes.len() < i32::MAX as usize);

        self.inner.write_i32::<NetworkEndian>(bytes.len() as i32)?;
        self.inner.write_all(bytes)?;

        Ok(())
    }

    // Date/time types

    // Date and time (no time zone)
    pub fn write_timestamp(&mut self, value: NaiveDateTime) -> Result<()> {
        // Microseconds starting from the PSQL epoch (2000-01-01)
        // This big number is a microseconds amount between UNIX epoch and PSQL epoch
        let us = (value.timestamp_nanos() / 1_000) - 946_684_800_000_000;

        self.inner.write_i32::<NetworkEndian>(8)?;
        self.inner.write_i64::<NetworkEndian>(us)
    }

    // Date and time (with time zone)
    pub fn write_timestamp_with_time_zone<Tz: TimeZone>(&mut self, value: DateTime<Tz>) -> Result<()> {
        self.write_timestamp(value.naive_utc())
    }

    pub fn write_date<T: Datelike>(&mut self, value: T) -> Result<()> {
        // 730_120 is a days amount from the "Day 1" to PSQL epoch date (2000-01-01)
        let days = value.num_days_from_ce() - 730_120;

        self.inner.write_i32::<NetworkEndian>(4)?;
        self.inner.write_i32::<NetworkEndian>(days)
    }

    pub fn write_time<T: Timelike>(&mut self, value: T) -> Result<()> {
        let us = i64::from(value.num_seconds_from_midnight()) * 1_000 * 1_000 + i64::from(value.nanosecond() / 1_000);

        self.inner.write_i32::<NetworkEndian>(8)?;
        self.inner.write_i64::<NetworkEndian>(us)
    }

    // Boolean type
    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/bool.c

    pub fn write_bool(&mut self, value: bool) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(1)?;
        self.inner.write_i8(value as i8)
    }

    // TODO: Enumerated types
    // TODO: Geometric types
    // TODO: Network address types
    // TODO: Bit String types

    // UUID type
    pub fn write_uuid(&mut self, value: Uuid) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(16)?;
        self.inner.write_all(value.as_bytes())?;

        Ok(())

    }
    // TODO: XML type
    // TODO: JSON types
    // TODO: Arrays
    // TODO: Composite types
    // TODO: Range types
}


// https://github.com/uwescience/myria/blob/master/src/edu/washington/escience/myria/PostgresBinaryTupleWriter.java#L79
// https://github.com/uwescience/myria/blob/master/test/edu/washington/escience/myria/PostgresBinaryTupleWriterTest.java
