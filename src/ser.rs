use serde::{
    Serialize,
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
};

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

    fn serialize_variant(&mut self, variant_index: u32) -> Result<(), Error> {
        self.writer.write_all(&[TAG])?;
        self.serialize_size(variant_index as usize)?;
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
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_variant(variant_index)?;
        self.serialize_unit()
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
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.serialize_variant(variant_index)?;
        value.serialize(&mut *self)
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
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_variant(variant_index)?;
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
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_variant(variant_index)?;
        self.serialize_struct(variant, len)
    }
}

impl<'ser, W: Write> SerializeSeq for &mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'ser, W: Write> SerializeTuple for &mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'ser, W: Write> SerializeTupleStruct for &mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'ser, W: Write> SerializeTupleVariant for &mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct MapSerializer<'a, 'ser, W: Write> {
    pub(self) serializer: &'a mut Serializer<'ser, W>,
    pub(self) kind: Option<ObjectKind>,
    pub(self) len: usize,
}

impl<'a, 'ser, W: Write> MapSerializer<'a, 'ser, W> {
    pub(self) fn set_key_type(&mut self, key_type: ObjectKind) -> Result<(), Error> {
        self.kind = Some(key_type);
        self.serializer.writer.write_all(&[key_type.header()])?;
        self.serializer.serialize_size(self.len)?;
        Ok(())
    }
}

impl<'a, 'ser, W: Write> SerializeMap for MapSerializer<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        key.serialize(KeySerializer { map: self })
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, 'ser, W: Write> SerializeStruct for MapSerializer<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        key.serialize(KeySerializer { map: self })?;
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, 'ser, W: Write> SerializeStructVariant for MapSerializer<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(KeySerializer { map: self })?;
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

struct KeySerializer<'a, 'b, 'ser, W: Write> {
    pub map: &'a mut MapSerializer<'b, 'ser, W>,
}

type Impossible = serde::ser::Impossible<(), Error>;

impl<'a, 'b, 'ser, W: Write> serde::Serializer for KeySerializer<'a, 'b, 'ser, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible;
    type SerializeTuple = Impossible;
    type SerializeTupleStruct = Impossible;
    type SerializeTupleVariant = Impossible;
    type SerializeMap = Impossible;
    type SerializeStruct = Impossible;
    type SerializeStructVariant = Impossible;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        match self.map.kind {
            Some(ObjectKind::I8) => {}
            None => self.map.set_key_type(ObjectKind::I8)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: ObjectKind::I8,
                    found,
                });
            }
        }

        self.map.serializer.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        match self.map.kind {
            Some(ObjectKind::I16) => {}
            None => self.map.set_key_type(ObjectKind::I16)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: ObjectKind::I16,
                    found,
                });
            }
        }

        self.map.serializer.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        match self.map.kind {
            Some(ObjectKind::I32) => {}
            None => self.map.set_key_type(ObjectKind::I32)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: ObjectKind::I32,
                    found,
                });
            }
        }

        self.map.serializer.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        match self.map.kind {
            Some(ObjectKind::I64) => {}
            None => self.map.set_key_type(ObjectKind::I64)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: ObjectKind::I64,
                    found,
                });
            }
        }

        self.map.serializer.serialize_i64(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        match self.map.kind {
            Some(ObjectKind::I128) => {}
            None => self.map.set_key_type(ObjectKind::I128)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: ObjectKind::I128,
                    found,
                });
            }
        }

        self.map.serializer.serialize_i128(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        match self.map.kind {
            Some(ObjectKind::U8) => {}
            None => self.map.set_key_type(ObjectKind::U8)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: ObjectKind::U8,
                    found,
                });
            }
        }

        self.map.serializer.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        match self.map.kind {
            Some(ObjectKind::U16) => {}
            None => self.map.set_key_type(ObjectKind::U16)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: ObjectKind::U16,
                    found,
                });
            }
        }

        self.map.serializer.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        match self.map.kind {
            Some(ObjectKind::U32) => {}
            None => self.map.set_key_type(ObjectKind::U32)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: ObjectKind::U32,
                    found,
                });
            }
        }

        self.map.serializer.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        match self.map.kind {
            Some(ObjectKind::U64) => {}
            None => self.map.set_key_type(ObjectKind::U64)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: ObjectKind::U64,
                    found,
                });
            }
        }

        self.map.serializer.serialize_u64(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        match self.map.kind {
            Some(ObjectKind::U128) => {}
            None => self.map.set_key_type(ObjectKind::U128)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: ObjectKind::U128,
                    found,
                });
            }
        }

        self.map.serializer.serialize_u128(v)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        match self.map.kind {
            Some(ObjectKind::String) => {}
            None => self.map.set_key_type(ObjectKind::String)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: ObjectKind::String,
                    found,
                });
            }
        }
        self.map.serializer.serialize_str_value(v.bytes())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidKey)
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
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Err(Error::InvalidKey)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::InvalidKey)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::InvalidKey)
    }
}

