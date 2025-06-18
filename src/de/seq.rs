use super::Deserializer;
use crate::{error::Error, headers::ArrayKind};
use serde::{
    de::{SeqAccess, Visitor},
    forward_to_deserialize_any,
};
use std::io::Read;

pub struct SeqDeserializer<'a, 'de, R: Read> {
    pub deserializer: &'a mut Deserializer<'de, R>,
    pub len: usize,
    pub index: usize,
    pub kind: ArrayKind,
}

impl<'a, 'de, R: Read> SeqDeserializer<'a, 'de, R> {
    pub fn new(deserializer: &'a mut Deserializer<'de, R>, len: usize, kind: ArrayKind) -> Self {
        Self {
            deserializer,
            len,
            kind,
            index: 0,
        }
    }
}

impl<'a, 'de, R: Read> SeqAccess<'de> for SeqDeserializer<'a, 'de, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.index == self.len {
            return Ok(None);
        }

        let kind = self.kind;
        seed.deserialize(ElementDeserializer { seq: self, kind })
            .map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct ElementDeserializer<'a, 'b, 'de, R: Read> {
    pub seq: &'a mut SeqDeserializer<'b, 'de, R>,
    pub kind: ArrayKind,
}

impl<'a, 'b, 'de, R: Read> serde::Deserializer<'de> for ElementDeserializer<'a, 'b, 'de, R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::Generic => self.seq.deserializer.deserialize_any(visitor),
            ArrayKind::BF16 => visitor.visit_f32(self.seq.deserializer.get_bf16_value()?),
            ArrayKind::F16 => visitor.visit_f32(self.seq.deserializer.get_f16_value()?),
            ArrayKind::String => self.deserialize_string(visitor),
            ArrayKind::Boolean => self.deserialize_bool(visitor),
            ArrayKind::I8 => self.deserialize_i8(visitor),
            ArrayKind::I16 => self.deserialize_i16(visitor),
            ArrayKind::I32 => self.deserialize_i32(visitor),
            ArrayKind::I64 => self.deserialize_i64(visitor),
            ArrayKind::I128 => self.deserialize_i128(visitor),
            ArrayKind::U8 => self.deserialize_u8(visitor),
            ArrayKind::U16 => self.deserialize_u16(visitor),
            ArrayKind::U32 => self.deserialize_u32(visitor),
            ArrayKind::U64 => self.deserialize_u64(visitor),
            ArrayKind::U128 => self.deserialize_u128(visitor),
            ArrayKind::F32 => self.deserialize_f32(visitor),
            ArrayKind::F64 => self.deserialize_f64(visitor),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::I8 => visitor.visit_i8(self.seq.deserializer.get_i8_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_i8(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::I8,
                found,
            }),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::I16 => visitor.visit_i16(self.seq.deserializer.get_i16_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_i16(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::I16,
                found,
            }),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::I32 => visitor.visit_i32(self.seq.deserializer.get_i32_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_i32(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::I32,
                found,
            }),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::I64 => visitor.visit_i64(self.seq.deserializer.get_i64_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_i64(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::I64,
                found,
            }),
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::I128 => visitor.visit_i128(self.seq.deserializer.get_i128_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_i128(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::I128,
                found,
            }),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::U8 => visitor.visit_u8(self.seq.deserializer.get_u8_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_u8(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::U8,
                found,
            }),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::U16 => visitor.visit_u16(self.seq.deserializer.get_u16_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_u16(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::U16,
                found,
            }),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::U32 => visitor.visit_u32(self.seq.deserializer.get_u32_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_u32(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::U32,
                found,
            }),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::U64 => visitor.visit_u64(self.seq.deserializer.get_u64_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_u64(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::U64,
                found,
            }),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::U128 => visitor.visit_u128(self.seq.deserializer.get_u128_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_u128(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::U128,
                found,
            }),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::F32 => visitor.visit_f32(self.seq.deserializer.get_f32_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_f32(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::F32,
                found,
            }),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::F64 => visitor.visit_f64(self.seq.deserializer.get_f64_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_f64(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::F64,
                found,
            }),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::Boolean => {
                let sub_index = self.seq.index % 8;
                let byte = if sub_index == 0 {
                    self.seq.deserializer.get_byte()?
                } else {
                    self.seq.deserializer.peek_byte()?
                };
                let bit = byte & (1 << sub_index);

                visitor.visit_bool(bit != 0)
            }
            ArrayKind::Generic => self.seq.deserializer.deserialize_bool(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::Boolean,
                found,
            }),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            ArrayKind::String => visitor.visit_string(self.seq.deserializer.get_string_value()?),
            ArrayKind::Generic => self.seq.deserializer.deserialize_string(visitor),
            found => Err(Error::MismatchedElementType {
                expected: ArrayKind::String,
                found,
            }),
        }
    }

    forward_to_deserialize_any! {
        char str bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
    }
}
