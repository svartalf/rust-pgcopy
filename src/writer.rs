use std::io;

use serde::Serialize;

use crate::{Result, Encoder};
use crate::serializer;

pub struct Writer<W: io::Write> {
    inner: Encoder<W>,
}

impl<W> Writer<W> where W: io::Write {
    pub fn from_writer(writer: W) -> Writer<W> {
        Writer {
            inner: Encoder::new(writer),
        }
    }

    pub fn write_header(&mut self) -> Result<()> {
        self.inner.write_header()
    }

    pub fn write_trailer(&mut self) -> Result<()> {
        self.inner.write_trailer()
    }

    pub fn serialize<S>(&mut self, record: S) -> Result<()> where S: Serialize {
        serializer::serialize(record, &mut self.inner)
    }
}
