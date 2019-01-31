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

#[derive(Debug)]
pub enum PgNumericSign {
    Positive, // Zero is considered as a positive number
    Negative,
}

impl From<PgNumericSign> for i16 {
    fn from(sign: PgNumericSign) -> Self {
        match sign {
            PgNumericSign::Positive => 0,
            PgNumericSign::Negative => 4,
        }
    }
}

pub type PgNumericWeight = i16;
pub type PgNumericDscale = i16;
pub type PgNumericDigits = Vec<i16>;

// ndigits, weight, sign, dscale] + digits
// 2,       0,      0,    2,        3, 1400

//	pq_sendint16(&buf, x.ndigits);
//	pq_sendint16(&buf, x.weight);
//	pq_sendint16(&buf, x.sign);
//	pq_sendint16(&buf, x.dscale);
//	for (i = 0; i < x.ndigits; i++)
//		pq_sendint16(&buf, x.digits[i]);
//
//	PG_RETURN_BYTEA_P(pq_endtypsend(&buf));

/// Convert type into PostgreSQL `numeric` type.
///
/// Since PostgreSQL does not support `+Inf` and `-Inf`, implementations should return `None`
/// in that cases, `NULL` will be written as a field value.
///
/// # Links
///
///  * https://www.postgresql.org/message-id/16572.1091489720%40sss.pgh.pa.us
///  * https://www.postgresql.org/message-id/491DC5F3D279CD4EB4B157DDD62237F404E27FE9%40zipwire.esri.com
pub trait PgNumeric {
    fn exponent(&self) -> i64;
    fn sign(&self) -> PgNumericSign;
    fn digits(&self) -> Vec<u8>;
}

#[cfg(feature = "chrono")]
mod chrono;

#[cfg(feature = "uuid")]
mod uuid;

#[cfg(feature = "with-bigdecimal")]
mod bigdecimal;
