use std::io;

use serde_json::{self, Value};

use crate::types::{PgType, PgJson, PgJsonb};


impl PgType for Value {
    fn to_writer<W: io::Write>(&self, writer: W) -> io::Result<()> {
        serde_json::to_writer(writer, self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl PgJson for Value {}

impl PgJsonb for Value {}
