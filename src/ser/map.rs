use super::Serializer;
use crate::{error::Error, headers::ObjectKind};
use serde::{
    Serialize,
    ser::{SerializeMap, SerializeStruct, SerializeStructVariant},
};
use std::io::Write;

pub struct MapSerializer<'a, 'ser, W: Write> {
    pub(super) serializer: &'a mut Serializer<'ser, W>,
    pub(super) kind: Option<ObjectKind>,
    pub(super) len: usize,
}

impl<'a, 'ser, W: Write> MapSerializer<'a, 'ser, W> {
    pub(super) fn set_key_type(&mut self, key_type: ObjectKind) -> Result<(), Error> {
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
