mod map;
mod seq;

pub use map::MapSerializer;
pub use seq::SeqSerializer;

use crate::{Value, error::Error, headers::*};
use std::io::Write;

pub struct Serializer<W: Write> {
    pub(self) writer: W,
}

impl<W: Write> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    fn serialize_num_array<T: Copy, const N: usize>(
        &mut self,
        v: &[T],
        f: fn(T) -> [u8; N],
    ) -> Result<(), Error> {
        self.serialize_size(v.len())?;
        let bytes = v.iter().flat_map(|v| f(*v)).collect::<Vec<_>>();
        self.writer.write_all(&bytes)?;
        Ok(())
    }

    fn serialize_size(&mut self, mut size: usize) -> Result<(), Error> {
        if size >= 2_usize.pow(62) {
            return Err(Error::TooLong);
        }

        size <<= 2;

        if size < 2_usize.pow(6) {
            self.writer.write_all(&[size as u8])?;
        } else if size < 2_usize.pow(14) {
            self.writer.write_all(&(size as u16 | 1).to_le_bytes())?;
        } else if size < 2_usize.pow(30) {
            self.writer.write_all(&(size as u32 | 2).to_le_bytes())?;
        } else {
            self.writer.write_all(&(size as u64 | 3).to_le_bytes())?;
        }

        Ok(())
    }

    fn serialize_str_value(&mut self, value: &[u8]) -> Result<(), Error> {
        self.serialize_size(value.len())?;
        self.writer.write_all(value)?;
        Ok(())
    }

    fn serialize_value(&mut self, value: &Value) -> Result<(), Error> {
        self.writer.write_all(&[value.header()])?;
        match value {
            Value::Null | Value::True | Value::False => {}

            Value::I8(v) => self.writer.write_all(&v.to_le_bytes())?,
            Value::I16(v) => self.writer.write_all(&v.to_le_bytes())?,
            Value::I32(v) => self.writer.write_all(&v.to_le_bytes())?,
            Value::I64(v) => self.writer.write_all(&v.to_le_bytes())?,
            Value::I128(v) => self.writer.write_all(&v.to_le_bytes())?,

            Value::U8(v) => self.writer.write_all(&[*v])?,
            Value::U16(v) => self.writer.write_all(&v.to_le_bytes())?,
            Value::U32(v) => self.writer.write_all(&v.to_le_bytes())?,
            Value::U64(v) => self.writer.write_all(&v.to_le_bytes())?,
            Value::U128(v) => self.writer.write_all(&v.to_le_bytes())?,

            Value::F32(v) => self.writer.write_all(&v.to_le_bytes())?,
            Value::F64(v) => self.writer.write_all(&v.to_le_bytes())?,

            Value::String(v) => self.serialize_str_value(v)?,

            Value::StringObject(v) => {
                self.serialize_size(v.len())?;
                for (k, v) in v {
                    self.serialize_str_value(k)?;
                    self.serialize_value(v)?;
                }
            }

            Value::I8Object(v) => {
                self.serialize_size(v.len())?;
                for (k, v) in v {
                    self.serialize_value(&Value::I8(*k))?;
                    self.serialize_value(v)?;
                }
            }
            Value::I16Object(v) => {
                self.serialize_size(v.len())?;
                for (k, v) in v {
                    self.serialize_value(&Value::I16(*k))?;
                    self.serialize_value(v)?;
                }
            }
            Value::I32Object(v) => {
                self.serialize_size(v.len())?;
                for (k, v) in v {
                    self.serialize_value(&Value::I32(*k))?;
                    self.serialize_value(v)?;
                }
            }
            Value::I64Object(v) => {
                self.serialize_size(v.len())?;
                for (k, v) in v {
                    self.serialize_value(&Value::I64(*k))?;
                    self.serialize_value(v)?;
                }
            }
            Value::I128Object(v) => {
                self.serialize_size(v.len())?;
                for (k, v) in v {
                    self.serialize_value(&Value::I128(*k))?;
                    self.serialize_value(v)?;
                }
            }

            Value::U8Object(v) => {
                self.serialize_size(v.len())?;
                for (k, v) in v {
                    self.serialize_value(&Value::U8(*k))?;
                    self.serialize_value(v)?;
                }
            }
            Value::U16Object(v) => {
                self.serialize_size(v.len())?;
                for (k, v) in v {
                    self.serialize_value(&Value::U16(*k))?;
                    self.serialize_value(v)?;
                }
            }
            Value::U32Object(v) => {
                self.serialize_size(v.len())?;
                for (k, v) in v {
                    self.serialize_value(&Value::U32(*k))?;
                    self.serialize_value(v)?;
                }
            }
            Value::U64Object(v) => {
                self.serialize_size(v.len())?;
                for (k, v) in v {
                    self.serialize_value(&Value::U64(*k))?;
                    self.serialize_value(v)?;
                }
            }
            Value::U128Object(v) => {
                self.serialize_size(v.len())?;
                for (k, v) in v {
                    self.serialize_value(&Value::U128(*k))?;
                    self.serialize_value(v)?;
                }
            }

            Value::F32Array(v) => {
                self.serialize_num_array(v, f32::to_le_bytes)?;
            }
            Value::F64Array(v) => {
                self.serialize_num_array(v, f64::to_le_bytes)?;
            }

            Value::I8Array(v) => {
                self.serialize_num_array(v, i8::to_le_bytes)?;
            }
            Value::I16Array(v) => {
                self.serialize_num_array(v, i16::to_le_bytes)?;
            }
            Value::I32Array(v) => {
                self.serialize_num_array(v, i32::to_le_bytes)?;
            }
            Value::I64Array(v) => {
                self.serialize_num_array(v, i64::to_le_bytes)?;
            }
            Value::I128Array(v) => {
                self.serialize_num_array(v, i128::to_le_bytes)?;
            }

            Value::U8Array(v) => {
                self.serialize_num_array(v, u8::to_le_bytes)?;
            }
            Value::U16Array(v) => {
                self.serialize_num_array(v, u16::to_le_bytes)?;
            }
            Value::U32Array(v) => {
                self.serialize_num_array(v, u32::to_le_bytes)?;
            }
            Value::U64Array(v) => {
                self.serialize_num_array(v, u64::to_le_bytes)?;
            }
            Value::U128Array(v) => {
                self.serialize_num_array(v, u128::to_le_bytes)?;
            }

            Value::BoolArray(len, v) => {
                self.serialize_size(*len)?;
                self.writer.write_all(v)?;
            }
            Value::StringArray(v) => {
                self.serialize_size(v.len())?;
                for s in v {
                    self.serialize_str_value(s)?;
                }
            }
            Value::GenericArray(v) => {
                self.serialize_size(v.len())?;
                for v in v {
                    self.serialize_value(v)?;
                }
            }

            Value::Tag(tag, v) => {
                self.serialize_size(*tag)?;
                self.serialize_value(v)?;
            }

            // Never serialized
            #[cfg(feature = "half")]
            Value::BF16(..) => unreachable!(),
            #[cfg(not(feature = "half"))]
            Value::BF16 => unreachable!(),
            #[cfg(feature = "half")]
            Value::BF16Array(..) => unreachable!(),
            #[cfg(not(feature = "half"))]
            Value::BF16Array => unreachable!(),
            #[cfg(feature = "half")]
            Value::F16(..) => unreachable!(),
            #[cfg(not(feature = "half"))]
            Value::F16 => unreachable!(),
            #[cfg(feature = "half")]
            Value::F16Array(..) => unreachable!(),
            #[cfg(not(feature = "half"))]
            Value::F16Array => unreachable!(),
            Value::F128
            | Value::F128Array
            | Value::Reserved
            | Value::Complex
            | Value::Matrix
            | Value::Delimiter => unreachable!(),
        }

        Ok(())
    }
}

