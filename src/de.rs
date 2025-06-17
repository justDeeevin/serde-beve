use std::io::Read;

use serde::{
    Deserializer as _,
    de::{EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor},
    forward_to_deserialize_any,
};

use crate::{Error, error::SpecialType, headers::*};

pub struct Deserializer<'de, R: Read> {
    reader: &'de mut R,
    peek: Option<u8>,
}

impl<'de, R: Read> Deserializer<'de, R> {
    pub fn new(reader: &'de mut R) -> Self {
        Self { reader, peek: None }
    }

    fn read_byte(&mut self) -> Result<u8, Error> {
        let mut buf = [0];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub(self) fn get_byte(&mut self) -> Result<u8, Error> {
        if let Some(peek) = self.peek.take() {
            return Ok(peek);
        }
        self.read_byte()
    }

    pub(self) fn peek_byte(&mut self) -> Result<u8, Error> {
        if let Some(peek) = self.peek {
            return Ok(peek);
        }
        let read = self.read_byte()?;
        self.peek = Some(read);
        Ok(read)
    }

    fn deserialize_bf16<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != BF16 {
            return Err(Error::WrongType {
                expected: header_name(BF16),
                found: header_name(self.get_byte()?),
            });
        }

        visitor.visit_f32(self.get_bf16_value()?)
    }

    fn deserialize_f16<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != F16 {
            return Err(Error::WrongType {
                expected: header_name(F16),
                found: header_name(self.get_byte()?),
            });
        }

