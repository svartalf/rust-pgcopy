use std::i32;
use std::io::{Write, Result};
use std::cmp;

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
//     pq_sendint16(&buf, x.ndigits);
//     pq_sendint16(&buf, x.weight);
//     pq_sendint16(&buf, x.sign);
//     pq_sendint16(&buf, x.dscale);
//     for (i = 0; i < x.ndigits; i++)
//         pq_sendint16(&buf, x.digits[i]);
    pub fn write_numeric<T: types::PgNumeric>(&mut self, value: Option<T>) -> Result<()> {
        match value {
            None => { // NaN case
                self.inner.write_i32::<NetworkEndian>(8)?;
                self.inner.write_i16::<NetworkEndian>(0)?; // Zero digits
                self.inner.write_i16::<NetworkEndian>(0)?; // First digit weight
                // PostgreSQL' C sources are using `0xc000` here, but it's overflowing for i16
                self.inner.write_u16::<NetworkEndian>(0xc000)?; // NaN marker instead of sign
                self.inner.write_u16::<NetworkEndian>(0) // Display scale is zero too
            },
            Some(val) => {
                let exponent = val.exponent();
                let sign = val.sign();
                let mut raw_digits = val.digits();
                let mut weight = 0;

                for _ in 0..(exponent % 4 + 4) {
                    raw_digits.push(0);
                }
                raw_digits.reverse();

                let mut weight = 0;
                let mut digits: Vec<i16> = vec![];
                let mut offset = 0usize;

                for (idx, chunk) in raw_digits.chunks(4).enumerate() {
                    if chunk.iter().filter(|d| **d > 0).count() > 0 {
                        break;
                    } else {
                        weight += 1;
                        offset += 4;
                    }
                }

                for (idx, chunk) in raw_digits[offset..].chunks(4).enumerate() {
                    let mut res = 0;
                    for (idx, digit) in chunk.iter().enumerate() {
                        res += *digit as i16 * (10_i16.pow(idx as u32));
                    }
                    digits.insert(0, res);
                }
                println!("weight0: {}", weight);
                println!("nte: {}", f32::floor(exponent as f32 / 4.0) as i16);
                let ndigits = digits.len() as i16;
                let dscale = -cmp::min(0, exponent);
                weight += (f32::floor(exponent as f32 / 4.0) as i16) + ndigits - 1;

                println!("ndigits: {:?}", ndigits);
                println!("weight: {:?}", weight);
                println!("sign: {:?}", sign);
                println!("dscale: {:?}", dscale);
                println!("exponent: {:?}", exponent);
                println!("digits: {:?}", digits);

                self.inner.write_i32::<NetworkEndian>(2 * (4 + digits.len() as i32))?;
                self.inner.write_i16::<NetworkEndian>(digits.len() as i16)?;
                self.inner.write_i16::<NetworkEndian>(weight)?;
                let sign_repr = match sign {
                    types::PgNumericSign::Positive => 0x0000,
                    types::PgNumericSign::Negative => 0x4000,
                };
                self.inner.write_u16::<NetworkEndian>(sign_repr)?;
                self.inner.write_u16::<NetworkEndian>(dscale as u16)?;
                for digit in digits {
                    self.inner.write_i16::<NetworkEndian>(digit);
                }

                Ok(())


//                let length = 2 + 2 + 2 + 2 + digits.len() as i32;
//                self.inner.write_i32::<NetworkEndian>(length)?;
//                self.inner.write_i16::<NetworkEndian>(digits.len() as i16)?;
//                self.inner.write_i16::<NetworkEndian>(weight)?;
//                let sign_repr = match sign {
//                    types::PgNumericSign::Positive => 0x0000,
//                    types::PgNumericSign::Negative => 0x4000,
//                };
//                self.inner.write_i16::<NetworkEndian>(sign_repr)?;;
//                self.inner.write_i16::<NetworkEndian>(0)?;
//
//                Ok(())
            }
        }
    }

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
        self.write_bytes(value.as_ref().as_bytes())
    }

    // Binary data types
    pub fn write_bytes<T: AsRef<[u8]>>(&mut self, value: T) -> Result<()> {
        let bytes = value.as_ref();
        // TODO: Should panic probably
        debug_assert!(bytes.len() < i32::MAX as usize);

        self.inner.write_i32::<NetworkEndian>(bytes.len() as i32)?;
        self.inner.write_all(bytes)?;

        Ok(())
    }

    // Date/time types

    // Date and time (no time zone)
    pub fn write_timestamp<T: types::PgTimestamp>(&mut self, value: T) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(8)?;
        self.inner.write_i64::<NetworkEndian>(value.to_timestamp())
    }

    // Date and time (with time zone)
    pub fn write_timestamp_with_time_zone<T: types::PgTimestampWithTimeZone>(&mut self, value: T) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(8)?;
        self.inner.write_i64::<NetworkEndian>(value.to_timestamp_with_time_zone())
    }

    pub fn write_date<T: types::PgDate>(&mut self, value: T) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(4)?;
        self.inner.write_i32::<NetworkEndian>(value.to_date())
    }

    pub fn write_time<T: types::PgTime>(&mut self, value: T) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(8)?;
        self.inner.write_i64::<NetworkEndian>(value.to_time())
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
    pub fn write_uuid<T: types::PgUuid>(&mut self, value: T) -> Result<()> {
        self.inner.write_i32::<NetworkEndian>(16)?;
        self.inner.write_all(value.to_uuid())?;

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
