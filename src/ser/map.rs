use super::{SeqSerializer, Serializer};
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
    key: bool,
}

impl<'a, W: Write> MapSerializer<'a, W> {
    pub fn new(serializer: &'a mut Serializer<W>, kind: Option<ObjectKind>) -> Self {
        serializer.write = false;
        Self {
            serializer,
            kind,
            keys: Vec::new(),
            values: Vec::new(),
            key: false,
        }
    }

    fn ensure_kind(&mut self, expected: ObjectKind) -> Result<(), Error> {
        if self.key {
            match self.kind {
                None => self.kind = Some(expected),
                Some(found) => {
                    if found != expected {
                        return Err(Error::MismatchedKeyType { expected, found });
                    }
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
        self.key = true;
        let key = key.serialize(&mut *self)?;
        self.keys.push(key);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.key = false;
        let value = value.serialize(&mut *self)?;
        self.values.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let value = match self.kind {
            None => Value::StringObject(vec![]),
            Some(ObjectKind::String) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::String(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Value::StringObject(fields)
            }
            Some(ObjectKind::I8) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::I8(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Value::I8Object(fields)
            }
            Some(ObjectKind::I16) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::I16(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Value::I16Object(fields)
            }
            Some(ObjectKind::I32) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::I32(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Value::I32Object(fields)
            }
            Some(ObjectKind::I64) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::I64(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Value::I64Object(fields)
            }
            Some(ObjectKind::I128) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::I128(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Value::I128Object(fields)
            }

            Some(ObjectKind::U8) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::U8(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Value::U8Object(fields)
            }
            Some(ObjectKind::U16) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::U16(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Value::U16Object(fields)
            }
            Some(ObjectKind::U32) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::U32(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Value::U32Object(fields)
            }
            Some(ObjectKind::U64) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::U64(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Value::U64Object(fields)
            }
            Some(ObjectKind::U128) => {
                let keys = self.keys.into_iter().map(|v| match v {
                    Value::U128(v) => v,
                    _ => unreachable!(),
                });
                let fields = keys.zip(self.values).collect();
                Value::U128Object(fields)
            }
        };

        self.serializer.write = true;
        self.serializer.serialize_value(&value)?;
        Ok(value)
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

impl<'a, 'b, W: Write> serde::Serializer for &'b mut MapSerializer<'a, W> {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SeqSerializer<'b, W>;
    type SerializeTuple = SeqSerializer<'b, W>;
    type SerializeTupleStruct = SeqSerializer<'b, W>;
    type SerializeTupleVariant = SeqSerializer<'b, W>;
    type SerializeMap = MapSerializer<'b, W>;
    type SerializeStruct = MapSerializer<'b, W>;
    type SerializeStructVariant = MapSerializer<'b, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer.serialize_bool(v)
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::I8)?;
        self.serializer.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::I16)?;
        self.serializer.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::I32)?;
        self.serializer.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::I64)?;
        self.serializer.serialize_i64(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::I128)?;
        self.serializer.serialize_i128(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::U8)?;
        self.serializer.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::U16)?;
        self.serializer.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::U32)?;
        self.serializer.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::U64)?;
        self.serializer.serialize_u64(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::U128)?;
        self.serializer.serialize_u128(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer.serialize_f32(v)
        }
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer.serialize_f64(v)
        }
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ensure_kind(ObjectKind::String)?;
        self.serializer.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer.serialize_bytes(v)
        }
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
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer.serialize_unit()
        }
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer
                .serialize_unit_variant(name, variant_index, variant)
        }
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.serializer.serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer
                .serialize_newtype_variant(name, variant_index, variant, value)
        }
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer.serialize_seq(len)
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer.serialize_tuple(len)
        }
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer.serialize_tuple_struct(name, len)
        }
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer
                .serialize_tuple_variant(name, variant_index, variant, len)
        }
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer.serialize_map(len)
        }
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer.serialize_struct(name, len)
        }
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        if self.key {
            Err(Error::InvalidKey)
        } else {
            self.serializer
                .serialize_struct_variant(name, variant_index, variant, len)
        }
    }
}