        visitor.visit_f32(self.get_f16_value()?)
    }

    fn deserialize_string_object<V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != STRING_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(STRING_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::String))
    }

    fn deserialize_i8_object<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != I8_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(I8_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::I8))
    }

    fn deserialize_i16_object<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != I16_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(I16_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::I16))
    }

    fn deserialize_i32_object<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != I32_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(I32_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::I32))
    }

    fn deserialize_i64_object<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != I64_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(I64_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::I64))
    }

    fn deserialize_i128_object<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != I128_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(I128_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::I128))
    }

    fn deserialize_u8_object<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != U8_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(U8_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::U8))
    }

    fn deserialize_u16_object<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != U16_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(U16_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::U16))
    }

    fn deserialize_u32_object<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != U32_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(U32_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::U32))
    }

    fn deserialize_u64_object<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != U64_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(U64_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::U64))
    }

    fn deserialize_u128_object<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != U128_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(U128_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::U128))
    }

    fn deserialize_bf16_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != BF16_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(BF16_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::BF16))
    }

    fn deserialize_f16_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != F16_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(F16_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::F16))
    }

    fn deserialize_f32_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != F32_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(F32_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::F32))
    }

    fn deserialize_f64_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != F64_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(F64_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::F64))
    }

    fn deserialize_i8_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != I8_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(I8_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::I8))
    }

    fn deserialize_i16_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != I16_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(I16_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::I16))
    }

    fn deserialize_i32_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != I32_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(I32_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::I32))
    }

    fn deserialize_i64_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != I64_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(I64_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::I64))
    }

    fn deserialize_i128_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != I128_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(I128_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::I128))
    }

    fn deserialize_u8_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != U8_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(U8_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::U8))
    }

    fn deserialize_u16_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != U16_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(U16_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::U16))
    }

    fn deserialize_u32_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != U32_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(U32_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::U32))
    }

    fn deserialize_u64_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != U64_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(U64_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::U64))
    }

    fn deserialize_u128_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != U128_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(U128_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::U128))
    }

    fn deserialize_bool_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != BOOL_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(BOOL_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::Boolean))
    }

    fn deserialize_string_array<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != STRING_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(STRING_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::String))
    }

    fn get_size(&mut self) -> Result<usize, Error> {
        let mut first = self.get_byte()?;
        let n_bytes = 2_usize.pow((first & 0b11) as u32);

        first >>= 2;

        if n_bytes == 1 {
            return Ok(first as usize);
        }

        let mut rest = vec![0; n_bytes - 1];
        self.reader.read_exact(&mut rest)?;
        #[cfg(target_pointer_width = "64")]
        let mut bytes = [0; 8];
        #[cfg(target_pointer_width = "32")]
        let mut bytes = [0; 4];
        #[cfg(target_pointer_width = "16")]
        let mut bytes = [0; 2];

        if rest.len() >= bytes.len() {
            return Err(Error::TooLong);
        }

        bytes[0] = first;

        for (i, byte) in rest.into_iter().enumerate() {
            bytes[i + 1] = byte;
        }

        Ok(usize::from_le_bytes(bytes))
    }

    pub(self) fn get_string_value(&mut self) -> Result<String, Error> {
        let size = self.get_size()?;
        let mut bytes = vec![0; size];
        self.reader.read_exact(&mut bytes)?;
        Ok(String::from_utf8(bytes)?)
    }

    pub(self) fn get_u8_value(&mut self) -> Result<u8, Error> {
        self.get_byte()
    }

    pub(self) fn get_u16_value(&mut self) -> Result<u16, Error> {
        let mut bytes = [0; 2];
        self.reader.read_exact(&mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }

    pub(self) fn get_u32_value(&mut self) -> Result<u32, Error> {
        let mut bytes = [0; 4];
        self.reader.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub(self) fn get_u64_value(&mut self) -> Result<u64, Error> {
        let mut bytes = [0; 8];
        self.reader.read_exact(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }

    pub(self) fn get_u128_value(&mut self) -> Result<u128, Error> {
        let mut bytes = [0; 16];
        self.reader.read_exact(&mut bytes)?;
        Ok(u128::from_le_bytes(bytes))
    }

    pub(self) fn get_i8_value(&mut self) -> Result<i8, Error> {
        Ok(i8::from_le_bytes([self.get_byte()?]))
    }

    pub(self) fn get_i16_value(&mut self) -> Result<i16, Error> {
        let mut bytes = [0; 2];
        self.reader.read_exact(&mut bytes)?;
        Ok(i16::from_le_bytes(bytes))
    }

    pub(self) fn get_i32_value(&mut self) -> Result<i32, Error> {
        let mut bytes = [0; 4];
        self.reader.read_exact(&mut bytes)?;
        Ok(i32::from_le_bytes(bytes))
    }

    pub(self) fn get_i64_value(&mut self) -> Result<i64, Error> {
        let mut bytes = [0; 8];
        self.reader.read_exact(&mut bytes)?;
        Ok(i64::from_le_bytes(bytes))
    }

    pub(self) fn get_i128_value(&mut self) -> Result<i128, Error> {
        let mut bytes = [0; 16];
        self.reader.read_exact(&mut bytes)?;
        Ok(i128::from_le_bytes(bytes))
    }

    pub(self) fn get_bf16_value(&mut self) -> Result<f32, Error> {
        #[cfg(feature = "half")]
        {
            let mut bytes = [0; 2];
            self.reader.read_exact(&mut bytes)?;
            Ok(half::bf16::from_le_bytes(bytes).to_f32())
        }
        #[cfg(not(feature = "half"))]
        {
            Err(Error::UnsupportedDataType(SpecialType::BrainFloat))
        }
    }

    pub(self) fn get_f16_value(&mut self) -> Result<f32, Error> {
        #[cfg(feature = "half")]
        {
            let mut bytes = [0; 2];
            self.reader.read_exact(&mut bytes)?;
            Ok(half::f16::from_le_bytes(bytes).to_f32())
        }
        #[cfg(not(feature = "half"))]
        {
            Err(Error::UnsupportedDataType(SpecialType::HalfFloat))
        }
    }

    pub(self) fn get_f32_value(&mut self) -> Result<f32, Error> {
        let mut bytes = [0; 4];
        self.reader.read_exact(&mut bytes)?;
        Ok(f32::from_le_bytes(bytes))
    }

    pub(self) fn get_f64_value(&mut self) -> Result<f64, Error> {
        let mut bytes = [0; 8];
        self.reader.read_exact(&mut bytes)?;
        Ok(f64::from_le_bytes(bytes))
    }

    pub(self) fn deserialize_variant<V: Visitor<'de>>(
        &mut self,
        visitor: V,
        range: Option<usize>,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != TAG {
            return Err(Error::WrongType {
                expected: header_name(TAG),
                found: header_name(self.get_byte()?),
            });
        }
        let index = self.get_size()? as u32;
        if range.is_some_and(|r| index as usize >= r) {
            return Err(Error::VariantOutOfRange);
        }
        visitor.visit_enum(EnumDeserializer {
            deserializer: self,
            index,
        })
    }
}

impl<'de, R: Read> serde::Deserializer<'de> for &mut Deserializer<'de, R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.peek_byte()? {
            NULL => self.deserialize_unit(visitor),
            TRUE | FALSE => self.deserialize_bool(visitor),

            BF16 => self.deserialize_bf16(visitor),
            F16 => self.deserialize_f16(visitor),
            F32 => self.deserialize_f32(visitor),
            F64 => self.deserialize_f64(visitor),
            F128 => Err(Error::UnsupportedDataType(SpecialType::F128)),

            I8 => self.deserialize_i8(visitor),
            I16 => self.deserialize_i16(visitor),
            I32 => self.deserialize_i32(visitor),
            I64 => self.deserialize_i64(visitor),
            I128 => self.deserialize_i128(visitor),

            U8 => self.deserialize_u8(visitor),
            U16 => self.deserialize_u16(visitor),
            U32 => self.deserialize_u32(visitor),
            U64 => self.deserialize_u64(visitor),
            U128 => self.deserialize_u128(visitor),

            STRING => self.deserialize_string(visitor),

            STRING_OBJECT => self.deserialize_string_object(visitor),

            I8_OBJECT => self.deserialize_i8_object(visitor),
            I16_OBJECT => self.deserialize_i16_object(visitor),
            I32_OBJECT => self.deserialize_i32_object(visitor),
            I64_OBJECT => self.deserialize_i64_object(visitor),
            I128_OBJECT => self.deserialize_i128_object(visitor),

            U8_OBJECT => self.deserialize_u8_object(visitor),
            U16_OBJECT => self.deserialize_u16_object(visitor),
            U32_OBJECT => self.deserialize_u32_object(visitor),
            U64_OBJECT => self.deserialize_u64_object(visitor),
            U128_OBJECT => self.deserialize_u128_object(visitor),

            BF16_ARRAY => self.deserialize_bf16_array(visitor),
            F16_ARRAY => self.deserialize_f16_array(visitor),
            F32_ARRAY => self.deserialize_f32_array(visitor),
            F64_ARRAY => self.deserialize_f64_array(visitor),
            F128_ARRAY => Err(Error::UnsupportedDataType(SpecialType::F128)),

            I8_ARRAY => self.deserialize_i8_array(visitor),
            I16_ARRAY => self.deserialize_i16_array(visitor),
            I32_ARRAY => self.deserialize_i32_array(visitor),
            I64_ARRAY => self.deserialize_i64_array(visitor),
            I128_ARRAY => self.deserialize_i128_array(visitor),

            U8_ARRAY => self.deserialize_u8_array(visitor),
            U16_ARRAY => self.deserialize_u16_array(visitor),
            U32_ARRAY => self.deserialize_u32_array(visitor),
            U64_ARRAY => self.deserialize_u64_array(visitor),
            U128_ARRAY => self.deserialize_u128_array(visitor),

            BOOL_ARRAY => self.deserialize_bool_array(visitor),
            STRING_ARRAY => self.deserialize_string_array(visitor),
            GENERIC_ARRAY => self.deserialize_seq(visitor),

            TAG => self.deserialize_variant(visitor, None),

            header => Err(Error::InvalidHeader(header)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.get_byte()? {
            TRUE => visitor.visit_bool(true),
            FALSE => visitor.visit_bool(false),
            header => Err(Error::WrongType {
                expected: header_name(TRUE),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            I8 => visitor.visit_i8(self.get_i8_value()?),
            header => Err(Error::WrongType {
                expected: header_name(I8),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            I16 => visitor.visit_i16(self.get_i16_value()?),
            header => Err(Error::WrongType {
                expected: header_name(I16),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            I32 => visitor.visit_i32(self.get_i32_value()?),
            header => Err(Error::WrongType {
                expected: header_name(I32),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            I64 => visitor.visit_i64(self.get_i64_value()?),
            header => Err(Error::WrongType {
                expected: header_name(I64),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            I128 => visitor.visit_i128(self.get_i128_value()?),
            header => Err(Error::WrongType {
                expected: header_name(I128),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            U8 => visitor.visit_u8(self.get_u8_value()?),
            header => Err(Error::WrongType {
                expected: header_name(U8),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            U16 => visitor.visit_u16(self.get_u16_value()?),
            header => Err(Error::WrongType {
                expected: header_name(U16),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            U32 => visitor.visit_u32(self.get_u32_value()?),
            header => Err(Error::WrongType {
                expected: header_name(U32),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            U64 => visitor.visit_u64(self.get_u64_value()?),
            header => Err(Error::WrongType {
                expected: header_name(U64),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            U128 => visitor.visit_u128(self.get_u128_value()?),
            header => Err(Error::WrongType {
                expected: header_name(U128),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            F32 => visitor.visit_f32(self.get_f32_value()?),
            header => Err(Error::WrongType {
                expected: header_name(F32),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            F64 => visitor.visit_f64(self.get_f64_value()?),
            header => Err(Error::WrongType {
                expected: header_name(F64),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            STRING => visitor.visit_string(self.get_string_value()?),
            header => Err(Error::WrongType {
                expected: header_name(STRING),
                found: header_name(header),
            }),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let kind = match self.get_byte()? {
            STRING_ARRAY => ArrayKind::String,
            BOOL_ARRAY => ArrayKind::Boolean,
            I8_ARRAY => ArrayKind::I8,
            I16_ARRAY => ArrayKind::I16,
            I32_ARRAY => ArrayKind::I32,
            I64_ARRAY => ArrayKind::I64,
            I128_ARRAY => ArrayKind::I128,
            U8_ARRAY => ArrayKind::U8,
            U16_ARRAY => ArrayKind::U16,
            U32_ARRAY => ArrayKind::U32,
            U64_ARRAY => ArrayKind::U64,
            U128_ARRAY => ArrayKind::U128,
            F32_ARRAY => ArrayKind::F32,
            F64_ARRAY => ArrayKind::F64,
            GENERIC_ARRAY => ArrayKind::Generic,
            header => {
                return Err(Error::WrongType {
                    expected: "array",
                    found: header_name(header),
                });
            }
        };
        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, kind))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let kind = match self.get_byte()? {
            STRING_OBJECT => ObjectKind::String,
            I8_OBJECT => ObjectKind::I8,
            I16_OBJECT => ObjectKind::I16,
            I32_OBJECT => ObjectKind::I32,
            I64_OBJECT => ObjectKind::I64,
            I128_OBJECT => ObjectKind::I128,
            U8_OBJECT => ObjectKind::U8,
            U16_OBJECT => ObjectKind::U16,
            U32_OBJECT => ObjectKind::U32,
            U64_OBJECT => ObjectKind::U64,
            U128_OBJECT => ObjectKind::U128,
            header => {
                return Err(Error::WrongType {
                    expected: "object",
                    found: header_name(header),
                });
            }
        };

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, kind))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_variant(visitor, Some(variants.len()))
    }

    forward_to_deserialize_any! {
        char str bytes byte_buf option unit unit_struct newtype_struct tuple tuple_struct struct
        identifier ignored_any
    }
}

struct SeqDeserializer<'a, 'de, R: Read> {
    pub deserializer: &'a mut Deserializer<'de, R>,
    pub len: usize,
    pub index: usize,
    pub kind: ArrayKind,
}

impl<'a, 'de, R: Read> SeqDeserializer<'a, 'de, R> {
    fn new(deserializer: &'a mut Deserializer<'de, R>, len: usize, kind: ArrayKind) -> Self {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
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
            found => Err(Error::MismatchedArrayType {
                expected: ArrayKind::String,
                found,
            }),
        }
    }

    forward_to_deserialize_any! {
        char str bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
    }
}

struct MapDeserializer<'a, 'de, R: Read> {
    pub deserializer: &'a mut Deserializer<'de, R>,
    pub len: usize,
    pub index: usize,
    pub kind: ObjectKind,
}

impl<'a, 'de, R: Read> MapDeserializer<'a, 'de, R> {
    fn new(deserializer: &'a mut Deserializer<'de, R>, len: usize, kind: ObjectKind) -> Self {
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

        let kind = self.kind;
        seed.deserialize(KeyDeserializer { map: self, kind })
            .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}

struct KeyDeserializer<'a, 'b, 'de, R: Read> {
    pub map: &'a mut MapDeserializer<'b, 'de, R>,
    pub kind: ObjectKind,
}

impl<'a, 'b, 'de, R: Read> serde::Deserializer<'de> for KeyDeserializer<'a, 'b, 'de, R> {
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
            ObjectKind::I8 => visitor.visit_i8(self.map.deserializer.get_i8_value()?),
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
            ObjectKind::I16 => visitor.visit_i16(self.map.deserializer.get_i16_value()?),
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
            ObjectKind::I32 => visitor.visit_i32(self.map.deserializer.get_i32_value()?),
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
            ObjectKind::I64 => visitor.visit_i64(self.map.deserializer.get_i64_value()?),
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
            ObjectKind::I128 => visitor.visit_i128(self.map.deserializer.get_i128_value()?),
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
            ObjectKind::U8 => visitor.visit_u8(self.map.deserializer.get_u8_value()?),
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
            ObjectKind::U16 => visitor.visit_u16(self.map.deserializer.get_u16_value()?),
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
            ObjectKind::U32 => visitor.visit_u32(self.map.deserializer.get_u32_value()?),
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
            ObjectKind::U64 => visitor.visit_u64(self.map.deserializer.get_u64_value()?),
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
            ObjectKind::U128 => visitor.visit_u128(self.map.deserializer.get_u128_value()?),
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
            ObjectKind::String => visitor.visit_string(self.map.deserializer.get_string_value()?),
            found => Err(Error::MismatchedKeyType {
                expected: ObjectKind::String,
                found,
            }),
        }
    }

    forward_to_deserialize_any! {
        bool f32 f64 char str bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map
        struct enum identifier ignored_any
    }
}

struct EnumDeserializer<'a, 'de, R: Read> {
    pub deserializer: &'a mut Deserializer<'de, R>,
    pub index: u32,
}

impl<'a, 'de, R: Read> EnumAccess<'de> for EnumDeserializer<'a, 'de, R> {
    type Error = Error;
    type Variant = VariantAccessor<'a, 'de, R>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        Ok((
            seed.deserialize(VariantDeserializer { index: self.index })?,
            VariantAccessor {
                deserializer: self.deserializer,
            },
        ))
    }
}

struct VariantDeserializer {
    pub index: u32,
}

impl<'de> serde::Deserializer<'de> for VariantDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Variant)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.index as u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.index as u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.index)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.index as u64)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(self.index as u128)
    }

    forward_to_deserialize_any! {
        bool byte_buf bytes char enum f32 f64 i8 i16 i32 i64 identifier ignored_any map newtype_struct
        option seq str string struct tuple tuple_struct unit unit_struct
    }
}

struct VariantAccessor<'a, 'de, R: Read> {
    pub deserializer: &'a mut Deserializer<'de, R>,
}

impl<'a, 'de, R: Read> VariantAccess<'de> for VariantAccessor<'a, 'de, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.deserializer.get_byte()?;
        Ok(())
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

pub fn from_reader<'de, R: Read, T: serde::Deserialize<'de>>(
    reader: &'de mut R,
) -> Result<T, Error> {
    let mut deserializer = Deserializer::new(reader);
    T::deserialize(&mut deserializer)
}
