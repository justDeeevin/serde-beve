use super::Serializer;
use crate::{Value, error::Error, headers::ObjectKind};
use serde::{
    Serialize,
    ser::{SerializeMap, SerializeStruct, SerializeStructVariant},
};
use std::io::Write;

pub struct MapSerializer<'a, W: Write> {
    serializer: &'a mut Serializer<W>,
    kind: Option<ObjectKind>,
    keys: Vec<Value>,
    values: Vec<Value>,
}

impl<'a, W: Write> MapSerializer<'a, W> {
    pub fn new(serializer: &'a mut Serializer<W>, kind: Option<ObjectKind>) -> Self {
        Self {
            serializer,
            kind,
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    fn ensure_kind(&mut self, expected: ObjectKind) -> Result<(), Error> {
        match self.kind {
            None => self.kind = Some(expected),
            Some(found) => {
                if found != expected {
                    return Err(Error::MismatchedKeyType {
                        expected: found,
                        found,
                    });
                }
            }
        }
        Ok(())
    }
}

impl<'a, W: Write> SerializeMap for MapSerializer<'a, W> {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let key = key.serialize(&mut *self)?;
        self.keys.push(key);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.values.push(value.serialize(&mut *self.serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.kind {
            None => Ok(Value::StringObject(vec![])),
            Some(ObjectKind::String) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::String(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Ok(Value::StringObject(fields))
            }
            Some(ObjectKind::I8) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::I8(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Ok(Value::I8Object(fields))
            }
            Some(ObjectKind::I16) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::I16(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Ok(Value::I16Object(fields))
            }
            Some(ObjectKind::I32) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::I32(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Ok(Value::I32Object(fields))
            }
            Some(ObjectKind::I64) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::I64(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Ok(Value::I64Object(fields))
            }
            Some(ObjectKind::I128) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::I128(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Ok(Value::I128Object(fields))
            }

            Some(ObjectKind::U8) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::U8(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Ok(Value::U8Object(fields))
            }
            Some(ObjectKind::U16) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::U16(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Ok(Value::U16Object(fields))
            }
            Some(ObjectKind::U32) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::U32(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Ok(Value::U32Object(fields))
            }
            Some(ObjectKind::U64) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::U64(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Ok(Value::U64Object(fields))
            }
            Some(ObjectKind::U128) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::U128(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Ok(Value::U128Object(fields))
            }
        }
    }
}

impl<'a, W: Write> SerializeStruct for MapSerializer<'a, W> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.serialize_key(key)?;
        self.serialize_value(value)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeMap::end(self)
    }
}

impl<'a, W: Write> SerializeStructVariant for MapSerializer<'a, W> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_key(key)?;
        self.serialize_value(value)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeMap::end(self)
    }
}

type Impossible = serde::ser::Impossible<Value, Error>;

impl<'a, W: Write> serde::Serializer for &mut MapSerializer<'a, W> {
    type Ok = Value;
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
        self.ensure_kind(ObjectKind::I8)?;
        Ok(Value::I8(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::I16)?;
        Ok(Value::I16(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::I32)?;
        Ok(Value::I32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::I64)?;
        Ok(Value::I64(v))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::I128)?;
        Ok(Value::I128(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::U8)?;
        Ok(Value::U8(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::U16)?;
        Ok(Value::U16(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::U32)?;
        Ok(Value::U32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::U64)?;
        Ok(Value::U64(v))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::U128)?;
        Ok(Value::U128(v))
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
        self.ensure_kind(ObjectKind::String)?;
        Ok(Value::String(v.bytes().collect()))
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
