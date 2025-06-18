use super::Deserializer;
use crate::{Error, headers::ObjectKind};
use serde::{
    de::{MapAccess, Visitor},
    forward_to_deserialize_any,
};
use std::io::Read;

pub struct MapDeserializer<'a, 'de, R: Read> {
    pub deserializer: &'a mut Deserializer<'de, R>,
    pub len: usize,
    pub index: usize,
    pub kind: ObjectKind,
}

impl<'a, 'de, R: Read> MapDeserializer<'a, 'de, R> {
    pub fn new(deserializer: &'a mut Deserializer<'de, R>, len: usize, kind: ObjectKind) -> Self {
        Self {
            deserializer,
            len,
            kind,
            index: 0,
        }
    }
}

impl<'a, 'de, R: Read> MapAccess<'de> for MapDeserializer<'a, 'de, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index == self.len {
            return Ok(None);
        }

        seed.deserialize(self).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}

impl<'a, 'de, R: Read> serde::Deserializer<'de> for &mut MapDeserializer<'a, 'de, R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ObjectKind::String => self.deserialize_string(visitor),
            ObjectKind::I8 => self.deserialize_i8(visitor),
            ObjectKind::I16 => self.deserialize_i16(visitor),
            ObjectKind::I32 => self.deserialize_i32(visitor),
            ObjectKind::I64 => self.deserialize_i64(visitor),
            ObjectKind::I128 => self.deserialize_i128(visitor),
            ObjectKind::U8 => self.deserialize_u8(visitor),
            ObjectKind::U16 => self.deserialize_u16(visitor),
            ObjectKind::U32 => self.deserialize_u32(visitor),
            ObjectKind::U64 => self.deserialize_u64(visitor),
            ObjectKind::U128 => self.deserialize_u128(visitor),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ObjectKind::I8 => visitor.visit_i8(self.deserializer.get_i8_value()?),
            ObjectKind::String => self.deserialize_string(visitor),
            found => Err(Error::MismatchedKeyType {
                expected: ObjectKind::I8,
                found,
            }),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ObjectKind::I16 => visitor.visit_i16(self.deserializer.get_i16_value()?),
            ObjectKind::String => self.deserialize_string(visitor),
            found => Err(Error::MismatchedKeyType {
                expected: ObjectKind::I16,
                found,
            }),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ObjectKind::I32 => visitor.visit_i32(self.deserializer.get_i32_value()?),
            ObjectKind::String => self.deserialize_string(visitor),
            found => Err(Error::MismatchedKeyType {
                expected: ObjectKind::I32,
                found,
            }),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ObjectKind::I64 => visitor.visit_i64(self.deserializer.get_i64_value()?),
            ObjectKind::String => self.deserialize_string(visitor),
            found => Err(Error::MismatchedKeyType {
                expected: ObjectKind::I64,
                found,
            }),
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ObjectKind::I128 => visitor.visit_i128(self.deserializer.get_i128_value()?),
            ObjectKind::String => self.deserialize_string(visitor),
            found => Err(Error::MismatchedKeyType {
                expected: ObjectKind::I128,
                found,
            }),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ObjectKind::U8 => visitor.visit_u8(self.deserializer.get_u8_value()?),
            ObjectKind::String => self.deserialize_string(visitor),
            found => Err(Error::MismatchedKeyType {
                expected: ObjectKind::U8,
                found,
            }),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ObjectKind::U16 => visitor.visit_u16(self.deserializer.get_u16_value()?),
            ObjectKind::String => self.deserialize_string(visitor),
            found => Err(Error::MismatchedKeyType {
                expected: ObjectKind::U16,
                found,
            }),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ObjectKind::U32 => visitor.visit_u32(self.deserializer.get_u32_value()?),
            ObjectKind::String => self.deserialize_string(visitor),
            found => Err(Error::MismatchedKeyType {
                expected: ObjectKind::U32,
                found,
            }),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ObjectKind::U64 => visitor.visit_u64(self.deserializer.get_u64_value()?),
            ObjectKind::String => self.deserialize_string(visitor),
            found => Err(Error::MismatchedKeyType {
                expected: ObjectKind::U64,
                found,
            }),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ObjectKind::U128 => visitor.visit_u128(self.deserializer.get_u128_value()?),
            ObjectKind::String => self.deserialize_string(visitor),
            found => Err(Error::MismatchedKeyType {
                expected: ObjectKind::U128,
                found,
            }),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ObjectKind::String => visitor.visit_string(self.deserializer.get_string_value()?),
            found => Err(Error::MismatchedKeyType {
                expected: ObjectKind::String,
                found,
            }),
        }
    }

    forward_to_deserialize_any! {
        bool f32 f64 char str bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
    }
}
