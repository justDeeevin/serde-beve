use super::Deserializer;
use crate::{ArrayKind, Error, ObjectKind, headers::*};
use serde::{de::MapAccess, forward_to_deserialize_any};
use std::io::Read;

pub struct MatrixDeserializer<'a, R: Read> {
    deserializer: &'a mut Deserializer<R>,
    layout: String,
    key: bool,
    index: usize,
    extent_type: Option<ArrayKind>,
    value_type: Option<ArrayKind>,
}

impl<'a, R: Read> MatrixDeserializer<'a, R> {
    pub fn new(deserializer: &'a mut Deserializer<R>, layout: String) -> Self {
        Self {
            deserializer,
            layout,
            key: false,
            index: 0,
            extent_type: None,
            value_type: None,
        }
    }
}

impl<'a, 'de, R: Read> MapAccess<'de> for MatrixDeserializer<'a, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.index {
            0 => {}
            1 => {
                self.extent_type = Some(match self.deserializer.get_byte()? {
                    U8_ARRAY => ArrayKind::U8,
                    U16_ARRAY => ArrayKind::U16,
                    U32_ARRAY => ArrayKind::U32,
                    U64_ARRAY => ArrayKind::U64,
                    U128_ARRAY => ArrayKind::U128,
                    _ => {
                        return Err(Error::InvalidMatrixType);
                    }
                })
            }
            2 => {
                self.value_type = Some(match self.deserializer.get_byte()? {
                    U8_ARRAY => ArrayKind::U8,
                    U16_ARRAY => ArrayKind::U16,
                    U32_ARRAY => ArrayKind::U32,
                    U64_ARRAY => ArrayKind::U64,
                    U128_ARRAY => ArrayKind::U128,
                    _ => {
                        return Err(Error::InvalidMatrixType);
                    }
                })
            }
            3 => {
                return Ok(None);
            }
            _ => unreachable!(),
        }
        self.key = true;
        seed.deserialize(self).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        self.key = false;
        let out = seed.deserialize(&mut *self);
        self.index += 1;
        out
    }
}

