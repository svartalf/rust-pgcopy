//! Serializer (*TBD*) and low-level encoder for PostgreSQL [`COPY WITH BINARY`](https://www.postgresql.org/docs/11/sql-copy.html#id-1.9.3.55.9.4)
//! format, which is "somewhat faster than the text and CSV formats".

mod encoder;
mod serializer;
mod writer;
mod error;
pub mod types;

pub use encoder::Encoder;
pub use writer::Writer;
pub use error::{Error, ErrorKind, Result};
