use std::io;
use std::i16;

use serde::ser;

use crate::error;
use crate::encoder::Encoder;


pub fn serialize<S: ser::Serialize, W: io::Write>(value: S, encoder: &mut Encoder<W>) -> error::Result<()> {
    value.serialize(&mut Serializer{
        encoder,
    })
}

pub struct Serializer<'w, W: 'w + io::Write> {
    encoder: &'w mut Encoder<W>,
}

impl<'s, 'w, W> ser::Serializer for &'s mut Serializer<'w, W> where W: 'w + io::Write {
    type Ok = ();
    type Error = error::Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.encoder.write_bool(value)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.encoder.write_smallint(value)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.encoder.write_int(value)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        self.encoder.write_bigint(value)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.encoder.write_real(value)
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        self.encoder.write_double(value)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.encoder.write_str(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.encoder.write_bytea(value)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.encoder.write_null()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
            where T: ?Sized + ser::Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.encoder.write_null()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _ame: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: ser::Serialize {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: ser::Serialize {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(self, _name: &str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        debug_assert!(len < i16::MAX as usize);

        self.encoder.write_tuple(len as i16)?;

        Ok(self)
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'s, 'w, W> ser::SerializeMap for &'s mut Serializer<'w, W> where W: 'w + io::Write {
    type Ok = ();
    type Error = error::Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
            where T: ?Sized + ser::Serialize {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
            where T: ?Sized + ser::Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'s, 'w, W> ser::SerializeSeq for &'s mut Serializer<'w, W> where W: 'w + io::Write {
    type Ok = ();
    type Error = error::Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
            where T: ?Sized + ser::Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'s, 'w, W> ser::SerializeTuple for &'s mut Serializer<'w, W> where W: 'w + io::Write {
    type Ok = ();
    type Error = error::Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
            where T: ?Sized + ser::Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}


impl<'s, 'w, W> ser::SerializeTupleVariant for &'s mut Serializer<'w, W> where W: 'w + io::Write {
    type Ok = ();
    type Error = error::Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
            where T: ?Sized + ser::Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'s, 'w, W> ser::SerializeTupleStruct for &'s mut Serializer<'w, W> where W: 'w + io::Write {
    type Ok = ();
    type Error = error::Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
            where T: ?Sized + ser::Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'s, 'w, W> ser::SerializeStruct for &'s mut Serializer<'w, W> where W: 'w + io::Write {
    type Ok = ();
    type Error = error::Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
            where T: ?Sized + ser::Serialize {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'s, 'w, W> ser::SerializeStructVariant for &'s mut Serializer<'w, W> where W: 'w + io::Write {
    type Ok = ();
    type Error = error::Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
            where T: ?Sized + ser::Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
