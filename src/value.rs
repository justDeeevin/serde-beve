use crate::headers::*;

#[derive(Debug, PartialEq)]
/// An intermediate representation of values used during serialization.
///
/// This is a nearly 1:1 mapping to the `HEADER | VALUE` format in which BEVE values are encoded.
///
/// Objects are represented as a vector of key-value pairs, and strings are represented as their
/// bytes.
pub enum Value {
    Null,
    True,
    False,
    #[cfg(feature = "half")]
    /// <div class="warning">
    /// When the <code>half</code> feature is disabled, this variant will carry no payload.
    /// </div>
    BF16(half::bf16),
    #[cfg(not(feature = "half"))]
    BF16,
    #[cfg(feature = "half")]
    /// <div class="warning">
    /// When the <code>half</code> feature is disabled, this variant will carry no payload.
    /// </div>
    F16(half::f16),
    #[cfg(not(feature = "half"))]
    F16,
    F32(f32),
    F64(f64),
    F128,

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),

    String(Vec<u8>),

    StringObject(Vec<(Vec<u8>, Value)>),

    I8Object(Vec<(i8, Value)>),
    I16Object(Vec<(i16, Value)>),
    I32Object(Vec<(i32, Value)>),
    I64Object(Vec<(i64, Value)>),
    I128Object(Vec<(i128, Value)>),

    U8Object(Vec<(u8, Value)>),
    U16Object(Vec<(u16, Value)>),
    U32Object(Vec<(u32, Value)>),
    U64Object(Vec<(u64, Value)>),
    U128Object(Vec<(u128, Value)>),

    #[cfg(feature = "half")]
    BF16Array(Vec<half::bf16>),
    #[cfg(not(feature = "half"))]
    BF16Array,
    #[cfg(feature = "half")]
    F16Array(Vec<half::f16>),
    #[cfg(not(feature = "half"))]
    F16Array,
    F32Array(Vec<f32>),
    F64Array(Vec<f64>),
    F128Array,

    I8Array(Vec<i8>),
    I16Array(Vec<i16>),
    I32Array(Vec<i32>),
    I64Array(Vec<i64>),
    I128Array(Vec<i128>),

    U8Array(Vec<u8>),
    U16Array(Vec<u16>),
    U32Array(Vec<u32>),
    U64Array(Vec<u64>),
    U128Array(Vec<u128>),

    BoolArray(usize, Vec<u8>),
    StringArray(Vec<Vec<u8>>),
    GenericArray(Vec<Value>),

    Delimiter,
    // Box for indirection
    // I hate it here
    Tag(usize, Box<Value>),
    // TODO
    Matrix,
    Complex,
    Reserved,
}

impl Value {
    pub fn header(&self) -> u8 {
        match self {
            Self::Null => NULL,
            Self::True => TRUE,
            Self::False => FALSE,
            #[cfg(feature = "half")]
            Self::BF16(..) => BF16,
            #[cfg(not(feature = "half"))]
            Self::BF16 => BF16,
            #[cfg(feature = "half")]
            Self::F16(..) => F16,
            #[cfg(not(feature = "half"))]
            Self::F16 => F16,
            Self::F32(..) => F32,
            Self::F64(..) => F64,
            Self::F128 => F128,

            Self::I8(..) => I8,
            Self::I16(..) => I16,
            Self::I32(..) => I32,
            Self::I64(..) => I64,
            Self::I128(..) => I128,

            Self::U8(..) => U8,
            Self::U16(..) => U16,
            Self::U32(..) => U32,
            Self::U64(..) => U64,
            Self::U128(..) => U128,

            Self::String(..) => STRING,

            Self::StringObject(..) => STRING_OBJECT,

            Self::I8Object(..) => I8_OBJECT,
            Self::I16Object(..) => I16_OBJECT,
            Self::I32Object(..) => I32_OBJECT,
            Self::I64Object(..) => I64_OBJECT,
            Self::I128Object(..) => I128_OBJECT,

            Self::U8Object(..) => U8_OBJECT,
            Self::U16Object(..) => U16_OBJECT,
            Self::U32Object(..) => U32_OBJECT,
            Self::U64Object(..) => U64_OBJECT,
            Self::U128Object(..) => U128_OBJECT,

            #[cfg(feature = "half")]
            Self::BF16Array(..) => BF16_ARRAY,
            #[cfg(not(feature = "half"))]
            Self::BF16Array => BF16_ARRAY,
            #[cfg(feature = "half")]
            Self::F16Array(..) => F16_ARRAY,
            #[cfg(not(feature = "half"))]
            Self::F16Array => F16_ARRAY,
            Self::F32Array(..) => F32_ARRAY,
            Self::F64Array(..) => F64_ARRAY,
            Self::F128Array => F128_ARRAY,

            Self::I8Array(..) => I8_ARRAY,
            Self::I16Array(..) => I16_ARRAY,
            Self::I32Array(..) => I32_ARRAY,
            Self::I64Array(..) => I64_ARRAY,
            Self::I128Array(..) => I128_ARRAY,

            Self::U8Array(..) => U8_ARRAY,
            Self::U16Array(..) => U16_ARRAY,
            Self::U32Array(..) => U32_ARRAY,
            Self::U64Array(..) => U64_ARRAY,
            Self::U128Array(..) => U128_ARRAY,

            Self::BoolArray(..) => BOOL_ARRAY,
            Self::StringArray(..) => STRING_ARRAY,
            Self::GenericArray(..) => GENERIC_ARRAY,

            Self::Delimiter => DELIMITER,
            Self::Tag(..) => TAG,
            Self::Matrix => MATRIX,
            Self::Complex => COMPLEX,
            Self::Reserved => RESERVED,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", header_name(self.header()))
    }
}
