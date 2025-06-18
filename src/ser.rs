mod map;
mod seq;

pub use map::MapSerializer;
pub use seq::SeqSerializer;

use crate::{error::Error, headers::*};
use std::io::Write;

pub struct Serializer<'ser, W: Write> {
    pub(self) writer: &'ser mut W,
}

impl<'ser, W: Write> Serializer<'ser, W> {
    pub fn new(writer: &'ser mut W) -> Self {
        Self { writer }
    }

    fn serialize_str_value(&mut self, bytes: std::str::Bytes<'_>) -> Result<(), Error> {
        self.writer.write_all(&bytes.collect::<Vec<_>>())?;
        Ok(())
    }

    fn serialize_size(&mut self, mut size: usize) -> Result<(), Error> {
        if size >= 2_usize.pow(62) {
            return Err(Error::TooLong);
        }

        size <<= 2;

        if size < 2_usize.pow(6) {
            self.writer.write_all(&[size as u8])?;
        } else if size < 2_usize.pow(14) {
            self.writer.write_all(&(size as u16 | 1).to_le_bytes())?;
        } else if size < 2_usize.pow(30) {
            self.writer.write_all(&(size as u32 | 2).to_le_bytes())?;
        } else {
            self.writer.write_all(&(size as u64 | 3).to_le_bytes())?;
        }

        Ok(())
    }
}

impl<'a, 'ser, W: Write> serde::Serializer for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, 'ser, W>;
    type SerializeTuple = SeqSerializer<'a, 'ser, W>;
    type SerializeTupleStruct = SeqSerializer<'a, 'ser, W>;
    type SerializeTupleVariant = SeqSerializer<'a, 'ser, W>;
    type SerializeMap = MapSerializer<'a, 'ser, W>;
    type SerializeStruct = MapSerializer<'a, 'ser, W>;
    type SerializeStructVariant = MapSerializer<'a, 'ser, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if v {
            self.writer.write_all(&[TRUE])?;
        } else {
            self.writer.write_all(&[FALSE])?;
        }

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[I8])?;
        self.writer.write_all(&v.to_le_bytes())?;

        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[I16])?;
        self.writer.write_all(&v.to_le_bytes())?;

        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[I32])?;
        self.writer.write_all(&v.to_le_bytes())?;

        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[I64])?;
        self.writer.write_all(&v.to_le_bytes())?;

        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[I128])?;
        self.writer.write_all(&v.to_le_bytes())?;

        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[U8])?;
        self.writer.write_all(&[v])?;

        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[U16])?;
        self.writer.write_all(&v.to_le_bytes())?;

        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[U32])?;
        self.writer.write_all(&v.to_le_bytes())?;

        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[U64])?;
        self.writer.write_all(&v.to_le_bytes())?;

        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[U128])?;
        self.writer.write_all(&v.to_le_bytes())?;

        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[F32])?;
        self.writer.write_all(&v.to_le_bytes())?;

        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[F64])?;
        self.writer.write_all(&v.to_le_bytes())?;

        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let bytes = v.bytes();
        self.writer.write_all(&[STRING])?;
        self.serialize_size(bytes.len())?;
        self.serialize_str_value(bytes)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[U8_ARRAY])?;
        self.serialize_size(v.len())?;
        self.writer.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[NULL])?;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut object = self.serialize_struct(name, 1)?;
        SerializeStruct::serialize_field(&mut object, variant, value)?;
        SerializeStruct::end(object)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let Some(len) = len else {
            return Err(Error::MissingLength);
        };

        Ok(SeqSerializer::new(self, len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_struct(name, 1)?;
        self.serialize_str(variant)?;
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let Some(len) = len else {
            return Err(Error::MissingLength);
        };

        Ok(MapSerializer {
            serializer: self,
            kind: None,
            len,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(MapSerializer {
            serializer: self,
            kind: Some(ObjectKind::String),
            len,
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_struct(name, 1)?;
        self.serialize_str_value(variant.bytes())?;
        self.serialize_struct("", len)
    }
}


/// Serializes the `value` into the `writer`.
pub fn to_writer<W: Write, T: serde::Serialize>(writer: &mut W, value: &T) -> Result<(), Error> {
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)
}
