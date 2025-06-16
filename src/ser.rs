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
    writer: &'ser mut W,
}

impl<'ser, W: Write> Serializer<'ser, W> {
    pub fn new(writer: &'ser mut W) -> Self {
        Self { writer }
    }

    fn serialize_str_value(&mut self, bytes: std::str::Bytes<'_>) -> Result<(), Error> {
        self.writer.write_all(&bytes.collect::<Vec<_>>())?;
        Ok(())
    }
}

impl<'a, 'ser, W: Write> serde::Serializer for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
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
        self.writer.write_all(&bytes.len().to_le_bytes())?;
        self.serialize_str_value(bytes)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[U8_ARRAY])?;
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
        self.writer.write_all(&[GENERIC_ARRAY])?;
        self.writer.write_all(&len.to_le_bytes())?;
        Ok(self)
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
            key_type: None,
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
            key_type: Some(KeyType::String),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    String,
}

impl KeyType {
    pub fn header(self) -> u8 {
        match self {
            KeyType::U8 => U8_OBJECT,
            KeyType::U16 => U16_OBJECT,
            KeyType::U32 => U32_OBJECT,
            KeyType::U64 => U64_OBJECT,
            KeyType::U128 => U128_OBJECT,
            KeyType::I8 => I8_OBJECT,
            KeyType::I16 => I16_OBJECT,
            KeyType::I32 => I32_OBJECT,
            KeyType::I64 => I64_OBJECT,
            KeyType::I128 => I128_OBJECT,
            KeyType::String => STRING_OBJECT,
        }
    }
}

impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyType::U8 => write!(f, "8-bit unsigned integer"),
            KeyType::U16 => write!(f, "16-bit unsigned integer"),
            KeyType::U32 => write!(f, "32-bit unsigned integer"),
            KeyType::U64 => write!(f, "64-bit unsigned integer"),
            KeyType::U128 => write!(f, "128-bit unsigned integer"),
            KeyType::I8 => write!(f, "8-bit signed integer"),
            KeyType::I16 => write!(f, "16-bit signed integer"),
            KeyType::I32 => write!(f, "32-bit signed integer"),
            KeyType::I64 => write!(f, "64-bit signed integer"),
            KeyType::I128 => write!(f, "128-bit signed integer"),
            KeyType::String => write!(f, "string"),
        }
    }
}

pub struct MapSerializer<'a, 'ser, W: Write> {
    serializer: &'a mut Serializer<'ser, W>,
    key_type: Option<KeyType>,
    len: usize,
}

impl<'a, 'ser, W: Write> MapSerializer<'a, 'ser, W> {
    pub fn set_key_type(&mut self, key_type: KeyType) -> Result<(), Error> {
        self.key_type = Some(key_type);
        self.serializer.writer.write_all(&[key_type.header()])?;
        self.serializer.writer.write_all(&self.len.to_le_bytes())?;
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
        key.serialize(KeySerializer { serializer: self })
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
        key.serialize(KeySerializer { serializer: self })?;
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
        key.serialize(KeySerializer { serializer: self })?;
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

struct KeySerializer<'a, 'b, 'ser, W: Write> {
    serializer: &'a mut MapSerializer<'b, 'ser, W>,
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
        match self.serializer.key_type {
            Some(KeyType::I8) => {}
            None => self.serializer.set_key_type(KeyType::I8)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: KeyType::I8,
                    found,
                });
            }
        }

        self.serializer.serializer.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        match self.serializer.key_type {
            Some(KeyType::I16) => {}
            None => self.serializer.set_key_type(KeyType::I16)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: KeyType::I16,
                    found,
                });
            }
        }

        self.serializer.serializer.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        match self.serializer.key_type {
            Some(KeyType::I32) => {}
            None => self.serializer.set_key_type(KeyType::I32)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: KeyType::I32,
                    found,
                });
            }
        }

        self.serializer.serializer.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        match self.serializer.key_type {
            Some(KeyType::I64) => {}
            None => self.serializer.set_key_type(KeyType::I64)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: KeyType::I64,
                    found,
                });
            }
        }

        self.serializer.serializer.serialize_i64(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        match self.serializer.key_type {
            Some(KeyType::I128) => {}
            None => self.serializer.set_key_type(KeyType::I128)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: KeyType::I128,
                    found,
                });
            }
        }

        self.serializer.serializer.serialize_i128(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        match self.serializer.key_type {
            Some(KeyType::U8) => {}
            None => self.serializer.set_key_type(KeyType::U8)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: KeyType::U8,
                    found,
                });
            }
        }

        self.serializer.serializer.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        match self.serializer.key_type {
            Some(KeyType::U16) => {}
            None => self.serializer.set_key_type(KeyType::U16)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: KeyType::U16,
                    found,
                });
            }
        }

        self.serializer.serializer.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        match self.serializer.key_type {
            Some(KeyType::U32) => {}
            None => self.serializer.set_key_type(KeyType::U32)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: KeyType::U32,
                    found,
                });
            }
        }

        self.serializer.serializer.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        match self.serializer.key_type {
            Some(KeyType::U64) => {}
            None => self.serializer.set_key_type(KeyType::U64)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: KeyType::U64,
                    found,
                });
            }
        }

        self.serializer.serializer.serialize_u64(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        match self.serializer.key_type {
            Some(KeyType::U128) => {}
            None => self.serializer.set_key_type(KeyType::U128)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: KeyType::U128,
                    found,
                });
            }
        }

        self.serializer.serializer.serialize_u128(v)
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
        match self.serializer.key_type {
            Some(KeyType::String) => {}
            None => self.serializer.set_key_type(KeyType::String)?,
            Some(found) => {
                return Err(Error::MismatchedKeyType {
                    expected: KeyType::String,
                    found,
                });
            }
        }
        self.serializer.serializer.serialize_str_value(v.bytes())
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

pub fn to_writer<W: Write, T: serde::Serialize>(writer: &mut W, value: &T) -> Result<(), Error> {
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)
}
