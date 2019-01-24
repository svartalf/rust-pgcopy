use std::io::{Write, Result};

use byteorder::{WriteBytesExt, BigEndian};

#[derive(Debug)]
pub struct Writer<W: Write> {
    inner: W,
}

impl<W: Write> Writer<W> {
    pub fn new(writer: W) -> Writer<W> {
        Writer {
            inner: writer,
        }
    }

    pub fn inner(&self) -> &W {
        &self.inner
    }

    pub fn write_header(&mut self) -> Result<()> {
        self.inner.write(b"PGCOPY\n\xff\r\n\0")?;
        self.inner.write_i32::<BigEndian>(0)?;  // flags, empty for now
        self.inner.write_i32::<BigEndian>(0)?;  // extension area length

        Ok(())
    }

    pub fn write_trailer(&mut self) -> Result<()> {
        self.inner.write_i16::<BigEndian>(-1)
    }

    // Each tuple begins with a 16-bit integer count of the number of fields in the tuple.
    // (Presently, all tuples in a table will have the same count, but that might not always be true.)
    pub fn write_tuple(&mut self, fields: i16) -> Result<()> {
        self.inner.write_i16::<BigEndian>(fields)
    }

    // Integer types

    pub fn write_i16(&mut self, value: i16) -> Result<()> {
        self.inner.write_i32::<BigEndian>(2)?;
        self.inner.write_i16::<BigEndian>(value)
    }

    pub fn write_i32(&mut self, value: i32) -> Result<()> {
        self.inner.write_i32::<BigEndian>(4)?;
        self.inner.write_i32::<BigEndian>(value)
    }

    pub fn write_i64(&mut self, value: i64) -> Result<()> {
        self.inner.write_i32::<BigEndian>(8)?;
        self.inner.write_i64::<BigEndian>(value)
    }

    pub fn write_u16(&mut self, value: u16) -> Result<()> {
        self.inner.write_i32::<BigEndian>(2)?;
        self.inner.write_u16::<BigEndian>(value)
    }

    pub fn write_u32(&mut self, value: u32) -> Result<()> {
        self.inner.write_i32::<BigEndian>(4)?;
        self.inner.write_u32::<BigEndian>(value)
    }

    pub fn write_u64(&mut self, value: u64) -> Result<()> {
        self.inner.write_i32::<BigEndian>(8)?;
        self.inner.write_u64::<BigEndian>(value)
    }

    // Arbitrary precision numbers
    // Floating point types

    pub fn write_f32(&mut self, value: f32) -> Result<()> {
        self.inner.write_i32::<BigEndian>(4)?;
        self.inner.write_f32::<BigEndian>(value)
    }

    pub fn write_f64(&mut self, value: f64) -> Result<()> {
        self.inner.write_i32::<BigEndian>(8)?;
        self.inner.write_f64::<BigEndian>(value)
    }

    // Serial types
    // Character types
    // Binary data types
    // Date/time types
    // Boolean type

    pub fn write_bool(&mut self, value: bool) -> Result<()> {
        self.inner.write_i32::<BigEndian>(1)?;
        self.inner.write_i8(value as i8)
    }
    // Enumerated types
    // Geometric types
    // Network address types
    // Bit String types
    // UUID type
    // XML type
    // JSON types
    // Arrays
    // Composite types
    // Range types
}


// https://github.com/uwescience/myria/blob/master/src/edu/washington/escience/myria/PostgresBinaryTupleWriter.java#L79
// https://github.com/uwescience/myria/blob/master/test/edu/washington/escience/myria/PostgresBinaryTupleWriterTest.java
