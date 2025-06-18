mod enums;
mod map;
mod seq;

use crate::{Error, error::SpecialType, headers::*};
use enums::EnumDeserializer;
use map::MapDeserializer;
use seq::SeqDeserializer;
use serde::{de::Visitor, forward_to_deserialize_any};
use std::io::Read;

pub struct Deserializer<R: Read> {
    reader: R,
    peek: Option<u8>,
}

impl<R: Read> Deserializer<R> {
    pub fn new(reader: R) -> Self {
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

    fn get_u8_array(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.get_size()?;
        let mut bytes = vec![0; size];
        self.reader.read_exact(&mut bytes)?;
        Ok(bytes)
    }

    fn deserialize_bf16<'de, V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != BF16 {
            return Err(Error::WrongType {
                expected: header_name(BF16),
                found: header_name(self.get_byte()?),
            });
        }

        visitor.visit_f32(self.get_bf16_value()?)
    }

    fn deserialize_f16<'de, V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value, Error> {
        if self.get_byte()? != F16 {
            return Err(Error::WrongType {
                expected: header_name(F16),
                found: header_name(self.get_byte()?),
            });
        }

        visitor.visit_f32(self.get_f16_value()?)
    }

    fn deserialize_string_object<'de, V: Visitor<'de>>(
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

    fn deserialize_i8_object<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != I8_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(I8_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::I8))
    }

    fn deserialize_i16_object<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != I16_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(I16_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::I16))
    }

    fn deserialize_i32_object<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != I32_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(I32_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::I32))
    }

    fn deserialize_i64_object<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != I64_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(I64_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::I64))
    }

    fn deserialize_i128_object<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != I128_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(I128_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::I128))
    }

    fn deserialize_u8_object<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != U8_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(U8_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::U8))
    }

    fn deserialize_u16_object<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != U16_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(U16_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::U16))
    }

    fn deserialize_u32_object<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != U32_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(U32_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::U32))
    }

    fn deserialize_u64_object<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != U64_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(U64_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::U64))
    }

    fn deserialize_u128_object<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != U128_OBJECT {
            return Err(Error::WrongType {
                expected: header_name(U128_OBJECT),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_map(MapDeserializer::new(self, size, ObjectKind::U128))
    }

    fn deserialize_bf16_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != BF16_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(BF16_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::BF16))
    }

    fn deserialize_f16_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != F16_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(F16_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::F16))
    }

    fn deserialize_f32_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != F32_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(F32_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::F32))
    }

    fn deserialize_f64_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != F64_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(F64_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::F64))
    }

    fn deserialize_i8_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != I8_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(I8_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::I8))
    }

    fn deserialize_i16_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != I16_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(I16_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::I16))
    }

    fn deserialize_i32_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != I32_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(I32_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::I32))
    }

    fn deserialize_i64_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != I64_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(I64_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::I64))
    }

    fn deserialize_i128_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != I128_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(I128_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::I128))
    }

    fn deserialize_u8_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != U8_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(U8_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::U8))
    }

    fn deserialize_u16_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != U16_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(U16_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::U16))
    }

    fn deserialize_u32_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != U32_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(U32_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::U32))
    }

    fn deserialize_u64_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != U64_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(U64_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::U64))
    }

    fn deserialize_u128_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != U128_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(U128_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::U128))
    }

    fn deserialize_bool_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
        if self.get_byte()? != BOOL_ARRAY {
            return Err(Error::WrongType {
                expected: header_name(BOOL_ARRAY),
                found: header_name(self.get_byte()?),
            });
        }

        let size = self.get_size()?;
        visitor.visit_seq(SeqDeserializer::new(self, size, ArrayKind::Boolean))
    }

    fn deserialize_string_array<'de, V: Visitor<'de>>(
        &mut self,
        visitor: V,
    ) -> Result<V::Value, Error> {
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
}

impl<'de, R: Read> serde::Deserializer<'de> for &mut Deserializer<R> {
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

            DELIMITER => {
                self.get_byte()?;
                visitor.visit_unit()
            }
            TAG => self.deserialize_enum("", &[], visitor),
            MATRIX => todo!(),
            COMPLEX => todo!(),

            RESERVED => Err(Error::Reserved),
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
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(EnumDeserializer { deserializer: self })
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(&self.get_u8_array()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_byte_buf(self.get_u8_array()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            STRING => {}
            header => {
                return Err(Error::WrongType {
                    expected: header_name(STRING),
                    found: header_name(header),
                });
            }
        }

        let string = self.get_string_value()?;
        if string.len() != 1 {
            return Err(Error::WrongType {
                expected: "character",
                found: header_name(STRING),
            });
        }
        let Some(char) = string.chars().next() else {
            return Err(Error::NoChar);
        };

        visitor.visit_char(char)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            STRING => {}
            header => {
                return Err(Error::WrongType {
                    expected: "string",
                    found: header_name(header),
                });
            }
        }

        let string = self.get_string_value()?;
        visitor.visit_str(&string)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.peek_byte()? {
            NULL => {
                self.get_byte()?;
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.get_byte()? {
            NULL => visitor.visit_unit(),
            header => Err(Error::WrongType {
                expected: header_name(NULL),
                found: header_name(header),
            }),
        }
    }

    forward_to_deserialize_any! {
        unit_struct tuple tuple_struct struct identifier ignored_any
    }
}

/// Deserializes the data from the `reader` as `T`.
pub fn from_reader<T: serde::de::DeserializeOwned>(reader: impl Read) -> Result<T, Error> {
    let mut deserializer = Deserializer::new(reader);
    T::deserialize(&mut deserializer)
}

/// Deserializes the data from the `bytes` as `T`.
pub fn from_bytes<'de, T: serde::de::Deserialize<'de>>(bytes: &'de [u8]) -> Result<T, Error> {
    let mut deserializer = Deserializer::new(bytes);
    T::deserialize(&mut deserializer)
}
