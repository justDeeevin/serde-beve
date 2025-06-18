use super::Deserializer;
use crate::{
    Error,
    headers::{NULL, header_name},
};
use serde::{
    Deserializer as _,
    de::{EnumAccess, VariantAccess, Visitor},
    forward_to_deserialize_any,
};
use std::io::Read;

pub struct EnumDeserializer<'a, R: Read> {
    pub deserializer: &'a mut Deserializer<R>,
}

impl<'a, 'de, R: Read> EnumAccess<'de> for EnumDeserializer<'a, R> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        Ok((seed.deserialize(&mut self)?, self))
    }
}

impl<'a, 'de, R: Read> VariantAccess<'de> for EnumDeserializer<'a, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        if self.deserializer.get_byte()? != NULL {
            Err(Error::WrongType {
                expected: header_name(NULL),
                found: header_name(self.deserializer.get_byte()?),
            })
        } else {
            Ok(())
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.deserializer)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_struct("", fields, visitor)
    }
}

impl<'a, 'de, R: Read> serde::Deserializer<'de> for &mut EnumDeserializer<'a, R> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::InvalidTag)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.deserializer.get_size()? as u64)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option
        unit unit_struct newtype_struct seq tuple tuple_struct map struct enum ignored_any
    }
}