impl<'a, W: Write> serde::Serializer for &'a mut Serializer<W> {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, W>;
    type SerializeTuple = SeqSerializer<'a, W>;
    type SerializeTupleStruct = SeqSerializer<'a, W>;
    type SerializeTupleVariant = SeqSerializer<'a, W>;
    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = MapSerializer<'a, W>;
    type SerializeStructVariant = MapSerializer<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let out = if v { Value::True } else { Value::False };
        self.serialize_value(&out)?;
        Ok(out)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        let out = Value::I8(v);
        self.serialize_value(&out)?;
        Ok(out)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        let out = Value::I16(v);
        self.serialize_value(&out)?;
        Ok(out)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let out = Value::I32(v);
        self.serialize_value(&out)?;
        Ok(out)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let out = Value::I64(v);
        self.serialize_value(&out)?;
        Ok(out)
    }
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        let out = Value::I128(v);
        self.serialize_value(&out)?;
        Ok(out)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        let out = Value::U8(v);
        self.serialize_value(&out)?;
        Ok(out)
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        let out = Value::U16(v);
        self.serialize_value(&out)?;
        Ok(out)
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        let out = Value::U32(v);
        self.serialize_value(&out)?;
        Ok(out)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let out = Value::U64(v);
        self.serialize_value(&out)?;
        Ok(out)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        let out = Value::U128(v);
        self.serialize_value(&out)?;
        Ok(out)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let out = Value::F32(v);
        self.serialize_value(&out)?;
        Ok(out)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let out = Value::F64(v);
        self.serialize_value(&out)?;
        Ok(out)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let out = Value::String(v.bytes().collect());
        self.serialize_value(&out)?;
        Ok(out)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // TODO: avoid copying?
        let out = Value::U8Array(v.to_vec());
        self.serialize_value(&out)?;
        Ok(out)
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
        self.serialize_value(&Value::Null)?;
        Ok(Value::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        let out = Value::Tag(variant_index as usize, Box::new(Value::Null));
        self.serialize_value(&out)?;
        Ok(out)
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
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let out = Value::Tag(
            variant_index as usize,
            Box::new(value.serialize(&mut *self)?),
        );
        self.serialize_value(&out)?;
        Ok(out)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer::new(self))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.writer.write_all(&[TAG])?;
        self.serialize_size(variant_index as usize)?;
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self, None))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(MapSerializer::new(self, Some(ObjectKind::String)))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.writer.write_all(&[TAG])?;
        self.serialize_size(variant_index as usize)?;
        self.serialize_struct("", len)
    }
}

/// Serializes the `value` into the `writer`.
///
/// The returned value is the intermediate value that was serialized. It can be ignored.
pub fn to_writer(writer: impl Write, value: &impl serde::Serialize) -> Result<Value, Error> {
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)
}
