use std::i32;
use std::io::{Write};

use byteorder::{WriteBytesExt, NetworkEndian};

use crate::Result;
use crate::types;


/// Low-level encoder for binary format.
///
/// End users are required to manually call all necessary methods in a right order.
///
/// ```rust
/// use pgcopy::Encoder;
///
/// let mut buf: Vec<u8> = vec![];
/// let mut encoder = Encoder::new(&mut buf);
///
/// encoder.write_header().unwrap();
///
/// encoder.write_tuple(3).unwrap(); // First tuple with three columns
/// encoder.write_smallint(1).unwrap(); // First column
/// encoder.write_bool(false).unwrap(); // Second
/// encoder.write_str("first").unwrap(); // Third
///
/// encoder.write_tuple(3).unwrap(); // Second tuple
/// encoder.write_smallint(2).unwrap();
/// encoder.write_bool(true).unwrap();
/// encoder.write_str("second").unwrap();
///
/// encoder.write_trailer().unwrap();
///
/// println!("{:?}", encoder.get_ref());
/// ```
#[derive(Debug, Clone)]
pub struct Encoder<W: Write> {
    inner: W,
}

impl<W> Encoder<W> where W: Write {
    /// Creates new encoder.
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

    /// Writes binary format header.
    ///
    /// Caller is required to invoke this method first before starting to write tuples data.
    pub fn write_header(&mut self) -> Result<()> {
        self.inner.write_all(b"PGCOPY\n\xff\r\n\0")?;
        self.inner.write_i32::<NetworkEndian>(0)?;  // flags, empty for now
        self.inner.write_i32::<NetworkEndian>(0)?;  // extension area length

        Ok(())
    }

    /// Writes binary format trailer.
    ///
    /// Caller is required to invoke this method last immediately after writing tuples data.
    pub fn write_trailer(&mut self) -> Result<()> {
        self.inner.write_i16::<NetworkEndian>(-1)?;

        Ok(())
    }

    /// Starts a new tuple.
    ///
    /// Each tuple begins with a signed 16-bit integer count of the number of fields in the tuple.
    /// Presently, all tuples in a table will have the same count.
    pub fn write_tuple(&mut self, fields: i16) -> Result<()> {
        self.inner.write_i16::<NetworkEndian>(fields)?;

        Ok(())
    }

    /// Writes `NULL` as a column value.
    pub fn write_null(&mut self) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(-1)?;

        Ok(())
    }

    // Numeric types

    // Integer types
    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/int.c
    // https://github.com/postgres/postgres/blob/master/src/backend/utils/adt/int8.c

    /// Writes `smallint` type value.
    pub fn write_smallint(&mut self, value: i16) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(2)?;
        self.inner.write_i16::<NetworkEndian>(value)?;

        Ok(())
    }

    /// Writes `int` type value.
    pub fn write_int(&mut self, value: i32) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(4)?;
        self.inner.write_i32::<NetworkEndian>(value)?;

        Ok(())
    }

    /// Writes `bigint` type value.
    pub fn write_bigint(&mut self, value: i64) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(8)?;
        self.inner.write_i64::<NetworkEndian>(value)?;

        Ok(())
    }

    // Arbitrary precision numbers

    #[doc(hidden)]
    pub fn write_numeric<T: types::Numeric>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    // Floating-point types

    /// Writes `real` type value.
    pub fn write_real(&mut self, value: f32) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(4)?;
        self.inner.write_f32::<NetworkEndian>(value)?;

        Ok(())
    }

    /// Writes `double precision` type value.
    pub fn write_double(&mut self, value: f64) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(8)?;
        self.inner.write_f64::<NetworkEndian>(value)?;

        Ok(())
    }

    // TODO: Monetary types

    // Character types
    /// Writes character type value.
    ///
    /// Any of `character varying(n)`, `character(n)` or `text` column type should be handled by this method.
    pub fn write_str<T: AsRef<str>>(&mut self, value: T) -> Result<()> {
        self.write_bytea(value.as_ref().as_bytes())
    }

    // Binary Data types

    /// Writes `bytea` type value.
    pub fn write_bytea<T: AsRef<[u8]>>(&mut self, value: T) -> Result<()> {
        let bytes = value.as_ref();
        debug_assert!(bytes.len() < i32::MAX as usize);

        self.inner.write_i32::<NetworkEndian>(bytes.len() as i32)?;
        self.inner.write_all(bytes)?;

        Ok(())
    }

    // Date/Time types
    /// Writes `timestamp` type value.
    ///
    /// See [Timestamp](types/trait.Timestamp.html) type implementors for available options here.
    pub fn write_timestamp<T: types::Timestamp>(&mut self, value: T) -> Result<()> {
        value.to_writer(&mut self.inner)?;

        Ok(())
    }

    /// Writes `timestamp with time zone` type value.
    ///
    /// See [TimestampWithTimeZone](types/trait.TimestampWithTimeZone.html) type implementors for available options here.
    pub fn write_timestamp_with_time_zone<T: types::TimestampWithTimeZone>(&mut self, value: T) -> Result<()> {
        value.to_writer(&mut self.inner)?;

        Ok(())
    }

    /// Writes `date` type value.
    ///
    /// See [Date](types/trait.Date.html) type implementors for available options here.
    pub fn write_date<T: types::Date>(&mut self, value: T) -> Result<()> {
        value.to_writer(&mut self.inner)?;

        Ok(())
    }

    /// Writes `time` type value.
    ///
    /// See [Time](types/trait.Time.html) type implementors for available options here.
    pub fn write_time<T: types::Time>(&mut self, value: T) -> Result<()> {
        value.to_writer(&mut self.inner)?;

        Ok(())
    }

    #[doc(hidden)]
    pub fn write_interval<T: types::Interval>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    // Boolean type
    /// Writes `bool` type value.
    pub fn write_bool<T: Into<bool>>(&mut self, value: T) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(1)?;
        self.inner.write_i8(value.into() as i8)?;

        Ok(())
    }

    // TODO: Enumerated Types
    // TODO: Geometric Types

    // Network Address Types
    #[doc(hidden)]
    pub fn write_cidr<T: types::Cidr>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    #[doc(hidden)]
    pub fn write_inet<T: types::Inet>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    #[doc(hidden)]
    pub fn write_macaddr<T: types::MacAddr>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    // TODO: Bit String Types
    // TODO: Text Search Types

    // UUID Type
    /// Writes `uuid` type value.
    ///
    /// See [Uuid](types/trait.Uuid.html) type implementors for available options here.
    pub fn write_uuid<T: types::Uuid>(&mut self, value: T) -> Result<()> {
        value.to_writer(&mut self.inner)?;

        Ok(())
    }

    // XML Type
    #[doc(hidden)]
    pub fn write_xml<T: types::Xml>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    // JSON Types
    #[doc(hidden)]
    pub fn write_json<T: types::Json>(&mut self, _value: T) -> Result<()> {
        unimplemented!()
    }

    #[doc(hidden)]
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
