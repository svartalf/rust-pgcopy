//! Conversion traits from various types into PostgreSQL `COPY WITH BINARY` format.

/// Convert type into PostgreSQL `timestamp` type.
///
/// Should return microseconds amount starting from the PSQL epoch (`2000-01-01T00:00:00`)
pub trait PgTimestamp {
    fn to_timestamp(&self) -> i64;
}

/// Convert type into PostgreSQL `timestamp with time zone` type.
///
/// Should return microseconds amount starting from the PSQL epoch (`2000-01-01T00:00:00`)
pub trait PgTimestampWithTimeZone {
    fn to_timestamp_with_time_zone(&self) -> i64;
}

/// Convert type into PostgreSQL `date` type.
///
/// Should return days amount starting from the PSQL epoch date (`2000-01-01`)
pub trait PgDate {
    fn to_date(&self) -> i32;
}

/// Convert type into PostgreSQL `time` type.
///
/// Should return microseconds amount starting from the `00:00:00`
pub trait PgTime {
    fn to_time(&self) -> i64;
}

/// Convert type into PostgreSQL `uuid` type.
///
/// Should return `[u8]` array.
pub trait PgUuid {
    fn to_uuid(&self) -> &[u8];
}

#[cfg(feature = "chrono")]
mod chrono;

#[cfg(feature = "uuid")]
mod uuid;
