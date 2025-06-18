use super::{MapSerializer, Serializer};
use crate::{error::Error, headers::ArrayKind};
use serde::{
    Serialize,
    ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant},
};
use std::io::Write;

pub struct SeqSerializer<'a, 'ser, W: Write> {
    serializer: &'a mut Serializer<'ser, W>,
    kind: Option<ArrayKind>,
    len: usize,
    hold: (usize, u8),
}

impl<'a, 'ser, W: Write> SeqSerializer<'a, 'ser, W> {
    pub fn new(serializer: &'a mut Serializer<'ser, W>, len: usize) -> Self {
        Self {
            serializer,
            kind: None,
            len,
            hold: (0, 0),
        }
    }

    pub(self) fn set_kind(&mut self, kind: ArrayKind) -> Result<(), Error> {
        self.kind = Some(kind);
        self.serializer.writer.write_all(&[kind.header()])?;
        self.serializer.serialize_size(self.len)?;
        Ok(())
    }
}

impl<'a, 'ser, W: Write> SerializeSeq for SeqSerializer<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(ElementSerializer { seq: self })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.hold.1 % 8 != 0 {
            self.serializer.writer.write_all(&[self.hold.1])?;
        }

        Ok(())
    }
}

impl<'a, 'ser, W: Write> SerializeTuple for SeqSerializer<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(ElementSerializer { seq: self })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<'a, 'ser, W: Write> SerializeTupleStruct for SeqSerializer<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(ElementSerializer { seq: self })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<'a, 'ser, W: Write> SerializeTupleVariant for SeqSerializer<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(ElementSerializer { seq: self })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

struct ElementSerializer<'a, 'b, 'ser, W: Write> {
    pub seq: &'a mut SeqSerializer<'b, 'ser, W>,
}

impl<'a, 'b, 'ser, W: Write> serde::Serializer for ElementSerializer<'a, 'b, 'ser, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, 'ser, W>;
    type SerializeTuple = SeqSerializer<'a, 'ser, W>;
    type SerializeTupleStruct = SeqSerializer<'a, 'ser, W>;
    type SerializeTupleVariant = SeqSerializer<'a, 'ser, W>;
    type SerializeMap = MapSerializer<'a, 'ser, W>;
    type SerializeStruct = MapSerializer<'a, 'ser, W>;
    type SerializeStructVariant = MapSerializer<'a, 'ser, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Boolean)?,
            Some(ArrayKind::Boolean) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Boolean,
                    found,
                });
            }
        }

        self.seq.hold.1 |= v as u8;
        self.seq.hold.1 <<= 1;
        self.seq.hold.0 += 1;

        if self.seq.hold.0 % 8 == 0 {
            self.seq.serializer.writer.write_all(&[self.seq.hold.1])?;
            self.seq.hold.1 = 0;
        }

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::I8)?,
            Some(ArrayKind::I8) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::I8,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::I16)?,
            Some(ArrayKind::I16) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::I16,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::I32)?,
            Some(ArrayKind::I32) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::I32,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::I64)?,
            Some(ArrayKind::I64) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::I64,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::I128)?,
            Some(ArrayKind::I128) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::I128,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::U8)?,
            Some(ArrayKind::U8) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::U8,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::U16)?,
            Some(ArrayKind::U16) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::U16,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::U32)?,
            Some(ArrayKind::U32) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::U32,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::U64)?,
            Some(ArrayKind::U64) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::U64,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::U128)?,
            Some(ArrayKind::U128) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::U128,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::F32)?,
            Some(ArrayKind::F32) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::F32,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::F64)?,
            Some(ArrayKind::F64) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::F64,
                    found,
                });
            }
        }

        self.seq.serializer.writer.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<(), Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::String)?,
            Some(ArrayKind::String) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::String,
                    found,
                });
            }
        }
        self.seq.serializer.serialize_str_value(v.bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<(), Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq
            .serializer
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq
            .serializer
            .serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq
            .serializer
            .serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_map(len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq.serializer.serialize_struct(name, len)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        match self.seq.kind {
            None => self.seq.set_kind(ArrayKind::Generic)?,
            Some(ArrayKind::Generic) => {}
            Some(found) => {
                return Err(Error::MismatchedElementType {
                    expected: ArrayKind::Generic,
                    found,
                });
            }
        }

        self.seq
            .serializer
            .serialize_struct_variant(name, variant_index, variant, len)
    }
}
