use super::{MapSerializer, Serializer};
use crate::{Value, error::Error, headers::ArrayKind};
use serde::{
    Serialize,
    ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant},
};
use std::io::Write;

pub struct SeqSerializer<'a, 'ser, W: Write> {
    serializer: &'a mut Serializer<'ser, W>,
    pub kind: Option<ArrayKind>,
    elements: Vec<Value>,
}

impl<'a, 'ser, W: Write> SeqSerializer<'a, 'ser, W> {
    pub fn new(serializer: &'a mut Serializer<'ser, W>) -> Self {
        Self {
            serializer,
            kind: None,
            elements: Vec::new(),
        }
    }

    pub(self) fn update_type(&mut self, new: ArrayKind) {
        match self.kind {
            None => self.kind = Some(new),
            Some(ArrayKind::Generic) => {}
            Some(kind) => {
                if kind != new {
                    self.kind = Some(ArrayKind::Generic);
                }
            }
        }
    }

    pub(self) fn ensure_generic(&mut self) {
        if self.kind != Some(ArrayKind::Generic) {
            self.kind = Some(ArrayKind::Generic);
        }
    }
}

impl<'a, 'ser, W: Write> SerializeSeq for SeqSerializer<'a, 'ser, W> {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(&mut *self)?;
        self.elements.push(v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let out = match self.kind {
            Some(ArrayKind::BF16) | Some(ArrayKind::F16) => {
                unreachable!()
            }
            None | Some(ArrayKind::Generic) => Value::GenericArray(self.elements),
            Some(ArrayKind::I8) => Value::I8Array(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::I8(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            Some(ArrayKind::I16) => Value::I16Array(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::I16(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            Some(ArrayKind::I32) => Value::I32Array(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::I32(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            Some(ArrayKind::I64) => Value::I64Array(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::I64(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            Some(ArrayKind::I128) => Value::I128Array(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::I128(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),

            Some(ArrayKind::U8) => Value::U8Array(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::U8(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            Some(ArrayKind::U16) => Value::U16Array(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::U16(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            Some(ArrayKind::U32) => Value::U32Array(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::U32(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            Some(ArrayKind::U64) => Value::U64Array(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::U64(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            Some(ArrayKind::U128) => Value::U128Array(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::U128(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),

            Some(ArrayKind::F32) => Value::F32Array(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::F32(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            Some(ArrayKind::F64) => Value::F64Array(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::F64(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),

            Some(ArrayKind::String) => Value::StringArray(
                self.elements
                    .into_iter()
                    .map(|v| match v {
                        Value::String(v) => v,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),

            Some(ArrayKind::Boolean) => {
                let values = self.elements.into_iter().map(|v| match v {
                    Value::True => true,
                    Value::False => false,
                    _ => unreachable!(),
                });
                let len = values.len();
                let mut bytes = Vec::with_capacity(len);
                let mut byte = 0;
                for (i, v) in values.enumerate() {
                    byte |= v as u8;
                    byte <<= 1;

                    if i % 8 == 7 {
                        bytes.push(byte);
                        byte = 0;
                    }
                }
                Value::BoolArray(len, bytes)
            }
            Some(ArrayKind::Complex) => unreachable!(),
        };

        self.serializer.serialize_value(&out)?;
        Ok(out)
    }
}

impl<'a, 'ser, W: Write> SerializeTuple for SeqSerializer<'a, 'ser, W> {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<'a, 'ser, W: Write> SerializeTupleStruct for SeqSerializer<'a, 'ser, W> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<'a, 'ser, W: Write> SerializeTupleVariant for SeqSerializer<'a, 'ser, W> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<'a, 'b, 'ser, W: Write> serde::Serializer for &'b mut SeqSerializer<'a, 'ser, W> {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SeqSerializer<'b, 'ser, W>;
    type SerializeTuple = SeqSerializer<'b, 'ser, W>;
    type SerializeTupleStruct = SeqSerializer<'b, 'ser, W>;
    type SerializeTupleVariant = SeqSerializer<'b, 'ser, W>;
    type SerializeMap = MapSerializer<'b, 'ser, W>;
    type SerializeStruct = MapSerializer<'b, 'ser, W>;
    type SerializeStructVariant = MapSerializer<'b, 'ser, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::Boolean);
        Ok(if v { Value::True } else { Value::False })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::I8);
        Ok(Value::I8(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::I16);
        Ok(Value::I16(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::I32);
        Ok(Value::I32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::I64);
        Ok(Value::I64(v))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::I128);
        Ok(Value::I128(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::U8);
        Ok(Value::U8(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::U16);
        Ok(Value::U16(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::U32);
        Ok(Value::U32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::U64);
        Ok(Value::U64(v))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::U128);
        Ok(Value::U128(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::F32);
        Ok(Value::F32(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::F64);
        Ok(Value::F64(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.update_type(ArrayKind::String);
        Ok(Value::String(v.bytes().collect()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.ensure_generic();
        self.serializer.serialize_bytes(v)
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
        self.ensure_generic();
        self.serializer.serialize_unit()
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
        self.ensure_generic();
        self.serializer
            .serialize_unit_variant(name, variant_index, variant)
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
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.ensure_generic();
        self.serializer
            .serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.ensure_generic();
        self.serializer.serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.ensure_generic();
        self.serializer.serialize_tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.ensure_generic();
        self.serializer.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.ensure_generic();
        self.serializer
            .serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.ensure_generic();
        self.serializer.serialize_map(len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.ensure_generic();
        self.serializer.serialize_struct(name, len)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.ensure_generic();
        self.serializer
            .serialize_struct_variant(name, variant_index, variant, len)
    }
}
