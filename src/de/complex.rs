use std::io::Read;

use serde::{de::SeqAccess, forward_to_deserialize_any};

use crate::{ArrayKind, Error};

use super::Deserializer;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ComplexKind {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
}

impl From<ComplexKind> for ArrayKind {
    fn from(kind: ComplexKind) -> Self {
        match kind {
            ComplexKind::I8 => ArrayKind::I8,
            ComplexKind::I16 => ArrayKind::I16,
            ComplexKind::I32 => ArrayKind::I32,
            ComplexKind::I64 => ArrayKind::I64,
            ComplexKind::I128 => ArrayKind::I128,
            ComplexKind::U8 => ArrayKind::U8,
            ComplexKind::U16 => ArrayKind::U16,
            ComplexKind::U32 => ArrayKind::U32,
            ComplexKind::U64 => ArrayKind::U64,
            ComplexKind::U128 => ArrayKind::U128,
            ComplexKind::F32 => ArrayKind::F32,
            ComplexKind::F64 => ArrayKind::F64,
        }
    }
}

pub struct ComplexDeserializer<'a, R: Read> {
    deserializer: &'a mut Deserializer<R>,
    kind: ComplexKind,
    index: usize,
}

impl<'a, R: Read> ComplexDeserializer<'a, R> {
    pub fn new(deserializer: &'a mut Deserializer<R>, kind: ComplexKind) -> Self {
        Self {
            deserializer,
            kind,
            index: 0,
        }
    }

    fn ensure_kind(&mut self, expected: ComplexKind) -> Result<(), Error> {
        if self.kind == expected {
            Ok(())
        } else {
            Err(Error::MismatchedElementType {
                expected: expected.into(),
                found: self.kind.into(),
            })
        }
    }
}

impl<'a, 'de, R: Read> SeqAccess<'de> for ComplexDeserializer<'a, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.index == 2 {
            Ok(None)
        } else {
            self.index += 1;
            seed.deserialize(self).map(Some)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(2)
    }
}

impl<'a, 'de, R: Read> serde::Deserializer<'de> for &mut ComplexDeserializer<'a, R> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::MismatchedElementType {
            expected: self.kind.into(),
            found: ArrayKind::Generic,
        })
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ensure_kind(ComplexKind::I8)?;
        visitor.visit_i8(self.deserializer.get_i8_value()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ensure_kind(ComplexKind::I16)?;
        visitor.visit_i16(self.deserializer.get_i16_value()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ensure_kind(ComplexKind::I32)?;
        visitor.visit_i32(self.deserializer.get_i32_value()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ensure_kind(ComplexKind::I64)?;
        visitor.visit_i64(self.deserializer.get_i64_value()?)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ensure_kind(ComplexKind::I128)?;
        visitor.visit_i128(self.deserializer.get_i128_value()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ensure_kind(ComplexKind::U8)?;
        visitor.visit_u8(self.deserializer.get_u8_value()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ensure_kind(ComplexKind::U16)?;
        visitor.visit_u16(self.deserializer.get_u16_value()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ensure_kind(ComplexKind::U32)?;
        visitor.visit_u32(self.deserializer.get_u32_value()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ensure_kind(ComplexKind::U64)?;
        visitor.visit_u64(self.deserializer.get_u64_value()?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ensure_kind(ComplexKind::U128)?;
        visitor.visit_u128(self.deserializer.get_u128_value()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ensure_kind(ComplexKind::F32)?;
        visitor.visit_f32(self.deserializer.get_f32_value()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ensure_kind(ComplexKind::F64)?;
        visitor.visit_f64(self.deserializer.get_f64_value()?)
    }

    forward_to_deserialize_any! {
        bool char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
    }
}

pub struct ComplexArrayDeserializer<'a, R: Read> {
    deserializer: ComplexDeserializer<'a, R>,
    len: usize,
    index: usize,
}

impl<'a, R: Read> ComplexArrayDeserializer<'a, R> {
    pub fn new(deserializer: ComplexDeserializer<'a, R>, len: usize) -> Self {
        Self {
            deserializer,
            len,
            index: 0,
        }
    }
}

impl<'a, 'de, R: Read> SeqAccess<'de> for ComplexArrayDeserializer<'a, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.index == self.len {
            Ok(None)
        } else {
            self.index += 1;
            seed.deserialize(&mut self.deserializer).map(Some)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'a, 'de, R: Read> serde::Deserializer<'de> for ComplexArrayDeserializer<'a, R> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::MismatchedElementType {
            expected: ArrayKind::Complex,
            found: ArrayKind::Generic,
        })
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(self.deserializer)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(self.deserializer)
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 bool char str string bytes byte_buf option unit unit_struct newtype_struct tuple_struct map struct enum identifier ignored_any
    }
}