impl<'a, 'de, R: Read> serde::Deserializer<'de> for &mut MatrixDeserializer<'a, R> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::InvalidMatrixType)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => Err(Error::InvalidMatrixType),
            1 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::I8,
                    })
                } else {
                    Err(Error::InvalidMatrixType)
                }
            }
            2 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::I8,
                    })
                } else {
                    match self.value_type.unwrap() {
                        ArrayKind::I8 => self.deserializer.deserialize_i8_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::I8,
                            found,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => Err(Error::InvalidMatrixType),
            1 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::I16,
                    })
                } else {
                    Err(Error::InvalidMatrixType)
                }
            }
            2 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::I16,
                    })
                } else {
                    match self.value_type.unwrap() {
                        ArrayKind::I16 => self.deserializer.deserialize_i16_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::I16,
                            found,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => Err(Error::InvalidMatrixType),
            1 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::I32,
                    })
                } else {
                    Err(Error::InvalidMatrixType)
                }
            }
            2 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::I32,
                    })
                } else {
                    match self.value_type.unwrap() {
                        ArrayKind::I32 => self.deserializer.deserialize_i32_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::I32,
                            found,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => Err(Error::InvalidMatrixType),
            1 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::I64,
                    })
                } else {
                    Err(Error::InvalidMatrixType)
                }
            }
            2 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::I64,
                    })
                } else {
                    match self.value_type.unwrap() {
                        ArrayKind::I64 => self.deserializer.deserialize_i64_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::I64,
                            found,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => Err(Error::InvalidMatrixType),
            1 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::I128,
                    })
                } else {
                    Err(Error::InvalidMatrixType)
                }
            }
            2 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::I128,
                    })
                } else {
                    match self.value_type.unwrap() {
                        ArrayKind::I128 => self.deserializer.deserialize_i128_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::I128,
                            found,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => Err(Error::InvalidMatrixType),
            1 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::U8,
                    })
                } else {
                    match self.extent_type.unwrap() {
                        ArrayKind::U8 => self.deserializer.deserialize_u8_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::U8,
                            found,
                        }),
                    }
                }
            }
            2 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::U8,
                    })
                } else {
                    match self.value_type.unwrap() {
                        ArrayKind::U8 => self.deserializer.deserialize_u8_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::U8,
                            found,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => Err(Error::InvalidMatrixType),
            1 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::U16,
                    })
                } else {
                    match self.extent_type.unwrap() {
                        ArrayKind::U16 => self.deserializer.deserialize_u16_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::U16,
                            found,
                        }),
                    }
                }
            }
            2 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::U16,
                    })
                } else {
                    match self.value_type.unwrap() {
                        ArrayKind::U16 => self.deserializer.deserialize_u16_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::U16,
                            found,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => Err(Error::InvalidMatrixType),
            1 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::U32,
                    })
                } else {
                    match self.extent_type.unwrap() {
                        ArrayKind::U32 => self.deserializer.deserialize_u32_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::U32,
                            found,
                        }),
                    }
                }
            }
            2 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::U32,
                    })
                } else {
                    match self.value_type.unwrap() {
                        ArrayKind::U32 => self.deserializer.deserialize_u32_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::U32,
                            found,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => Err(Error::InvalidMatrixType),
            1 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::U64,
                    })
                } else {
                    match self.extent_type.unwrap() {
                        ArrayKind::U64 => self.deserializer.deserialize_u64_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::U64,
                            found,
                        }),
                    }
                }
            }
            2 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::U64,
                    })
                } else {
                    match self.value_type.unwrap() {
                        ArrayKind::U64 => self.deserializer.deserialize_u64_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::U64,
                            found,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => Err(Error::InvalidMatrixType),
            1 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::U128,
                    })
                } else {
                    match self.extent_type.unwrap() {
                        ArrayKind::U128 => self.deserializer.deserialize_u128_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::U128,
                            found,
                        }),
                    }
                }
            }
            2 => {
                if self.key {
                    Err(Error::MismatchedKeyType {
                        expected: ObjectKind::String,
                        found: ObjectKind::U128,
                    })
                } else {
                    match self.value_type.unwrap() {
                        ArrayKind::U128 => self.deserializer.deserialize_u128_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::U128,
                            found,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => Err(Error::InvalidMatrixType),
            1 => {
                if self.key {
                    Err(Error::InvalidKey)
                } else {
                    Err(Error::InvalidMatrixType)
                }
            }
            2 => {
                if self.key {
                    Err(Error::InvalidKey)
                } else {
                    match self.value_type.unwrap() {
                        ArrayKind::F32 => self.deserializer.deserialize_f32_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::F32,
                            found,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => Err(Error::InvalidMatrixType),
            1 => {
                if self.key {
                    Err(Error::InvalidKey)
                } else {
                    Err(Error::InvalidMatrixType)
                }
            }
            2 => {
                if self.key {
                    Err(Error::InvalidKey)
                } else {
                    match self.value_type.unwrap() {
                        ArrayKind::F64 => self.deserializer.deserialize_f64_array(visitor),
                        found => Err(Error::MismatchedElementType {
                            expected: ArrayKind::F64,
                            found,
                        }),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.index {
            0 => {
                if self.key {
                    visitor.visit_str("layout")
                } else {
                    visitor.visit_str(&self.layout)
                }
            }
            1 => {
                if self.key {
                    visitor.visit_str("extents")
                } else {
                    Err(Error::MismatchedElementType {
                        expected: self.extent_type.unwrap(),
                        found: ArrayKind::String,
                    })
                }
            }
            2 => {
                if self.key {
                    visitor.visit_str("value")
                } else {
                    Err(Error::MismatchedElementType {
                        expected: self.value_type.unwrap(),
                        found: ArrayKind::String,
                    })
                }
            }
            _ => unreachable!(),
        }
    }

    forward_to_deserialize_any! {
        bool char bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
    }
}
