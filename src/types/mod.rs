//! Conversion traits from various types into PostgreSQL `COPY WITH BINARY` format.

pub trait Numeric {}

pub trait Timestamp {}

pub trait TimestampWithTimeZone {}

pub trait Date {}

pub trait Time {}

pub trait Interval {}

pub trait Cidr {}

pub trait Inet {}

pub trait MacAddr {}

pub trait Uuid {}

pub trait Xml {}

pub trait Json {}

pub trait Jsonb {}

