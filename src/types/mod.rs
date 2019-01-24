//! Conversion traits from various types into PostgreSQL binary format.
//!
//! Trait implementations should properly write bytes into supplied writer
//! according to PostgreSQL binary format.

use std::io;

#[doc(hidden)]
pub trait Numeric {}

/// Trait for `timestamp` type implementations.
///
/// Implementors should write:
/// 1. signed 4 bytes of the following data length, value is required to be `8`
/// 2. signed 8 bytes of the timestamp, expressed as a microseconds amount from `2000-01-01T00:00:00+00:00`.
pub trait Timestamp {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
}

/// Trait for `timestamp with time zone` type implementations.
///
/// Implementors should write:
/// 1. signed 4 bytes of the following data length, value is required to be `8`
/// 2. signed 8 bytes of the timestamp, expressed as a microseconds amount from `2000-01-01T00:00:00+00:00`.
///
/// Datetime with some timezone specified should be converted into datetime with UTC timezone before.
pub trait TimestampWithTimeZone {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
}

/// Trait for `date` type implementations.
///
/// Implementors should write:
/// 1. signed 4 bytes of the following data length, value is required to be `4`
/// 2. signed 4 bytes of the date, expressed as a days amount from `2000-01-01`.
pub trait Date {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
}

/// Trait for `time` type implementations.
///
/// Implementors should write:
/// 1. signed 4 bytes of the following data length, value is required to be `8`
/// 2. signed 8 bytes of the date, expressed as a microseconds amount starting from the `00:00:00`.
pub trait Time {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
}

#[doc(hidden)]
pub trait Interval {}

#[doc(hidden)]
pub trait Cidr {}

#[doc(hidden)]
pub trait Inet {}

#[doc(hidden)]
pub trait MacAddr {}

/// Trait for `uuid` type implementations.
///
/// Implementors should write:
/// 1. signed 4 bytes of the following data length, value is required to be `32`
/// 2. 16 bytes of the UUID
pub trait Uuid {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
}

#[doc(hidden)]
pub trait Xml {}

#[doc(hidden)]
pub trait Json {}

#[doc(hidden)]
pub trait Jsonb {}

mod implementation;