pub struct SeqSerializer<'a, 'ser, W: Write> {
    serializer: &'a mut Serializer<'ser, W>,
    kind: Option<ArrayKind>,
    len: usize,
    hold: (usize, u8),
}

impl<'a, 'ser, W: Write> SeqSerializer<'a, 'ser, W> {
    pub fn new(serializer: &'a mut Serializer<'ser, W>, len: usize) -> Self {
        Self {
            serializer,
            kind: None,
            len,
            hold: (0, 0),
        }
    }

    pub(self) fn set_kind(&mut self, kind: ArrayKind) -> Result<(), Error> {
        self.kind = Some(kind);
        self.serializer.writer.write_all(&[kind.header()])?;
        self.serializer.serialize_size(self.len)?;
        Ok(())
    }
}

impl<'a, 'ser, W: Write> SerializeSeq for SeqSerializer<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(ElementSerializer { seq: self })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.hold.1 % 8 != 0 {
            self.serializer.writer.write_all(&[self.hold.1])?;
        }

        Ok(())
    }
}

impl<'a, 'ser, W: Write> SerializeTuple for SeqSerializer<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(ElementSerializer { seq: self })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<'a, 'ser, W: Write> SerializeTupleStruct for SeqSerializer<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(ElementSerializer { seq: self })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<'a, 'ser, W: Write> SerializeTupleVariant for SeqSerializer<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(ElementSerializer { seq: self })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

struct ElementSerializer<'a, 'b, 'ser, W: Write> {
    pub seq: &'a mut SeqSerializer<'b, 'ser, W>,
}

impl<'a, 'b, 'ser, W: Write> serde::Serializer for ElementSerializer<'a, 'b, 'ser, W> {
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
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Boolean)?,
            Some(ArrayKind::Boolean) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Boolean,
                    found,
                });
            }
        }

        self.seq.hold.1 |= v as u8;
        self.seq.hold.1 <<= 1;
        self.seq.hold.0 += 1;

        if self.seq.hold.0 % 8 == 0 {
            self.seq.serializer.writer.write_all(&[self.seq.hold.1])?;
            self.seq.hold.1 = 0;
        }

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::I8)?,
            Some(ArrayKind::I8) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::I8,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::I16)?,
            Some(ArrayKind::I16) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::I16,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::I32)?,
            Some(ArrayKind::I32) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::I32,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::I64)?,
            Some(ArrayKind::I64) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::I64,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::I128)?,
            Some(ArrayKind::I128) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::I128,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::U8)?,
            Some(ArrayKind::U8) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::U8,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::U16)?,
            Some(ArrayKind::U16) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::U16,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::U32)?,
            Some(ArrayKind::U32) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::U32,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::U64)?,
            Some(ArrayKind::U64) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::U64,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::U128)?,
            Some(ArrayKind::U128) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::U128,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::F32)?,
            Some(ArrayKind::F32) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::F32,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::F64)?,
            Some(ArrayKind::F64) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::F64,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<(), Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::String)?,
            Some(ArrayKind::String) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::String,
                    found,
                });
            }
        }
        self.seq.serializer.serialize_str_value(v.bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq
            .serializer
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq
            .serializer
            .serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq
            .serializer
            .serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_map(len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_struct(name, len)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedArrayType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq
            .serializer
            .serialize_struct_variant(name, variant_index, variant, len)
    }
}

pub fn to_writer<W: Write, T: serde::Serialize>(writer: &mut W, value: &T) -> Result<(), Error> {
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)
}
